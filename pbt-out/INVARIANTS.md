# Confirmed invariants (encode_out)

## Behavioral invariants
- Canonical DX-port forms encode as Intel fixed bytes: outb→EE, outw→66 EF, outl→EF when data is al/ax/eax and port is dx.
- Canonical imm8-port forms: outb→E6 ib, outw→66 E7 ib, outl→E7 ib for imm in the domain llvm-mc accepts (0..=255 and signed byte equivalents).
- outw encoding is always a single 0x66 prefix prepended to the corresponding outl encoding (same operand shape).
- Arity other than 0 or 2 returns Err containing `requires 0 or 2 operands`.
- **Bug:** any Register+Register pair is accepted and emits EE/EF without checking AL/AX/EAX and DX (system.rs:59-63).
- **Bug:** imm port is truncated with `*val as u8` with no range check (system.rs:68) — e.g. $256 → E6 00.
- **Bug:** AT&T `(%dx)` memory port form is rejected (`unsupported … operands`); x86-64 sibling and llvm-mc accept it as DX-port OUT.

## Environment
- llvm-mc: /home/toan/tools/llvm15-official/bin/llvm-mc (LLVM 15), `-triple=i686 -show-encoding`.
- Harness: src/backend/i686/assembler/encoder/encode_out_pbt.rs, cargo test --lib encode_out, proptest cases=1000.
- Dispatch: encoder/mod.rs "outb"|"outw"|"outl" => encode_out(ops, mnemonic).
- coverage_gaps had no LLVM profraw for this Rust target (C++ reporter listed unrelated binaries).

## Quirks
- encode_out is `pub(super)`; tests reach it via public `InstructionEncoder::encode`.
- Operand order is AT&T (source data first, destination port last).
