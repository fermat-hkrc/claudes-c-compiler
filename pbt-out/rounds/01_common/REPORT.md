# PBT Campaign Report: src/common (ccc C compiler)

## Summary

**Verdict:** Yes — fix now. 1 critical: `f64_to_f128_bytes_lossless` underflows (`biased_exp as u128 - 1023`) for **every** f64 with |x| < 1.0 — the compiler panics in debug builds and silently emits garbage-magnitude long double constants in release builds on ARM64/RISC-V. Plus 1 high (subnormal f64 → x87 mis-encode), 2 medium (f128→f64 truncating conversion off by 1 ulp — same-job sibling gate satisfied, see B3; `__builtin_bswap32` folding unsigned results as signed I32).
**Date:** 2026-02-27
**Repository:** /home/shuhao/fermat-users/leo/github/claudes-c-compiler
**Modules tested:** common::encoding, common::const_arith, common::const_eval, common::long_double, common::types, common::fx_hash, common::symbol_table
**Tests:** 16 property-based tests (1024 generated cases each, proptest 1.11.0) + 4 deterministic regression witnesses
**Result:** 12 passing, 6 failing entries → 4 SUT bugs (B1 critical, B2 high, B3 medium, B4 medium)
**Change surface:** (no change source given — commit:HEAD touches no source function; whole-scope campaign over src/common)
**Coverage evidence:** file-level (symbol presence) — no line-level data: this Rust build produced no .gcda/.profraw and the coverage_gaps tool's symbol-presence fallback is unreliable here (it lists functions the tests demonstrably executed — Rust inlining drops symbols). Execution evidence for tested functions is the passing/failing properties themselves (direct calls). Recorded in COVERAGE_STATUS.md.
**Tier:** standard (≈30 min, 5–8+ properties per target, ≥1000 runs set explicitly, 1 contract-surface sweep round, 1 metamorphic/differential property required — P9/P10 delivered).

**Build (user contract, run verbatim):** `PATH="$HOME/.cargo/bin:$PATH" cargo check --lib` in /home/shuhao/fermat-users/leo/github/claudes-c-compiler → success (1 pre-existing warning, exit 0). Test-target swap of the same command: `PATH="$HOME/.cargo/bin:$PATH" cargo test --lib --no-run` → built real test binary `target/debug/deps/ccc-405c0348f94fe50d` (exit 0); full run `cargo test --lib -- --test-threads=1` → 506 passed, 6 failed (the 6 PBT witness properties below), 10 ignored. The pre-existing suite (493 baseline) is unbroken.

## Modules Tested

| Module | Tests | Bugs | Oracles Used |
|--------|-------|------|--------------|
| common::encoding | 1 | 0 | algebraic.round_trip (PUA encode/decode) |
| common::const_arith | 6 | 0 | reference (bit ops), algebraic (C99 div/rem identity), negative_error (div by zero), invariant (comparison totality), differential (native f64), involution |
| common::const_eval | 3 | 1 (B4) | reference differential (GCC/Rust native bitops), involution |
| common::long_double | 6 | 3 (B1, B2, B3) | algebraic.round_trip, metamorphic/differential (single-rounding vs native f64; same-job converter bridge) |
| common::types | 1 | 0 | algebraic.invariant (align_up laws) |
| common::fx_hash | 1 | 0 | algebraic.invariant (Hasher write consistency) |
| common::symbol_table | 1 | 0 | state_machine (scope shadowing) |

## Bugs Found

