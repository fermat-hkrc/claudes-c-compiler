# PBT Campaign Report: encode_dsb

## Summary

**Verdict:** 1 high, 3 medium: encode_dsb ignores Operand::Imm so `dsb #0` encodes as `dsb sy` (wrong barrier), and it silently encodes extra, omitted, and non-barrier operands as SY instead of rejecting them the way GNU as and llvm-mc do.
**Date:** 2026-10-05
**Repository:** /home/toan/github/claudes-c-compiler
**Modules tested:** encode_dsb
**Tests:** 8 properties (plus 3 KAT, 4 regression witnesses)
**Result:** 4 passing, 4 bugs
**Change surface:** 1 changed function (encode_dsb), 1 with a property, 0 error-handling changes
**Coverage evidence:** file-level (symbol presence) — coverage_gaps reported no .gcda/.profraw and claimed encode_dsb NOT LINKED because it inspected unrelated C++ binaries. `cargo test --lib encode_dsb` executed the real production symbol (4 passing / 4 failing properties).
**Tier:** standard

## Modules Tested

| Module | Tests | Bugs | Oracles Used |
|--------|-------|------|-------------|
| encode_dsb | 8 | 4 | differential, algebraic.metamorphic, algebraic.invariant, negative_error |

## Bugs Found

### B1: encode_dsb ignores Imm and encodes SY

**Formal:** ∀ crm ∈ {0,…,15}. encode_dsb([Imm(crm)]) = llvm-mc("dsb #" + crm) as Word
**Contract evidence:** inferred (README.md:12 gas-compatible assembly; GNU as and llvm-mc encode `dsb #imm` for imm in 0..=15 as CRm=imm; parser.rs produces Operand::Imm for `#n`; encode() at mod.rs:967 passes operands through)
**Documentation conflict:** (none) — encode_dsb has no rustdoc covering Imm; system.rs:49 "DSB: 0xD503309F | (option << 8)" states the encoding formula but does not mention immediates
**Severity:** high
**Counterexample:** encode_dsb(&[Operand::Imm(0)])  (`dsb #0`)
**Expected / Actual:** Word(0xd503309f) / Word(0xd5033f9f)
**Impact:** Valid GNU-style `dsb #imm` assembles as a full-system barrier. Requested CRm values 0–14 become SY, changing memory-ordering semantics.
**Root cause:** system.rs:47 `_ => 0b1111` — Imm is not matched, so every immediate falls through to SY.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/system.rs:47`
```rust
        _ => 0b1111,
```
**Suggested fix:** Treat Imm in 0..=15 as CRm; reject immediates outside that range.
```rust
        Some(Operand::Imm(n)) if (0..=15).contains(n) => *n as u32,
        Some(Operand::Imm(n)) => return Err(format!("dsb immediate out of range: {}", n)),
```
**Bug report:** bug_reports/encode_dsb_imm_ignored.md
**Repro seed:** cc c5614437345ff82a5cc5a5d8c06e99a694c826b52825b26e31c45e7e2f0f733d
**Raw output:**
```text
Test failed: assertion failed: `(left == right)`
  left: `3573759903`,
 right: `3573756063`: SUT vs llvm-mc for dsb #0
minimal failing input: crm = 0
```

### B2: encode_dsb ignores extra operands

**Formal:** ∀ name ∈ NamedDsb, ∀ extra ∈ Operand. llvm-mc("dsb " + name + ", …") is Err ⇒ encode_dsb([Barrier(name), extra]) is Err
**Contract evidence:** inferred (README.md:12 gas-compatible assembly; GNU as rejects `dsb sy, x0`; llvm-mc rejects extra operands)
**Documentation conflict:** (none)
**Severity:** medium
**Counterexample:** encode_dsb(&[Operand::Barrier("sy".into()), Operand::Reg("x0".into())])  (`dsb sy, x0`)
**Expected / Actual:** Err / Ok(Word(0xd5033f9f))
**Impact:** Extra operands are dropped; `dsb sy, x0` silently becomes `dsb sy`.
**Root cause:** system.rs:31 `operands.first()` — only the first operand is examined; length is never checked.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/system.rs:31`
```rust
    let option = match operands.first() {
```
**Suggested fix:** Reject a slice longer than one operand.
```rust
    if operands.len() > 1 {
        return Err("dsb: extra operand".to_string());
    }
```
**Bug report:** bug_reports/encode_dsb_extra_operand.md
**Repro seed:** (deterministic; name = "sy", extra = Reg("x0"))
**Raw output:**
```text
Test failed: extra operand must Err (llvm-mc rejects dsb sy, x0)
minimal failing input: name = "sy", extra = Reg("x0")
```

