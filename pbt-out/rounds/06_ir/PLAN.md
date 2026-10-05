# PBT Campaign: src/ir (round 06)

## Scan findings

- **Spec:** `src/ir/README.md` — the module's own design document (44 KB): IR type system,
  `IrConst` construction/coercion contracts (including the documented U8/U16/U32-as-I64
  zero-extension convention), float encoding utilities (`f64_to_f128_bytes`,
  `f64_to_x87_bytes` with format definitions), `IrBinOp`/`IrCmpOp` eval semantics
  (wrapping arithmetic, `None` for division/remainder by zero, "signed and unsigned
  comparison variants are equivalent" for floats), CFG analysis (`FlatAdj` CSR,
  `build_cfg` "Build predecessor and successor lists", Cooper-Harvey-Kennedy dominators,
  dominance frontiers), SSA construction (6-step mem2reg, "proper SSA form"), and phi
  elimination (copies in predecessors, trampolines for critical edges). Spec clauses
  carried into PROPERTIES.md with citations.
- **Test layout:** inline `#[cfg(test)] mod pbt_tests` / `mod pbt_regression` modules
  inside source files, run via `cargo test --lib` (repo convention from `src/common/*`,
  `src/frontend/*`, rounds 01–05; no `tests/` dir; `pub(crate)` visibility rules out
  integration tests). proptest 1.11.0 is a declared dev-dependency (Cargo.toml).
  proptest-regressions/ holds persisted seeds per module path.
- **Buildability probe:** `PATH="$HOME/.cargo/bin:$PATH" cargo test --lib` (unchanged
  project test target, scratch CWD `pbt-out/rounds/06_ir/run/`, log
  `pbt-out/rounds/06_ir/run/probe.log`) → compiles clean; **567 passed, 30 failed,
  11 ignored**. All 30 failures are prior rounds' documented intentionally-red bug
  witnesses (round-01 `common::long_double`/`common::const_eval`, round-02
  `frontend::lexer`, round-03 `frontend::preprocessor`, round-05 `frontend::sema` —
  same set as recorded in rounds/05_sema/PLAN.md plus round-05's own 10 new red
  witnesses). Zero failures touch `src/ir`; `src/ir` has only 4 small deterministic
  unit tests (promote/phi_eliminate) and NO property tests. Rung 1 confirmed.
- **Harness placement:** rung 1 — extend the repo's own inline test convention. New
  `#[cfg(test)] mod pbt_tests` at the bottom of `src/ir/ops.rs`, `src/ir/constants.rs`,
  `src/ir/analysis.rs`, `src/ir/mem2reg/promote.rs`, `src/ir/mem2reg/phi_eliminate.rs`
  (`IrFunction::new` and `FlatAdj::from_vecs_usize` are existing `#[cfg(test)]`
  constructors usable from these modules). Runs from scratch CWD
  `pbt-out/rounds/06_ir/run/` via the user contract command with the target swapped
  (`cargo test --lib ir::`).
- **Change surface (commit:HEAD = the round-05 archive commit `8bf7a66c`):** 11 changed
  functions, NONE inside this round's scope `src/ir`. They are (a) round-05 test scratch
  C files (`main` in pbt-out/rounds/05_sema/run/{negu,ovf,uackat,uackat2}.c — test data,
  not SUT code), and (b) round-04/05 test helpers and sema functions
  (`TShape::Leaf`, `is_line_marker`, `sema_of` ×2, `normalize_atomic_size_suffix`,
  `ctype_from_type_spec_with_derived`, `layout`) — every one of which was targeted,
  tested, and closed by round 05 (see pbt-out/rounds/05_sema/PLAN.md: P14/P15 + P1–P13,
  5 bug reports b1–b5). Per this campaign's scope contract ("src/ir; outside this path
  only read necessary dependencies; do not expand into a whole-repo campaign") they are
  recorded as covered-by-round-05 in COVERAGE.md rather than re-tested here.
- **Candidate modules:** ops.rs (IrBinOp/IrCmpOp eval_i64/eval_i128/eval_f64,
  can_trap/is_commutative), constants.rs (IrConst from_i64/coerce_to_with_src/
  cast_float_to_target representation convention, f64_to_f128_bytes/f64_to_x87_bytes
  IEEE-754 encoding, bool_normalize C11 6.3.1.2), analysis.rs (build_cfg transpose
  consistency, compute_dominators vs textbook reference, compute_dominance_frontiers
  vs definition), mem2reg (promote SSA validity + phi completeness on random IR,
  phi_eliminate structural preservation).
- **Skipped modules:** `src/ir/lowering/*` (40 files, ~720 fns) — not skipped as a
  module: its public surface is reached end-to-end through `promote_allocas` on
  pipeline-produced IR, but per-function properties are impractical (every fn is a
  private `LoweringContext` method driven by AST+sema state; round budget). Individual
  direct property targets were the five files above. `intrinsics.rs::is_pure` — static
  classification table, exercised indirectly by passes (round budget); no independent
  oracle beyond its own list. `reexports.rs`/`mod.rs` — re-export hubs, no logic.
  Change-surface functions listed above — outside scope, covered by round 05.

## Module: ir

- [x] Scan: identify targets (spec read; FUNCTION_INDEX.md written; probe green)
- [x] Plan: formalize properties (13 entries approved with IR blocks; see PROPERTIES.md)
- [x] Test: write and run (13 properties @1024 cases + 4 KAT gates; run log in run/)
- [x] Review: triage results (see REPORT.md; 5 failing → SUT bugs b1–b5, serial-reconfirmed)

## Contract-surface sweep (standard tier: 1 round — done)
- `coverage_gaps` called after the first full run. No line-level coverage (no
  gcovr/lcov on this box; instrumentation inactive) — file-level symbol-presence
  evidence only. All five property target files are linked into the `cargo test --lib`
  binary and executed (properties call the SUT functions directly). The change-surface
  symbols resolve to frontend/sema files outside `src/ir` scope — covered by round 05.
- No additional documented-behavior gap surfaced beyond the targeted contracts (each
  documented behavior of ops.rs/constants.rs/analysis.rs/mem2reg carries a property in
  PROPERTIES.md; the lowering tree is exercised structurally through P12/P13).
  Sweep round consumed; campaign closed on "tier's rounds done".
