# PBT Campaign: encode_bt (i686)

## Scan findings
- **Spec:** (none found — no requirement doc; Intel SDM Vol.2 BT/BTS/BTR/BTC + llvm-mc i686 as reference)
- **Test layout:** Project-owned Rust inline tests: `src/backend/i686/assembler/encoder/*_pbt.rs` registered via `#[cfg(test)] mod ...` in `encoder/mod.rs`; runner `cargo test --lib <filter>`. Framework: proptest 1.11.0 (dev-dependency in Cargo.toml).
- **Buildability probe:** `cargo test --lib encode_ldrsw_kat_llvm_mc_x0_x1 -- --test-threads=1` (user build contract) — pre-campaign success (pbt-out/build.log). Neighbouring PBT modules (`encode_test_pbt`, `encode_bsr_bsf_pbt`) compile under the same cargo test harness.
- **Harness placement:** extend existing cargo lib test target — new file `src/backend/i686/assembler/encoder/encode_bt_pbt.rs` + one `#[cfg(test)] mod encode_bt_pbt;` line in `encoder/mod.rs` (rung 1).
- **Candidate modules:** encode_bt (gp_integer.rs:1012) — sole HARD-scope / change-surface target
- **Skipped modules:** all other functions in gp_integer.rs (HARD scope: test only encode_bt); files outside src/backend/i686/assembler/encoder/gp_integer.rs

### Target analysis: encode_bt
- Signature: `pub(super) fn encode_bt(&mut self, ops: &[Operand], mnemonic: &str) -> Result<(), String>`
- Dispatch (mod.rs:274): `"btl" | "btsl" | "btrl" | "btcl" | "bt" | "bts" | "btr" | "btc" => self.encode_bt`
- Opcodes (Intel SDM / AT&T bit-offset, bit-base):
  - RR/RM: bt→0F A3, bts→0F AB, btr→0F B3, btc→0F BB; ModRM.reg = offset reg
  - Imm: 0F BA /ext with ext={bt:4,bts:5,btr:6,btc:7} + imm8
  - Forms: (Reg,Reg), (Reg,Mem), (Imm,Reg), (Imm,Mem), (Imm,Label), (Reg,Label)
- Doc comment: (none on function)
- **Doc contract:** (none) — no function-level comment
- Oracle classification:
  - State machine: rejected — pure encoding, no lifecycle
  - Algebraic round-trip: rejected — no in-tree i686 decoder for BT family
  - **Differential (strongest):** llvm-mc `-triple=i686 -show-encoding` (LLVM 15)
  - Algebraic.invariant / algebraic.metamorphic / negative_error as secondary
- Defects found:
  1. Memory arms skip `emit_segment_prefix` before opcode
  2. Register arms accept r16/r8 via `reg_num` aliasing
  3. Imm arms truncate OOR values with `*val as u8`

## Module: encode_bt
- [x] Scan: identify targets
- [x] Plan: formalize properties
- [x] Test: write and run
- [x] Review: triage results

## Sweep
- [x] coverage_gaps round 1 (standard): no line-level data; file-level matcher missed Rust mangled `encode_bt` (false NOT LINKED); nm/KAT/proptest runs prove link/execution. Documented behaviors already have properties (rr/imm/mem/label success, segment, arity, width, OOR imm, opc invariants, bare≡*l meta). No additional gap properties required.
