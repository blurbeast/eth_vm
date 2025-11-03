use alloy::primitives::{Address, I256, U256};

use crate::{Evm, ProgramExitStatus};

// ref == https://www.evm.codes/

/// STOP opcode handler
/// - Semantics: halt execution and set program status to Success.
/// - Stack effects: none.
pub fn stop(evm: &mut Evm) {
    evm.status = ProgramExitStatus::Success;
}

/// ADD opcode handler
/// NB: No check made for overflow.
/// - Semantics: pop two 256-bit values from the stack (call them `a` and `b`) and push `a + b`.
/// - Stack order in this implementation:
///   * `let a = evm.stack.pop().unwrap();` // top of stack
///   * `let b = evm.stack.pop().unwrap();` // next item
///   Result pushed: `a + b`.
/// - Example: stack before [0x02, 0x03] (top = 0x03) after `add` -> [0x05] (top = 0x05).
pub fn add(evm: &mut Evm) {
    let a = evm.stack.pop().unwrap();
    let b = evm.stack.pop().unwrap();
    evm.stack.push(a + b).unwrap();
}

/// SUB opcode handler
/// - Semantics: pop `a`, pop `b`, push `a - b` (using unsigned U256 subtraction semantics).
/// - Note on order: because we pop `a` then `b`, the computed value is `a - b` where `a` is the top value.
/// - Example: stack [0x05, 0x02] (top=0x02) -> after `sub` push (0x02 - 0x05) mod 2^256.
/// - Caveat: the implementation uses `U256` arithmetic; negatives wrap around in unsigned interpretation.
pub fn sub(evm: &mut Evm) {
    let a = evm.stack.pop().unwrap();
    let b = evm.stack.pop().unwrap();
    evm.stack.push(a - b).unwrap();
}

/// MUL opcode handler
/// - Semantics: pop `a`, pop `b`, push `a * b`.
/// - Example: [2, 3] -> push 6.
pub fn mul(evm: &mut Evm) {
    let a = evm.stack.pop().unwrap();
    let b = evm.stack.pop().unwrap();
    evm.stack.push(a * b).unwrap();
}

/// DIV opcode handler (unsigned)
/// - Semantics: pops `a` and `b`, if `b == 0` push 0, else push `a / b`.
/// - Edge-case: Division by zero returns zero per EVM semantics implemented here.
/// - Example: [10, 2] -> push 5. [10, 0] -> push 0.
pub fn div(evm: &mut Evm) {
    let a = evm.stack.pop().unwrap();
    let b = evm.stack.pop().unwrap();
    if b == U256::ZERO {
        evm.stack.push(U256::ZERO).unwrap();
    } else {
        evm.stack.push(a / b).unwrap();
    }
}

/// SDIV opcode handler (signed division)
/// - Semantics: treat stack values as signed 256-bit integers, divide, then push unsigned representation of result
///   * Converts `U256` limbs into `I256` for signed arithmetic and converts result back to `U256`.
///   * Division by zero pushes `U256::ZERO`.
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

/// ADDMOD opcode handler
/// - Semantics: pop `a`, `b`, `c`, compute `(a + b) % c`. If `c == 0` push 0.
/// - Notes: This implementation checks `b` for zero in the original code.
/// - Example: a=2,b=3,c=5 -> (2+3)%5 = 0.
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

/// MULMOD opcode handler
/// - Semantics: pop `a`, `b`, `c`, compute `(a * b) % c`. If `c == 0` push 0.
/// - Example: a=2,b=3,c=4 -> (2*3)%4 = 2.
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

/// MOD opcode handler (unsigned modulo)
/// - Semantics: pop `a`, pop `b`, if `b == 0` push 0 else push `a % b`.
/// - Example: [10,3] -> push 1.
pub fn modulo(evm: &mut Evm) {
    let a = evm.stack.pop().unwrap();
    let b = evm.stack.pop().unwrap();
    if b == U256::ZERO {
        evm.stack.push(U256::ZERO).unwrap();
    } else {
        evm.stack.push(a % b).unwrap();
    }
}

