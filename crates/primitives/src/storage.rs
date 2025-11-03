use std::collections::HashMap;

use alloy::primitives::{Address, U256};

use crate::evm_types::{EvmAccount, EvmStorage};

impl EvmStorage {
    pub fn default() -> Self {
        EvmStorage {
            data: HashMap::new(),
        }
    }

    pub fn s_load(&mut self, address: Address, key: U256) -> U256 {
        self.data
            .get(&address)
            .and_then(|evm_account: &EvmAccount| evm_account.word.get(&key).copied())
            .unwrap()
    }

    pub fn s_store(&mut self, address: Address, key: U256, value: U256) {
        self.data
            .entry(address)
            .or_insert_with(EvmAccount::default)
            .word
            .insert(key, value);
    }
}
