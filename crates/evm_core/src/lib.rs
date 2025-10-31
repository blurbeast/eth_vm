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