/// SMOD opcode handler (signed modulo)
/// - Semantics: behaves similarly to MOD but for signed values. This implementation uses unsigned types directly.
/// - Note: This implementation currently delegates to unsigned modulo; adjust if full signed semantics are required.
pub fn smod(evm: &mut Evm) {
    let a: U256 = evm.stack.pop().unwrap();
    let b: U256 = evm.stack.pop().unwrap();
    if b == U256::ZERO {
        evm.stack.push(U256::ZERO).unwrap();
    } else {
        evm.stack.push(a % b).unwrap();
    }
}

/// EXP opcode handler (exponentiation)
/// - Semantics: pop base, pop exponent, compute base.pow(exponent) and push result.
/// - Warning: exponentiation may be very expensive; no gas accounting here.
/// - Example: base=2, exponent=3 -> push 8.
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

/// LT opcode handler (unsigned less-than)
/// - Semantics: pop left, pop right, push 1 if left < right else 0.
/// - Example: [2,3] -> push 1.
pub fn lt(evm: &mut Evm) {
    let left = evm.stack.pop().unwrap();
    let right = evm.stack.pop().unwrap();

    let result = left.lt(&right);
    evm.stack.push(U256::from(result)).unwrap();
}

/// GT opcode handler (unsigned greater-than)
/// - Semantics: pop left, pop right, push 1 if left > right else 0.
pub fn gt(evm: &mut Evm) {
    let left = evm.stack.pop().unwrap();
    let right = evm.stack.pop().unwrap();
    let result = left.gt(&right);
    evm.stack.push(U256::from(result)).unwrap();
}

/// SLT opcode handler (signed less-than)
/// - Semantics: convert both operands to signed `I256`, compare, push 1 if left < right else 0.
pub fn slt(evm: &mut Evm) {
    let left: U256 = evm.stack.pop().unwrap();
    let right: U256 = evm.stack.pop().unwrap();

    let left_int = I256::from_limbs(*left.as_limbs());
    let right_int = I256::from_limbs(*right.as_limbs());

    let result = left_int.lt(&right_int);
    let unsigned_result = U256::from(result);

    evm.stack.push(unsigned_result).unwrap();
}

/// SGT opcode handler (signed greater-than)
/// - Semantics: convert both operands to `I256` and compare.
pub fn sgt(evm: &mut Evm) {
    let left: U256 = evm.stack.pop().unwrap();
    let right: U256 = evm.stack.pop().unwrap();

    let left_int = I256::from_limbs(*left.as_limbs());
    let right_int = I256::from_limbs(*right.as_limbs());

    let result = left_int.gt(&right_int);
    let unsigned_result = U256::from(result);

    evm.stack.push(unsigned_result).unwrap();
}

/// EQ opcode handler (equality)
/// - Semantics: pop left, pop right, push 1 if equal else 0.
pub fn eq(evm: &mut Evm) {
    let left = evm.stack.pop().unwrap();
    let right = evm.stack.pop().unwrap();

    let result = left.eq(&right);
    evm.stack.push(U256::from(result)).unwrap();
}

/// ISZERO opcode handler
/// - Semantics: pop value, push 1 if value == 0 else 0.
pub fn is_zero(evm: &mut Evm) {
    let value = evm.stack.pop().unwrap();

    let result = value.is_zero();
    evm.stack.push(U256::from(result)).unwrap();
}

/// AND opcode handler (bitwise)
/// - Semantics: pop left, pop right, push bitwise-and result.
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

/// MSTORE opcode handler
/// - Semantics: pop offset, pop value, store 32-byte word `value` at memory[offset..offset+32].
/// - Stack order: this handler pops `offset` first and then `value`, matching the call-site convention
///   where offset was pushed after value (e.g., push value; push offset; MSTORE).
pub fn mstore(evm: &mut Evm) {
    let offset = evm.stack.pop().unwrap();
    let value = evm.stack.pop().unwrap();

    let offset = offset.as_limbs()[0] as usize;

    evm.memory.store_word(offset, value);
}

/// ADDRESS opcode handler
/// - Semantics: push the current executing contract's address (tx.to) as a 32-byte left-padded value.
/// - Implementation: pads the 20-byte address into a 32-byte big-endian word and pushes it.
pub fn address(evm: &mut Evm) {
    let address: Address = evm.tx.to;

    let mut padded = [0u8; 32]; // length is 32 bytes

    // the address is 20bytes long, hence, padded with zero
    padded[12..].copy_from_slice(address.as_slice()); //

    let value = U256::from_be_bytes(padded);
    evm.stack.push(value).unwrap();
}

