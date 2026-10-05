# PBT Campaign Report: src/ir (round 06)

## Summary

**Verdict:** 5 confirmed SUT bugs to fix — 2 high (IrConst unsigned-constant writers
`cast_float_to_target`/`cast_long_double_to_target` and `coerce_to_with_src` violate
the documented zero-extension storage convention, so any constant fold reading a U8/U16
constant above its signed range through `to_i64()` silently gets a negative value),
1 medium (`f64_to_f128_bytes`/`f64_to_x87_bytes` misencode every f64 subnormal by ~2^51),
1 medium (`build_cfg` records duplicate predecessor entries, corrupting the preds/succs
transpose invariant six passes build on), 1 low (dominance frontiers can never contain
the entry block for itself; unreachable from today's C lowering but a definitional
violation and future-pass hazard). 11 properties passing, 5 failing (each failing
property serial-reconfirmed and backed by a red regression test).
**Date:** 2026-10-05
**Repository:** /home/shuhao/fermat-users/leo/github/claudes-c-compiler
**Modules tested:** src/ir (ops.rs, constants.rs, analysis.rs, mem2reg/promote.rs, mem2reg/phi_eliminate.rs)
**Tests:** 16 PBT properties (+4 KAT/red-regression deterministics) this round; full lib suite 578 passed / 35 failed / 11 ignored (baseline before this round: 567/30/11 — all 30 pre-existing failures are prior rounds' documented red witnesses; the 5 new failures are this round's bugs)
**Result:** 11 passing, 5 failing → 5 bugs (b1–b5)
**Change surface:** 11 changed functions, 0 with a property here — every one is a
round-05 test artifact or frontend/sema function OUTSIDE this round's scope
contract (src/ir), already covered and closed by round 05 (see
pbt-out/rounds/06_ir/PLAN.md "Change surface" and COVERAGE.md); the 4
error-handling changes among them carried round-05 failure-path properties
(P14/P15 + round-05 bugs b1–b5).
**Coverage evidence:** file-level (symbol presence) — no line-level coverage on this
machine (no gcovr/lcov; build tree predates the campaign; no flags injected by hand).
`coverage_gaps` returned only change-surface symbols: 7 NOT LINKED (known false
negative of the symbol probe for #[cfg(test)]-nested Rust fns, documented in
round-05 PLAN.md) and 4 "linked main (...)" scratch C files; no src/ir gap surfaced.

## Modules Tested

