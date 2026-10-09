# Confirmed invariants / environment quirks

## encode_in / encode_out (i686 system encoder)

- Intel IN/OUT fix data register to AL/AX/EAX and port to DX (or imm8).
- AT&T operand order: OUT is `outb %al, %dx` / `outb %al, $imm`; IN is `inb %dx, %al` / `inb $imm, %al`.
- AT&T parenthesized port `(%dx)` is accepted by llvm-mc as alias of `%dx` (same EC/ED or EE/EF bytes).
- Operand-size override: inw/outw = `[0x66] ++ inl/outl` for the same port shape.
- llvm-mc path used by campaigns: `/home/toan/tools/llvm15-official/bin/llvm-mc -triple=i686 -show-encoding`.
- proptest cases set explicitly to 1000 for standard tier.
- Known defect class (both encode_in and encode_out): (1) any Register,Register pair accepted; (2) `*val as u8` truncates OOR imm; (3) missing Memory port form on i686 (x86-64 sibling handles it).

## Harness

- PBT files: `src/backend/i686/assembler/encoder/*_pbt.rs` + `#[cfg(test)] mod` in `encoder/mod.rs`.
- Build contract form: `cargo test --lib <filter> -- --test-threads=1`.
