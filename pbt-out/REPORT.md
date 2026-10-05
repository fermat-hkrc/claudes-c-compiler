# PBT Campaign Report: encode_neon_fcvtn

## Summary

**Verdict:** 1 high, 2 medium: encode_neon_fcvtn accepts dest `.8b` and source `.2s` so invalid FCVTN encodes as a different valid instruction; it also silently drops a third operand and encodes a bare V dest as `Vd.4H`.
**Date:** 2026-10-05
**Repository:** /home/toan/github/claudes-c-compiler
**Modules tested:** encode_neon_fcvtn
**Tests:** 9
**Result:** 6 passing, 3 bugs
**Change surface:** 1 changed function, 1 with a property, 0 error-handling changes
**Coverage evidence:** file-level (symbol presence) — coverage_gaps reported no .gcda/.profraw (C++ reporter listed unrelated binaries and claimed NOT LINKED). The cargo test run executed encode_neon_fcvtn via encode_neon_fcvtn_pbt.rs.
**Tier:** standard

## Modules Tested

| Module | Tests | Bugs | Oracles Used |
|--------|-------|------|-------------|
| encode_neon_fcvtn | 9 | 3 | differential, algebraic.metamorphic, algebraic.invariant, negative_error |

## Bugs Found

### B1: encode_neon_fcvtn ignores a third operand

**Formal:** ∀ rd,rn,extra ∈ {0..31}, ∀ valid (tb,ta,is_high). llvm-mc("fcvtn{2} Vd.tb, Vn.ta, Ve.tb") = Err ∧ encode_neon_fcvtn([Vd.tb,Vn.ta,Ve.tb], is_high) = Err
**Contract evidence:** documented src/backend/arm/assembler/README.md:12 "It accepts the same textual assembly that GCC's gas would consume"
**Documentation conflict:** (none)
**Severity:** medium
**Counterexample:** encode_neon_fcvtn([v0.4h, v0.4s, v0.4h], is_high=false)
**Expected / Actual:** Err / Ok(Word(0x0e216800))
**Impact:** Trailing junk after FCVTN is silently dropped, so a mistyped extra register does not fail the assemble
**Root cause:** neon.rs:1653–1654 read only operands[0] and operands[1]; there is no maximum-arity check
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:1653`
```rust
    let (rd, _) = get_neon_reg(operands, 0)?;
    let (rn, arr_n) = get_neon_reg(operands, 1)?;
```
**Suggested fix:** Reject anything other than exactly 2 operands
```rust
    if operands.len() != 2 {
        return Err("fcvtn requires 2 operands".to_string());
    }
```
**Bug report:** bug_reports/encode_neon_fcvtn_extra_operand.md
**Repro seed:** rd = 0, rn = 0, extra = 0, pair = ("4h", "4s", false)
**Raw output:** Test failed: 3 operands must Err (llvm-mc rejects fcvtn v0.4h, v0.4s, v0.4h) at src/backend/arm/assembler/encoder/encode_neon_fcvtn_pbt.rs:348.
minimal failing input: rd = 0, rn = 0, extra = 0, pair = (
    "4h",
    "4s",
    false,
)

### B2: encode_neon_fcvtn accepts dest 8B and source 2S

**Formal:** ∀ rd,rn ∈ {0..31}, ∀ tb,ta ∈ NEON arrangements, ∀ is_high ∈ {false,true}. ¬valid_pair(tb,ta,is_high) ⇒ llvm-mc rejects ∧ encode_neon_fcvtn([Vd.tb,Vn.ta], is_high) = Err
**Contract evidence:** inferred (ARM FCVTN admits only Ta in {4S,2D} with matching Tb {4H/8H, 2S/4S}; README.md:12 gas-compatible assembly)
**Documentation conflict:** neon.rs:1651 "FCVTN: single→half or double→single narrowing float convert" states the conversions, not arrangement counts. neon.rs:1656 "fcvtn: unsupported source:" is the `_` arm of a match that explicitly accepts "2s". Neither comment declares dest 8B or source 2S invalid / out of domain.
**Severity:** high
**Counterexample:** encode_neon_fcvtn([v0.8b, v0.2s], is_high=false)
**Expected / Actual:** Err / Ok(Word(0x0e216800))
**Impact:** Invalid assembly encodes as a different valid FCVTN (sz from source only, dest discarded), producing wrong machine code instead of an assemble error
**Root cause:** neon.rs:1653 discards the dest arrangement; neon.rs:1655 matches source "2s" as sz=0
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:1653`
```rust
    let (rd, _) = get_neon_reg(operands, 0)?;
    let (rn, arr_n) = get_neon_reg(operands, 1)?;
    let sz = match arr_n.as_str() { "4s" | "2s" => 0u32, "2d" => 1,
        _ => return Err(format!("fcvtn: unsupported source: {}", arr_n)), };
```
**Suggested fix:** Accept only source 4s/2d and require the ARM-mandated dest arrangement
```rust
    let (rd, arr_d) = get_neon_reg(operands, 0)?;
    let (rn, arr_n) = get_neon_reg(operands, 1)?;
    let sz = match arr_n.as_str() {
        "4s" => 0u32,
        "2d" => 1,
        _ => return Err(format!("fcvtn: unsupported source: {}", arr_n)),
    };
    let expected_tb = match (arr_n.as_str(), is_high) {
        ("4s", false) => "4h",
        ("4s", true) => "8h",
        ("2d", false) => "2s",
        ("2d", true) => "4s",
        _ => return Err(format!("fcvtn: unsupported source: {}", arr_n)),
    };
    if arr_d != expected_tb {
        return Err(format!("fcvtn: source {} requires dest {}", arr_n, expected_tb));
    }
```
**Bug report:** bug_reports/encode_neon_fcvtn_mismatched_ta_tb.md
**Repro seed:** rd = 0, rn = 0, tb = "8b", ta = "2s", is_high = false
**Raw output:** Test failed: invalid/mismatched Tb/Ta must Err (ARM FCVTN Ta in {4S,2D} with matching Tb; llvm-mc rejects fcvtn v0.8b, v0.2s) at src/backend/arm/assembler/encoder/encode_neon_fcvtn_pbt.rs:375.
minimal failing input: rd = 0, rn = 0, tb = "8b", ta = "2s", is_high = false

