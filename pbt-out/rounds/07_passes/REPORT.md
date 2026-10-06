# PBT Campaign Report: src/passes (round 07)

## Summary

**Verdict:** 2 medium miscompiles to fix: (b1) constant_fold's 32-bit fast path sign-extends I8/I16 operands before logical ops, so `LShr((I8)0xB8,1)` folds to -36 instead of 92 (latent silent-wrong-constant path — narrow Phase 1 creates exactly those sub-int-typed instructions and the pipeline re-runs constant_fold); (b2) narrow Phase 2 rewrites an I64 `Or` of sign-extended operands down to I32 with no re-extension when the single use is the I64 return — `run_passes(-O2, x86-64)` then computes `f(1,-1) = 4294967295` instead of `-1`, violating the pass README's own "restricted to I32/U32 output" restriction. Both carry deterministic red regression tests and reproduce serially.
**Date:** 2026-10-05
**Repository:** claudes-c-compiler (/home/shuhao/fermat-users/leo/github/claudes-c-compiler)
**Modules tested:** src/passes (div_by_const, constant_fold, simplify, gvn+copy_prop, dce, if_convert, cfg_simplify, narrow, pipeline integration)
**Tests:** 14 PBT properties (+2 red-regression deterministics) this round
**Result:** 12 passing, 2 failing → 2 bugs (b1, b2)
**Change surface:** 5 changed functions — all round-06 committed PBT test code inside `src/ir` (pure addition; no production code), outside this round's scope; re-executed verbatim via `cargo test --lib ir::` (p5 round-06 red witness for #575 fails as documented, 8 ir tests pass).
**Coverage evidence:** none (native) — `coverage_gaps` answered "Not in a PBT campaign" (this round was resumed in a manual session; no instrumented coverage data exists). File-level evidence instead: every pass entry point in scope was executed by ≥1 property (see COVERAGE.md rows). The contract-surface sweep (standard tier, 1 round) was performed as a ledger review against the README contract table — every documented per-pass behavior in `src/passes/README.md` (rewrite tables, UB preservation, fixpoints, skip graph, narrowing restrictions) has a property; the two failures are exactly where code and README disagree.
**Tier:** standard

## Modules Tested

| Module | Tests | Bugs | Oracles Used |
|--------|-------|------|--------------|
| passes::div_by_const | 4 (P1–P4) | 0 | reference (C11 division semantics, native/widened/64-bit forms) |
| passes::constant_fold | 2 (P5–P6) + 1 red regression | 1 (b1) | differential (independent C11 interpreter), negative_error (div-by-zero UB preservation) |
| passes::simplify | 2 (P7–P8) | 0 | algebraic.metamorphic (identity injection, strength reduction) |
| passes::gvn + copy_prop | 1 (P9) | 0 | algebraic.invariant (CSE value preservation) |
| passes::dce | 1 (P10) | 0 | algebraic.invariant (dead-chain preservation) |
| passes::if_convert | 1 (P11) | 0 | algebraic.invariant (diamond→select preservation) |
| passes::cfg_simplify | 1 (P12) | 0 | algebraic.invariant (value + reachability + targets-in-range on random CFGs) |
| passes::narrow | 1 (P13) + 1 red regression | 1 (b2) | algebraic.invariant (three-phase narrowing preservation; negative side for unsigned cmps on signed source) |
| passes::run_passes (pipeline) | 1 (P14) | 0 | algebraic.invariant (end-to-end value preservation, loop shapes reaching licm/ivsr) |

Final serial run (RUST_TEST_THREADS=1, filter passes::, log run/passes_final_serial.log): 279 passed / 4 failed / 2 ignored in 631.91s — the 4 failures are exactly this round's two documented bug witnesses (p5, p13) and their two deterministic red regressions; no other passes:: test regressed. Baseline before this round (whole lib): 578/39/11 with all 39 = prior rounds' documented red witnesses, none touching src/passes.

## Bugs Found

