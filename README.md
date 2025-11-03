evm/README.md

# eth_vm — Technical Reference

This repository implements a small, Rust-based Ethereum Virtual Machine (EVM) prototype. It provides:

- a minimal `Evm` execution context,
- an `Opcode` enumeration (canonical bytes -> names),
- a 256-entry jump table which dispatches bytes to function handlers,
- a set of operation implementations in `operations/ariths.rs`,
- primitive runtime structures (memory, stack, storage, transaction and block env).

This README is a developer-focused reference describing the code layout, the real "goto" dispatch mapping (jump-table), how the opcode enumeration is organized, where the handlers live, and practical notes for extending the implementation. For behavioral details (stack effects, gas, edge cases) use the canonical documentation at https://www.evm.codes/.

Repository layout (relevant files and modules)

- `crates/evm_core` — core EVM logic, jump table, opcodes, operation implementations.
- `crates/primitives` — runtime building blocks: `Memory`, `Stack`, `EvmStorage`, `Transaction`, `BlockEnv`.
- `bins/evm` — minimal runner.

Key files (open these while reading this doc)

- `crates/evm_core/src/opcodes.rs` — opcode enum and conversion from bytes

```crates/evm_core/src/opcodes.rs#L1-220
pub enum Opcode {
    STOP = 0x00,
    ADD = 0x01,
    MUL = 0x02,
    SUB = 0x03,
    DIV = 0x04,
    SDIV = 0x05,
    MOD = 0x06,
    SMOD = 0x07,
    ADDMOD = 0x08,
    MULMOD = 0x09,
    EXP = 0x0A,
    SIGNEXTEND = 0x0B,

    LT = 0x10,
    GT = 0x11,
    SLT = 0x12,
    SGT = 0x13,
    EQ = 0x14,
    ISZERO = 0x15,
    AND = 0x16,
    OR = 0x17,
    XOR = 0x18,
    NOT = 0x19,
    BYTE = 0x1A,
    SHL = 0x1B,
    SHR = 0x1C,
    SAR = 0x1D,

    KECCAK256 = 0x20,

    ADDRESS = 0x30,
    BALANCE = 0x31,
    ORIGIN = 0x32,
    CALLER = 0x33,
    CALLVALUE = 0x34,
    CALLDATALOAD = 0x35,
    CALLDATASIZE = 0x36,
    CALLDATACOPY = 0x37,
    CODESIZE = 0x38,
    CODECOPY = 0x39,
    GASPRICE = 0x3A,
    EXTCODESIZE = 0x3B,
    EXTCODECOPY = 0x3C,
    RETURNDATASIZE = 0x3D,
    RETURNDATACOPY = 0x3E,
    EXTCODEHASH = 0x3F,

    BLOCKHASH = 0x40,
    COINBASE = 0x41,
    TIMESTAMP = 0x42,
    NUMBER = 0x43,
    DIFFICULTY = 0x44,
    GASLIMIT = 0x45,
    CHAINID = 0x46,
    SELFBALANCE = 0x47,
    BASEFEE = 0x48,
    BLOBHASH = 0x49,
    BLOBBASEFEE = 0x4A,

    POP = 0x50,
    MLOAD = 0x51,
    MSTORE = 0x52,
    MSTORE8 = 0x53,
    SLOAD = 0x54,
    SSTORE = 0x55,
    JUMP = 0x56,
    JUMPI = 0x57,
    PC = 0x58,
    MSIZE = 0x59,
    GAS = 0x5A,
    JUMPDEST = 0x5B,
    TLOAD = 0x5C,
    TSTORE = 0x5D,
    MCOPY = 0x5E,

    PUSH0 = 0x5F,
    PUSH1 = 0x60,
    PUSH2 = 0x61,
    PUSH3 = 0x62,
    PUSH4 = 0x63,
    PUSH5 = 0x64,
    PUSH6 = 0x65,
    PUSH7 = 0x66,
    PUSH8 = 0x67,
    PUSH9 = 0x68,
    PUSH10 = 0x69,
    PUSH11 = 0x6A,
    PUSH12 = 0x6B,
    PUSH13 = 0x6C,
    PUSH14 = 0x6D,
    PUSH15 = 0x6E,
    PUSH16 = 0x6F,
    PUSH17 = 0x70,
    PUSH18 = 0x71,
    PUSH19 = 0x72,
    PUSH20 = 0x73,
    PUSH21 = 0x74,
    PUSH22 = 0x75,
    PUSH23 = 0x76,
    PUSH24 = 0x77,
    PUSH25 = 0x78,
    PUSH26 = 0x79,
    PUSH27 = 0x7A,
    PUSH28 = 0x7B,
    PUSH29 = 0x7C,
    PUSH30 = 0x7D,
    PUSH31 = 0x7E,
    PUSH32 = 0x7F,

    DUP1 = 0x80,
    DUP2 = 0x81,
    DUP3 = 0x82,
    DUP4 = 0x83,
    DUP5 = 0x84,
    DUP6 = 0x85,
    DUP7 = 0x86,
    DUP8 = 0x87,
    DUP9 = 0x88,
    DUP10 = 0x89,
    DUP11 = 0x8A,
    DUP12 = 0x8B,
    DUP13 = 0x8C,
    DUP14 = 0x8D,
    DUP15 = 0x8E,
    DUP16 = 0x8F,

    SWAP1 = 0x90,
    SWAP2 = 0x91,
    SWAP3 = 0x92,
    SWAP4 = 0x93,
    SWAP5 = 0x94,
    SWAP6 = 0x95,
    SWAP7 = 0x96,
    SWAP8 = 0x97,
    SWAP9 = 0x98,
    SWAP10 = 0x99,
    SWAP11 = 0x9A,
    SWAP12 = 0x9B,
    SWAP13 = 0x9C,
    SWAP14 = 0x9D,
    SWAP15 = 0x9E,
    SWAP16 = 0x9F,

    LOG0 = 0xA0,
    LOG1 = 0xA1,
    LOG2 = 0xA2,
    LOG3 = 0xA3,
    LOG4 = 0xA4,

    DATALOAD = 0xD0,
    DATALOADN = 0xD1,
    DATASIZE = 0xD2,
    DATACOPY = 0xD3,

    RJUMP = 0xE0,
    RJUMPI = 0xE1,
    RJUMPV = 0xE2,
    CALLF = 0xE3,
    RETF = 0xE4,
    JUMPF = 0xE5,
    DUPN = 0xE6,
    SWAPN = 0xE7,
    EXCHANGE = 0xE8,

    EOFCREATE = 0xEC,
    RETURNCONTRACT = 0xEE,

    CREATE = 0xF0,
    CALL = 0xF1,
    CALLCODE = 0xF2,
    RETURN = 0xF3,
    DELEGATECALL = 0xF4,
    CREATE2 = 0xF5,

    RETURNDATALOAD = 0xF7,
    EXTCALL = 0xF8,
    EXTDELEGATECALL = 0xF9,
    STATICCALL = 0xFA,
    EXTSTATICCALL = 0xFB,

    REVERT = 0xFD,
    INVALID = 0xFE,
    SELFDESTRUCT = 0xFF,
}
```

