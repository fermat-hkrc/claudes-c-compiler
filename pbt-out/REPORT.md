# PBT Campaign Report: encode_neon_bsl

## Summary

**Verdict:** 4 medium: encode_neon_bsl silently encodes extra operands, illegal arrangements (as .8b), mismatched T, and GPR/SP/bare-V/FP as Q=0 BSL, so invalid assembly becomes a plausible NEON word.
**Date:** 2026-10-05
**Repository:** /home/toan/github/claudes-c-compiler
**Modules tested:** encode_neon_bsl
**Tests:** 9 properties (plus 4 KAT + 6 regression witnesses)
**Result:** 5 passing, 4 bugs
**Change surface:** 1 changed function (encode_neon_bsl), 1 with a property, 0 error-handling changes
**Coverage evidence:** file-level (symbol presence) — coverage_gaps reported no .gcda/.profraw (C++ reporter listed unrelated binaries and claimed NOT LINKED). Cargo lib tests executed encode_neon_bsl. Sweep: 1/1 manual arm audit (uppercase V alt-spellings passing).
**Tier:** standard

## Modules Tested

| Module | Tests | Bugs | Oracles Used |
|--------|-------|------|-------------|
| encode_neon_bsl | 9 properties (5 passing, 4 failing) | 4 | differential, algebraic.metamorphic, algebraic.invariant, negative_error |

## Bugs Found

### B1: encode_neon_bsl ignores a fourth operand

**Formal:** ∀ rd,rn,rm,extra ∈ {0..31}, t ∈ {8b,16b}. llvm-mc rejects "bsl Vd.t, Vn.t, Vm.t, Vextra.t" ⇒ encode_neon_bsl([Vd.t,Vn.t,Vm.t,Vextra.t]) is Err
**Contract evidence:** documented neon.rs:734 "Encode NEON BSL (bitwise select): BSL Vd.T, Vn.T, Vm.T" (exactly three arranged operands)
**Documentation conflict:** neon.rs:737 "bsl requires 3 operands" only fires when len < 3; it does not declare extra operands valid. The asserted form is three operands; extra is accepted. (not independently verified as an exclusion)
**Severity:** medium
**Counterexample:** encode_neon_bsl([v0.8b, v0.8b, v0.8b, v0.8b])
**Expected / Actual:** Err / Ok(Word(0x2e601c00))
**Impact:** Invalid four-operand BSL is assembled as three-operand BSL; the extra register is dropped
**Root cause:** neon.rs:736 `operands.len() < 3` ignores operands after the first three
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:736`
```rust
    if operands.len() < 3 {
        return Err("bsl requires 3 operands".to_string());
    }
```
**Suggested fix:** Reject arity other than 3
```rust
    if operands.len() != 3 {
        return Err("bsl requires 3 operands".to_string());
    }
```
**Bug report:** bug_reports/encode_neon_bsl_extra_operand.md
**Repro seed:** rd = 0, rn = 0, rm = 0, extra = 0, t = "8b"
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_neon_bsl_pbt::encode_neon_bsl_neg_extra' panicked at src/backend/arm/assembler/encoder/encode_neon_bsl_pbt.rs:165:1:
Test failed: extra operand must Err (llvm-mc rejects bsl v0.8b, v0.8b, v0.8b, v0.8b) at src/backend/arm/assembler/encoder/encode_neon_bsl_pbt.rs:271.
minimal failing input: rd = 0, rn = 0, rm = 0, extra = 0, t = "8b"
	successes: 0
	local rejects: 0
	global rejects: 0
```

### B2: encode_neon_bsl encodes illegal BSL arrangements as .8b

**Formal:** ∀ rd,rn,rm ∈ {0..31}, t ∈ {4h,8h,2s,4s,2d,1d,4b,8d,2h,1s}. llvm-mc rejects "bsl Vd.t, Vn.t, Vm.t" ⇒ encode_neon_bsl is Err
**Contract evidence:** inferred (ARM Advanced SIMD BSL T ∈ {8B,16B}; README.md:12 gas-compat; llvm-mc rejects .4h)
**Documentation conflict:** (none) — the function comment names Vd.T without restricting T; it does not declare .4h valid
**Severity:** medium
**Counterexample:** encode_neon_bsl([v0.4h, v0.4h, v0.4h])
**Expected / Actual:** Err / Ok(Word(0x2e601c00)) identical to bsl v0.8b, v0.8b, v0.8b
**Impact:** Illegal arrangement is encoded as Q=0 BSL; the object file does not match the assembly text
**Root cause:** neon.rs:743 sets Q=1 iff arr_d == "16b", else Q=0, with no 8b/16b check
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:743`
```rust
    let q: u32 = if arr_d == "16b" { 1 } else { 0 };