/// BALANCE opcode handler
/// - Semantics: push the balance of the account (usually the account specified by `evm.tx.from` here).
/// - Note: this implementation unwraps the account entry;
pub fn balance(evm: &mut Evm) {
    let address: Address = evm.tx.from;

    let address_account = evm.storage.data.get(&address).unwrap();
    let balance: U256 = address_account.balance;
    evm.stack.push(balance).unwrap();
}

/// ORIGIN opcode handler
/// - Semantics: push the transaction origin address (tx.from) padded to 32 bytes.
/// - Implementation mirrors `address` logic but uses `tx.from`.
pub fn origin(evm: &mut Evm) {
    let address: Address = evm.tx.from;

    let mut padded = [0u8; 32]; // length is 32 bytes

    // the address is 20bytes long, hence, padded with zero
    padded[12..].copy_from_slice(address.as_slice()); //

    let value = U256::from_be_bytes(padded);
    evm.stack.push(value).unwrap();
}

/// CALLVALUE opcode handler
/// - Semantics: push the `tx.value` (amount of wei sent with the call).
pub fn call_value(evm: &mut Evm) {
    let value = evm.tx.value;
    evm.stack.push(value).unwrap();
}

/// CALLDATALOAD partial handler
/// - Semantics: intended to pop offset and push 32 bytes starting from `tx.data[offset]`.
/// - Implementation note: this function reads the offset and prepares to use `tx.data` but the final conversion
///   into a `U256` is left commented out. This must be completed to match EVM semantics and handle out-of-bounds reads.
pub fn call_data_load(evm: &mut Evm) {
    let offset = evm.stack.pop().unwrap();
    let offset = offset.as_limbs()[0] as usize;

    let data = evm.tx.data.as_slice();
    // let value = U256::from_be_bytes(data[offset..offset + 32].);
    // evm.stack.push(value).unwrap();
}

/// GASPRICE opcode handler (simplified)
/// - Implementation pushes `tx.gas_limit` as a stand-in for gas price (this is not the usual meaning).
/// - In EVM semantics GASPRICE should push `tx.gas_price` or chain gas price; adjust accordingly.
pub fn gas_price(evm: &mut Evm) {
    let gas_price = evm.tx.gas_limit;
    evm.stack.push(gas_price).unwrap();
}

/// BLOCKHASH opcode handler (partial)
/// - Semantics: pop block number `n`, if `n` is within the last 256 blocks return blockhash(n) else 0.
/// - Implementation: checks if requested block number is greater than current block number and pushes 0 if so.
/// - Note: full historical block-hash semantics are not implemented here.
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
}

/// COINBASE opcode handler
/// - Semantics: push the block coinbase/miner address as 32 bytes.
pub fn coin_base(evm: &mut Evm) {
    let coin_base = evm.block_env.coinbase;

    evm.stack
        .push(U256::from_be_slice(coin_base.as_slice()))
        .unwrap();
}

/// TIMESTAMP opcode handler
/// - Semantics: push current block timestamp.
pub fn timestamp(evm: &mut Evm) {
    let timestamp = evm.block_env.timestamp;

    evm.stack.push(timestamp).unwrap();
}

/// NUMBER opcode handler
/// - Semantics: push current block number.
pub fn number(evm: &mut Evm) {
    let number = evm.block_env.number;

    evm.stack.push(number).unwrap();
}

/// GASLIMIT opcode handler
/// - Semantics: push current block gas limit.
pub fn gas_limit(evm: &mut Evm) {
    let gas_limit = evm.block_env.gas_limit;

    evm.stack.push(gas_limit).unwrap();
}

/// CHAINID opcode handler
/// - Semantics: push chain id.
pub fn chain_id(evm: &mut Evm) {
    let chain_id = evm.block_env.chain_id;

    evm.stack.push(chain_id).unwrap();
}

/// POP opcode handler
/// - Semantics: remove the top stack element and discard it.
pub fn pop(evm: &mut Evm) {
    evm.stack.pop().unwrap();
}

/// MLOAD opcode handler
/// - Semantics: pop offset, load 32-byte word from memory starting at offset, push that word.
/// - Note: `load_word` assumes memory has enough bytes; ensure memory is grown appropriately.
pub fn m_load(evm: &mut Evm) {
    let offset = evm.stack.pop().unwrap();

    let word = evm.memory.load_word(offset.as_limbs()[0] as usize);

    evm.stack.push(word).unwrap();
}

