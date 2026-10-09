# PBT Campaign: encode_inc_dec (i686)

## Scan findings
- **Spec:** (none found — symbol-targeted campaign via `--func encode_inc_dec`)
- **Test layout:** Project-owned Rust module tests under `src/backend/i686/assembler/encoder/*_pbt.rs`, registered with `#[cfg(test)] mod ...` in `encoder/mod.rs`; runner is `cargo test --lib <filter>`. Framework: proptest (dev-dep in Cargo.toml).
- **Buildability probe:** `cargo test --lib encode_ldrsw_kat_llvm_mc_x0_x1 -- --test-threads=1` → pass (prebuilt SUT; log pbt-out/build.log). Canonical contract form: `cargo test --lib <target> -- --test-threads=1`.
- **Harness placement:** extend existing test target (rung 1) — new file `src/backend/i686/assembler/encoder/encode_inc_dec_pbt.rs` + one `#[cfg(test)] mod encode_inc_dec_pbt;` line in `mod.rs`.
- **Candidate modules:** encode_inc_dec in `src/backend/i686/assembler/encoder/gp_integer.rs:806` (HARD scope sole target)
- **Skipped modules:** all other functions in gp_integer.rs and outside scope — HARD scope is `encode_inc_dec` only.

### Target analysis
- Doc contract (gp_integer.rs:801-805): 32-bit mode compact 0x40+reg / 0x48+reg for size=4 regs; FE/FF /0|/1 for mem and byte forms; 0x66 for word.
- Dispatch (mod.rs:233-238): incl/inc→(0,4), incw→(0,2), incb→(0,1), decl/dec→(1,4), decw→(1,2), decb→(1,1).
- Memory arm does **not** call `emit_segment_prefix` (core.rs:31-42) — same defect class as prior i686 campaigns.
- Register arm uses `reg_num` which aliases xmm/mm/st → accepts non-GP.
- Register arm does not gate `reg_size(name) == size` → width-mismatched regs encode as same-numbered GP.

### Oracle classification
- State machine: rejected — pure encoding, no lifecycle
- Differential: primary — llvm-mc `-triple=i686 -show-encoding` (LLVM 15), independent assembler
- Algebraic.invariant: compact opcode 0x40+n / 0x48+n; FE/FF /ext
- Algebraic.metamorphic: segmented mem = seg_prefix ‖ bare mem
- Negative/error: arity ≠ 1; non-GP; mismatched width

## Module: encode_inc_dec
- [x] Scan: identify targets
- [ ] Plan: formalize properties
- [ ] Test: write and run
- [ ] Review: triage results