### B1: f64_to_f128_bytes_lossless corrupts every |x| < 1.0 (u128 underflow) — CRITICAL
**Formal:** ∀ finite x:f64. f128_bytes_to_f64(f64_to_f128_bytes_lossless(x)).to_bits() = x.to_bits()
**Contract evidence:** inferred (function name and role: the "lossless" f64→f128 conversion used for f128 long-double emission and arithmetic; the value domain is all finite f64)
**Documentation conflict:** (none — no comment addresses the < 1.0 range; the f128 path's doc says it exists to "preserve full 112-bit precision")
**Severity:** critical
**Counterexample:** `f64_to_f128_bytes_lossless(0.5)` (biased_exp = 1022): `biased_exp as u128 - 1023` underflows → panic in debug, garbage exponent in release. Subnormals (biased_exp = 0) included.
**Expected / Actual:** round-trip bitwise-identical / panic `attempt to subtract with overflow` at long_double.rs:1040 (debug) or silently wrong constant (release)
**Impact:** Every double/long double constant < 1.0 on ARM64/RISC-V: compiler panic (debug) or silent wrong-code (release).
**Root cause:** long_double.rs:1040 `let exp15 = (d.biased_exp as u128 - 1023 + 16383) as u128;` — unsigned intermediate underflow; also no subnormal branch.
**Bug report:** rounds/01_common/bug_reports/f64_to_f128_bytes_lossless_lt1_underflow.md
**Repro seed:** deterministic regression test (no seed needed)
**Raw output:** `thread 'common::long_double::pbt_tests::pbt_p7_f128_lossless_roundtrip' panicked at src/common/long_double.rs:1040:18: attempt to subtract with overflow`

### B2: f64_to_x87_bytes_simple mis-encodes subnormal f64 values — HIGH
**Formal:** ∀ finite x:f64. x87_bytes_to_f64(f64_to_x87_bytes_simple(x)).to_bits() = x.to_bits()
**Contract evidence:** documented src/common/long_double.rs:1141 "This is a widening conversion that zero-fills the extra mantissa bits." — widening is exact, so narrowing must restore the value
**Documentation conflict:** the doc says the conversion is an exact widening; the code has no subnormal branch and assumes an implicit leading 1 that subnormals do not have.
**Severity:** high
**Counterexample:** `f64_to_x87_bytes_simple(5e-324)` (bits 0x1): encodes ≈2.22e-308, decodes to +0.0.
**Expected / Actual:** decode back to 5e-324 / decode to 0.0
**Impact:** Subnormal double constants converted through this x86/i686 path become ~2^752 times larger — silent wrong constant.
**Root cause:** long_double.rs:1143 — only zero/special/normal branches; subnormals fall into the normal path with a bogus implicit bit.
**Bug report:** rounds/01_common/bug_reports/f64_to_x87_bytes_simple_subnormal.md
**Repro seed:** deterministic (bits = 1)
**Raw output:** `left: 0, right: 1: x=5e-324 bits=0x1`

### B3: f128_bytes_to_f64 truncates instead of round-to-nearest — MEDIUM (same-job sibling gate: SATISFIED, same caller, same output slot)
**Formal:** ∀ normal x87 bytes b. f128_bytes_to_f64(x87_bytes_to_f128_bytes(b)).to_bits() = x87_bytes_to_f64(b).to_bits()
**Contract evidence:** same-job sibling performing the same conversion on the same caller-visible output — ONE caller, `eval_const_binop_float` (src/common/const_arith.rs), computes the identical folded-long-double `approx` through two same-job narrowings into the identical slot: the ARM64/RISC-V branch at const_arith.rs:174/179/184/189 `let approx = long_double::f128_bytes_to_f64(&result); Some(IrConst::long_double_with_bytes(approx, result))` and the x86/i686 branch at const_arith.rs:239 `let approx = long_double::x87_to_f64(&result_x87); Some(IrConst::long_double_with_bytes(approx, result_f128))` (x87_to_f64 = x87_bytes_to_f64, long_double.rs:1869-1871, which documents and implements round-to-nearest: "Round to nearest: check bit 10 (the first dropped bit)", long_double.rs:655). The bridge x87_bytes_to_f128_bytes is exact (both formats bias 16383; 63-bit explicit mantissa zero-extended into 112 bits — no precision lost), so both branch computations of the SAME input see the identical value and must deliver the identical caller-visible f64 into `IrConst::long_double_with_bytes`'s approx field ("f64: approximate value for computations", ir/constants.rs:22). They disagree — at least one same-job converter is wrong, and the sibling's documented round-to-nearest plus IEEE 754 default identify the truncating one.
**Caller receiving the wrong value:** const_arith.rs:174-190 — every folded `long double` Add/Sub/Mul/Div/Mod constant on ARM64/RISC-V stores the truncated approx. By the single-rounding law (P9: f128 arithmetic on f64-exact operands is exact, ≤106 significant bits < 113), the correctly-rounded approx is bitwise-equal to native f64 `a op b`, so the caller receives a value bitwise-different from the correctly-rounded result of its own computation — not merely an alternate acceptable rounding.
**Documentation conflict:** sibling comment (long_double.rs:655) documents round-to-nearest; f128_bytes_to_f64's comment ("Take top 52 bits of the 112-bit stored mantissa") implements truncation — the two same-job converters contradict each other on the same output.
**Severity:** medium
**Counterexample:** x87 2^0 × 1.111...1 (all-ones 64-bit mantissa): direct converter gives 2.0 (rounding carries into the exponent), via exact f128 bridge gives 1.9999999999999998.
**Expected / Actual:** 2.0 / 1.9999999999999998
**Impact:** Folded long double arithmetic on ARM64/RISC-V (`eval_const_binop_float` approx, and any f128_bytes_to_f64 consumer) is up to 1 ulp low whenever the dropped 60 bits are ≥ half an ulp.
**Root cause:** long_double.rs:917 `let mantissa52 = (stored >> 60) as u64;` — no round bit / sticky / tie-even, no mantissa-overflow carry.
**Bug report:** rounds/01_common/bug_reports/f128_bytes_to_f64_truncates.md
**Repro seed:** deterministic
**Raw output:** `left: 2355634373914480375, right: 2355634373914480376: direct=3.2257192316651656e-151 via_f128=3.225719231665165e-151`

### B4: __builtin_bswap32 folds unsigned results ≥ 2^31 into signed I32 — MEDIUM
**Formal:** ∀ v:i64. builtin_bswap32(v) = (v as u32).swap_bytes() zero-extended (unsigned int result), ∧ involution bswap32(bswap32(v)) = (v as u32)
**Contract evidence:** documented src/common/const_arith.rs:88-96: unsigned 32-bit results "must use I64 with zero-extension because IrConst::I32 is signed and cannot correctly represent unsigned values >= 2^31 … Using I64(4294967295) preserves the correct unsigned value." — the module's own representation rule, implemented by the arithmetic path, violated by the bswap32 fold (documented-and-violated).
**Documentation conflict:** the codebase's own comment mandates zero-extension for unsigned 32-bit constants; const_eval.rs:95 stores `I32(swap_bytes() as i32)`.
**Severity:** medium
**Counterexample:** `__builtin_bswap32(v)` with v = -5495501120125551105 (low u32 after bswap = 0xFFBBD010 ≥ 2^31)
**Expected / Actual:** folded constant 4290586368 / -4380928
**Impact:** `(__int128)__builtin_bswap32(x)`-style widening via eval_const_binop_i128 (`rhs.to_i64()? as u64 as u128` with rhs_unsigned=true) sign-extends the wrong representation (0xFFFFFFFFFFBBD010 instead of 0xFFBBD010); any consumer reading to_i64() directly gets a negative value for an unsigned-int constant.
**Root cause:** src/common/const_eval.rs:95 — `Some(IrConst::I32(v.swap_bytes() as i32))` instead of zero-extended I64.
**Bug report:** rounds/01_common/bug_reports/bswap32_signed_representation.md
**Repro seed:** deterministic
**Raw output:** `left: Some(-4380928), right: Some(4290586368): bswap32 v=-5495501120125551105`

## Design Caveats (if any)

- `irconst_to_bits` (src/common/const_eval.rs:316) converts float constants by *value* (`*v as i64 as u64`, saturating/truncating) despite the doc claiming "raw bit representation"; no property was written against it this round (its only caller `eval_const_expr_as_bits` is sema-internal). Doc evidence: src/common/const_eval.rs:310 "Evaluate a constant expression as raw u64 bits, with fallback to value conversion." — the comment itself admits the fallback, so this is a documented trade-off, flagged for the next round.
- C shift counts ≥ width rely on Rust wrapping_shl masking; C makes those UB, so any behavior is conforming. Not tested beyond internal consistency.

## Test Files Created

| File | Tests |
|------|-------|
| src/common/encoding.rs (pbt_tests) | 1 property |
| src/common/const_arith.rs (pbt_tests) | 6 properties |
| src/common/const_eval.rs (pbt_tests + pbt_regression) | 3 properties + 1 regression |
| src/common/long_double.rs (pbt_tests + pbt_regression) | 6 properties + 3 regressions |
| src/common/types.rs (pbt_tests) | 1 property |
| src/common/fx_hash.rs (pbt_tests) | 1 property |
| src/common/symbol_table.rs (pbt_tests) | 1 property |

## Reproduction

```bash
cd /home/shuhao/fermat-users/leo/github/claudes-c-compiler
# User build contract (verbatim):
PATH="$HOME/.cargo/bin:$PATH" cargo check --lib
# Whole test suite (serial):
PATH="$HOME/.cargo/bin:$PATH" cargo test --lib -- --test-threads=1
# The 6 failing PBT witness properties (reproduce serially; bugs B1–B4):
PATH="$HOME/.cargo/bin:$PATH" cargo test --lib -- --test-threads=1 pbt_p7_f128_lossless_roundtrip pbt_p9_f128_single_rounding pbt_p10_x87_f128_bridge pbt_p11_x87_f64_roundtrip pbt_p12b_bswap_semantics pbt_p7_p11_boundary_constants
# Deterministic regression witnesses (all fail today; un-ignore after fixing):
PATH="$HOME/.cargo/bin:$PATH" cargo test --lib -- --ignored common::long_double::pbt_regression common::const_eval::pbt_regression
```

## Output Directories

- pbt-out/rounds/01_common/PLAN.md — campaign plan, probes, placement decisions (mirrored at pbt-out/PLAN.md)
- pbt-out/rounds/01_common/PROPERTIES.md — property ledger (16 entries, final statuses)
- pbt-out/rounds/01_common/FUNCTION_INDEX.md — 382 functions indexed across 14 files
- pbt-out/rounds/01_common/REPORT.md — this report's round copy
- pbt-out/rounds/01_common/report.json — machine-readable report (mirrored at pbt-out/report.json)
- pbt-out/rounds/01_common/COVERAGE.md / COVERAGE_STATUS.md — coverage ledger + evidence level
- pbt-out/rounds/01_common/INVARIANTS.md — confirmed invariants for the next campaign
- pbt-out/rounds/01_common/bug_reports/<slug>.md — one per confirmed bug (4)
- pbt-out/rounds/01_common/run/ — scratch run directory (test execution CWD, test.log)
- Customer-facing HTML: pbt-out/REPORT.html + pbt-out/bug_reports/<slug>.html (auto-rendered from report.json)

## Coverage Report

# PBT Coverage Status

> Last updated: 2026-10-05 08:11 (campaign: coverage)
> Files: 7/14 scanned (50%) | Functions: 24/382 total | PBT candidates: 24 | Tested: 24 (100%) | 15 pass, 5 fail, 4 other

## Summary

| Metric | Value |
|--------|-------|
| Total source files | 14 |
| Files scanned | 7 / 14 (50%) |
| Total functions (all files) | 382 |
| PBT candidates (from FUNCTION_INDEX) | 24 |
| **Tested (of PBT candidates)** | **24 / 24 (100%)** |
| &nbsp;&nbsp;↳ Pass / Fail / Other | 15 / 5 / 4 |
| **Overall (tested / all functions)** | **24 / 382 (6%)** |
| Untested | 0 |
| Skipped | 0 |

## Module Breakdown

| Module | Scanned | Tested | Skipped | Coverage |
|--------|---------|--------|---------|----------|
|  | 24 | 24 | 0 | 100% |

## Oracle Type Distribution

| Oracle Type | Total | Covered | Skipped | Coverage |
|-------------|-------|---------|---------|----------|
| unknown | 24 | 24 | 0 | 100% |

## File Coverage

| Source File | Funcs | Candidates | Tested | Coverage | Status |
|-------------|-------|------------|--------|----------|--------|
| src/common/asm_constraints.rs | 1 | 1 | 0 | 0% | untested |
| src/common/const_arith.rs | 15 | 15 | 0 | 0% | untested |
| src/common/const_eval.rs | 5 | 5 | 0 | 0% | untested |
| src/common/encoding.rs | 10 | 10 | 0 | 0% | untested |
| src/common/error.rs | 45 | 45 | 0 | 0% | untested |
| src/common/fx_hash.rs | 8 | 8 | 0 | 0% | untested |
| src/common/long_double.rs | 183 | 183 | 0 | 0% | untested |
| src/common/source.rs | 17 | 17 | 0 | 0% | untested |
| src/common/symbol_table.rs | 7 | 7 | 0 | 0% | untested |
| src/common/temp_files.rs | 9 | 9 | 0 | 0% | untested |
| src/common/type_builder.rs | 10 | 10 | 0 | 0% | untested |
| src/common/types.rs | 72 | 72 | 0 | 0% | untested |

## Files Not Yet Scanned (12)

| Source File | Module |
|-------------|--------|
| src/common/asm_constraints.rs | src |
| src/common/const_arith.rs | src |
| src/common/const_eval.rs | src |
| src/common/encoding.rs | src |
| src/common/error.rs | src |
| src/common/fx_hash.rs | src |
| src/common/long_double.rs | src |
| src/common/source.rs | src |
| src/common/symbol_table.rs | src |
| src/common/temp_files.rs | src |
| src/common/type_builder.rs | src |
| src/common/types.rs | src |

## Recommended Focus

> **Priority 1 — Fix failing tests**
> These functions have failing PBT properties — fix before adding new tests.

| Function | Source |
|----------|--------|
| eval_builtin_call | const_eval.rs |
| f64_to_f128_bytes_lossless | long_double.rs |
| f128_bytes_to_f64 | long_double.rs |
| x87_bytes_to_f64 | long_double.rs |
| f64_to_x87_bytes_simple | long_double.rs |

> **Priority 3 — Scan uncovered files**
> 12 file(s) not yet scanned: src (12 files)
> Run `pi-pbt scan <dir>` to add them to FUNCTION_INDEX.md.
