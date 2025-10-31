use alloy::primitives::{I256, U256};

use crate::{ProgramExitStatus, Evm};

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
    }else {
        evm.stack.push(a/b).unwrap();
    }
}

pub fn sdiv(evm: &mut Evm) {
    let a = evm.stack.pop().unwrap();
    let b = evm.stack.pop().unwrap();
    
    let a_int = I256::from_limbs(*a.as_limbs());
    let b_int = I256::from_limbs(*b.as_limbs());
    
    if b_int == I256::ZERO {
        evm.stack.push(U256::ZERO).unwrap();
    }else {
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
    }else {
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
    }else {
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
    }else {
        evm.stack.push(a % b).unwrap();
    }
}


pub fn smod(evm: &mut Evm) {
    let a: U256 = evm.stack.pop().unwrap();
    let b: U256 = evm.stack.pop().unwrap();
    if b == U256::ZERO {
        evm.stack.push(U256::ZERO).unwrap();
    }else {
        evm.stack.push(a % b).unwrap();
    }
}




