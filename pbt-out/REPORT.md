# PBT Campaign Report: encode_neon_pmull

## Summary

**Verdict:** 1 high: encode_neon_pmull hardcodes size=11, so valid `pmull v0.8h, v0.8b, v0.8b` is encoded as 64-bit PMULL (`0x0ee0e000` instead of `0x0e20e000`); plus 3 medium bugs (extra operand ignored, invalid arrangement encoded, GPR dest encoded as V0).
**Date:** 2026-10-05
**Repository:** /home/toan/github/claudes-c-compiler
**Modules tested:** encode_neon_pmull
**Tests:** 10
**Result:** 6 passing, 4 bugs
**Change surface:** 1 changed function (encode_neon_pmull), 1 with a property, 0 error-handling changes
**Coverage evidence:** file-level (symbol presence) — coverage_gaps reported no .gcda/.profraw (C++ reporter listed unrelated binaries and claimed encode_neon_pmull NOT LINKED). Rust cargo tests executed the symbol; sweep was a manual arm audit plus alt-spellings/nonreg properties.
**Tier:** standard

## Modules Tested

| Module | Tests | Bugs | Oracles Used |
|--------|-------|------|-------------|
| encode_neon_pmull | 10 | 4 | differential, algebraic.metamorphic, algebraic.invariant, negative_error |

## Bugs Found

### B1: encode_neon_pmull encodes 8-bit PMULL as 64-bit PMULL

**Formal:** ∀ rd,rn,rm ∈ {0..31}, is_pmull2 ∈ {false,true}. let Tb = 8b if ¬is_pmull2 else 16b. encode_neon_pmull([Vd.8h, Vn.Tb, Vm.Tb], is_pmull2) = llvm-mc(pmull{2} Vd.8h, Vn.Tb, Vm.Tb) under -triple=aarch64 -mattr=+aes -show-encoding
**Contract evidence:** inferred (assembler README.md:11 gas-compatible textual assembly; README.md:230 lists pmull under NEON widen/long without restricting Ta to 1Q; ARM three-different PMULL size=00 for Ta=8H)
**Documentation conflict:** neon.rs:1138-1139 document the 1q/size=11 encoding only; they do not declare 8H invalid or out of domain. (none as an exclusion)
**Severity:** high
**Counterexample:** encode_neon_pmull([v0.8h, v0.8b, v0.8b], false)
**Expected / Actual:** 0x0e20e000 / 0x0ee0e000
**Impact:** Valid 8-bit polynomial-multiply-long assembly is assembled as the crypto 64-bit form, so callers of `pmull vD.8h, vN.8b, vM.8b` execute the wrong instruction
**Root cause:** neon.rs:1141 hardcodes `(0b11 << 22)` and discards arrangements, so size cannot be 00
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:1141`
```rust
    let word = ((q << 30) | (0b001110 << 24) | (0b11 << 22) | (1 << 21)
        | (rm << 16) | (0b11100 << 11)) | (rn << 5) | rd;
```
**Suggested fix:** Derive size from Ta/Tb (8H → 00, 1Q → 11)
```rust
    let size = match (arr_d.as_str(), arr_n.as_str()) {
        ("8h", "8b") | ("8h", "16b") => 0b00u32,
        ("1q", "1d") | ("1q", "2d") => 0b11u32,
        _ => return Err(format!("unsupported pmull arrangement: {}.{}", arr_d, arr_n)),
    };
    let word = ((q << 30) | (0b001110 << 24) | (size << 22) | (1 << 21)
        | (rm << 16) | (0b11100 << 11)) | (rn << 5) | rd;
```
**Bug report:** bug_reports/encode_neon_pmull_8h_as_64bit.md
**Repro seed:** rd = 0, rn = 0, rm = 0, is_pmull2 = false
**Raw output:**
```text
Test failed: assertion failed: `(left == right)`
  left: `249618432`,
 right: `237035520`: mismatch for pmull v0.8h, v0.8b, v0.8b
