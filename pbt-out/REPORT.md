# PBT Campaign Report: src/backend root (round 08)

## Summary

**Verdict:** 1 low bug to fix (#765): `parse_integer_expr` debug-panics on multiplication overflow and shift amounts ≥ 64 instead of returning `Err` (release wraps silently — debug/release divergence); carried by a deterministic red regression. Everything else in the backend's target-independent core holds: cast classification matches its table laws exactly, call-ABI caller/callee classification agrees on every scalar list × 4 documented configs, liveness intervals match an independent textbook dataflow EXACTLY (512 random CFGs), and the linear-scan register allocator satisfies disjointness/callee-saved/eligibility soundness across 4 register-pool shapes.
**Date:** 2026-10-05
**Repository:** claudes-c-compiler (/home/shuhao/fermat-users/leo/github/claudes-c-compiler)
**Modules tested:** src/backend root files (cast, call_abi, liveness, regalloc, asm_expr + shared pbt_backend_common)
**Tests:** 8 PBT properties (+1 red-regression deterministic)
**Result:** 8 passing, 1 bug (#765)
**Change surface:** (none — whole-scope round; round-07 baseline commit 74fb6845)
**Coverage evidence:** none (native) — `coverage_gaps` answered "Not in a PBT campaign" (manual session). File-level: every root analysis/classification entry point executed by ≥1 property (COVERAGE.md); glue files skipped with per-file reasons in PLAN.md. Contract-surface sweep: the README's per-module contracts for the root files (cast table, call-ABI agreement + category table, liveness equations, three-phase regalloc + whitelist, SlotAddr/RegCache, F128 orchestration) each have a property or a recorded skip reason; the glue files (generation/traits/state/common/emission) are exercised by the per-arch rounds that consume them.
**Tier:** standard

## Modules Tested

| Module | Tests | Bugs | Oracles Used |
|--------|-------|------|--------------|
| backend::cast (classify_cast_with_f128) | 3 (P1–P3) | 0 | reference (table laws, independent type model), differential (flag independence) |
| backend::call_abi | 2 (P4–P5) | 0 | differential (caller vs callee agreement), algebraic.invariant (capacity/skip bookkeeping) |
| backend::liveness (compute_live_intervals) | 1 (P6) | 0 | reference (naive textbook dataflow, exact equality) |
| backend::regalloc (allocate_registers) | 1 (P7) | 0 | algebraic.invariant (disjoint/callee-saved/eligibility soundness) |
| backend::asm_expr (parse_integer_expr) | 1 (P8) + red regression | 1 (#765) | differential (independent evaluator; oracle adversarially verified against GNU as + llvm-mc) |

Final serial run (`cargo test --lib -- backend::`, RUST_TEST_THREADS=1): **201 passed / 1 failed (the #765 red witness) / 6 ignored, 0.47s**. Baseline before this round: 193/0/6 — no pre-existing test regressed.

## Bugs Found

### B1: parse_integer_expr panics in debug on arithmetic overflow and out-of-range shifts instead of returning Err
**Formal:** ∀ s ∈ expression strings: parse_integer_expr(s) never panics — it returns Ok(v) or Err(msg). Falsified by s = "9223372036854775807 * 9223372036854775807" (debug panic: attempt to multiply with overflow) and s = "1 << 64" (attempt to shift left with overflow).
**Contract evidence:** inferred (the `Result` return type is the error contract; every other invalid-input path — bad tokens, trailing junk — already returns Err)
**Documentation conflict:** (none — the operator list documents neither overflow nor panic behavior; GNU as warns "shift count out of range" and yields 0, llvm-mc wraps silently — the references disagree, so the Err contract is the repo's own Result signature)
**Severity:** low
**Counterexample:** call `parse_integer_expr("1 << 64")`
**Expected / Actual:** Err(...) / debug panic; release wraps to 1
**Impact:** debug-compiler crash on oversized asm constant expressions (macro-computed `.byte`/`.align` values); silent value change between debug and release profiles.
**Root cause:** eval_tokens' mul level uses plain `val *=` on i64 (asm_expr.rs:181); shift levels use plain `<<=`/`>>` with no amount check.
**Bug report:** bug_reports/backend_asmexpr_arith_panic.md
**Repro seed:** deterministic red witness: test_asmexpr_regression_arith_panic_not_err
**Raw output:**
```
thread '...' panicked at src/backend/asm_expr.rs:181:48:
attempt to multiply with overflow
```

## Design Caveats (if any)

- **LivenessResult.block_loop_depth is empty for zero-value functions** (the num_values==0 early return yields the empty triple) despite the struct doc "for each block (block_index -> depth)". Sole in-tree consumer (regalloc.rs:120) iterates the vec — caller-safe today. Doc evidence: src/backend/liveness.rs:51 field doc vs the early return at liveness.rs:164.
- **asm `>>` is LOGICAL (unsigned)**: verified GNU as AND llvm-mc both evaluate `(-1)>>1` to 0x7FFFFFFFFFFFFFFF — the SUT's u64 cast matches the reference assemblers; recorded because it contradicts naive C intuition (my first oracle assumed arithmetic shift and was wrong). Out-of-range shifts: GAS warns+0, llvm-mc wraps — no unified convention (context for #765's low severity).
- **classify_params_full always passes is_variadic=true** to the core, encoding variadicity in config.variadic_floats_in_gp (documented in its comment) — configs must be paired per-function; harness note for future rounds.

## Test Files Created

| File | Tests |
|------|-------|
| src/backend/cast.rs (`pbt_tests`) | P1–P3 |
| src/backend/call_abi.rs (`pbt_tests`) | P4–P5 |
| src/backend/liveness.rs (`pbt_tests`) | P6 |
| src/backend/regalloc.rs (`pbt_tests`) | P7 |
| src/backend/asm_expr.rs (`pbt_tests2` + `pbt_regression`) | P8 + red regression (#765) |
| src/backend/pbt_backend_common.rs | shared CFG builder + naive liveness reference (registered by one `#[cfg(test)]` line in backend/mod.rs) |

## Reproduction

Whole round (serial):
```bash
cd /home/shuhao/fermat-users/leo/github/claudes-c-compiler
PATH="$HOME/.cargo/bin:$PATH" RUST_TEST_THREADS=1 cargo test --lib -- backend:: --nocapture
```
B1 (fails, red witness):
```bash
cd /home/shuhao/fermat-users/leo/github/claudes-c-compiler
PATH="$HOME/.cargo/bin:$PATH" RUST_TEST_THREADS=1 cargo test --lib backend::asm_expr::pbt_regression::test_asmexpr_regression_arith_panic_not_err --exact --nocapture
```

## Output Directories

- pbt-out/rounds/08_backend_core/ — PLAN.md, PROPERTIES.md, REPORT.md (this file), report.json, COVERAGE.md, bug_reports/backend_asmexpr_arith_panic.{md,html}, run/ (probe.log, backend_final_serial.log)
- Customer-facing HTML: pbt-out/rounds/08_backend_core/bug_reports/backend_asmexpr_arith_panic.html (+ copy at pbt-out/bug_reports/)
- proptest seed replays: proptest-regressions/ (per-file, replayed automatically)
