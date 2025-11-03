use std::vec;

use alloy::primitives::{Address, U256};
use evm_core::Evm;
use primitives::{
    evm_types::{BlockEnv, EvmStorage, Transaction},
    memory::Memory,
    stack::Stack,
};

fn main() {
    // for contract deployment
    //
    // This example constructs a tiny EVM bytecode sequence (raw bytes) in `call_data`
    // that demonstrates:
    //   1) pushing two small immediates (6 and 7) onto the stack using PUSH1 (0x60),
    //   2) adding them to produce 13 (0x0d),
    //   3) storing the 32-byte word containing that value at memory offset 0 using MSTORE (0x52),
    //   4) loading the stored word back with MLOAD (0x51),
    //   5) and halting with STOP (0x00).
    //
    // Byte-level breakdown (human-readable):
    //   0x60 0x06    -> PUSH1 0x06      ; push the constant 6 onto the stack
    //   0x60 0x07    -> PUSH1 0x07      ; push the constant 7 onto the stack
    //   0x01         -> ADD             ; pop 7 and 6, push (6 + 7) = 13
    //   0x60 0x00    -> PUSH1 0x00      ; push memory offset 0 onto the stack (offset where we'll store)
    //   0x52         -> MSTORE          ; store 32-byte word (value) at memory[offset]
    //                    // MSTORE consumes (offset, value) from the stack (see NOTE below)
    //   0x60 0x00    -> PUSH1 0x00      ; push memory offset 0 to read back the stored value
    //   0x51         -> MLOAD           ; load 32-byte word from memory[offset] and push it on the stack
    //   0x00         -> STOP            ; halt execution
    //
    // NOTE about stack order and MSTORE:
    //   EVM op semantics require careful ordering so MSTORE sees the expected items on the stack.
    //   - After ADD the top of stack is the numeric result (13).
    //   - We then PUSH1 0x00 (the offset). At that point the stack (top -> bottom) is: [offset=0, value=13].
    //   - MSTORE will pop offset and value and write value at memory[offset].
    //   Different implementations or interpretations may describe pop order differently; this code
    //   arranges the pushes so the MSTORE call receives the correct pair for this VM's handlers.
    //
    // Raw bytes (literal sequence used as the transaction data / init code):
    //   [0x60,0x06, 0x60,0x07, 0x01, 0x60,0x00, 0x52, 0x60,0x00, 0x51, 0x00]
    //
    // We put these bytes into `call_data` and also into `tx.data` so the `Evm` instance has the code
    // available in the transaction payload. If you want to treat this as deployed contract code, you
    // would instead write it to a storage account's `code` and set `tx.to` accordingly.
    let call_data: Vec<u8> = vec![
        0x60, 0x06, // PUSH1 0x06  -> push 6
        0x60, 0x07, // PUSH1 0x07  -> push 7
        0x01, // ADD         -> pop 7,6 push 13
        0x60, 0x00, // PUSH1 0x00  -> push memory offset 0
        0x52, // MSTORE      -> store 32-byte word at memory[offset]
        0x60, 0x00, // PUSH1 0x00  -> push memory offset 0 (to read back)
        0x51, // MLOAD       -> load 32-byte word from memory[offset]
        0x00, // STOP        -> halt
    ];

    // Block environment and memory initialization
    let block_env = BlockEnv::default();
    // initialize memory with 1 KiB (1024 bytes) so it has a default size before growth
    let memory: Memory = Memory::new_with_data(vec![0u8; 1024]);
    let stack = Stack::default();
    let storage = EvmStorage::default();

    // Transaction: put our bytecode into `data` so the EVM can load/process it.
    // If you want this to behave like contract creation code, you can set `tx.to` to Address::ZERO
    // (already the case here) and the VM code that seeds memory from `tx.data` will place these
    // bytes into memory for execution.
    let tx: Transaction = Transaction {
        from: Address::from_slice(&[1]),
        to: Address::ZERO,
        value: U256::ZERO,
        nonce: U256::ZERO,
        data: call_data.clone(), // transaction payload contains our raw opcodes
        gas_limit: U256::from(100000),
    };

    // Create the EVM instance with the prepared environment and transaction.
    let mut evm = Evm::new(block_env, tx, memory, stack, storage);

    // NOTE: this example only constructs the EVM state and populates tx.data.
    // To actually execute the bytecode you need to call `evm.execute()` or `evm.run()`
    // depending on how you want to drive execution. Execution will depend on how the
    // VM is implemented to load the tx.data into memory and interpret it.
    //
    // Example: if the VM expects contract code in memory when tx.to == Address::ZERO,
    // calling `evm.execute()` (or `evm.run()`) should iterate through the opcodes and
    // perform the pushes, arithmetic, MSTORE/MLOAD, and STOP as described above.
    //
    // You can print the transaction data for verification:
    // println!(\"tx.data = {:x?}\", evm.tx.data);

    // println!(\"Hello, world!\");
}
