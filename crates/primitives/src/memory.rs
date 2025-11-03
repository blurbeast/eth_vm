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