### B3: encode_dsb encodes omitted option as SY

**Formal:** encode_dsb([]) is Err
**Contract evidence:** inferred (README.md:12 gas-compatible assembly; GNU as: "missing immediate expression at operand 1"; llvm-mc: "too few operands")
**Documentation conflict:** (none) — ARM ARM lists the option as optional with default SY, but this assembler claims gas compatibility and gas requires an operand
**Severity:** medium
**Counterexample:** encode_dsb(&[])
**Expected / Actual:** Err / Ok(Word(0xd5033f9f))
**Impact:** A `dsb` with no operand, rejected by gas and llvm-mc, is encoded as a full-system barrier.
**Root cause:** system.rs:47 `_ => 0b1111` — `operands.first()` is None and defaults CRm to SY.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/system.rs:47`
```rust
        _ => 0b1111,
```
**Suggested fix:** Return Err when the operand list is empty.
```rust
        None => return Err("dsb requires a barrier option".to_string()),
```
**Bug report:** bug_reports/encode_dsb_empty_defaults_sy.md
**Repro seed:** (deterministic; _n = 0)
**Raw output:**
```text
Test failed: empty operands must Err (gas/llvm-mc reject omitted dsb option)
minimal failing input: _n = 0
```

### B4: encode_dsb encodes non-barrier operands as SY

**Formal:** ∀ op ∈ {Imm(n) | n ∉ 0..=15} ∪ {Reg, Mem, Cond, Shift, Label, …}. llvm-mc rejects the corresponding assembly ⇒ encode_dsb([op]) is Err
**Contract evidence:** inferred (README.md:12 gas-compatible assembly; GNU as / llvm-mc reject registers and `#imm` outside 0..=15)
**Documentation conflict:** (none)
**Severity:** medium
**Counterexample:** encode_dsb(&[Operand::Imm(-1)])  (`dsb #-1`)
**Expected / Actual:** Err / Ok(Word(0xd5033f9f))
**Impact:** Invalid operands such as `dsb #-1` and `dsb x0` encode as `dsb sy` instead of an assembler error.
**Root cause:** system.rs:47 `_ => 0b1111` — Imm, Reg, Mem, and every other non-Barrier/non-Symbol kind take the SY default.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/system.rs:47`
```rust
        _ => 0b1111,
```
**Suggested fix:** Reject non-barrier kinds and immediates outside 0..=15.
```rust
        Some(Operand::Imm(n)) if (0..=15).contains(n) => *n as u32,
        Some(Operand::Imm(n)) => return Err(format!("dsb immediate out of range: {}", n)),
        Some(_) => return Err("dsb: invalid operand".to_string()),
```
**Bug report:** bug_reports/encode_dsb_wrong_kind_defaults_sy.md
**Repro seed:** (deterministic; op = Imm(-1))
**Raw output:**
```text
Test failed: non-named / out-of-range operand must Err, got Ok(Word(3573759903))
minimal failing input: op = Imm(-1)
```

## Design Caveats

(none)

## Test Files Created

| File | Tests |
|------|-------|
| src/backend/arm/assembler/encoder/encode_dsb_pbt.rs | 8 properties, 3 KAT, 4 regression witnesses |
| src/backend/arm/assembler/encoder/mod.rs | one-line `#[cfg(test)] mod encode_dsb_pbt` registration |

