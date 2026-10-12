# PBT Campaign: encode_bsr_bsf (i686)

## Scan findings
- **Spec:** (none found — no requirement doc; Intel SDM Vol.2 BSF/BSR + llvm-mc i686 as reference)
- **Test layout:** Project-owned Rust inline tests: `src/backend/i686/assembler/encoder/*_pbt.rs` registered via `#[cfg(test)] mod ...` in `encoder/mod.rs`; runner `cargo test --lib <filter>`. Framework: proptest 1.11.0 (dev-dependency).
- **Buildability probe:** `cargo test --lib encode_ldrsw_kat_llvm_mc_x0_x1 -- --test-threads=1` (user build contract) — pre-campaign success (pbt-out/build.log). Neighbouring PBT modules (`encode_bsr_bsf_16_pbt`, `encode_bit_count_pbt`) compile under the same cargo test harness.
- **Harness placement:** extend existing cargo lib test target — new file `src/backend/i686/assembler/encoder/encode_bsr_bsf_pbt.rs` + one `#[cfg(test)] mod encode_bsr_bsf_pbt;` line in `encoder/mod.rs` (rung 1).
- **Candidate modules:** encode_bsr_bsf (gp_integer.rs:984) — sole HARD-scope / change-surface target
- **Skipped modules:** all other functions in gp_integer.rs (HARD scope: test only encode_bsr_bsf); files outside src/backend/i686/assembler/encoder/gp_integer.rs; encode_bsr_bsf_16 (already covered in prior campaign; different function)

### Target analysis: encode_bsr_bsf
- Signature: `pub(super) fn encode_bsr_bsf(&mut self, ops: &[Operand], mnemonic: &str) -> Result<(), String>`
- Dispatch (mod.rs:270): `"bsrl" | "bsfl" | "bsr" | "bsf" => self.encode_bsr_bsf`
- Opcodes: bsfl/bsf → `0F BC /r`; bsrl/bsr → `0F BD /r`; dest in ModRM.reg (AT&T src,dst)
- Forms: (Register,Register) and (Memory,Register); arity≠2 and other shapes → Err
- Doc comment: (none on function)
- **Doc contract:** (none) — no function-level comment
- Oracle classification:
  - State machine: rejected — pure encoding, no lifecycle
  - Algebraic round-trip: rejected — no in-tree i686 decoder for BSF/BSR
  - **Differential (strongest):** llvm-mc `-triple=i686 -show-encoding` (LLVM 15) — independent assembler, shared AT&T contract
  - Algebraic.invariant: fixed `0F opc ModRM(3,dst,src)` layout for r32 RR
  - Algebraic.metamorphic: bare `bsf`/`bsr` ≡ `bsfl`/`bsrl` for r32 pairs
  - Negative/error: arity, wrong width, unsupported ops
- Defects found:
  1. Memory arm skips `emit_segment_prefix` before opcode
  2. Register arm accepts r16/r8 via `reg_num` aliasing

## Module: encode_bsr_bsf
- [x] Scan: identify targets
- [x] Plan: formalize properties
- [x] Test: write and run
- [x] Review: triage results

## Sweep
- [x] coverage_gaps round 1 (standard): no line-level data; file-level matcher missed Rust mangled `encode_bsr_bsf` (false NOT LINKED); nm + KAT prove link/execution. Documented behaviors already have properties (rr/mem success, segment, arity, width, unsupported, opc invariant, bare≡*l meta). No additional gap properties required.
