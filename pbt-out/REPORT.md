# PBT Campaign Report: encode_neon_xtl

## Summary

**Verdict:** 3 medium: encode_neon_xtl silently encodes extra operands, mismatched UXTL/SXTL arrangements, and GPR/bare destinations that gas/llvm-mc reject, so invalid GNU-style assembly becomes wrong SIMD machine code.
**Date:** 2026-10-05
**Repository:** /home/toan/github/claudes-c-compiler
**Modules tested:** encode_neon_xtl
**Tests:** 9
**Result:** 6 passing, 3 bugs
**Change surface:** 1 changed function, 1 with a property, 0 error-handling changes
**Coverage evidence:** file-level (symbol presence) — coverage_gaps reported no .gcda/.profraw (Rust cargo test is not the C++ gcov reporter); it listed unrelated C++ binaries and claimed encode_neon_xtl NOT LINKED. Sweep used a manual arm audit of encode_neon_xtl plus a nonreg property. Native line-level data was not produced.
**Effort tier:** standard

## Modules Tested

| Module | Tests | Bugs | Oracles Used |
|--------|-------|------|-------------|
| encode_neon_xtl | 9 | 3 | differential, algebraic.metamorphic, algebraic.invariant, negative_error |

## Bugs Found

### B1: encode_neon_xtl ignores extra operands

**Formal:** ∀ valid 2-operand UXTL inputs, ∀ extra. llvm-mc rejects the 3-operand form ⇒ encode_neon_xtl(ops++[extra], u_bit, is_high) = Err
**Contract evidence:** inferred (README.md:12 gas compatibility; llvm-mc rejects a third operand on uxtl/sxtl)
**Documentation conflict:** (none)
**Severity:** medium
**Counterexample:** encode_neon_xtl([v0.8h, v0.8b, v0.8h], u_bit=0, is_high=false) — asm `sxtl v0.8h, v0.8b, v0.8h`
**Expected / Actual:** Err / Ok(Word) (same encoding as `sxtl v0.8h, v0.8b`)
**Impact:** A typo or extra register in GNU-style `uxtl`/`sxtl` assembly is silently encoded as the two-operand form
**Root cause:** neon.rs:164 checks only `operands.len() < 2`, so operands past index 1 are ignored
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:164`
```rust
    if operands.len() < 2 {
        return Err("NEON uxtl/sxtl requires 2 operands".to_string());
    }
```
**Suggested fix:** Reject any arity other than 2
```rust
    if operands.len() != 2 {
        return Err("NEON uxtl/sxtl requires 2 operands".to_string());
    }
```
**Bug report:** bug_reports/encode_neon_xtl_extra_operand.md
**Repro seed:** cc 5e83412a8ab5e9dcf20991445379a2394eea8459d99da05b182bbd96d17fff03
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_neon_xtl_pbt::encode_neon_xtl_neg_extra_operand' panicked at src/backend/arm/assembler/encoder/encode_neon_xtl_pbt.rs:366:1:
Test failed: 3 operands must Err (llvm-mc rejects sxtl v0.8h, v0.8b, v0.8h) at src/backend/arm/assembler/encoder/encode_neon_xtl_pbt.rs:386.
minimal failing input: rd = 0, rn = 0, extra = 0, pair = (
    "8h",
    "8b",
    false,
), u_bit = 0
	successes: 0
	local rejects: 0
	global rejects: 0
```

### B2: encode_neon_xtl encodes mismatched UXTL/SXTL arrangements

**Formal:** ∀ rd,rn, ∀ (ta,tb,is_high) not in mandated UXTL pairs. llvm-mc rejects "{uxtl|sxtl}{2?} Vd.ta, Vn.tb" ⇒ encode_neon_xtl = Err
**Contract evidence:** inferred (ARM UXTL Ta in {8H,4S,2D} paired with Tb 8B/4H/2S or 16B/8H/4S; llvm-mc rejects `sxtl v0.8b, v0.8b`)
**Documentation conflict:** (none)
**Severity:** medium
**Counterexample:** encode_neon_xtl([v0.8b, v0.8b], u_bit=0, is_high=false) — asm `sxtl v0.8b, v0.8b`
**Expected / Actual:** Err / Ok(Word) (dest `.8b` discarded; source `.8b` treated as legal SXTL)
**Impact:** Invalid arrangement pairs (wrong dest Ta, or UXTL with a UXTL2 source) encode as if the pairing were legal
**Root cause:** neon.rs:167 binds dest arrangement as `_arr_d` and never checks it; Q comes only from `is_high`
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:167`
```rust
    let (rd, _arr_d) = get_neon_reg(operands, 0)?;
```
**Suggested fix:** Require dest Ta in {8h,4s,2d} and Tb matching (Ta, Q)
```rust
    let (rd, arr_d) = get_neon_reg(operands, 0)?;
    let expected_tb = match (arr_d.as_str(), is_high) {
        ("8h", false) => "8b",
        ("8h", true) => "16b",
        ("4s", false) => "4h",
        ("4s", true) => "8h",
        ("2d", false) => "2s",
        ("2d", true) => "4s",
        _ => return Err(format!("uxtl/sxtl: unsupported dest arrangement: {}", arr_d)),
    };
