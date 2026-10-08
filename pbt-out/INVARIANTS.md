# Confirmed invariants (encode_prefetch_0f0d)

## Behavioral invariants
- `prefetchw` encodes as `0F 0D /1` (hint hardcoded at encoder/mod.rs:746), plus ModR/M memory addressing.
- ModRM.reg field is exactly 1 for all successful public-API encodes; opcode bytes are always `0F 0D` (no segment).
- Base+disp, SIB (index≠ESP), ESP-forced SIB, EBP-forced disp8, disp8/disp32 boundaries, and absolute disp32 forms agree with llvm-mc `-triple=i686` when no segment override is present.
- Under-arity (≠1 operand) returns Err containing `prefetchw requires 1 operand`.
- Non-memory operands return Err containing `memory operand`.
- **Bug:** segment overrides (`%fs:`, `%gs:`, `%es:`, …) are dropped — SUT omits `emit_segment_prefix` before `0F 0D`. See bug_reports/encode_prefetch_0f0d_missing_segment_prefix.md. Same class as encode_prefetch (0F 18).

## Environment
- llvm-mc: /home/toan/tools/llvm15-official/bin/llvm-mc (LLVM 15), `-triple=i686 -show-encoding`.
- Harness: src/backend/i686/assembler/encoder/encode_prefetch_0f0d_pbt.rs, cargo test --lib encode_prefetch_0f0d, proptest cases=1000.
- Dispatch: encoder/mod.rs "prefetchw" => encode_prefetch_0f0d(ops, 1).
- coverage_gaps had no LLVM profraw for this Rust target (C++ reporter listed unrelated binaries).

## Quirks
- encode_prefetch_0f0d is `pub(super)`; tests reach it via public `InstructionEncoder::encode`.
- Bare gas mnemonic `prefetch` is 0F 0D /0 — not dispatched by i686 encoder (out of scope).
- Function takes `hint: u8` but only hint=1 is reachable via public mnemonic table.

## Prior (encode_prefetch) still holds
- prefetcht0/t1/t2/nta = 0F 18 /hint; same missing-segment bug class.
