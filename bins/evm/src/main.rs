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
    let call_data = vec!["0x60", "0x06", "0x60", "0x00", "0x60", "0x0"];
    let block_env = BlockEnv::default();
    // let memory = Memory::default();
    // initialize memory with 1 KiB (1024 bytes) so it has a default size before growth
    let memory: Memory = Memory::new_with_data(vec![0u8; 1024]);
    let stack = Stack::default();
    let storage = EvmStorage::default();

    let tx: Transaction = Transaction{
        from: Address::from_slice(&[1]),
        to: Address::ZERO,
        value: U256::ZERO,
        nonce: U256::ZERO,
        data: Vec::new(),
        gas_limit: U256::from(100000),
    };

    let mut evm = Evm::new(block_env, tx, memory, stack, storage);
    // println!("Hello, world!");
}
