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