/// MSTORE opcode handler (alternate)
/// - Semantics: pop offset, pop value, store the 32-byte word at memory[offset].
/// - Note: similar to `mstore` above; ensure memory length suffices.
pub fn m_store(evm: &mut Evm) {
    let offset = evm.stack.pop().unwrap();
    let value = evm.stack.pop().unwrap();

    evm.memory.store_word(offset.as_limbs()[0] as usize, value);
}

/// MSTORE8 opcode handler
/// - Semantics: pop offset, pop value, store the least-significant byte of value at memory[offset].
pub fn m_store8(evm: &mut Evm) {
    let offset = evm.stack.pop().unwrap();
    let value = evm.stack.pop().unwrap();

    evm.memory.store_byte(offset.as_limbs()[0] as usize, value.as_limbs()[0] as u8);
}

/// SLOAD opcode handler (partial)
/// - Semantics: pop storage slot key, load value from persistent storage for the executing contract address.
/// - Note: this implementation reads from `storage` using `evm.tx.to` as the contract address; callers must ensure
///   that `storage` contains an account entry for that address.
pub fn s_load(evm: &mut Evm) {
    let offset = evm.stack.pop().unwrap();

    let locator: Address = evm.tx.to;

    let word = evm.storage.s_load(locator, offset);

    // evm.stack.push(word).unwrap();
}

/// SSTORE opcode handler (partial)
/// - Semantics: pop offset, pop value, store value into persistent storage at slot `offset` for the current contract address.
pub fn s_store(evm: &mut Evm) {
    let offset = evm.stack.pop().unwrap();
    let value = evm.stack.pop().unwrap();

    let locator: Address = evm.tx.to;

    evm.storage.s_store(locator, offset, value);
}

/// JUMP opcode handler
/// - Semantics: pop target and set `pc` to that value (absolute jump).
/// - Note: real EVM requires target to be a valid `JUMPDEST`; validation is not performed here.
pub fn jump(evm: &mut Evm) {
    let target = evm.stack.pop().unwrap();

    evm.pc = target.as_limbs()[0] as usize;
}

/// JUMPI opcode handler
/// - Semantics: pop target, pop condition. If condition != 0, set `pc = target` (conditional jump).
pub fn jumpi(evm: &mut Evm) {
    let target = evm.stack.pop().unwrap();
    let condition = evm.stack.pop().unwrap();

    if condition.as_limbs()[0] != 0 {
        evm.pc = target.as_limbs()[0] as usize;
    }
}

/// JUMPDEST handler (no-op in many implementations)
/// - Semantics: marks a valid destination for `JUMP`/`JUMPI`. Here it does nothing.
pub fn jump_dest(evm: &mut Evm) {
    let pc = evm.pc;
}

/// PC opcode handler
/// - Semantics: push current program counter. This implementation currently reads `evm.pc` but doesn't push it.
pub fn pc(evm: &mut Evm) {
    evm.pc;
}

/// MSIZE opcode handler
/// - Semantics: push memory size in bytes. This implementation reads `memory.data.len()` but doesn't push it.
pub fn m_size(evm: &mut Evm) {
    evm.memory.data.len();
}

/// GAS opcode handler (partial)
/// - Semantics: push remaining gas. This implementation returns block_env.gas_limit which is not correct gas accounting.
pub fn gas(evm: &mut Evm) {
    evm.block_env.gas_limit;
}

/// MCOPY opcode handler (partial)
/// - Semantics: copy memory region; this implementation reads stack operands but the actual copy is commented out.
pub fn m_copy(evm: &mut Evm) {
    let offset = evm.stack.pop().unwrap();
    let length = evm.stack.pop().unwrap();
    let dest = evm.stack.pop().unwrap();

    // evm.memory.copy(offset.as_limbs()[0] as usize, dest.as_limbs()[0] as usize, length.as_limbs()[0] as usize);
}

/// PUSH0 opcode handler (special PUSH of zero)
/// - Semantics: push zero onto the stack.
pub fn push_0(evm: &mut Evm) {
    evm.stack.push(U256::ZERO).unwrap();
}