| Module | Tests | Bugs | Oracles Used |
|--------|-------|------|--------------|
| ir::ops | 5 (P1–P4, P4b) | 0 | algebraic (C11 division identity), negative_error, metamorphic (width), reference (IEEE 754 / two's complement) |
| ir::constants | 5 (P5–P8) + 3 red regressions | 3 (b1, b2, b3) | differential (same-job writers), reference (IEEE 754 field decoder), reference (C11 6.3.1.2) |
| ir::analysis | 4 (P9–P11, P11b) + 1 red regression | 2 (b4, b5) | invariant (transpose), reference (naive dominators dataflow, Cytron DF definition) |
| ir::mem2reg | 2 (P12, P13) | 0 | structural invariants (SSA validity / phi completeness / elimination preservation) |

## Bugs Found

### B1: cast_float_to_target / cast_long_double_to_target break the U8/U16/U32 zero-extension storage convention
**Formal:** ∀ ty ∈ {U8,U16,U32}, v ∈ [0, 2^bits(ty)): cast_float_to_target(v as f64, ty).to_i64() = v ∧ cast_long_double_to_target(v,bytes,ty).to_i64() = v (same as from_i64(v,ty).to_i64()).
**Contract evidence:** documented — src/ir/constants.rs:437 "Store unsigned sub-64-bit types (U8, U16, U32) as I64 with zero-extended values to preserve unsigned semantics. Storing them in their native IrConst variants (I8, I16, I32) would cause to_i64() to sign-extend, turning e.g. U8(255) into -1 instead of 255." The function's own doc claims the correct outcome ("200.0 as u8 = 200").
**Documentation conflict:** the author documents the convention the function then violates: the wrap `fv as u8` is bit-correct but the result is stored in the native variant the convention forbids. It does not declare the input invalid; the comment states the behavior IS handled ("= 200, not saturated"), which the code contradicts on read-back.
**Severity:** high
**Counterexample:** cast_float_to_target(200.0, IrType::U8) → IrConst::I8(-56) → to_i64() = -56
**Expected / Actual:** Some(200) / Some(-56)
**Impact:** constant folding / GVN / algebraic simplification read unsigned constants above the signed range (U8 > 127, U16 > 32767) as negative after any float→int constant cast — silent wrong folded values.
**Root cause:** src/ir/constants.rs:277 `IrType::U8 => IrConst::I8(fv as u8 as i8)` (and U16/I16, in both cast functions) instead of the I64 zero-extended form used by from_i64 and narrowed_to.
**Bug report:** pbt-out/rounds/06_ir/bug_reports/ir_constants_unsigned_repr_cast_float.md
**Repro seed:** proptest persisted seed in proptest-regressions/src/ir/constants.rs (shrunk: ty_idx=0, raw=272865408)
**Raw output:** `left: Some(-128), right: Some(128): cast_float_to_target(U8, 128) = I8(-128) reads back Some(-128)`

### B2: coerce_to_with_src early-return keeps the forbidden sign-extending variant for U8/U16 targets
**Formal:** ∀ ty ∈ {U8,U16}, v ∈ (2^(bits-1), 2^bits): from_i64(v, signed(ty)).coerce_to(ty).to_i64() = v.
**Contract evidence:** documented — same from_i64 convention comment (src/ir/constants.rs:437); narrowed_to (src/ir/constants.rs:579) implements the correct behavior, proving the two same-job writers disagree.
**Documentation conflict:** (none beyond b1's convention comment; the early-return arm carries no comment claiming intent)
**Severity:** high
**Counterexample:** IrConst::from_i64(200, IrType::I8).coerce_to(IrType::U8) → I8(-56) → to_i64() = -56
**Expected / Actual:** Some(200) / Some(-56)
**Impact:** the universal constant-to-instruction-type coercion step produces constants that read back negative for U8/U16 values above their signed range — same corruption class as b1 through the coercion path.
**Root cause:** src/ir/constants.rs:384 `(IrConst::I8(_), IrType::I8 | IrType::U8) => return *self` (same for I16/U16) — the unsigned target must fall through to from_i64 normalization.
**Bug report:** pbt-out/rounds/06_ir/bug_reports/ir_constants_coerce_unsigned_early_return.md
**Repro seed:** proptest-regressions/src/ir/constants.rs (shrunk: kind=0, raw=0)
**Raw output:** `left: Some(-128), right: Some(128): I8(-128).coerce_to(U8) = I8(-128) reads back -128`

### B3: f64 subnormals are misencoded by f64_to_f128_bytes and f64_to_x87_bytes
**Formal:** ∀ v ∈ finite f64 (incl. subnormals): ieee_decode_f128(f64_to_f128_bytes(v)) = v ∧ ieee_decode_x87(f64_to_x87_bytes(v)) = v.
**Contract evidence:** documented — src/ir/constants.rs:50 "Convert an f64 value to IEEE 754 binary128 (quad-precision) encoding" — a faithful encoding preserves the value; both target formats represent every f64 exactly.
**Documentation conflict:** (none — no comment addresses subnormals; the code's "Normal number" arm simply receives them)
**Severity:** medium
**Counterexample:** f64_to_f128_bytes(5e-324) decodes to 1.1125369292536e-308 (≈2^-1023) instead of 5e-324 (2^-1074)
**Expected / Actual:** 5e-324 / 1.1125369292536e-308
**Impact:** subnormal constants flowing into long-double data emission produce wrong bytes on ARM64/RISC-V (f128 emitted verbatim) and wrong x87 constants on x86.
**Root cause:** src/ir/constants.rs:50/:82 — `exp11 == 0 && mantissa52 != 0` (subnormals) falls through to the normal path, which assumes an implicit integer bit the subnormal does not have.
**Bug report:** pbt-out/rounds/06_ir/bug_reports/ir_constants_subnormal_f128_x87_encoding.md
**Repro seed:** proptest-regressions/src/ir/constants.rs (shrunk: v=5e-324)
**Raw output:** `Test failed: f128: v=0.0000…005 decoded=0.0000…011125369292536007`

### B4: build_cfg records duplicate predecessor entries for multi-target terminators with equal labels
**Formal:** ∀ CFG F: ∀ i,b: multiplicity of edge (i→b) in succs equals its multiplicity in preds ∧ neither list contains duplicates.
**Contract evidence:** documented — src/ir/analysis.rs:106 "Build predecessor and successor lists from the function's CFG. Returns (preds, succs) as flat adjacency lists (CSR format)." — one graph as a transpose pair; consumers named below count predecessors.
**Documentation conflict:** (none — no comment claims duplicate preds are intended; the succs `contains` guards show dedup intent)
**Severity:** medium
**Counterexample:** block0: CondBranch{cond:1, true:L1, false:L1}; block1: Return → preds[1] = [0,0], succs[0] = [1]
**Expected / Actual:** preds[1] = [0] / [0,0]
**Impact:** six passes consume preds counts (if_convert.rs:305/386 diamond/merge gates, gvn.rs:439 join detection, mem2reg's DF join gate and phi-cost estimate); today's effects are masked but the invariant they build on is violated — latent correctness trap. C-level `case 1: case 2:` naturally produces duplicate switch labels reaching this path.
**Root cause:** src/ir/analysis.rs:131-146 — `preds[f].push(i32)` runs unconditionally in the CondBranch/Switch/IndirectBranch arms while the succs push is `contains`-guarded.
**Bug report:** pbt-out/rounds/06_ir/bug_reports/ir_analysis_build_cfg_pred_dup.md
**Repro seed:** proptest-regressions/src/ir/analysis.rs
**Raw output:** `Test failed: duplicate predecessor 2 in row 2`

### B5: dominance frontiers can never contain the entry block for itself (entry cycles)
**Formal:** ∀ CFG F where entry participates in a cycle: entry ∈ DF(entry) (Cytron DF_local: y ∈ succ(n) ∧ n does not strictly dominate y).
**Contract evidence:** documented — src/ir/analysis.rs:302 "DF(b) = set of blocks where b's dominance ends (join points)"; Cytron et al. TOPLAS 1991 §2.
**Documentation conflict:** (none — no comment addresses the entry case; the ≥2-preds gate and runner-stop-at-idom[entry] are both silent on it)
**Severity:** low (not reachable from today's C lowering: no in-tree producer creates edges into the entry block — lower_label_stmt always starts a fresh block; filed for the definitional violation and future-pass hazard, e.g. block merging redirecting a back edge into entry)
**Counterexample:** single block, terminator Branch(entry) → DF(0) = {} but must contain 0 (witness p11b, both self-edge and 0→1→{0,1} cycle forms)
**Expected / Actual:** DF(0) ∋ 0 / DF(0) = {}
**Impact:** if a future pass creates an entry cycle, mem2reg would omit the entry phi for loop-carried variables → silent miscompile of the carried value.
**Root cause:** src/ir/analysis.rs:307 `if preds.len(b) < 2 { continue; }` plus the runner walk stopping at idom[entry]==entry.
**Bug report:** pbt-out/rounds/06_ir/bug_reports/ir_analysis_df_entry_cycle.md
**Repro seed:** deterministic witness (no seed needed)
**Raw output:** `Test failed: DF(0) mismatch (n=1, succs=[[0]]) left: {} right: {0}`

## Design Caveats (if any)

(none)

## Test Files Created

| File | Tests |
|------|-------|
| src/ir/ops.rs (`mod pbt_tests`) | 5 (P1, P2, P3, P4, P4b) |
| src/ir/constants.rs (`mod pbt_tests` + `mod pbt_regression`) | 5 + 3 red regressions |
| src/ir/analysis.rs (`mod pbt_tests` + `mod pbt_regression`) | 4 (incl. red witness P11b) + 1 red regression |
| src/ir/mem2reg/promote.rs (`mod pbt_tests`) | 1 (P12) |
| src/ir/mem2reg/phi_eliminate.rs (`mod pbt_tests`) | 1 (P13) |

## Reproduction

Whole suite (from the campaign scratch dir):
```bash
cd /home/shuhao/fermat-users/leo/github/claudes-c-compiler/pbt-out/rounds/06_ir/run
PATH="$HOME/.cargo/bin:$PATH" cargo test --lib ir::
PATH="$HOME/.cargo/bin:$PATH" RUST_TEST_THREADS=1 cargo test --lib ir::
```
Build (user contract, unchanged): `PATH="$HOME/.cargo/bin:$PATH" cargo check --lib` in
/home/shuhao/fermat-users/leo/github/claudes-c-compiler (prebuilt SUT log:
pbt-out/rounds/06_ir/build.log; this round's test compile:
`cargo test --lib ir:: --no-run` → clean).

Per bug (narrowed):
```bash
cd /home/shuhao/fermat-users/leo/github/claudes-c-compiler/pbt-out/rounds/06_ir/run
PATH="$HOME/.cargo/bin:$PATH" cargo test --lib ir::constants::pbt_tests::p5_unsigned_repr_writer_differential          # b1
PATH="$HOME/.cargo/bin:$PATH" cargo test --lib ir::constants::pbt_tests::p6_coerce_to_unsigned_normalization          # b2
PATH="$HOME/.cargo/bin:$PATH" cargo test --lib ir::constants::pbt_tests::p7_f128_x87_encoding_roundtrip               # b3
PATH="$HOME/.cargo/bin:$PATH" cargo test --lib ir::analysis::pbt_tests::p9_build_cfg_transpose_consistency            # b4
PATH="$HOME/.cargo/bin:$PATH" cargo test --lib ir::analysis::pbt_tests::p11b_entry_selfloop_df                        # b5
```
Deterministic red regressions: `cargo test --lib ir::constants::pbt_regression` and
`cargo test --lib ir::analysis::pbt_regression` (same workdir).

## Output Directories

All artifacts under pbt-out/rounds/06_ir/ (this round's campaign output); this
summary is the top-level close-out ledger pbt-out/REPORT.md:
- pbt-out/rounds/06_ir/PLAN.md, PROPERTIES.md, FUNCTION_INDEX.md, CHANGE_SURFACE.md (harness-provided)
- pbt-out/REPORT.md (this file) + pbt-out/REPORT.html (auto-rendered from report.json)
- pbt-out/rounds/06_ir/COVERAGE.md, COVERAGE_STATUS.md, INVARIANTS.md
- pbt-out/report.json (machine-readable; source of truth for REPORT.html)
- pbt-out/rounds/06_ir/bug_reports/ir_constants_unsigned_repr_cast_float.md (+ .html)
- pbt-out/rounds/06_ir/bug_reports/ir_constants_coerce_unsigned_early_return.md (+ .html)
- pbt-out/rounds/06_ir/bug_reports/ir_constants_subnormal_f128_x87_encoding.md (+ .html)
- pbt-out/rounds/06_ir/bug_reports/ir_analysis_build_cfg_pred_dup.md (+ .html)
- pbt-out/rounds/06_ir/bug_reports/ir_analysis_df_entry_cycle.md (+ .html)
- pbt-out/rounds/06_ir/run/probe.log (buildability probe), run/ (scratch CWD for all test runs)
- Tests live in the repo tree (rung 1, inline modules) — see Test Files Created.

## Coverage Report

# PBT Coverage Status

> Last updated: 2026-10-05 10:55 (campaign: coverage)
> Files: 8/53 scanned (15%) | Functions: 22/834 total | PBT candidates: 22 | Tested: 22 (100%) | 12 pass, 7 fail, 3 other

## Summary

| Metric | Value |
|--------|-------|
| Total source files | 53 |
| Files scanned | 8 / 53 (15%) |
| Total functions (all files) | 834 |
| PBT candidates (from FUNCTION_INDEX) | 22 |
| **Tested (of PBT candidates)** | **22 / 22 (100%)** |
| &nbsp;&nbsp;↳ Pass / Fail / Other | 12 / 7 / 3 |
| **Overall (tested / all functions)** | **22 / 834 (3%)** |
| Untested | 0 |
| Skipped | 0 |

## Module Breakdown

| Module | Scanned | Tested | Skipped | Coverage |
|--------|---------|--------|---------|----------|
|  | 22 | 22 | 0 | 100% |

## Oracle Type Distribution

| Oracle Type | Total | Covered | Skipped | Coverage |
|-------------|-------|---------|---------|----------|
| unknown | 22 | 22 | 0 | 100% |

## Recommended Focus

> **Priority 1 — Fix failing tests**
> These functions have failing PBT properties — fix before adding new tests.

| Function | Source |
|----------|--------|
| IrConst::cast_float_to_target | constants.rs |
| IrConst::cast_long_double_to_target | constants.rs |
| IrConst::coerce_to_with_src / coerce_to | constants.rs |
| f64_to_f128_bytes | constants.rs |
| f64_to_x87_bytes | constants.rs |
| build_cfg | analysis.rs |
| compute_dominance_frontiers | analysis.rs |
