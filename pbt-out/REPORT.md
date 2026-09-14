# PBT Campaign Report: encode_ldaxr_stlxr

## Summary

**Date:** 2026-09-14
**Repository:** /home/toan/github/claudes-c-compiler
**Modules tested:** encode_ldaxr_stlxr
**Tests:** 9 properties + 8 KAT + 9 regression witnesses
**Result:** 5 passing properties, 4 failing properties (9 SUT bugs); 8 KAT pass; 9 regression witnesses fail
**Effort tier:** standard (1 coverage-driven sweep round; ≥1000 generator cases; ≥1 metamorphic/differential required)

## Modules Tested

| Module | Tests | Bugs | Oracles Used |
|--------|-------|------|-------------|
| encode_ldaxr_stlxr | 5 passing / 4 failing properties + 8 KAT pass + 9 regression fail | 9 | differential, algebraic.invariant, algebraic.metamorphic, negative_error |

## Bugs Found

1. **encode_ldaxr_stlxr silently ignores a surplus operand**
   - Law: LDAXR is 2-operand (`Rt, [Xn|SP]`); STLXR is 3-operand (`Ws, Rt, [Xn|SP]`). Extra operand must Err (llvm-mc `invalid operand`).
   - Minimal input: rt=0, rn=0, ws=0, is_load=false, extra=Reg("x2") — `stlxr w0, w0, [x0], x2`
   - Expected: `Err`
   - Actual: `Ok(Word(0x8800FC00))` encoding of `stlxr w0, w0, [x0]`
   - Root cause: no `operands.len()` check; `get_reg` / `operands.get` only read the required indices
   - Impact: typos/extra operands assemble silently
   - Severity: medium
   - Serial reconfirmation: `PBT_TEST_JOBS=1` / `--test-threads=1`
   - Bug report: `pbt-out/bug_reports/encode_ldaxr_stlxr_extra_operand.md`
   - Regression: `test_encode_ldaxr_stlxr_regression_extra_operand` (fails as witness)

2. **encode_ldaxr_stlxr encodes SP as ZR in Rt**
   - Law: Exclusive Rt is Wt/Xt with register 31 meaning ZR, never SP. llvm-mc rejects `ldaxr sp, [x0]`.
   - Minimal input: kind=0, n=0 — `ldaxr sp, [x0]`
   - Expected: `Err`
   - Actual: `Ok(Word(0xC85FFC1F))` = `ldaxr xzr, [x0]`
   - Root cause: `parse_reg_num` maps `sp`/`wsp` to 31 with no SP-vs-ZR check
   - Impact: stack-pointer dest silently rewritten to the zero register
   - Severity: medium
   - Serial reconfirmation: `PBT_TEST_JOBS=1` / `--test-threads=1`
   - Bug report: `pbt-out/bug_reports/encode_ldaxr_stlxr_sp_as_rt.md`
   - Regression: `test_encode_ldaxr_stlxr_regression_sp_as_rt` (fails as witness)

3. **encode_ldaxr_stlxr accepts a W register as exclusive base**
   - Law: Exclusive addressing is `[Xn|SP]`. llvm-mc rejects `ldaxr x0, [w1]`.
   - Minimal input: `ldaxr x0, [w1]`
   - Expected: `Err`
   - Actual: `Ok(Word)` encoding Rn=1 as if the base were x1
   - Root cause: `parse_reg_num` accepts `w` prefix
   - Impact: 32-bit base mnemonic encoded instead of rejected
   - Severity: medium
   - Serial reconfirmation: `PBT_TEST_JOBS=1` / `--test-threads=1`
   - Bug report: `pbt-out/bug_reports/encode_ldaxr_stlxr_w_base.md`
   - Regression: `test_encode_ldaxr_stlxr_regression_w_base` (fails as witness)

