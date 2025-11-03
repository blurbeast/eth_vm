pub mod jump_tables;
pub mod opcodes;
pub mod operations {
    pub mod ariths;
}

use alloy::primitives::Address;
use primitives::{
    evm_types::{BlockEnv, EvmStorage, Transaction},
    memory::Memory,
    stack::Stack,
};

use crate::{jump_tables::build_jump_table, opcodes::Opcode};

/// Program exit states for the VM. The VM loop (`run`) uses this to determine when to stop.
/// - `Success` indicates the program stopped successfully (for example via `STOP` opcode).
/// - `Failure` indicates a trap/exception (e.g. invalid opcode, maybe an out-of-gas ).
/// - `Default` means "still running" or uninitialized status; the run loop continues while status is `Default`.
#[derive(Debug, Clone, Default, PartialEq)]
pub enum ProgramExitStatus {
    Success,
    Failure,
    #[default]
    Default,
}

/// The EVM runtime structure.
///
/// This struct aggregates all pieces of state the interpreter needs to execute bytecode:
/// - `block_env`: on-chain block/environment information used by environment opcodes (timestamp, number, coinbase,)
/// - `tx`: the transaction context (caller, callee, call value, calldata). This crate uses `tx.data` for code when
///   `tx.to == Address::ZERO` (contract creation / init-style behavior).
/// - `memory`: linear byte-addressable memory used by MSTORE/MLOAD and other memory ops.
/// - `stack`: the 1024-deep evaluation stack used by all stack-based opcodes.
/// - `storage`: persistent per-account contract storage accessible via SLOAD/SSTORE (map keyed by Address).
/// - `pc`: program counter (index into `memory` where current instruction is read).
/// - `status`: current program exit status (controls `run()` loop).
///
/// - `block_env: BlockEnv`
///     - Contains block-scoped values such as `number`, `timestamp`, `coinbase`, `gas_limit`, `base_fee`, `block_hash`, `chain_id`.
///     - Necessary for opcodes like `TIMESTAMP`, `NUMBER`, `COINBASE`, `GASLIMIT`, `CHAINID`, `BASEFEE` and `BLOCKHASH`.
///     - Example: `TIMESTAMP` returns `block_env.timestamp`. If you simulate a block at time `t`, set `block_env.timestamp = U256::from(t)`.
/// - `tx: Transaction`
///     - Transaction-level context: `from` (caller), `to` (destination), `value` (wei), `nonce`, `data` (calldata or init code), and `gas_limit`.
///     - Used by CALL* opcodes, `CALLVALUE`, `CALLER`, `CALLDATALOAD`, and for contract creation the `data` can be treated as creation code.
///     - Example: when testing a contract call that sends 1 ether, set `tx.value = U256::from(1_000_000_000_000_000_000u128)` and `tx.from` to the caller address.
/// - `memory: Memory`
///     - Linear, zero-indexed byte array that is transient during execution (not persisted between transactions).
///     - Used by `MSTORE`, `MLOAD`, `CALLDATACOPY`, `CODECOPY`, etc.
///     - Important: this implementation expects memory to have sufficient length before reads/writes.
///     - Example: to store a 32-byte word at offset 0 call `MSTORE` with offset `0` and the word; `memory.store_word(0, word)` writes 32 bytes starting at `memory.data[0]`.
/// - `stack: Stack`
///     - LIFO stack that holds 256-bit values (`U256`). EVM opcodes push/pop values here.
///     - Typical opcodes: `PUSH1..PUSH32` push values, arithmetic opcodes `ADD,SUB` pop operands and push results, `POP` discards top value.
///     - Example: after `PUSH1 0x05; PUSH1 0x03; ADD`, the top of the stack contains `0x08`.
/// - `storage: EvmStorage`
///     - Persistent mapping from account address -> account storage (account fields include `code`, `balance`, `word` map).
///     - Used by `SLOAD`/`SSTORE` to persist contract state across transactions. Must be keyed by the contract address that is being executed.
///     - Example: after `SSTORE` of key `k` to value `v` for contract address `A`, subsequent calls to the same contract can read it with `SLOAD` and get `v`.
/// - `pc: usize`
///     - Program counter (index into `memory.data` where the next opcode byte is read).
///     - `pc` must point at the first byte of an opcode. For `PUSHn` opcodes handlers must advance `pc` by the size of immediates they consumed (or set `pc` appropriately); the `step()` here increments by 1 after the handler by design so handlers that mutate `pc` should account for that.
/// - `status: ProgramExitStatus`
///     - Controls the `run()` loop. When a handler sets `status` to `Success` or `Failure`, `run()` will stop.
///
/// The EVM struct stores all of the runtime state required to fetch, decode, and execute opcodes.
#[derive(Debug, Clone, Default)]
pub struct Evm {
    pub block_env: BlockEnv,
    pub tx: Transaction,
    pub memory: Memory,
    pub stack: Stack,
    pub storage: EvmStorage,
    pub pc: usize,
    pub status: ProgramExitStatus,
}