- `crates/evm_core/src/jump_tables.rs` — builds the dispatch table (the actual "goto" mapping).

```crates/evm_core/src/jump_tables.rs#L1-80
use crate::{Evm, opcodes::Opcode, operations::ariths::*};

pub type OpcodeFn = fn(&mut Evm);

pub fn noop(_evm: &mut Evm) {}

pub fn build_jump_table() -> [OpcodeFn; 256] {
    let mut jump_table: [fn(&mut Evm); 256] = [noop as OpcodeFn; 256];
    jump_table[Opcode::STOP as usize] = stop;
    jump_table[Opcode::ADD as usize] = add;
    jump_table[Opcode::SUB as usize] = sub;
    jump_table[Opcode::MUL as usize] = mul;
    jump_table[Opcode::DIV as usize] = div;
    jump_table[Opcode::SDIV as usize] = sdiv;
    jump_table[Opcode::SMOD as usize] = smod;
    jump_table[Opcode::MOD as usize] = modulo;
    // jump_table[Opcode::MLOAD as usize] = m_load;
    // jump_table[Opcode::CHAINID as usize] = chain_id;
    // jump_table[Opcode::COINBASE as usize] = coin_base;


    jump_table
}
```

- `crates/evm_core/src/operations/ariths.rs` — implementations for several arithmetic and environment-facing opcodes.