4. **encode_ldaxr_stlxr encodes XZR as SP in the exclusive base**
   - Law: Address register 31 is SP, never XZR. llvm-mc rejects `ldaxr x0, [xzr]`.
   - Minimal input: `ldaxr x0, [xzr]`
   - Expected: `Err`
   - Actual: `Ok(Word)` encoding Rn=31 (SP)
   - Root cause: `parse_reg_num` maps both `xzr` and `sp` to 31
   - Impact: zero-register base silently rewritten to SP
   - Severity: medium
   - Serial reconfirmation: `PBT_TEST_JOBS=1` / `--test-threads=1`
   - Bug report: `pbt-out/bug_reports/encode_ldaxr_stlxr_xzr_as_base.md`
   - Regression: `test_encode_ldaxr_stlxr_regression_xzr_as_base` (fails as witness)

5. **encode_ldaxr_stlxr encodes SIMD/FP names as GPR Rt**
   - Law: LDAXR/STLXR data registers are GPRs. llvm-mc rejects `ldaxr d0, [x1]`.
   - Minimal input: `ldaxr d0, [x1]`
   - Expected: `Err`
   - Actual: `Ok(Word)` treating `d0` as GPR 0
   - Root cause: `parse_reg_num` accepts `d`/`s`/`q`/`v`/`h`/`b` prefixes
   - Impact: FP dest silently treated as same-numbered GPR
   - Severity: medium
   - Serial reconfirmation: `PBT_TEST_JOBS=1` / `--test-threads=1`
   - Bug report: `pbt-out/bug_reports/encode_ldaxr_stlxr_fp_as_rt.md`
   - Regression: `test_encode_ldaxr_stlxr_regression_fp_as_rt` (fails as witness)

6. **encode_ldaxr_stlxr accepts an X register as STLXR status**
   - Law: STLXR status is Ws. llvm-mc rejects `stlxr x0, x1, [x2]`.
   - Minimal input: `stlxr x0, x1, [x2]`
   - Expected: `Err`
   - Actual: `Ok(Word)` encoding Ws=0 as if the status were w0
   - Root cause: `let (ws, _) = get_reg(...)` discards width
   - Impact: 64-bit status mnemonic encoded instead of rejected
   - Severity: medium
   - Serial reconfirmation: `PBT_TEST_JOBS=1` / `--test-threads=1`
   - Bug report: `pbt-out/bug_reports/encode_ldaxr_stlxr_x_as_ws.md`
   - Regression: `test_encode_ldaxr_stlxr_regression_x_as_ws` (fails as witness)

7. **encode_ldaxr_stlxr accepts Xt on LDAXRB/LDAXRH**
   - Law: Byte/half exclusive forms take Wt. llvm-mc rejects `ldaxrb x0, [x1]`.
   - Minimal input: `ldaxrb x0, [x1]`
   - Expected: `Err`
   - Actual: `Ok(Word)` encoding size=00 with Rt=0
   - Root cause: `forced_size` overrides data-register width from `get_reg`
   - Impact: 64-bit dest on byte exclusive encoded as 32-bit
   - Severity: medium
   - Serial reconfirmation: `PBT_TEST_JOBS=1` / `--test-threads=1`
   - Bug report: `pbt-out/bug_reports/encode_ldaxr_stlxr_x_data_byte.md`
   - Regression: `test_encode_ldaxr_stlxr_regression_x_data_byte` (fails as witness)

8. **encode_ldaxr_stlxr ignores a nonzero exclusive offset**
   - Law: Exclusive addressing is `[Xn|SP]` or `[Xn|SP, #0]`. llvm-mc: `index must be absent or #0`.
   - Minimal input: is_load=false, shape=8, rt=0, offset=-1 — `stlxr w1, x0, [x2, #-1]`
   - Expected: `Err`
   - Actual: `Ok(Word(0xC801FC40))` encoding of the same instruction with offset 0
   - Root cause: `Operand::Mem { base, .. }` ignores `offset`
   - Impact: nonzero offset silently dropped
   - Severity: medium
   - Serial reconfirmation: `PBT_TEST_JOBS=1` / `--test-threads=1`
   - Bug report: `pbt-out/bug_reports/encode_ldaxr_stlxr_nonzero_offset.md`
   - Regression: `test_encode_ldaxr_stlxr_regression_nonzero_offset` (fails as witness)