impl Evm {
    /// Construct a new EVM instance.

    /// - `block_env`: pass the block environment you want opcodes to observe. For tests set values explicitly:
    ///     BlockEnv { number: U256::from(123), timestamp: U256::from(1_700_000_000), coinbase: addr, gas_limit: U256::from(30_000_000), ... }
    /// - `tx`: transaction payload. For contract creation put creation bytecode in `tx.data` and `tx.to = Address::ZERO`.
    /// - `memory`: linear memory buffer. It's acceptable to provide a pre-allocated buffer (e.g. 1 KiB) for convenience.
    /// - `stack`: initial stack - normally empty, but tests may pre-populate it for synthetic runs.
    /// - `storage`: the node's account storage map. Provide pre-existing accounts if needed (e.g. balances, code).
    ///
    /// Example usage:
    /// ```
    /// let evm = Evm::new(block_env, tx, Memory::new_with_data(vec![0u8;1024]), Stack::default(), EvmStorage::default());
    /// ```
    pub fn new(
        block_env: BlockEnv,
        tx: Transaction,
        memory: Memory,
        stack: Stack,
        storage: EvmStorage,
    ) -> Self {
        Evm {
            block_env,
            tx,
            memory,
            stack,
            storage,
            pc: 0,
            status: ProgramExitStatus::default(),
        }
    }

    /// What this `execute()` currently does (implementation-specific):
    /// - If `tx.to == Address::ZERO` (contract creation / deployment), it attempts to copy `tx.data` into memory so the init code is available for execution.
    /// - If `tx.to != Address::ZERO`, it attempts to load the touched contract's `code` from `storage` into memory.

    /// - The current `execute()` uses `self.stack.data.is_empty()` NB: this does not reflect real-world checks.
    /// - This Implementation uses the unwrap which means errors are not handled.
    pub fn execute(&mut self) {
        // If transaction is a contract creation (to == ZERO), copy tx.data into memory as initial code.
        if self.tx.to == Address::ZERO && !self.stack.data.is_empty() {
            for (i, value) in self.tx.data.iter().enumerate() {
                // Writing each byte of tx.data into memory at its corresponding offset.
                println!("Value at index {}: {}", i, value);
                self.memory.store_byte(i, *value);
            }
        } else if self.tx.to != Address::ZERO {
            // If tx.to is set, we are calling an existing contract: load its code into memory.
            let touched_contract: Address = self.tx.to;
            // The code is expected to be found in storage.data[address].code
            // NOTE: .unwrap() will panic if address not present; not production frienly
            for (i, v) in self
                .storage
                .data
                .get(&touched_contract)
                .unwrap()
                .code
                .iter()
                .enumerate()
            {
                self.memory.store_byte(i, *v);
            }
        } else {
        }
    }

    /// Execute a single instruction at the current `pc`.
    ///
    /// 1. `raw_instruction = self.memory.load_byte(self.pc)`:
    ///    - The VM reads a single byte from linear `memory` at position `pc`.
    ///    - This byte is the opcode code (0x00..0xff). For example 0x60 is `PUSH1`.
    ///    - Ensure `memory` has been seeded with code (via `execute()` ) and `pc` points to the correct start.
    /// 2. `instruction = Opcode::from_u8(raw_instruction).unwrap()`:
    ///    - Convert the raw byte into the typed `Opcode` enum. If the byte is unknown, `from_u8` returns `None`.
    ///    - `unwrap()` will panic on undefined bytes — not production friendly,
    /// 3. `let jump_tables = build_jump_table()`:
    ///    - Builds (currently on every `step`) a 256-entry table that maps opcode numeric values to handler functions (`fn(&mut Evm)`).
    /// 4. `jump_tables[instruction as usize](self)`:
    ///    - Call the handler function for the current opcode. Handlers mutate `stack`, `memory`, `pc`, `status`, and other parts of the EVM as needed.
    ///    - Handlers that consume immediate bytes (e.g., `PUSH1..PUSH32`)

    pub fn step(&mut self) {
        // Fetch the byte at the program counter from memory.
        let raw_instruction = self.memory.load_byte(self.pc);

        // Decode: map the raw byte into a strongly typed Opcode enum.
        let instruction: Opcode = Opcode::from_u8(raw_instruction).unwrap();

        // Build dispatch table and call the handler for the decoded instruction.
        // Note: building the table on every step is simple but inefficient; use a cached static table for performance.
        let jump_tables: [fn(&mut Evm); 256] = build_jump_table();
        jump_tables[instruction as usize](self);

        // self.pc += 1; // to be handled in the opcode handler
    }

    pub fn run(&mut self) {
        while self.status == ProgramExitStatus::default() {
            self.step();
        }
    }
}
