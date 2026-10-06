# PBT Campaign: src/passes (round 07)

## Scan findings

- **Spec:** `src/passes/README.md` — the module's own design document (36 KB): four-phase
  pipeline (Phase 0 inline + cleanup, Phase 0.5 IsConstant resolution, main loop ≤3
  iterations with dirty tracking / per-pass skip graph / 5% diminishing-returns early exit,
  Phase 11 dead-static elimination); per-pass contracts with exact rewrite tables:
  cfg_simplify's seven sub-passes run to fixpoint, copy_prop's transitive chain resolution
  (depth ≤64, multi-def detection), div_by_const's Hacker's Delight magic-number
  rewrites ("All transformations produce correct results for the full range of inputs,
  including edge cases (0, 1, -1, INT_MIN, UINT_MAX, INT64_MIN, UINT64_MAX)"), narrow's
  three-phase widen-operate-narrow pattern table, simplify's integer identity table +
  strength reductions + float-restricted identities, constant_fold's C11/IEEE folding
  contract ("Integer division/remainder by zero is not folded (preserving undefined
  behavior)"), GVN/LICM/IVSR shared-analysis soundness notes, if_convert, DCE, inline
  eligibility/heuristics, IPCP, dead statics. Spec clauses carried into PROPERTIES.md
  with citations.
- **Test layout:** inline `#[cfg(test)] mod pbt_tests` / `mod pbt_regression` modules
  inside source files, run via `cargo test --lib` (repo convention from rounds 01–06; no
  `tests/` dir). proptest 1.11.0 is a declared dev-dependency (Cargo.toml).
  proptest-regressions/ holds persisted seeds per module path. Existing deterministic
  `mod tests` live inside 13 of the 17 pass files.
- **Buildability probe:** `PATH="$HOME/.cargo/bin:$PATH" cargo test --lib` (user build
  contract with the target swapped to the test target; run from the repo root, log
  `pbt-out/rounds/07_passes/run/probe.log`) → compiles clean; **578 passed, 39 failed,
  11 ignored**. All 39 failures are prior rounds' documented intentionally-red bug
  witnesses (round-01 common, round-02 lexer, round-03 preprocessor, round-05 sema,
  round-06 ir — 30 baseline + 9 round-06 red regressions). Zero failures touch
  `src/passes`. Rung 1 confirmed.
- **Harness placement:** rung 1 — extend the repo's own inline test convention. New
  `#[cfg(test)] mod pbt_tests` at the bottom of `src/passes/div_by_const.rs`,
  `constant_fold.rs`, `simplify.rs`, `gvn.rs`, `copy_prop.rs`, `dce.rs`,
  `if_convert.rs`, `cfg_simplify.rs`, `narrow.rs`, and `mod.rs` (pipeline integration),
  plus one shared `#[cfg(test)] pub(crate) mod pbt_common;` file
  (`src/passes/pbt_common.rs`, registered by one additive `#[cfg(test)]` line in
  `src/passes/mod.rs`) holding the independent C11-semantics IR interpreter and the
  random-function builders. Runs via the user contract command with the target swapped
  (`cargo test --lib passes::`).
- **Change surface (commit:HEAD = the round-06 archive commit `98e10b76`):** 5 changed
  functions, ALL of them round-06 PBT test code added under `#[cfg(test)] mod pbt_tests`
  inside `src/ir` files (pure 1288-line addition; no production code changed):
  `func_from_descriptors` (analysis.rs:374 test helper), `p5_unsigned_repr_writer_differential`
  (constants.rs:642 — a round-06 property that FOUND issue #575 and is intentionally red),
  `build_random_func` (phi_eliminate.rs:583 and promote.rs:1348 test helpers),
  `p1_div_rem_c11_identity` (ops.rs:218). These are outside this round's scope
  (`src/passes`) and are tests, not SUT code: they were themselves run by round 06 and are
  the round-06 ledger. Per this campaign's scope contract ("src/passes; outside this path
  only read necessary dependencies") they are re-executed verbatim as part of this
  campaign's probe (`cargo test --lib ir::` subset re-run below) and recorded in
  COVERAGE.md as covered-by-round-06 rather than re-derived here.
- **Candidate modules:** div_by_const.rs (magic-number correctness, native-32 and
  in-I64 widened forms, 64-bit I128 forms, negative divisors, rem identity),
  constant_fold.rs (C11/i128 differential + div-by-zero negative contract + idempotence),
  simplify.rs (documented identity table via metamorphic injection, strength reductions),
  gvn.rs + copy_prop.rs (CSE value preservation), dce.rs (dead-chain preservation),
  if_convert.rs (diamond→select preservation), cfg_simplify.rs (value preservation +
  reachability on random CFGs), narrow.rs (widen-operate-narrow preservation),
  mod.rs run_passes (end-to-end pipeline preservation incl. loop shapes reaching
  licm/ivsr), loop_analysis.rs (natural-loop structural invariants).
- **Skipped modules:** `inline.rs` / `ipcp.rs` / `dead_statics.rs` per-function properties
  — multi-function module-level passes needing cross-function IR (caller/callee pairs,
  static linkages); exercised structurally through the P14 `run_passes` pipeline property
  on 2-function modules rather than per-function properties (round budget; their own
  budgets/heuristic tables are size cutoffs, not value contracts). `resolve_asm.rs` —
  requires inline-asm symbol plumbing absent from the generated fragment. `loop_analysis.rs`
  direct properties — merged into P14's structured-loop shapes (find_natural_loops /
  merge_loops_by_header / find_preheader run inside licm/ivsr); noted, not separately
  enumerated. Change-surface functions — outside scope, test code, re-run green/red as
  committed by round 06 (see above).

## Module: passes

- [x] Scan: identify targets (spec read; FUNCTION_INDEX.md written; probe green)
- [x] Plan: formalize properties
- [x] Test: write and run
- [x] Review: triage results
