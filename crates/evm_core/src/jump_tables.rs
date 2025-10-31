use crate::{Evm, opcodes::Opcode, operations::ariths::*};

pub type OpcodeFn = fn(&mut Evm);

pub fn noop(_evm: &mut Evm) {}

pub fn build_jump_table() -> [OpcodeFn; 256] {
    let mut jump_table: [fn(&mut Evm); 256] = [noop as OpcodeFn; 256];
    jump_table[Opcode::STOP as usize] = stop;
    jump_table[Opcode::ADD as usize] = add;
    jump_table[Opcode::SUB as usize] = sub;
    jump_table[Opcode::MUL as usize] = mul;
    jump_table[Opcode::DIV as usize] = div;
    jump_table[Opcode::SDIV as usize] = sdiv;
    jump_table[Opcode::SMOD as usize] = smod;
    jump_table[Opcode::MOD as usize] = modulo;
    

    jump_table
}