9. **encode_ldaxr_stlxr encodes STLXR when Ws aliases a source**
   - Law: ARM CONSTRAINED UNPREDICTABLE / llvm-mc: "unpredictable STXR instruction, status is also a source" when Ws aliases Rt or Xn (WZR vs SP allowed).
   - Minimal input: rt=0, rn=0, variant=0, is_64=false, overlap_rt=false — `stlxr w0, w0, [x0]`
   - Expected: `Err`
   - Actual: `Ok(Word(0x8800FC00))`
   - Root cause: no overlap check between Ws and Rt/Rn
   - Impact: UNPREDICTABLE exclusive store emitted instead of rejected
   - Severity: medium
   - Serial reconfirmation: `PBT_TEST_JOBS=1` / `--test-threads=1`
   - Bug report: `pbt-out/bug_reports/encode_ldaxr_stlxr_ws_overlap.md`
   - Regression: `test_encode_ldaxr_stlxr_regression_ws_overlap` (fails as witness)

## Design Caveats

(none)

## Test Files Created

| File | Tests |
|------|-------|
| src/backend/arm/assembler/encoder/load_store.rs (mod encode_ldaxr_stlxr_pbt) | 9 properties (5 passing / 4 failing) + 8 KAT + 9 failing regression witnesses |

## Output Directories

- `pbt-out/PLAN.md` — campaign checklist
- `pbt-out/PROPERTIES.md` — property ledger
- `pbt-out/REPORT.md` — this report
- `pbt-out/COVERAGE.md` — per-function coverage row
- `pbt-out/COVERAGE_STATUS.md` — campaign coverage stats
- `pbt-out/FUNCTION_INDEX.md` — merged function index (encode_ldaxr_stlxr now a candidate)
- `pbt-out/INVARIANTS.md` — confirmed invariants for encode_ldaxr_stlxr
- `pbt-out/bug_reports/encode_ldaxr_stlxr_*.md` — 9 bug reports

## Contract-surface sweep

Round 1 of 1 (standard tier). `coverage_gaps` reported no LLVM profraw; closed by a manual arm audit of `encode_ldaxr_stlxr` (load/store paths, Mem vs non-Mem, invalid base, forced_size 00/01/None, is_64, get_reg errors). Added `encode_ldaxr_stlxr_diff_alt_spellings` (x31 / uppercase / lr / W-form vs llvm-mc, 1000 cases, passing) and `encode_ldaxr_stlxr_neg_mem_index` (MemRegOffset / MemExpr ⇒ Err, 1000 cases, passing). Closed because the tier's one round is done and every documented behavior of this symbol has a property.

## Skipped targets

(none) — campaign restricted to encode_ldaxr_stlxr; `cargo check --lib` was the user build contract and was not re-run as a compile exploration. Tests ran via `cargo test --lib encode_ldaxr_stlxr_pbt` (official harness, target swapped).

## Coverage Report

# PBT Coverage Status

> Last updated: 2026-09-14 15:07 (campaign: coverage)
> Files: 8/8 scanned (100%) | Functions: 65/253 total | PBT candidates: 65 | Tested: 65 (100%) | 0 pass, 65 fail

## Summary

| Metric | Value |
|--------|-------|
| Total source files | 8 |
| Files scanned | 8 / 8 (100%) |
| Total functions (all files) | 253 |
| PBT candidates (from FUNCTION_INDEX) | 65 |
| **Tested (of PBT candidates)** | **65 / 65 (100%)** |
| &nbsp;&nbsp;↳ Pass / Fail / Other | 0 / 65 / 0 |
| **Overall (tested / all functions)** | **65 / 253 (26%)** |
| Untested | 0 |
| Skipped | 0 |

## Module Breakdown

| Module | Scanned | Tested | Skipped | Coverage |
|--------|---------|--------|---------|----------|
|  | 65 | 65 | 0 | 100% |

