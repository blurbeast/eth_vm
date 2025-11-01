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

pub fn addmod(evm: &mut Evm) {
    let a = evm.stack.pop().unwrap();
    let b = evm.stack.pop().unwrap();
    let c = evm.stack.pop().unwrap();

    if b == U256::ZERO {
        evm.stack.push(U256::ZERO).unwrap();
    } else {
        let addition = a + b;
        let result = addition % c;
        evm.stack.push(result).unwrap();
    }
}

pub fn mulmod(evm: &mut Evm) {
    let a = evm.stack.pop().unwrap();
    let b = evm.stack.pop().unwrap();
    let c = evm.stack.pop().unwrap();

    if b == U256::ZERO {
        evm.stack.push(U256::ZERO).unwrap();
    } else {
        let multiplication = a * b;
        let result = multiplication % c;
        evm.stack.push(result).unwrap();
    }
}

pub fn modulo(evm: &mut Evm) {
    let a = evm.stack.pop().unwrap();
    let b = evm.stack.pop().unwrap();
    if b == U256::ZERO {
        evm.stack.push(U256::ZERO).unwrap();
    } else {
        evm.stack.push(a % b).unwrap();
    }
}

pub fn smod(evm: &mut Evm) {
    let a: U256 = evm.stack.pop().unwrap();
    let b: U256 = evm.stack.pop().unwrap();
    if b == U256::ZERO {
        evm.stack.push(U256::ZERO).unwrap();
    } else {
        evm.stack.push(a % b).unwrap();
    }
}

pub fn exp(evm: &mut Evm) {
    let base: U256 = evm.stack.pop().unwrap();
    let exponent: U256 = evm.stack.pop().unwrap();
    let result: U256 = base.pow(exponent);
    evm.stack.push(result).unwrap();
}

pub fn signextend(evm: &mut Evm) {
    let size_in_byte = evm.stack.pop().unwrap();
    let integer = evm.stack.pop().unwrap();

    // size_in_byte.to_be_bytes().unwrap();
    if size_in_byte >= U256::from(32) {
        evm.stack.push(integer).unwrap();
    } else {
        let mask = U256::from(2).pow(size_in_byte * U256::from(8)) - U256::ONE;
        let extended = integer & mask;
        evm.stack.push(extended).unwrap();
    }
}

pub fn lt(evm: &mut Evm) {
    let left = evm.stack.pop().unwrap();
    let right = evm.stack.pop().unwrap();

    let result = left.lt(&right);
    evm.stack.push(U256::from(result)).unwrap();
}

pub fn gt(evm: &mut Evm) {
    let left = evm.stack.pop().unwrap();
    let right = evm.stack.pop().unwrap();
    let result = left.gt(&right);
    evm.stack.push(U256::from(result)).unwrap();
}

pub fn slt(evm: &mut Evm) {
    let left: U256 = evm.stack.pop().unwrap();
    let right: U256 = evm.stack.pop().unwrap();

    let left_int = I256::from_limbs(*left.as_limbs());
    let right_int = I256::from_limbs(*right.as_limbs());

    let result = left_int.lt(&right_int);
    let unsigned_result = U256::from(result);

    evm.stack.push(unsigned_result).unwrap();
}

pub fn sgt(evm: &mut Evm) {
    let left: U256 = evm.stack.pop().unwrap();
    let right: U256 = evm.stack.pop().unwrap();

    let left_int = I256::from_limbs(*left.as_limbs());
    let right_int = I256::from_limbs(*right.as_limbs());

    let result = left_int.gt(&right_int);
    let unsigned_result = U256::from(result);

    evm.stack.push(unsigned_result).unwrap();
}

pub fn eq(evm: &mut Evm) {
    let left = evm.stack.pop().unwrap();
    let right = evm.stack.pop().unwrap();

    let result = left.eq(&right);
    evm.stack.push(U256::from(result)).unwrap();
}

pub fn is_zero(evm: &mut Evm) {
    let value = evm.stack.pop().unwrap();

    let result = value.is_zero();
    evm.stack.push(U256::from(result)).unwrap();
}

pub fn and(evm: &mut Evm) {
    let left = evm.stack.pop().unwrap();
    let right = evm.stack.pop().unwrap();

    let result = left.bitand(right);
    evm.stack.push(result).unwrap();
}