```crates/evm_core/src/operations/ariths.rs#L1-220
use alloy::primitives::{Address, I256, U64, U256};

use crate::{Evm, ProgramExitStatus};

// ref == https://www.evm.codes/
pub fn stop(evm: &mut Evm) {
    evm.status = ProgramExitStatus::Success;
}

pub fn add(evm: &mut Evm) {
    let a = evm.stack.pop().unwrap();
    let b = evm.stack.pop().unwrap();
    evm.stack.push(a + b).unwrap();
}

pub fn sub(evm: &mut Evm) {
    let a = evm.stack.pop().unwrap();
    let b = evm.stack.pop().unwrap();
    evm.stack.push(a - b).unwrap();
}

pub fn mul(evm: &mut Evm) {
    let a = evm.stack.pop().unwrap();
    let b = evm.stack.pop().unwrap();
    evm.stack.push(a * b).unwrap();
}

pub fn div(evm: &mut Evm) {
    let a = evm.stack.pop().unwrap();
    let b = evm.stack.pop().unwrap();
    if b == U256::ZERO {
        evm.stack.push(U256::ZERO).unwrap();
    } else {
        evm.stack.push(a / b).unwrap();
    }
}

pub fn sdiv(evm: &mut Evm) {
    let a = evm.stack.pop().unwrap();
    let b = evm.stack.pop().unwrap();

    let a_int = I256::from_limbs(*a.as_limbs());
    let b_int = I256::from_limbs(*b.as_limbs());

    if b_int == I256::ZERO {
        evm.stack.push(U256::ZERO).unwrap();
    } else {
        let result = a_int / b_int;
        let result_unsigned = U256::from_limbs(*result.as_limbs());
        evm.stack.push(result_unsigned).unwrap();
    }
}
...
pub fn push_0(evm: &mut Evm) {
    evm.stack.push(U256::ZERO).unwrap();
}
```

- `crates/evm_core/src/lib.rs` — `Evm` structure and fetch-decode-dispatch loop.

```crates/evm_core/src/lib.rs#L1-220
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

#[derive(Debug, Clone, Default, PartialEq)]
pub enum ProgramExitStatus {
    Success,
    Failure,
    #[default]
    Default,
}

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

    pub fn execute(&mut self) {
        if self.tx.to == Address::ZERO && !self.stack.data.is_empty() {
            for (i, value) in self.tx.data.iter().enumerate() {
                // Process each value in the stack
                println!("Value at index {}: {}", i, value);
                self.memory.store_byte(i, *value);
            }
        } else if self.tx.to != Address::ZERO {
            let touched_contract: Address = self.tx.to;
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

    pub fn step(&mut self) {
        let raw_instruction = self.memory.load_byte(self.pc);
        let instruction: Opcode = Opcode::from_u8(raw_instruction).unwrap();

        let jump_tables: [fn(&mut Evm); 256] = build_jump_table();
        jump_tables[instruction as usize](self);
        self.pc += 1;
    }

    pub fn run(&mut self) {
        while self.status == ProgramExitStatus::default() {
            self.step();
        }
    }
}
```

Dispatch / "goto" behavior (the actual runtime mapping)

- Dispatch kernel is `Evm::step`:
  - read instruction byte at `memory[pc]` (`Memory::load_byte`),
  - convert to `Opcode` via `Opcode::from_u8`,
  - build the jump table (`build_jump_table()`),
  - call the function pointer in `jump_table[instruction as usize]`,
  - increment `pc`.

