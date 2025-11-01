use std::collections::HashMap;

use alloy::primitives::{Address, U256};

#[derive(Debug, Clone, Default)]
pub struct Transaction {
    pub from: Address,
    pub to: Address,
    pub value: U256,
    pub nonce: U256,
    pub data: Vec<u8>,
    pub gas_limit: U256,
}

#[derive(Debug, Clone, Default)]
pub struct BlockEnv {
    pub number: U256,
    pub timestamp: U256,
    pub coinbase: Address,
    pub difficulty: U256,
    pub gas_limit: U256,
    pub base_fee: U256,
    pub block_hash: U256,
    pub chain_id: U256,
}

#[derive(Debug, Clone, Default)]
pub struct EvmAccount {
    pub balance: U256,
    pub nonce: U256,
    pub code: Vec<u8>,
}

#[derive(Debug, Clone, Default)]
pub struct EvmStorage {
    pub data: HashMap<Address, EvmAccount>,
}

