# PBT Campaign: encode_mov_seg (i686)

## Scan findings
- **Spec:** (none found beyond in-source doc comment) — Intel SDM MOV to/from segment register: 8C /r (Sreg→r/m16) and 8E /r (r/m16→Sreg); on IA-32 r32 forms zero-extend; doc at system.rs:273
- **Test layout:** project-owned Rust module tests under `src/backend/i686/assembler/encoder/`; pattern `encode_*_pbt.rs` + `#[cfg(test)] mod` in `encoder/mod.rs`; runner `cargo test --lib <filter> -- --test-threads=1`; framework `proptest = "1.11.0"` (dev-dependency)
- **Buildability probe:** `cargo test --lib encode_mov_cr_kat_llvm_mc_cr0_eax -- --test-threads=1` → PASS (1 passed; existing suite builds/runs)
- **Harness placement:** extend existing cargo lib-test target — new file `src/backend/i686/assembler/encoder/encode_mov_seg_pbt.rs` + one `#[cfg(test)] mod encode_mov_seg_pbt;` line in `encoder/mod.rs` (rung 1)
- **Candidate modules:** encode_mov_seg (system.rs:274) — sole HARD-scope target
- **Skipped modules:** encode_prefetch, encode_prefetch_0f0d, encode_out, encode_in, encode_invlpg, encode_verw, encode_lsl, encode_system_table, encode_lmsw, encode_smsw, encode_mov_cr, encode_pop16, encode_bsr_bsf_16 — HARD scope: test only encode_mov_seg; HEAD/prior-campaign symbols outside this function
- **Oracle (strongest):** differential vs llvm-mc `-triple=i686 -show-encoding` (independent assembler reference); plus algebraic.invariant (8C|8E + ModRM.reg = sreg), algebraic.metamorphic (sreg↔gp share ModRM; only opc 8C↔8E differs), negative_error (arity / r8 / mismatched width / non-seg pairs)
- **Doc contract:** system.rs:273 `"Encode MOV to/from segment register"` — asserted fingerprint f66b8f37
- **Dispatch path:** `encode_mov` (gp_integer.rs:21-34) routes when either operand is `is_segment_reg` (es/cs/ss/ds/fs/gs) or Register+Memory with segment; public entry via mnemonic `movl`/`movw`/`mov`
- **Known risk classes (from siblings):** (1) memory arms skip `emit_segment_prefix`; (2) `reg_num` aliases r8/r16 onto r32 encoding; (3) size/mnemonic ignored once routed — `movw %sreg, %r16` needs 0x66
- **Contract-surface sweep:** 1 round via `coverage_gaps` after first full run — documented behaviors covered: sreg↔gp32/r16 differential, memory base/disp/SIB/abs, segmented mem (+SIB strengthen), opcode/ModRM invariant, read↔write metamorphic, arity error, r8 negative, mnemonic aliases. Three root-cause bugs filed. Closed because tier's one sweep round is done and every documented behavior has a property.

## Module: encode_mov_seg
- [x] Scan: identify targets
- [x] Plan: formalize properties
- [x] Test: write and run
- [x] Review: triage results
