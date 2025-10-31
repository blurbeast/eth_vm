use alloy::{
    primitives::{I256, U256}
};

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