```
**Suggested fix:** Reject arrangements other than 8b/16b before encoding Q
```rust
    if arr_d != "8b" && arr_d != "16b" {
        return Err(format!("bsl: unsupported arrangement: {}", arr_d));
    }
    let q: u32 = if arr_d == "16b" { 1 } else { 0 };
```
**Bug report:** bug_reports/encode_neon_bsl_invalid_t.md
**Repro seed:** rd = 0, rn = 0, rm = 0, t = "4h"
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_neon_bsl_pbt::encode_neon_bsl_neg_invalid_t' panicked at src/backend/arm/assembler/encoder/encode_neon_bsl_pbt.rs:165:1:
Test failed: invalid T must Err (only .8b/.16b; llvm-mc rejects bsl v0.4h, v0.4h, v0.4h) at src/backend/arm/assembler/encoder/encode_neon_bsl_pbt.rs:292.
minimal failing input: rd = 0, rn = 0, rm = 0, t = "4h"
	successes: 0
	local rejects: 0
	global rejects: 0
```

### B3: encode_neon_bsl ignores mismatched Vn/Vm arrangements

**Formal:** ∀ rd,rn,rm ∈ {0..31}, td,tn,tm ∈ {8b,16b} with ¬(td=tn=tm). llvm-mc rejects ⇒ encode_neon_bsl is Err
**Contract evidence:** documented neon.rs:734 "BSL Vd.T, Vn.T, Vm.T" (same T on all three)
**Documentation conflict:** (none) — the comment states the same T; the body discards source arrangements
**Severity:** medium
**Counterexample:** encode_neon_bsl([v0.8b, v0.8b, v0.16b])
**Expected / Actual:** Err / Ok(Word(0x2e601c00))
**Impact:** Mixed 8b/16b BSL is accepted; Q follows dest only
**Root cause:** neon.rs:740-741 discard source arrangements; Q is taken only from dest
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:740`
```rust
    let (rn, _) = get_neon_reg(operands, 1)?;
    let (rm, _) = get_neon_reg(operands, 2)?;
```
**Suggested fix:** Require Vn and Vm arrangements to equal dest T
```rust
    let (rn, arr_n) = get_neon_reg(operands, 1)?;
    let (rm, arr_m) = get_neon_reg(operands, 2)?;
    if arr_n != arr_d || arr_m != arr_d {
        return Err("bsl: operand arrangement mismatch".to_string());
    }
```
**Bug report:** bug_reports/encode_neon_bsl_mismatch_t.md
**Repro seed:** rd = 0, rn = 0, rm = 0, td = "8b", tn = "8b", tm = "16b"
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_neon_bsl_pbt::encode_neon_bsl_neg_mismatch_t' panicked at src/backend/arm/assembler/encoder/encode_neon_bsl_pbt.rs:165:1:
Test failed: mismatched T must Err (llvm-mc rejects bsl v0.8b, v0.8b, v0.16b) at src/backend/arm/assembler/encoder/encode_neon_bsl_pbt.rs:316.
minimal failing input: rd = 0, rn = 0, rm = 0, td = "8b", tn = "8b", tm = "16b"
	successes: 0
	local rejects: 0
	global rejects: 0
```

### B4: encode_neon_bsl accepts GPR, SP, bare V, and FP as BSL operands

**Formal:** ∀ rd,rn,rm ∈ {0..31}, t ∈ {8b,16b}, kind ∈ {x-gpr, w-dest, sp, bare-v, d-reg, s-dest, q-dest}. llvm-mc rejects ⇒ encode_neon_bsl is Err
**Contract evidence:** documented neon.rs:734 "BSL Vd.T, Vn.T, Vm.T" (arranged NEON registers)
**Documentation conflict:** (none) — the comment requires Vd.T; get_neon_reg still accepts Operand::Reg
**Severity:** medium
**Counterexample:** encode_neon_bsl([x0, x0, x0])
**Expected / Actual:** Err / Ok(Word(0x2e601c00)) same as bsl v0.8b, v0.8b, v0.8b
**Impact:** GPR/SP/bare-V/FP BSL becomes a Q=0 NEON BSL word (SP→v31)
**Root cause:** get_neon_reg neon.rs:14 accepts Operand::Reg with empty arrangement; Q then defaults to 0
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:14`
```rust
        Some(Operand::Reg(name)) => {
            let num = parse_reg_num(name)
                .ok_or_else(|| format!("invalid register: {}", name))?;
            Ok((num, String::new()))
        }
