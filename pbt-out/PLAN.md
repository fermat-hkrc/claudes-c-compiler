# PBT Campaign: encode_smsw (i686)

## Scan findings
- **Spec:** (none found beyond in-source doc comment) — Intel SDM SMSW 0F 01 /4; doc at system.rs:222-224
- **Test layout:** project-owned Rust inline/module tests under `src/backend/i686/assembler/encoder/`; pattern `encode_*_pbt.rs` + `#[cfg(test)] mod` in `encoder/mod.rs`; runner `cargo test --lib <filter> -- --test-threads=1`; framework `proptest = "1.11.0"` (dev-dependency)
- **Buildability probe:** `cargo test --lib encode_ldrsw_kat_llvm_mc_x0_x1 -- --test-threads=1` (user build contract) → success (pbt-out/build.log). Sibling probe pattern `cargo test --lib encode_lmsw -- --test-threads=1` is the established harness for this encoder family.
- **Harness placement:** extend existing project test target (rung 1) — new `src/backend/i686/assembler/encoder/encode_smsw_pbt.rs` + one `#[cfg(test)] mod encode_smsw_pbt;` line in `encoder/mod.rs`
- **Candidate modules:** encode_smsw (system.rs:225) — sole HARD-scope target
- **Skipped modules:** all other functions in system.rs (HARD: test only encode_smsw); HEAD changes outside src/backend/i686/assembler/encoder/system.rs

### Doc contract (encode_smsw)
- system.rs:222-224: "Encode SMSW (Store Machine Status Word): 0F 01 /4 / Accepts a 16-bit register or memory operand. / Register form gets a 66h prefix for 16-bit operand size." — asserted (opcode/ext + 66h for r16); domain note incomplete vs Intel/llvm-mc which also accept r32 (smswl). fingerprint 7a3c9e12
- Evidence for differential: llvm-mc -triple=i686; sibling LMSW/system_table/invlpg campaigns; core.rs emit_segment_prefix; Intel SDM SMSW r/m16 and r32/m16

### Oracle classification
- State machine: rejected — pure encoding, no lifecycle
- Algebraic round-trip: rejected — no in-tree i686 decoder for smsw
- Differential (strongest): encode_smsw vs llvm-mc i686 (independent assembler reference)
- Algebraic invariant: 0F 01 /4 + 0x66 on r16
- Algebraic metamorphic: same mem shape as lidt; only ModRM.reg differs (4 vs 3)
- Negative/error: arity ≠ 1; imm/label; 8-bit registers (llvm-mc rejects)

### Results
- 9 properties passing, 3 failing (2 root-cause bugs)
- B1: missing segment prefix (high)
- B2: accepts r8 register (medium)
- Contract-surface sweep: 1 round closed — every documented behavior has a property

## Module: encode_smsw
- [x] Scan: identify targets
- [x] Plan: formalize properties
- [x] Test: write and run
- [x] Review: triage results