minimal failing input: rd = 0, rn = 0, rm = 0, is_pmull2 = false
```

### B2: encode_neon_pmull ignores a fourth operand

**Formal:** ∀ rd,rn,rm,extra ∈ {0..31}, is_pmull2 ∈ {false,true}. llvm-mc rejects pmull{2} Vd.1q, Vn.Tb, Vm.Tb, Vextra.Tb ⇒ encode_neon_pmull([Vd.1q,Vn.Tb,Vm.Tb,Vextra.Tb], is_pmull2) is Err
**Contract evidence:** documented neon.rs:1130 "pmull requires 3 operands" plus llvm-mc/gas rejection of a fourth operand
**Documentation conflict:** neon.rs:1130 "pmull requires 3 operands" states the arity; the check is `len < 3`, so extra operands are accepted. The comment asserts the arity rather than declaring extra out of domain as a documented limitation — documented-and-violated.
**Severity:** medium
**Counterexample:** encode_neon_pmull([v0.1q, v0.1d, v0.1d, v0.1d], false)
**Expected / Actual:** Err / Ok(Word(0x0ee0e000))
**Impact:** Extra operands are dropped; invalid assembly is encoded as the three-operand form
**Root cause:** neon.rs:1129 uses `operands.len() < 3`
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:1129`
```rust
    if operands.len() < 3 {
        return Err("pmull requires 3 operands".to_string());
    }
```
**Suggested fix:** Reject arity other than 3
```rust
    if operands.len() != 3 {
        return Err("pmull requires 3 operands".to_string());
    }
```
**Bug report:** bug_reports/encode_neon_pmull_extra_operand.md
**Repro seed:** rd = 0, rn = 0, rm = 0, extra = 0, is_pmull2 = false
**Raw output:**
```text
Test failed: 4 operands must Err (llvm-mc rejects pmull v0.1q, v0.1d, v0.1d, v0.1d)
minimal failing input: rd = 0, rn = 0, rm = 0, extra = 0, is_pmull2 = false
```

### B3: encode_neon_pmull accepts invalid arrangements

**Formal:** ∀ rd,rn,rm ∈ {0..31}, is_pmull2 ∈ {false,true}, (Td,Tn,Tm) not a valid PMULL{2} Ta/Tb triple. llvm-mc rejects the assembly ⇒ encode_neon_pmull([Vd.Td,Vn.Tn,Vm.Tm], is_pmull2) is Err
**Contract evidence:** inferred (ARM PMULL Ta in {8H,1Q} with matching Tb; llvm-mc/gas reject other T; README.md:11 gas-compatible)
**Documentation conflict:** (none) — neon.rs:1138 asserts the 1q encoding; it does not declare `.8b` invalid
**Severity:** medium
**Counterexample:** encode_neon_pmull([v0.8b, v0.8b, v0.8b], false)
**Expected / Actual:** Err / Ok(Word(0x0ee0e000))
**Impact:** A mistyped arrangement is assembled as 64-bit PMULL instead of being diagnosed
**Root cause:** neon.rs:1132-1134 discards all three arrangements
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:1132`
```rust
    let (rd, _) = get_neon_reg(operands, 0)?;
    let (rn, _) = get_neon_reg(operands, 1)?;
    let (rm, _) = get_neon_reg(operands, 2)?;
```
**Suggested fix:** Keep arrangements and reject triples that are not the four ARM-legal PMULL{2} pairs
```rust
    let (rd, arr_d) = get_neon_reg(operands, 0)?;
    let (rn, arr_n) = get_neon_reg(operands, 1)?;
    let (rm, arr_m) = get_neon_reg(operands, 2)?;
    let legal = matches!(
        (arr_d.as_str(), arr_n.as_str(), arr_m.as_str(), is_pmull2),
        ("1q", "1d", "1d", false) | ("1q", "2d", "2d", true)
            | ("8h", "8b", "8b", false) | ("8h", "16b", "16b", true)
    );
    if !legal {
        return Err(format!("unsupported pmull arrangement: {}/{}/{}", arr_d, arr_n, arr_m));
    }