pub fn byte(evm: &mut Evm) {
    let index = evm.stack.pop().unwrap();
    let value = evm.stack.pop().unwrap();
    
    if index.as_limbs()[0] > 32 {
        evm.stack.push(U256::ZERO).unwrap();
    } else {
        let byte_index = index.as_limbs()[0] as usize;
        let byte = value.as_limbs()[byte_index];
        evm.stack.push(U256::from(byte)).unwrap();
    }
}

pub fn mstore(evm: &mut Evm) {
    let offset = evm.stack.pop().unwrap();
    let value = evm.stack.pop().unwrap();

    let offset = offset.as_limbs()[0] as usize;
    
    evm.memory.store_word(offset, value);

}

pub fn address(evm: &mut Evm) {
    let address: Address = evm.tx.to;
   
    let mut padded = [0u8; 32]; // length is 32 bytes
    
    // the address is 20bytes long, hence, padded with zero 
    padded[12..].copy_from_slice(address.as_slice()); // 
    
    let value = U256::from_be_bytes(padded);
    evm.stack.push(value).unwrap();
}

pub fn balance(evm: &mut Evm) {
    let address: Address = evm.tx.from;
    
    let address_account = evm.storage.data.get(&address).unwrap();
    let balance: U256 = address_account.balance;
    evm.stack.push(balance).unwrap();
}

pub fn origin(evm: &mut Evm) {
    let address: Address = evm.tx.from;
    
    let mut padded = [0u8; 32]; // length is 32 bytes
    
    // the address is 20bytes long, hence, padded with zero 
    padded[12..].copy_from_slice(address.as_slice()); // 
    
    let value = U256::from_be_bytes(padded);
    evm.stack.push(value).unwrap();
}


pub fn call_value(evm: &mut Evm) {
    let value = evm.tx.value;
    evm.stack.push(value).unwrap();
}

pub fn call_data_load(evm: &mut Evm) {
    let offset = evm.stack.pop().unwrap();
    let offset = offset.as_limbs()[0] as usize;
    
    let data = evm.tx.data.as_slice();
    // let value = U256::from_be_bytes(data[offset..offset + 32].);
    // evm.stack.push(value).unwrap();
}

pub fn gas_price(evm: &mut Evm) {
    let gas_price = evm.tx.gas_limit;
    evm.stack.push(gas_price).unwrap();
}

pub fn block_hash(evm: &mut Evm) {
    // get the request block number from the stack
    let block_number = evm.stack.pop().unwrap();
    
    // get the current block number from the block environment
    let current_block_number = evm.block_env.number;
    
    // check if the requested block number 
    // is within the range of the current block number
    if block_number > current_block_number {
        evm.stack.push(U256::ZERO).unwrap();
    } else {
        
        // get the block hash from the block environment
        let block_hash = evm.block_env.block_hash.as_limbs()[0];
        
        // evm.stack.push(block_hash).unwrap();
    }
    
    
    pub fn coin_base(evm: &mut Evm) {
        let coin_base = evm.block_env.coinbase;
        
        evm.stack.push(U256::from_be_slice(coin_base.as_slice())).unwrap();
    }
    
    pub fn timestamp(evm: &mut Evm) {
        let timestamp = evm.block_env.timestamp;
        
        evm.stack.push(timestamp).unwrap();
    }
    
    pub fn number(evm: &mut Evm) {
        let number = evm.block_env.number;
        
        evm.stack.push(number).unwrap();
    }
    
    pub fn gas_limit(evm: &mut Evm) {
        let gas_limit = evm.block_env.gas_limit;
        
        evm.stack.push(gas_limit).unwrap();
    }
    
    pub fn chain_id(evm: &mut Evm) {
        let chain_id = evm.block_env.chain_id;
        
        evm.stack.push(chain_id).unwrap();
    }
    
    pub fn pop(evm: &mut Evm) {
        evm.stack.pop().unwrap();
    }
    
    pub fn m_load(evm: &mut Evm) {
        let offset = evm.stack.pop().unwrap();
        
        
        evm.memory.load_word(offset.as_limbs()[0] as usize);
    }
}
