# PBT Campaign: encode_setcc (i686)

## Scan findings
- **Spec:** (none found — symbol-targeted campaign on `encode_setcc`)
- **Test layout:** project-owned Rust lib tests via `#[cfg(test)] mod encode_*_pbt` under `src/backend/i686/assembler/encoder/`, discovered by `cargo test --lib`; proptest 1.11.0 in `[dev-dependencies]`.
- **Buildability probe:** `cargo test --lib encode_ldrsw_kat_llvm_mc_x0_x1 -- --test-threads=1` (user build contract) — pre-campaign SUCCESS (log: `pbt-out/build.log`). Incremental runs reuse that form with the new filter.
- **Harness placement:** extend existing project test target (rung 1) — add `src/backend/i686/assembler/encoder/encode_setcc_pbt.rs` + one `#[cfg(test)] mod encode_setcc_pbt;` line in `encoder/mod.rs`.
- **Candidate modules:** encode_setcc (`src/backend/i686/assembler/encoder/gp_integer.rs:1075`)
- **Skipped modules:** all other functions in `gp_integer.rs` and outside this path (HARD scope: only `encode_setcc`); ARM/x86-64/RISC-V encoders (out of scope).
- **Doc contract:** (none on `encode_setcc` itself — no doc comment). Behavior evidenced by Intel SDM SETcc (r/m8, opcode 0F 90+cc /0), dispatch list in `mod.rs:279-281`, `cc_from_mnemonic` in `registers.rs:118`, sibling x86/i686 memory encoders that call `emit_segment_prefix`, and llvm-mc `-triple=i686`.
- **Oracle:** Differential (llvm-mc i686) strongest applicable; State machine rejected (pure encoder); Algebraic round-trip rejected (no i686 decoder); Algebraic metamorphic/invariant + negative_error also apply.
- **Likely defects (from scan):** (1) memory arm never calls `emit_segment_prefix` before `0F 90+cc` (same class as encode_smsw/invlpg/…); (2) register arm accepts r16/r32/non-GP via `reg_num` aliasing (Intel SETcc is r/m8 only; llvm-mc rejects `%eax`/`%ax`). Confirmed both.

## Module: encode_setcc
- [x] Scan: identify targets
- [x] Plan: formalize properties
- [x] Test: write and run
- [x] Review: triage results

## Contract-surface sweep
- [x] coverage_gaps round 1 (standard): no line-level data; file-level note recorded; documented behaviors already have properties. Close.