```
**Bug report:** bug_reports/encode_neon_pmull_invalid_t.md
**Repro seed:** rd = 0, rn = 0, rm = 0, is_pmull2 = false, td = "8b", tn = "8b", tm = "8b"
**Raw output:**
```text
Test failed: invalid Ta/Tb must Err (ARM PMULL Ta in {8H,1Q} with matching Tb; llvm-mc rejects pmull v0.8b, v0.8b, v0.8b)
minimal failing input: rd = 0, rn = 0, rm = 0, is_pmull2 = false, td = "8b", tn = "8b", tm = "8b"
```

### B4: encode_neon_pmull encodes a GPR destination as a NEON register

**Formal:** ∀ rd,rn,rm ∈ {0..31}, is_pmull2 ∈ {false,true}, dest ∈ {xN, wN, vN-bare, dN, sN, qN, sp}. llvm-mc rejects the assembly ⇒ encode_neon_pmull([dest, Vn.Tb, Vm.Tb], is_pmull2) is Err
**Contract evidence:** inferred (ARM requires Vd.<Ta>; llvm-mc/gas reject `pmull x0, v0.1d, v0.1d`; README.md:11 gas-compatible)
**Documentation conflict:** (none)
**Severity:** medium
**Counterexample:** encode_neon_pmull([Reg("x0"), v0.1d, v0.1d], false)
**Expected / Actual:** Err / Ok(Word(0x0ee0e000))
**Impact:** A GPR dest is encoded as Vd with the same register number, so invalid assembly becomes a well-formed NEON instruction
**Root cause:** neon.rs:1132 uses get_neon_reg, which accepts Operand::Reg and parse_reg_num on x/w prefixes
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:1132`
```rust
    let (rd, _) = get_neon_reg(operands, 0)?;
```
**Suggested fix:** Require Operand::RegArrangement with a V prefix and a legal Ta
```rust
    let (rd, arr_d) = get_neon_reg(operands, 0)?;
    if arr_d.is_empty() {
        return Err("pmull dest must be a V register with arrangement".to_string());
    }
```
**Bug report:** bug_reports/encode_neon_pmull_gpr_dest.md
**Repro seed:** rd = 0, rn = 0, rm = 0, is_pmull2 = false, kind = 0, fp_prefix = "x"
**Raw output:**
```text
Test failed: GPR/bare/non-arrangement kind=0 must Err (llvm-mc rejects pmull x0, v0.1d, v0.1d)
minimal failing input: rd = 0, rn = 0, rm = 0, is_pmull2 = false, kind = 0, fp_prefix = "x"
```

## Design Caveats

(none)

## Test Files Created

| File | Tests |
|------|-------|
| src/backend/arm/assembler/encoder/encode_neon_pmull_pbt.rs | 10 properties + 1 KAT + 4 regression witnesses |
| src/backend/arm/assembler/encoder/mod.rs | one-line `#[cfg(test)] mod encode_neon_pmull_pbt` |

## Reproduction

Whole suite:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_neon_pmull -- --test-threads=1
```

B1:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_pmull_regression_8h_as_64bit -- --test-threads=1 --exact
```

B2:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_pmull_regression_extra_operand -- --test-threads=1 --exact
```

B3:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_pmull_regression_invalid_t -- --test-threads=1 --exact
```

B4:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_pmull_regression_gpr_dest -- --test-threads=1 --exact
```

## Output Directories

- pbt-out/REPORT.md
- pbt-out/REPORT.html
- pbt-out/PROPERTIES.md
- pbt-out/PLAN.md
- pbt-out/COVERAGE.md
- pbt-out/COVERAGE_STATUS.md
- pbt-out/report.json
- pbt-out/INVARIANTS.md
- pbt-out/FUNCTION_INDEX.md
- pbt-out/bug_reports/encode_neon_pmull_8h_as_64bit.md
- pbt-out/bug_reports/encode_neon_pmull_8h_as_64bit.html
- pbt-out/bug_reports/encode_neon_pmull_extra_operand.md
- pbt-out/bug_reports/encode_neon_pmull_extra_operand.html
- pbt-out/bug_reports/encode_neon_pmull_invalid_t.md
- pbt-out/bug_reports/encode_neon_pmull_invalid_t.html
- pbt-out/bug_reports/encode_neon_pmull_gpr_dest.md
- pbt-out/bug_reports/encode_neon_pmull_gpr_dest.html
- pbt-out/run/kat.log, pbt-out/run/full.log, pbt-out/run/full2.log
- proptest-regressions/backend/arm/assembler/encoder/encode_neon_pmull_pbt.txt

## Coverage Report

# PBT Coverage Status

> Last updated: 2026-10-05 13:53 (campaign: coverage)
> Files: 10/10 scanned (100%) | Functions: 120/289 total | PBT candidates: 120 | Tested: 120 (100%) | 0 pass, 120 fail

## Summary

| Metric | Value |
|--------|-------|
| Total source files | 10 |
| Files scanned | 10 / 10 (100%) |
| Total functions (all files) | 289 |
| PBT candidates (from FUNCTION_INDEX) | 120 |
| **Tested (of PBT candidates)** | **120 / 120 (100%)** |
| &nbsp;&nbsp;↳ Pass / Fail / Other | 0 / 120 / 0 |
| **Overall (tested / all functions)** | **120 / 289 (42%)** |
| Untested | 0 |
| Skipped | 0 |

## Module Breakdown

| Module | Scanned | Tested | Skipped | Coverage |
|--------|---------|--------|---------|----------|
|  | 120 | 120 | 0 | 100% |

## Oracle Type Distribution

| Oracle Type | Total | Covered | Skipped | Coverage |
|-------------|-------|---------|---------|----------|
| unknown | 120 | 120 | 0 | 100% |

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
| neon.rs | 68 | 35 | 35 | 100% | covered |
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
