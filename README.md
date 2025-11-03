# eth_vm — Concise reference

A minimal Rust EVM prototype for experimentation. This file is intentionally short and points to the most relevant locations.

Purpose

- Small EVM core for learning and extension.
- Not production-ready: no gas metering, limited validation, and some panics (`unwrap()`).

Project layout (essential)

- `crates/evm_core` — VM core: `Opcode` enum, jump table, handlers.
- `crates/primitives` — runtime primitives: `Memory`, `Stack`, `EvmStorage`, `Transaction`, `BlockEnv`.
- `bins/evm` — example runner that creates an `Evm` instance.

Dispatch (runtime)

- `Evm::step()` reads a byte from `memory[pc]`, converts it with `Opcode::from_u8`, looks up the handler in the 256-entry table from `build_jump_table()`, calls the handler (`fn(&mut Evm)`), then increments `pc`. Handlers that perform jumps must set `evm.pc` directly.

Short opcode snippet (Rust)

```crates/evm_core/src/opcodes.rs#L1-20
pub enum Opcode {
    STOP = 0x00,
    ADD = 0x01,
    MUL = 0x02,
    SUB = 0x03,
    DIV = 0x04,
    SDIV = 0x05,
    MOD = 0x06,
    SMOD = 0x07,
    ADDMOD = 0x08,
    MULMOD = 0x09,
    EXP = 0x0A,
    SIGNEXTEND = 0x0B,
    // ... remainder in file
}
```

How to add an opcode (3 steps)

1. Add/confirm enum entry and `from_u8` mapping in `crates/evm_core/src/opcodes.rs`.
2. Implement handler `fn(&mut Evm)` in `crates/evm_core/src/operations/`.
3. Register it in `crates/evm_core/src/jump_tables.rs`:

```rust
jump_table[Opcode::MYOP as usize] = myop_handler;
```

Notes

- For exact semantics and gas rules consult https://www.evm.codes/.
- To run locally: from repo root use `cargo build --workspace`

