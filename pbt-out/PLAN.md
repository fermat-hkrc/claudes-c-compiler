# PBT Campaign: src/ir (round 06)

> Mirrored from pbt-out/rounds/06_ir/PLAN.md (canonical round artifact).

## Scan findings

- **Spec:** `src/ir/README.md` — the module's own design document (44 KB): IR type system,
  `IrConst` construction/coercion contracts (including the documented U8/U16/U32-as-I64
  zero-extension convention), float encoding utilities (`f64_to_f128_bytes`,
  `f64_to_x87_bytes` with format definitions), `IrBinOp`/`IrCmpOp` eval semantics
  (wrapping arithmetic, `None` for division/remainder by zero, "signed and unsigned
  comparison variants are equivalent" for floats), CFG analysis (`FlatAdj` CSR,
  `build_cfg` "Build predecessor and successor lists", Cooper-Harvey-Kennedy dominators,
  dominance frontiers), SSA construction (6-step mem2reg, "proper SSA form"), and phi
  elimination (copies in predecessors, trampolines for critical edges).
- **Test layout:** inline `#[cfg(test)] mod pbt_tests` / `mod pbt_regression` modules
  inside source files, run via `cargo test --lib` (repo convention; proptest 1.11.0
  dev-dependency; proptest-regressions/ holds persisted seeds).
- **Buildability probe:** `PATH="$HOME/.cargo/bin:$PATH" cargo test --lib` (unchanged
  project test target, scratch CWD `pbt-out/rounds/06_ir/run/`, log
  `pbt-out/rounds/06_ir/run/probe.log`) → **567 passed, 30 failed, 11 ignored**; all 30
  failures are prior rounds' documented intentionally-red bug witnesses; zero in
  `src/ir`. Rung 1 confirmed.
- **Harness placement:** rung 1 — inline `#[cfg(test)] mod pbt_tests` at the bottom of
  `src/ir/ops.rs`, `src/ir/constants.rs`, `src/ir/analysis.rs`,
  `src/ir/mem2reg/promote.rs`, `src/ir/mem2reg/phi_eliminate.rs`.
- **Change surface (commit:HEAD = round-05 archive commit `8bf7a66c`):** 11 changed
  functions, NONE inside this round's scope `src/ir`. They are (a) round-05 test
  scratch C files (`main` in pbt-out/rounds/05_sema/run/*.c — test data, not SUT code)
  and (b) round-04/05 test helpers and sema functions (`TShape::Leaf`,
  `is_line_marker`, `sema_of` ×2, `normalize_atomic_size_suffix`,
  `ctype_from_type_spec_with_derived`, `layout`) — every one targeted, tested, and
  closed by round 05 (rounds/05_sema/PLAN.md: P14/P15 + P1–P13, bugs b1–b5). Per this
  campaign's scope contract (src/ir only) they are recorded as covered-by-round-05 in
  COVERAGE.md rather than re-tested here.
- **Candidate modules:** ops.rs, constants.rs, analysis.rs, mem2reg (promote,
  phi_eliminate).
- **Skipped modules:** `src/ir/lowering/*` per-function (private LoweringContext
  methods; reached structurally through promote/eliminate on pipeline-shaped IR);
  `intrinsics.rs::is_pure` (static classification list, no independent oracle);
  `reexports.rs`/`mod.rs` (re-export hubs); change-surface functions (outside scope,
  covered by round 05).

## Module: ir

- [x] Scan: identify targets (spec read; FUNCTION_INDEX.md written; probe green)
- [x] Plan: formalize properties (13 entries approved with IR blocks; see PROPERTIES.md)
- [x] Test: write and run (16 PBT tests @512–1024 cases + KAT gates + 4 red regressions)
- [x] Review: triage results (5 failing → SUT bugs b1–b5, serial-reconfirmed with
  RUST_TEST_THREADS=1; 5 bug reports + deterministic red regressions)

## Contract-surface sweep (standard tier: 1 round — done)
- `coverage_gaps` called after the first full run. No line-level coverage (no
  gcovr/lcov; instrumentation inactive) — file-level symbol-presence evidence only.
  It reported only change-surface symbols (all outside src/ir; the 7 NOT LINKED
  entries are the known false negative for #[cfg(test)]-nested Rust fns, and the 4
  "linked main (...)" entries are round-05 scratch C files).
- No additional documented-behavior gap surfaced beyond the targeted contracts.
  Sweep round consumed; campaign closed on "tier's rounds done".