## Oracle Type Distribution

| Oracle Type | Total | Covered | Skipped | Coverage |
|-------------|-------|---------|---------|----------|
| unknown | 65 | 65 | 0 | 100% |

## File Coverage

| Source File | Funcs | Candidates | Tested | Coverage | Status |
|-------------|-------|------------|--------|----------|--------|
| cast.rs | 6 | 1 | 1 | 100% | covered |
| compare_branch.rs | 21 | 18 | 18 | 100% | covered |
| constants.rs | 34 | 1 | 1 | 100% | covered |
| data_processing.rs | 36 | 24 | 24 | 100% | covered |
| gp_integer.rs | 29 | 1 | 1 | 100% | covered |
| load_store.rs | 20 | 6 | 6 | 100% | covered |
| neon.rs | 68 | 13 | 13 | 100% | covered |
| pseudo.rs | 44 | 1 | 1 | 100% | covered |

## Recommended Focus

> **Priority 1 — Fix failing tests**
> These functions have failing PBT properties — fix before adding new tests.

| Function | Source |
|----------|--------|
| encode_shift | gp_integer.rs |
| encode_add_sub | data_processing.rs |
| cast_float_to_target | constants.rs |
| classify_cast_with_f128 | cast.rs |
| encode_adc | data_processing.rs |
| encode_adr | load_store.rs |
| encode_bic | data_processing.rs |
| encode_neon_three_diff_narrow | neon.rs |
| encode_bics | data_processing.rs |
| encode_bl | compare_branch.rs |
| encode_blr | compare_branch.rs |
| encode_br | compare_branch.rs |
| encode_branch | compare_branch.rs |
| encode_cbz | compare_branch.rs |
| encode_ccmp_ccmn | compare_branch.rs |
| encode_cinc | compare_branch.rs |
| encode_cinv | compare_branch.rs |
| encode_cmn | compare_branch.rs |
| encode_cmp | compare_branch.rs |
| encode_cneg | compare_branch.rs |
| encode_csel | compare_branch.rs |
| encode_cset | compare_branch.rs |
| encode_csetm | compare_branch.rs |
| encode_csinc | compare_branch.rs |
| encode_csinv | compare_branch.rs |
| encode_csneg | compare_branch.rs |
| encode_div | data_processing.rs |
| encode_eon | data_processing.rs |
| encode_ldar_stlr | load_store.rs |
| encode_neon_across_long | neon.rs |
| encode_neon_float_cmp_zero | neon.rs |
| encode_neon_sli | neon.rs |
| encode_ldur_stur | load_store.rs |
| encode_ldxp_stxp | load_store.rs |
| encode_neon_float_three_same | neon.rs |
| encode_ldxr_stxr | load_store.rs |
| encode_logical | data_processing.rs |
| encode_madd | data_processing.rs |
| encode_movk | data_processing.rs |
| encode_movn | data_processing.rs |
| encode_movz | data_processing.rs |
| encode_neon_qshrn | neon.rs |
| encode_msub | data_processing.rs |
| encode_mul | data_processing.rs |
| encode_mvn | data_processing.rs |
| encode_neon_shift_right | neon.rs |
| encode_neg | pseudo.rs |
| encode_negs | data_processing.rs |
| encode_neon_shift_imm | neon.rs |
| encode_neon_tbl | neon.rs |
| encode_orn | data_processing.rs |
| encode_ret | compare_branch.rs |
| encode_sbc | data_processing.rs |
| encode_neon_shll | neon.rs |
| encode_neon_sqshrun | neon.rs |
| encode_smull | data_processing.rs |
| encode_sxth | data_processing.rs |
| encode_sxtw | data_processing.rs |
| encode_neon_shift_left_imm | neon.rs |
| encode_umaddl | data_processing.rs |
| encode_umulh | data_processing.rs |
| encode_neon_rbit | neon.rs |
| encode_umull | data_processing.rs |
| encode_uxtw | data_processing.rs |
| encode_ldaxr_stlxr | load_store.rs |