- The jump-table is a 256-element array of `fn(&mut Evm)` that defaults to `noop`.
  - Currently the jump-table explicitly assigns handlers for the following opcodes:
    - `0x00` (STOP) -> `operations::ariths::stop`
    - `0x01` (ADD) -> `operations::ariths::add`
    - `0x03` (SUB) -> `operations::ariths::sub`
    - `0x02` (MUL) -> `operations::ariths::mul`
    - `0x04` (DIV) -> `operations::ariths::div`
    - `0x05` (SDIV) -> `operations::ariths::sdiv`
    - `0x07` (SMOD) -> `operations::ariths::smod`
    - `0x06` (MOD) -> `operations::ariths::modulo`
  - All other opcodes fall back to `noop` until added.

Canonical opcode reference

- This project defines the opcode set in `crates/evm_core/src/opcodes.rs`. Use https://www.evm.codes/ as the authoritative specification for:
  - opcode semantics,
  - stack arguments and results,
  - gas costs,
  - valid/invalid behavior (e.g., DIV by zero semantics, SSTORE changes, etc).

How to extend / add a new opcode (summary)

1. Add the opcode constant to `opcodes.rs` (if not present already).
   - The enum maps byte value to variant; `Opcode::from_u8` must map the byte as well (add a match arm).
2. Implement the handler function in `crates/evm_core/src/operations/*`.
   - Signature: `pub fn handler(evm: &mut Evm) { ... }`.
   - Use primitives (`evm.stack`, `evm.memory`, `evm.storage`, `evm.block_env`, `evm.tx`) for operations.
3. Wire the handler in `crates/evm_core/src/jump_tables.rs` inside `build_jump_table()`:
   - `jump_table[Opcode::MYOP as usize] = my_handler;`
4. Add tests that exercise correct stack effects, memory & storage mutations, and any edge-case behaviors.
5. Consider gas accounting and invalid opcode behavior — this implementation currently does not track gas.

Important implementation details & current limitations (things to know)

- `jump_tables::build_jump_table()` creates a new jump table on every `step()` call. For performance you may want to make the jump table a static once-initialized array. Right now the mapping is rebuilt each instruction.
- Many functions use `.unwrap()` on `stack.pop()` and other lookups. This will panic when the stack is empty or maps are missing. Production implementations should return proper trap/error codes and handle stack underflow gracefully.
- `Memory` is a simple `Vec<u8>` and uses direct slices. `store_word`, `load_word`, and `store_byte` assume the vector has sufficient length; currently there is no automatic growth logic. You must call `memory.data.resize(...)` or implement memory grow behavior to avoid panics.
  - See `crates/primitives/src/memory.rs` for current methods:

```crates/primitives/src/memory.rs#L1-160
//!

use alloy::primitives::U256;

#[derive(Default, Debug, Clone)]
pub struct Memory {
    // Fields
    pub data: Vec<u8>,
}

impl Memory {
    pub fn new() -> Self {
        Memory { data: Vec::new() }
    }

    pub fn new_with_data(data: Vec<u8>) -> Self {
        Memory { data }
    }

    pub fn store_word(&mut self, offset: usize, word: U256) {
        let word_to_bytes: [u8; 32] = word.to_be_bytes::<32>();
        self.data[offset..offset + 32].copy_from_slice(&word_to_bytes);
    }

    pub fn load_word(&self, offset: usize) -> U256 {
         let bytes = &self.data[offset..offset + 32];

         U256::from_be_slice(bytes.try_into().unwrap())
    }

    pub fn store_byte(&mut self, offset: usize, byte: u8) {
        self.data[offset] = byte;
    }

    pub fn load_byte(&self, offset: usize) -> u8 {
        self.data[offset]
    }

    pub fn copy(&mut self, offset: usize, dest: usize, length: usize) -> u8 {
        let data = &self.data[offset..offset + length];
        0
    }
}
```

- `Stack` enforces a maximum depth (1024). Pushing beyond that returns `EvmErrors::StackTooDeep`. See `crates/primitives/src/stack.rs`.

