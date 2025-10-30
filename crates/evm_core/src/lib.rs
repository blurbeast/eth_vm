
pub mod opcodes;
pub mod jump_tables;

use alloy::primitives::Address;
use primitives::{evm_types::{ BlockEnv, EvmStorage, Transaction }, memory::Memory, stack::Stack};


#[derive(Debug, Clone, Default)]
pub enum ProgramExitStatus {
    Success,
    #[default]
    Failure,
}

#[derive(Debug, Clone, Default)]
pub struct Evm {
    pub block_env: BlockEnv,
    pub tx: Transaction,
    pub memory: Memory,
    pub stack: Stack,
    pub storage: EvmStorage,
    pub pc: usize,
    pub state: ProgramExitStatus,
}


impl Evm {
    pub fn new(block_env: BlockEnv, tx: Transaction, memory: Memory, stack: Stack, storage: EvmStorage,) -> Self {
        Evm {
            block_env,
            tx,
            memory,
            stack,
            storage,
            pc: 0,
            state: ProgramExitStatus::default(),
        }
    }
    
    pub fn execute(&mut self) {
        if self.tx.to == Address::ZERO && !self.stack.data.is_empty() {
            for (i, value) in self.tx.data.iter().enumerate() {
                // Process each value in the stack
                println!("Value at index {}: {}", i, value);
                self.memory.store_byte(i, *value);
            }
        }
        else if self.tx.to != Address::ZERO {
            let touched_contract: Address = self.tx.to;
            for (i, v) in self.storage.data.get(&touched_contract).unwrap().code.iter().enumerate() {
                self.memory.store_byte(i, *v);
            } 
        }
        else {}
    }
    
    pub fn step(&mut self) {
        let raw_instruction = self.memory.load_byte(self.pc);
        
    }
}