### B3: encode_neon_fcvtn encodes a bare V dest as FCVTN Vd.4H

**Formal:** ∀ rd,rn ∈ {0..31}, ∀ kind ∈ GPR/bare/non-V. llvm-mc rejects the corresponding asm ∧ encode_neon_fcvtn(ops(kind), false) = Err
**Contract evidence:** inferred (ARM FCVTN dest is Vd.Tb; README.md:12 gas-compatible assembly; llvm-mc rejects `fcvtn v0, v0.4s`)
**Documentation conflict:** (none)
**Severity:** medium
**Counterexample:** encode_neon_fcvtn([Reg("v0"), v0.4s], is_high=false)
**Expected / Actual:** Err / Ok(Word(0x0e216800))
**Impact:** A dest without arrangement is encoded as the corresponding V register, so a mistyped `fcvtn v0, v0.4s` silently becomes `fcvtn v0.4h, v0.4s`
**Root cause:** neon.rs:1653 calls get_neon_reg which accepts Operand::Reg, then discards dest arrangement
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:1653`
```rust
    let (rd, _) = get_neon_reg(operands, 0)?;
```
**Suggested fix:** Reject a destination that is not a V-prefixed RegArrangement
```rust
    match &operands[0] {
        Operand::RegArrangement { reg, .. } if reg.to_ascii_lowercase().starts_with('v') => {}
        _ => return Err("fcvtn: destination must be a V register with arrangement".to_string()),
    }
```
**Bug report:** bug_reports/encode_neon_fcvtn_bare_dest.md
**Repro seed:** rd = 0, rn = 0, kind = 2, fp_prefix = "x"
**Raw output:** Test failed: GPR/bare/non-arrangement kind=2 must Err (llvm-mc rejects fcvtn v0, v0.4s) at src/backend/arm/assembler/encoder/encode_neon_fcvtn_pbt.rs:438.
minimal failing input: rd = 0, rn = 0, kind = 2, fp_prefix = "x"

## Design Caveats

(none)

## Test Files Created

| File | Tests |
|------|-------|
| src/backend/arm/assembler/encoder/encode_neon_fcvtn_pbt.rs | 9 properties + 2 KAT + 4 regression witnesses |

## Reproduction

Whole suite:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_neon_fcvtn -- --test-threads=1
```

B1 extra operand:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_neon_fcvtn_neg_extra_operand -- --test-threads=1
```

B2 mismatched arrangements:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_neon_fcvtn_neg_mismatched_ta_tb -- --test-threads=1
```

B3 bare dest:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_neon_fcvtn_neg_gpr_or_bare -- --test-threads=1
```

## Output Directories

- pbt-out/REPORT.md — this report
- pbt-out/REPORT.html — customer-facing overview (rendered from report.json)
- pbt-out/PROPERTIES.md — property ledger
- pbt-out/PLAN.md — campaign checklist
- pbt-out/COVERAGE.md — coverage ledger
- pbt-out/COVERAGE_STATUS.md — coverage statistics
- pbt-out/INVARIANTS.md — confirmed invariants
- pbt-out/report.json — machine-readable report
- pbt-out/bug_reports/encode_neon_fcvtn_extra_operand.md
- pbt-out/bug_reports/encode_neon_fcvtn_extra_operand.html
- pbt-out/bug_reports/encode_neon_fcvtn_mismatched_ta_tb.md
- pbt-out/bug_reports/encode_neon_fcvtn_mismatched_ta_tb.html
- pbt-out/bug_reports/encode_neon_fcvtn_bare_dest.md
- pbt-out/bug_reports/encode_neon_fcvtn_bare_dest.html
- pbt-out/FUNCTION_INDEX.md — merged function index
- pbt-out/CHANGE_SURFACE.md — change surface
- src/backend/arm/assembler/encoder/encode_neon_fcvtn_pbt.rs — harness

## Coverage Report

# PBT Coverage Status

> Last updated: 2026-10-05 20:53 (campaign: coverage)
> Files: 10/10 scanned (100%) | Functions: 141/289 total | PBT candidates: 141 | Tested: 141 (100%) | 0 pass, 141 fail

## Summary

| Metric | Value |
|--------|-------|
| Total source files | 10 |
| Files scanned | 10 / 10 (100%) |
| Total functions (all files) | 289 |
| PBT candidates (from FUNCTION_INDEX) | 141 |
| **Tested (of PBT candidates)** | **141 / 141 (100%)** |
| &nbsp;&nbsp;↳ Pass / Fail / Other | 0 / 141 / 0 |
| **Overall (tested / all functions)** | **141 / 289 (49%)** |
| Untested | 0 |
| Skipped | 0 |

## Module Breakdown

| Module | Scanned | Tested | Skipped | Coverage |
|--------|---------|--------|---------|----------|
|  | 141 | 141 | 0 | 100% |

## Oracle Type Distribution

| Oracle Type | Total | Covered | Skipped | Coverage |
|-------------|-------|---------|---------|----------|
| unknown | 141 | 141 | 0 | 100% |

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
| neon.rs | 68 | 56 | 56 | 100% | covered |
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
