# PBT Campaign: encode_cmovcc (i686)

## Scan findings
- **Spec:** (none found — symbol-targeted campaign on `encode_cmovcc`)
- **Test layout:** project-owned Rust lib tests via `#[cfg(test)] mod encode_*_pbt` under `src/backend/i686/assembler/encoder/`, discovered by `cargo test --lib`; proptest 1.11.0 in `[dev-dependencies]`.
- **Buildability probe:** `cargo test --lib encode_ldrsw_kat_llvm_mc_x0_x1 -- --test-threads=1` (user build contract) — pre-campaign SUCCESS (log: `pbt-out/build.log`). Incremental runs reuse that form with the new filter `encode_cmovcc`.
- **Harness placement:** extend existing project test target (rung 1) — add `src/backend/i686/assembler/encoder/encode_cmovcc_pbt.rs` + one `#[cfg(test)] mod encode_cmovcc_pbt;` line in `encoder/mod.rs`.
- **Candidate modules:** encode_cmovcc (`src/backend/i686/assembler/encoder/gp_integer.rs:1097`)
- **Skipped modules:** all other functions in `gp_integer.rs` and outside this path (HARD scope: only `encode_cmovcc`); ARM/x86-64/RISC-V encoders (out of scope); HEAD changes outside scope.
- **Doc contract:** inline comment at gp_integer.rs:1103 `"Strip size suffix if present, otherwise use as-is (unsuffixed = 32-bit default)"` — domain-restriction/design for mnemonic size default only; does not declare r8/non-GP/segment-less memory invalid. No function-level doc comment.
- **Oracle:** Differential (llvm-mc i686) strongest applicable; State machine rejected (pure encoder); Algebraic round-trip rejected (no i686 decoder); Algebraic metamorphic/invariant + negative_error also apply.
- **Likely defects (from scan):** (1) memory arm never calls `emit_segment_prefix` before `0F 40+cc` (same class as encode_setcc/smsw/…); (2) size driven only by mnemonic suffix — unsuffixed r16 emits without 0x66 while llvm-mc infers 16-bit; (3) no width/GP gate — r8 and non-GP accepted via `reg_num` aliasing; mismatched widths accepted. **All three confirmed.**

## Module: encode_cmovcc
- [x] Scan: identify targets
- [x] Plan: formalize properties
- [x] Test: write and run
- [x] Review: triage results

## Contract-surface sweep
- [x] coverage_gaps round 1 (standard): no line-level data; file-level note recorded; strengthen mem_w16 PASS; documented behaviors already have properties. Close.
