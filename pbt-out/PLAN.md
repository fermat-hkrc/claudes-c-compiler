# PBT Campaign: encode_alu (i686)

## Scan findings
- **Spec:** (none found — function-scoped campaign via `--func encode_alu`)
- **Test layout:** project-owned cargo lib tests; PBT modules live as `src/backend/i686/assembler/encoder/*_pbt.rs` registered via `#[cfg(test)] mod` in `encoder/mod.rs`; runner `cargo test --lib <filter> -- --test-threads=1`; framework proptest 1.11.0 (dev-dependency).
- **Buildability probe:** `cargo test --lib encode_ldrsw_kat_llvm_mc_x0_x1 -- --test-threads=1` (user build contract) — pre-campaign success, log at `pbt-out/build.log`; re-probe this campaign: 2 passed, 5607 filtered out.
- **Harness placement:** extend existing cargo lib-test target — `src/backend/i686/assembler/encoder/encode_alu_pbt.rs` + one `#[cfg(test)] mod encode_alu_pbt;` line in `encoder/mod.rs` (rung 1).
- **Candidate modules:** encode_alu (gp_integer.rs:433) — sole HARD-scope target.
- **Skipped modules:** all other gp_integer symbols (encode_pop, encode_push, encode_mov_*, …) — HARD scope is encode_alu only; HEAD changes outside `src/backend/i686/assembler/encoder/gp_integer.rs` skipped.
- **Doc contract:** no doc comment on encode_alu; inline comments document short-form EAX imm32 (line 457), GOTPC for `_GLOBAL_OFFSET_TABLE_`, SymbolDiff / label-as-memory forms.
- **Oracle class:** Differential (llvm-mc -triple=i686) primary; algebraic.invariant (opcode = base+alu_op*8, ModRM); algebraic.metamorphic (segmented mem = seg_prefix ‖ bare); negative_error (arity, mismatched width, non-GP).
- **Evidence:** Intel SDM Vol.2 ADD/OR/ADC/SBB/AND/SUB/XOR/CMP; mod.rs:204-211; x86-64 sibling encode_alu calls emit_segment_prefix; core.rs:30-42 emit_segment_prefix.
- **Seed:** encode_mov_rr_pbt.rs / encode_pop_pbt.rs / encode_mov_reg_mem_pbt.rs patterns.
- **Findings (Test):** 4 SUT bugs — missing segment prefix; AL imm8 short form omitted; mismatched-width GP accepted; non-GP accepted via reg_num.

## Module: encode_alu
- [x] Scan: identify targets
- [x] Plan: formalize properties
- [x] Test: write and run
- [x] Review: triage results
  - [x] Contract-surface sweep (coverage_gaps ×1 — no Rust .profraw; residual GOTPC/SymbolDiff/Label reloc arms noted, not invented)