```
**Bug report:** bug_reports/encode_neon_xtl_mismatched_ta_tb.md
**Repro seed:** cc 5e83412a8ab5e9dcf20991445379a2394eea8459d99da05b182bbd96d17fff03
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_neon_xtl_pbt::encode_neon_xtl_neg_mismatched_ta_tb' panicked at src/backend/arm/assembler/encoder/encode_neon_xtl_pbt.rs:394:1:
Test failed: invalid/mismatched Ta/Tb must Err (ARM UXTL Ta in {8H,4S,2D} with matching Tb; llvm-mc rejects sxtl v0.8b, v0.8b) at src/backend/arm/assembler/encoder/encode_neon_xtl_pbt.rs:414.
minimal failing input: rd = 0, rn = 0, ta = "8b", tb = "8b", u_bit = 0, is_high = false
	successes: 0
	local rejects: 0
	global rejects: 0
```

### B3: encode_neon_xtl encodes GPR or bare-V destinations as SIMD V registers

**Formal:** ∀ kind ∈ {GPR dest, bare-V dest, bare-V src, x-prefix dest arrangement, GPR src}. llvm-mc rejects the asm ⇒ encode_neon_xtl = Err
**Contract evidence:** inferred (README.md:12 gas compatibility; llvm-mc rejects `uxtl x0, v0.8b`)
**Documentation conflict:** (none)
**Severity:** medium
**Counterexample:** encode_neon_xtl([Reg("x0"), v0.8b], u_bit=1, is_high=false) — asm `uxtl x0, v0.8b`
**Expected / Actual:** Err / Ok(Word) (x0 parsed as register 0; dest arrangement discarded)
**Impact:** A wrong register class in the .s file becomes silent SIMD code (`uxtl x0, v0.8b` encodes as `uxtl v0.8h, v0.8b`)
**Root cause:** neon.rs:167 calls get_neon_reg, whose Operand::Reg arm (neon.rs:14) accepts x/w prefixes via parse_reg_num
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:14`
```rust
        Some(Operand::Reg(name)) => {
            let num = parse_reg_num(name)
                .ok_or_else(|| format!("invalid register: {}", name))?;
            Ok((num, String::new()))
        }
```
**Suggested fix:** Require RegArrangement with a V-prefixed register at both dest and source
```rust
        Some(Operand::RegArrangement { reg, arrangement }) if reg.to_lowercase().starts_with('v') => {
            let num = parse_reg_num(reg)
                .ok_or_else(|| format!("invalid NEON register: {}", reg))?;
            Ok((num, arrangement.clone()))
        }
```
**Bug report:** bug_reports/encode_neon_xtl_gpr_or_bare.md
**Repro seed:** cc 5e83412a8ab5e9dcf20991445379a2394eea8459d99da05b182bbd96d17fff03
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_neon_xtl_pbt::encode_neon_xtl_neg_gpr_or_bare' panicked at src/backend/arm/assembler/encoder/encode_neon_xtl_pbt.rs:422:1:
Test failed: GPR/bare/non-arrangement kind=0 must Err (llvm-mc rejects uxtl x0, v0.8b) at src/backend/arm/assembler/encoder/encode_neon_xtl_pbt.rs:477.
minimal failing input: rd = 0, rn = 0, kind = 0, fp_prefix = "x"
	successes: 0
	local rejects: 0
	global rejects: 0
```

## Design Caveats

(none)

## Test Files Created

| File | Tests |
|------|-------|
| src/backend/arm/assembler/encoder/encode_neon_xtl_pbt.rs | 9 properties + 2 KAT + 3 regression witnesses |

## Reproduction

```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_neon_xtl -- --test-threads=1
```

B1 extra operand:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_xtl_regression_extra_operand -- --test-threads=1
```

B2 mismatched Ta/Tb:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_xtl_regression_mismatched_dest_ta -- --test-threads=1
```

B3 GPR dest:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_xtl_regression_gpr_dest -- --test-threads=1
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
- pbt-out/CHANGE_SURFACE.md
- pbt-out/run/encode_neon_xtl.log
- pbt-out/bug_reports/encode_neon_xtl_extra_operand.md
- pbt-out/bug_reports/encode_neon_xtl_extra_operand.html
- pbt-out/bug_reports/encode_neon_xtl_mismatched_ta_tb.md
- pbt-out/bug_reports/encode_neon_xtl_mismatched_ta_tb.html
- pbt-out/bug_reports/encode_neon_xtl_gpr_or_bare.md
- pbt-out/bug_reports/encode_neon_xtl_gpr_or_bare.html

## Coverage Report

# PBT Coverage Status

> Last updated: 2026-10-05 16:35 (campaign: coverage)
> Files: 10/10 scanned (100%) | Functions: 128/289 total | PBT candidates: 128 | Tested: 128 (100%) | 0 pass, 128 fail

## Summary

| Metric | Value |
|--------|-------|
| Total source files | 10 |
| Files scanned | 10 / 10 (100%) |
| Total functions (all files) | 289 |
| PBT candidates (from FUNCTION_INDEX) | 128 |
| **Tested (of PBT candidates)** | **128 / 128 (100%)** |
| &nbsp;&nbsp;↳ Pass / Fail / Other | 0 / 128 / 0 |
| **Overall (tested / all functions)** | **128 / 289 (44%)** |
| Untested | 0 |
| Skipped | 0 |

## Module Breakdown

| Module | Scanned | Tested | Skipped | Coverage |
|--------|---------|--------|---------|----------|
|  | 128 | 128 | 0 | 100% |

## Oracle Type Distribution

| Oracle Type | Total | Covered | Skipped | Coverage |
|-------------|-------|---------|---------|----------|
| unknown | 128 | 128 | 0 | 100% |

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
| neon.rs | 68 | 43 | 43 | 100% | covered |
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