```crates/primitives/src/stack.rs#L1-120
use crate::errors::EvmErrors;
use alloy::primitives::U256;

#[derive(Debug, Clone, Default)]
pub struct Stack {
    pub data: Vec<U256>,
}

impl Stack {
    /// Push a value onto the stack.
    /// Returns `Err(EvmErrors::StackTooDeep)` if the stack would exceed 1024 items.
    pub fn push(&mut self, value: U256) -> Result<(), EvmErrors> {
        if self.data.len() >= 1024 {
            return Err(EvmErrors::StackTooDeep);
        }
        self.data.push(value);
        Ok(())
    }

    /// Pop a value from the stack. Returns `None` if the stack is empty.
    pub fn pop(&mut self) -> Option<U256> {
        self.data.pop()
    }

    /// Return current stack size. This is useful for testing and diagnostics.
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Return whether the stack is empty.
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}
```

Notes about specific opcodes currently implemented in handlers

- Arithmetic & logical: `ADD`, `SUB`, `MUL`, `DIV`, `SDIV`, `MOD`, `SMOD`, `ADDMOD`, `MULMOD`, `EXP`, `SIGNEXTEND`, `LT`, `GT`, `SLT`, `SGT`, `EQ`, `ISZERO`, `AND`, `XOR`, `NOT`, `BYTE`, `SHL`, `SHR`, `SAR` — many implementations exist in `ariths.rs`. But not all are wired into the jump table yet.
- Environment & blockchain: `ADDRESS`, `BALANCE`, `ORIGIN`, `CALLER`, `CALLVALUE`, `CALLDATALOAD`, `GASPRICE`, `BLOCKHASH`, `COINBASE`, `TIMESTAMP`, `NUMBER`, `GASLIMIT`, `CHAINID` — implementations present for reading environment fields but beware: some functions assume values exist (e.g., `storage.data.get(&address).unwrap()`).
- Stack / Memory / Storage ops: `POP`, `MLOAD`, `MSTORE`, `MSTORE8`, `SLOAD`, `SSTORE` — implementations present but not all wired into jump table.
- Control flow: `JUMP`, `JUMPI`, `JUMPDEST` — present as simple implementations (e.g., `jump` sets `evm.pc = target`), but there is no full validity check (JUMPDEST must be validated in canonical EVM; this implementation currently does not validate whether target is a `JUMPDEST`).
- Complex/extended opcodes (`CALL`, `CREATE`, `STATICCALL`, etc.) are enumerated but not implemented.

Practical "goto" mapping cheat-sheet

- Where is the runtime goto performed? `Evm::step`:
  1. `let raw_instruction = self.memory.load_byte(self.pc);`
  2. `let instruction: Opcode = Opcode::from_u8(raw_instruction).unwrap();`
  3. `let jump_tables: [fn(&mut Evm); 256] = build_jump_table();`
  4. `jump_tables[instruction as usize](self);`
  5. `self.pc += 1;`
- Where to add a new mapping (example `MYOP`):
  - Add enum value to `opcodes.rs` and arm to `from_u8`.
  - Implement `pub fn myop(evm: &mut Evm) { ... }` in `operations/*`.
  - Add `jump_table[Opcode::MYOP as usize] = myop;` in `build_jump_table()`.

Security / correctness considerations

- Replace `unwrap()` usage with error handling to avoid panics in malformed inputs (stack underflow, missing storage entries, memory OOB).
- Add memory growth semantics: EVM memory grows in 32-byte words and gas must be accounted for.
- Implement full JUMPDEST validation to prevent jumping to arbitrary bytes.
- Implement gas metering per opcode to allow early halting (out-of-gas) and resource accounting.

Useful references

- Official opcode semantics and descriptions: https://www.evm.codes/ — use it as the primary reference for how each opcode should behave including gas, pop/push counts, and specific rules.
- Current project entrypoint (for experimentation): `bins/evm/src/main.rs`.

```bins/evm/src/main.rs#L1-80
use evm_core::Evm;
use primitives::{evm_types::{BlockEnv, EvmStorage, Transaction}, memory::Memory, stack::Stack};

fn main() {

    let block_env = BlockEnv::default();
    let memory = Memory::default();
    let stack = Stack::default();
    let storage = EvmStorage::default();

    let tx: Transaction = Transaction::default();


    let mut evm = Evm::new(block_env, tx, memory, stack, storage);
    // println!("Hello, world!");
}
```
