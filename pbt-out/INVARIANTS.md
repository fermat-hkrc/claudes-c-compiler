# Confirmed invariants (encode_prefetch)

## Behavioral invariants
- `prefetcht0` / `prefetcht1` / `prefetcht2` / `prefetchnta` encode as `0F 18 /hint` with hint ∈ {1,2,3,0} respectively, plus ModR/M memory addressing.
- ModRM.reg field is exactly the hint; changing only the mnemonic changes only that 3-bit field (SIB/disp unchanged).
- Base+disp, SIB (index≠ESP), ESP-forced SIB, EBP-forced disp8, disp8/disp32 boundaries, and absolute disp32 forms agree with llvm-mc `-triple=i686` when no segment override is present.
- Under-arity (≠1 operand) returns Err containing `prefetch requires 1 operand`.
- Non-memory operands return Err containing `memory operand`.
- **Bug:** segment overrides (`%fs:`, `%gs:`, `%es:`, …) are dropped — SUT omits `emit_segment_prefix` before `0F 18`. See bug_reports/encode_prefetch_missing_segment_prefix.md.

## Environment
- llvm-mc: /home/toan/tools/llvm15-official/bin/llvm-mc (LLVM 15), `-triple=i686 -show-encoding`.
- Harness: src/backend/i686/assembler/encoder/encode_prefetch_pbt.rs, cargo test --lib encode_prefetch, proptest cases=1000.
- Dispatch: encoder/mod.rs "prefetcht0|t1|t2|prefetchnta" => encode_prefetch(ops, hint).
- coverage_gaps had no LLVM profraw for this Rust target (C++ reporter listed unrelated binaries).

## Quirks
- encode_prefetch is `pub(super)`; tests reach it via public `InstructionEncoder::encode`.
- Bare gas mnemonic `prefetch` is 0F 0D /0 (prefetchw family) — out of scope (encode_prefetch_0f0d).