### B1: constant_fold sign-extends sub-int operands before logical ops (LShr/UDiv/URem at I8/I16 fold wrong)
**Formal:** ∀ E ∈ all-constant expression DAGs over I8..U64: fold∘copy_prop to fixpoint leaves Return = Const(interp(E)) — the independent C11 interpreter is the reference. Falsified at E = `LShr(Const(I8(-72)), Const(I8(1)), I8)`.
**Contract evidence:** inferred (the IrType-typed BinOp semantics: the pass itself truncates operands to `ty` first, so the logical arms must zero-extend within `ty`'s width too; the `as u32` cast models the 32-bit types, not I8/I16)
**Documentation conflict:** (none — no comment speaks to the sub-int logical-op widening)
**Severity:** medium
**Counterexample:** one-instruction function `d = LShr(Const(I8(-72)), Const(I8(1)), I8)`, return d
**Expected / Actual:** 92 / -36 (UDiv form: 92 / -36)
**Impact:** latent silent miscompile — narrow Phase 1 moves I64 ops down to the narrowing cast's type (including I8/I16), and the pipeline re-runs constant_fold on the next iteration; any negative sub-int constant feeding a logical op folds wrong. No C-level repro found (the frontend promotes sub-int ops before emitting shifts).
**Root cause:** `fold_binop` (src/passes/constant_fold.rs:684) `is_32bit` arms: `(lhs as u32)` sign-extends the truncated sub-int value (0xB8 → 0xFFFFFFB8) before LShr/UDiv/URem.
**Bug report:** bug_reports/passes_constant_fold_subint_logical_fold.md
**Repro seed:** deterministic red witness (no seed needed): test_constant_fold_regression_subint_lshr_zero_extend
**Raw output:**
```
left: `Value(-36)`
right: `Value(92)`
test result: FAILED. 0 passed; 1 failed
```

### B2: narrow Phase 2 rewrites I64 bitwise ops to I32 without checking the use — I64 consumers get the zero-extended value
**Formal:** ∀ pat ∈ documented narrowing patterns, ∀ args: interp(narrow_function(f[pat]), args) = interp(f[pat], args). Falsified at f(a,b) = `Or(Cast a I32→I64, Cast b I32→I64, I64)`, args (1,-1); pipeline-confirmed via run_passes(-O2, X86_64).
**Contract evidence:** documented — src/passes/README.md "Integer Narrowing" Phase 2: "restricted to I32/U32 output to avoid sub-int register complications" (the restriction is not enforced — the code checks only the rewrite TARGET type, never the output USE); plus the pipeline's value-preservation contract
**Documentation conflict:** the README states the output restriction; the code (narrow.rs:264, rewrite at :346-356) never checks the use's type — the shipped document contradicts the shipped code.
**Severity:** medium
**Counterexample:** `fn f(a: i32, b: i32) -> i64 { %w0=Cast a; %w1=Cast b; %r=Or %w0,%w1,I64; Return %r }`, call f(1, -1)
**Expected / Actual:** -1 / 4294967295 (post-pipeline IR: `Or@I32` feeding the I64 Return with no Cast)
**Impact:** pipeline-level silent miscompile for any IR producer emitting the widen-widen-bitwise@I64 shape (And/Xor share the code path). No C-level repro found with straightforward sources (the frontend emits Or-at-I32 + single cast), but the pipeline contract is violated on well-formed input.
**Root cause:** `narrow_binops_without_cast` checks use_counts == 1, operand narrowability and target width — never the USE's required type; inserts no re-extension.
**Bug report:** bug_reports/passes_narrow_phase2_i64_use_miscompile.md
**Repro seed:** deterministic red witness (no seed needed): test_narrow_regression_phase2_i64_bitwise_i64_use
**Raw output:**
```
left: `Value(4294967295)`
right: `Value(-1)`
test result: FAILED. 0 passed; 1 failed
```

## Design Caveats (if any)

- **div_by_const unsigned power-of-2 modulo** is documented as the simplify pass's job, not div_by_const's (README: "Power-of-2 unsigned modulo is handled separately by the simplify pass as `x & (C-1)`"). P3 asserts the negative side (no rewrite by div_by_const) — documented behavior, not a bug. Doc evidence: src/passes/README.md div_by_const section.
- **compute_unsigned_magic_32 fallback**: "Some large divisors near u32::MAX cannot be handled with 32-bit magic numbers. Fall back to native division for these rare cases." (src/passes/div_by_const.rs, compute_unsigned_magic_32) — documented limitation on an accepted input; P3 excludes these divisors from the must-rewrite domain via the pass's own predicate (the value-preservation check still covers them when a rewrite does occur). Doc evidence: src/passes/div_by_const.rs (fn compute_unsigned_magic_32).
- **constant_fold's Select constant-condition rule** emits `Copy(Value(arm))` rather than the arm's resolved constant, and the const map is single-level `Copy(Const)` by design (fold_function doc comment); the documented deployment interleaves copy_prop (README: "constant_fold depends on copy_prop"). P5 models the interleaving with the real `propagate_copies`. Not a bug — pipeline resolves it.
- **fold's sub-int AShr promotion semantics** (`(int)255 >> 1 = 127`) matches C11's integer promotion; the test-side C11 interpreter models width-local sign-extension instead. The combination (AShr at unsigned sub-int type) is not frontend-reachable (frontend promotes before arithmetic) and is excluded from P5's generator domain on that ground. No SUT defect claimed.

## Test Files Created

| File | Tests |
|------|-------|
| src/passes/div_by_const.rs (`pbt_tests`) | P1–P4 |
| src/passes/constant_fold.rs (`pbt_tests`) | P5–P6 + red regression (b1) |
| src/passes/simplify.rs (`pbt_tests`) | P7–P8 |
| src/passes/gvn.rs (`pbt_tests`) | P9 |
| src/passes/dce.rs (`pbt_tests`) | P10 |
| src/passes/if_convert.rs (`pbt_tests`) | P11 |
| src/passes/cfg_simplify.rs (`pbt_tests`) | P12 |
| src/passes/narrow.rs (`pbt_tests`) | P13 + red regression (b2) |
| src/passes/mod.rs (`pbt_tests`) | P14 |
| src/passes/pbt_common.rs | shared C11 interpreter + builders (registered by one `#[cfg(test)]` line in mod.rs) |

## Reproduction

Whole suite (serial, as run for the final ledger):
```bash
cd /home/shuhao/fermat-users/leo/github/claudes-c-compiler
PATH="$HOME/.cargo/bin:$PATH" RUST_TEST_THREADS=1 cargo test --lib -- passes:: --nocapture
```
B1 (fails, red witness):
```bash
cd /home/shuhao/fermat-users/leo/github/claudes-c-compiler
PATH="$HOME/.cargo/bin:$PATH" RUST_TEST_THREADS=1 cargo test --lib passes::constant_fold::pbt_tests::test_constant_fold_regression_subint_lshr_zero_extend --exact --nocapture
```
B2 (fails, red witness):
```bash
cd /home/shuhao/fermat-users/leo/github/claudes-c-compiler
PATH="$HOME/.cargo/bin:$PATH" RUST_TEST_THREADS=1 cargo test --lib passes::narrow::pbt_tests::test_narrow_regression_phase2_i64_bitwise_i64_use --exact --nocapture
```
PBT properties replay persisted seeds automatically (proptest-regressions/passes/).

## Output Directories

- pbt-out/rounds/07_passes/ — this round: PLAN.md, PROPERTIES.md, REPORT.md (this file), report.json, COVERAGE.md, CHANGE_SURFACE.md, FUNCTION_INDEX.md, bug_reports/passes_constant_fold_subint_logical_fold.{md,html}, bug_reports/passes_narrow_phase2_i64_use_miscompile.{md,html}, run/ (probe, run1, run2, per-property serial logs, final serial suite log), build.log
- proptest-regressions/passes/ — persisted failing/replayed seeds (cfg_simplify.txt, constant_fold.txt, div_by_const.txt, narrow.txt)
- src/passes/*.rs — inline `pbt_tests` modules (repo convention) + src/passes/pbt_common.rs
- Customer-facing HTML: pbt-out/rounds/07_passes/bug_reports/passes_constant_fold_subint_logical_fold.html, pbt-out/rounds/07_passes/bug_reports/passes_narrow_phase2_i64_use_miscompile.html