```
**Suggested fix:** Require RegArrangement with T in {8b,16b} in encode_neon_bsl
```rust
    if arr_d != "8b" && arr_d != "16b" {
        return Err(format!("bsl: unsupported arrangement: {}", arr_d));
    }
```
**Bug report:** bug_reports/encode_neon_bsl_gpr_bare_sp.md
**Repro seed:** rd = 0, rn = 0, rm = 0, t = "8b", kind = 0
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_neon_bsl_pbt::encode_neon_bsl_neg_gpr_bare_sp' panicked at src/backend/arm/assembler/encoder/encode_neon_bsl_pbt.rs:165:1:
Test failed: non-arranged NEON / GPR / SP / FP must Err (llvm-mc rejects bsl x0, x0, x0) at src/backend/arm/assembler/encoder/encode_neon_bsl_pbt.rs:373.
minimal failing input: rd = 0, rn = 0, rm = 0, t = "8b", kind = 0
	successes: 0
	local rejects: 0
	global rejects: 0
```

## Design Caveats

(none)

## Test Files Created

| File | Tests |
|------|-------|
| src/backend/arm/assembler/encoder/encode_neon_bsl_pbt.rs | 9 properties + 4 KAT + 6 regression witnesses |

## Reproduction

Valid-domain suite (includes failing negative properties):
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_neon_bsl -- --test-threads=1
```

B1 extra operand:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_bsl_regression_extra_operand -- --test-threads=1 --exact
```

B2 invalid T:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_bsl_regression_invalid_t -- --test-threads=1 --exact
```

B3 mismatched T:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_bsl_regression_mismatch_t -- --test-threads=1 --exact
```

B4 GPR dest:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_bsl_regression_gpr_dest -- --test-threads=1 --exact
```

## Output Directories

- pbt-out/REPORT.md
- pbt-out/REPORT.html
- pbt-out/PROPERTIES.md
- pbt-out/PLAN.md
- pbt-out/COVERAGE.md
- pbt-out/COVERAGE_STATUS.md
- pbt-out/INVARIANTS.md
- pbt-out/FUNCTION_INDEX.md
- pbt-out/report.json
- pbt-out/bug_reports/encode_neon_bsl_extra_operand.md
- pbt-out/bug_reports/encode_neon_bsl_extra_operand.html
- pbt-out/bug_reports/encode_neon_bsl_invalid_t.md
- pbt-out/bug_reports/encode_neon_bsl_invalid_t.html
- pbt-out/bug_reports/encode_neon_bsl_mismatch_t.md
- pbt-out/bug_reports/encode_neon_bsl_mismatch_t.html
- pbt-out/bug_reports/encode_neon_bsl_gpr_bare_sp.md
- pbt-out/bug_reports/encode_neon_bsl_gpr_bare_sp.html
- pbt-out/run/ (test scratch)
- src/backend/arm/assembler/encoder/encode_neon_bsl_pbt.rs (harness)
- proptest-regressions/backend/arm/assembler/encoder/encode_neon_bsl_pbt.txt (proptest failure corpus)

## Coverage Report

# PBT Coverage Status

> Last updated: 2026-10-05 12:21 (campaign: coverage)
> Files: 10/10 scanned (100%) | Functions: 115/289 total | PBT candidates: 115 | Tested: 115 (100%) | 0 pass, 115 fail

## Summary

| Metric | Value |
|--------|-------|
| Total source files | 10 |
| Files scanned | 10 / 10 (100%) |
| Total functions (all files) | 289 |
| PBT candidates (from FUNCTION_INDEX) | 115 |
| **Tested (of PBT candidates)** | **115 / 115 (100%)** |
| &nbsp;&nbsp;↳ Pass / Fail / Other | 0 / 115 / 0 |
| **Overall (tested / all functions)** | **115 / 289 (40%)** |
| Untested | 0 |
| Skipped | 0 |

## Module Breakdown

| Module | Scanned | Tested | Skipped | Coverage |
|--------|---------|--------|---------|----------|
|  | 115 | 115 | 0 | 100% |

## Oracle Type Distribution

| Oracle Type | Total | Covered | Skipped | Coverage |
|-------------|-------|---------|---------|----------|
| unknown | 115 | 115 | 0 | 100% |

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
| neon.rs | 68 | 30 | 30 | 100% | covered |
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