## Reproduction

Whole suite (serial, as run):
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_dsb -- --test-threads=1
```

B1 (`dsb #0` encodes as SY):
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_dsb_regression_imm_crm0 -- --test-threads=1 --nocapture
```

B2 (extra operand):
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_dsb_regression_extra_sy_x0 -- --test-threads=1 --nocapture
```

B3 (omitted option):
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_dsb_regression_empty -- --test-threads=1 --nocapture
```

B4 (`dsb #-1`):
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_dsb_regression_imm_neg1 -- --test-threads=1 --nocapture
```

## Output Directories

- pbt-out/REPORT.md — this campaign report
- pbt-out/REPORT.html — customer-facing overview (rendered from report.json)
- pbt-out/PROPERTIES.md — property ledger
- pbt-out/PLAN.md — campaign checklist
- pbt-out/COVERAGE.md — per-function coverage table
- pbt-out/COVERAGE_STATUS.md — coverage statistics
- pbt-out/report.json — machine-readable report
- pbt-out/INVARIANTS.md — confirmed invariants for later campaigns
- pbt-out/FUNCTION_INDEX.md — merged function index
- pbt-out/bug_reports/encode_dsb_imm_ignored.md + .html
- pbt-out/bug_reports/encode_dsb_extra_operand.md + .html
- pbt-out/bug_reports/encode_dsb_empty_defaults_sy.md + .html
- pbt-out/bug_reports/encode_dsb_wrong_kind_defaults_sy.md + .html
- pbt-out/run/encode_dsb.log — cargo test log
- proptest-regressions/backend/arm/assembler/encoder/encode_dsb_pbt.txt — shrunk seeds

## Coverage Report

# PBT Coverage Status

> Last updated: 2026-10-05 23:51 (campaign: coverage)
> Files: 11/11 scanned (100%) | Functions: 150/307 total | PBT candidates: 150 | Tested: 150 (100%) | 0 pass, 150 fail

## Summary

| Metric | Value |
|--------|-------|
| Total source files | 11 |
| Files scanned | 11 / 11 (100%) |
| Total functions (all files) | 307 |
| PBT candidates (from FUNCTION_INDEX) | 150 |
| **Tested (of PBT candidates)** | **150 / 150 (100%)** |
| &nbsp;&nbsp;↳ Pass / Fail / Other | 0 / 150 / 0 |
| **Overall (tested / all functions)** | **150 / 307 (49%)** |
| Untested | 0 |
| Skipped | 0 |

## Module Breakdown

| Module | Scanned | Tested | Skipped | Coverage |
|--------|---------|--------|---------|----------|
|  | 150 | 150 | 0 | 100% |

## Oracle Type Distribution

| Oracle Type | Total | Covered | Skipped | Coverage |
|-------------|-------|---------|---------|----------|
| unknown | 150 | 150 | 0 | 100% |

## File Coverage

| Source File | Funcs | Candidates | Tested | Coverage | Status |
|-------------|-------|------------|--------|----------|--------|
| cast.rs | 6 | 1 | 1 | 100% | covered |
| compare_branch.rs | 21 | 18 | 18 | 100% | covered |
| constants.rs | 34 | 1 | 1 | 100% | covered |
| data_processing.rs | 36 | 25 | 25 | 100% | covered |
| fp_scalar.rs | 13 | 10 | 11 | 110% | covered |
| gp_integer.rs | 29 | 1 | 1 | 100% | covered |
| load_store.rs | 20 | 11 | 11 | 100% | covered |
| neon.rs | 68 | 63 | 63 | 100% | covered |
| pseudo.rs | 44 | 1 | 1 | 100% | covered |

## Recommended Focus

> **Priority 1 — Fix failing tests**
> These functions have failing PBT properties — fix before adding new tests.

| Function | Source |
|----------|--------|
| encode_ubfx | bitfield.rs |
| encode_ubfm | bitfield.rs |
| encode_sbfx | bitfield.rs |
| encode_sbfm | bitfield.rs |
| encode_sbfiz | bitfield.rs |
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
| encode_ldrsw | load_store.rs |
| encode_ldtr_sized | load_store.rs |
| encode_prfm | load_store.rs |
| encode_smulh | data_processing.rs |
| encode_fcvt_rounding | fp_scalar.rs |
| encode_fp_1src | fp_scalar.rs |
| encode_int_to_float | fp_scalar.rs |
| encode_fcmp | fp_scalar.rs |
| encode_fcvt_precision | fp_scalar.rs |
| encode_neon_aes | neon.rs |
| encode_bfi | bitfield.rs |
| encode_bfxil | bitfield.rs |
| encode_cas | load_store.rs |
| encode_cls | bitfield.rs |
| encode_clz | bitfield.rs |
| encode_extr | bitfield.rs |
| encode_fmov | fp_scalar.rs |
| encode_fp_arith | fp_scalar.rs |
| encode_rbit | bitfield.rs |
| encode_rev | bitfield.rs |
| encode_rev16 | bitfield.rs |
| encode_rev32 | bitfield.rs |
| encode_ubfiz | bitfield.rs |
| encode_bfm | bitfield.rs |
| encode_neon_float_two_misc | neon.rs |
| encode_fabs | fp_scalar.rs |
| encode_fmadd_fmsub | fp_scalar.rs |
| encode_fneg | fp_scalar.rs |
| encode_fsqrt | fp_scalar.rs |
| encode_neon_dup | neon.rs |
| encode_ldrs | load_store.rs |
| encode_neon_ldnr | neon.rs |
| encode_neon_ld1r | neon.rs |
| encode_neon_ld_st_single | neon.rs |
| encode_neon_ld_st_multi | neon.rs |
| encode_neon_tbx | neon.rs |
| encode_neon_ins | neon.rs |
| encode_neon_umov | neon.rs |
| encode_neon_ext | neon.rs |
| encode_neon_movi | neon.rs |
| encode_neon_mvni | neon.rs |
| encode_cnt | neon.rs |
| encode_neon_not | neon.rs |
| encode_neon_rev64 | neon.rs |
| encode_neon_bsl | neon.rs |
| encode_neon_addv | neon.rs |
| encode_neon_across | neon.rs |
| encode_neon_zip_uzp | neon.rs |
| encode_neon_eor3 | neon.rs |
| encode_neon_pmull | neon.rs |
| encode_neon_add_sub | neon.rs |
| encode_neon_ushr | neon.rs |
| encode_neon_sshr | neon.rs |
| encode_neon_shl | neon.rs |
| encode_neon_sri | neon.rs |
| encode_neon_shrn | neon.rs |
| encode_neon_two_misc | neon.rs |
| encode_neon_xtl | neon.rs |
| encode_neon_mul | neon.rs |
| encode_neon_pmul | neon.rs |
| encode_neon_mla | neon.rs |
| encode_neon_mls | neon.rs |
| encode_neon_three_same | neon.rs |
| encode_neon_three_diff | neon.rs |
| encode_neon_logical | neon.rs |
| encode_neon_cmp_zero | neon.rs |
| encode_neon_elem_long | neon.rs |
| encode_neon_elem | neon.rs |
| encode_neon_float_elem | neon.rs |
| encode_neon_fcvtl | neon.rs |
| encode_neon_fcvtn | neon.rs |
| encode_neon_bitwise_insert | neon.rs |
| encode_neon_faddp | neon.rs |
| encode_neon_scalar_three_same | neon.rs |
| encode_neon_scalar_addp | neon.rs |
| encode_neon_scalar_two_misc | neon.rs |
| encode_neon_scalar_qshrn | neon.rs |
| encode_neon_two_misc_narrow | neon.rs |
| encode_dmb | system.rs |
| encode_dsb | system.rs |
