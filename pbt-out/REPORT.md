# PBT Campaign Report: encode_neon_shrn

## Summary

**Verdict:** 4 medium: encode_neon_shrn silently accepts invalid GNU-style SHRN/RSHRN (fourth operand, mismatched dest Tb, bare V dest, i64 shift truncated to a legal #1), so gas/llvm-mc-rejected assembly is encoded as a different legal instruction.
**Date:** 2026-10-05
**Repository:** /home/toan/github/claudes-c-compiler
**Modules tested:** encode_neon_shrn
**Tests:** 11
**Result:** 7 passing, 4 bugs
**Change surface:** 1 changed function (encode_neon_shrn), 1 with a property, 0 error-handling changes
**Coverage evidence:** file-level (symbol presence) — `coverage_gaps` reported no .gcda/.profraw (C++ reporter listed unrelated binaries and claimed encode_neon_shrn NOT LINKED). Manual arm audit of the function body plus three sweep properties (alt-spellings, nonreg, unsupported source).
**Tier:** standard

## Modules Tested

| Module | Tests | Bugs | Oracles Used |
|--------|-------|------|-------------|
| encode_neon_shrn | 11 | 4 | differential, algebraic.metamorphic, algebraic.invariant, negative_error |

## Bugs Found

### B1: encode_neon_shrn ignores a fourth operand

**Formal:** ∀ rd,rn,extra ∈ [0,31], Ta ∈ {8h,4s,2d}, is_high ∈ {0,1}, opcode ∈ {100001,100011}, shift ∈ [1, dest_esize(Ta)]. encode_neon_shrn([Vd.Tb, Vn.Ta, #shift, Vextra.Tb], opcode, is_high) = Err ∧ llvm-mc(4-operand) = Err
**Contract evidence:** inferred (ARM SHRN is a 3-operand instruction; README.md:12 gas compatibility; llvm-mc rejects a fourth operand)
**Documentation conflict:** (none)
**Severity:** medium
**Counterexample:** encode_neon_shrn([v0.8b, v0.8h, #1, v0.8b], opcode=0b100001, is_high=false)
**Expected / Actual:** Err / Ok(Word) same as `shrn v0.8b, v0.8h, #1`
**Impact:** Invalid assembly with a trailing operand is silently encoded instead of diagnosed
**Root cause:** neon.rs:1437 checks `operands.len() < 3` only, so extra operands after the first three are ignored
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:1437`
```rust
    if operands.len() < 3 { return Err("shrn/rshrn requires 3 operands".to_string()); }
```
**Suggested fix:** Reject arity other than 3
```rust
    if operands.len() != 3 {
        return Err("shrn/rshrn requires 3 operands".to_string());
    }
```
**Bug report:** bug_reports/encode_neon_shrn_extra_operand.md
**Repro seed:** cc 872bcb3b6a9a03cbe965e0ecb7dc25a9f559225bdf5e947ee29baf88366eb0df
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_neon_shrn_pbt::encode_neon_shrn_neg_extra_operand' (2365137) panicked at src/backend/arm/assembler/encoder/encode_neon_shrn_pbt.rs:393:1:
Test failed: 4 operands must Err (llvm-mc rejects shrn v0.8b, v0.8h, #1, v0.8b) at src/backend/arm/assembler/encoder/encode_neon_shrn_pbt.rs:419.
minimal failing input: rd = 0, rn = 0, extra = 0, ta_shift = (
    "8h",
    1,
), is_high = false, opcode = 33
	successes: 0
	local rejects: 0
	global rejects: 0
```

### B2: encode_neon_shrn ignores destination arrangement Tb

**Formal:** ∀ rd,rn ∈ [0,31], Tb,Ta arrangements, shift ∈ [1,64], is_high, opcode. (Tb,Ta,is_high) not a valid SHRN pair ⇒ encode_neon_shrn = Err ∧ llvm-mc = Err
**Contract evidence:** inferred (ARM SHRN Ta in {8H,4S,2D} with matching Tb 8B/16B, 4H/8H, 2S/4S; README.md:12 gas compatibility; llvm-mc rejects `shrn v0.8b, v0.2d, #1`)
**Documentation conflict:** (none)
**Severity:** medium
**Counterexample:** encode_neon_shrn([v0.8b, v0.2d, #1], opcode=0b100001, is_high=false)
**Expected / Actual:** Err / Ok(Word) encoded as if dest were v0.2s
**Impact:** Wrong-arrangement SHRN is silently accepted and encoded as a different legal instruction
**Root cause:** neon.rs:1438 discards the destination arrangement (`let (rd, _)`), so Q comes only from is_high and esize only from the source
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:1438`
```rust
    let (rd, _) = get_neon_reg(operands, 0)?;
```
**Suggested fix:** Require Tb to match Ta and Q
```rust
    let (rd, arr_d) = get_neon_reg(operands, 0)?;
    let expected_tb = match (arr_n.as_str(), is_high) {
        ("8h", false) => "8b",
        ("8h", true) => "16b",
        ("4s", false) => "4h",
        ("4s", true) => "8h",
        ("2d", false) => "2s",
        ("2d", true) => "4s",
        _ => return Err(format!("shrn: unsupported source: {}", arr_n)),
    };
    if arr_d != expected_tb {
        return Err(format!("shrn: dest arrangement {} does not match source {}", arr_d, arr_n));
    }
```
**Bug report:** bug_reports/encode_neon_shrn_mismatched_dest_tb.md
**Repro seed:** (none — deterministic regression)
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_neon_shrn_pbt::encode_neon_shrn_neg_invalid_arrangement' (2365167) panicked at src/backend/arm/assembler/encoder/encode_neon_shrn_pbt.rs:427:1:
Test failed: invalid/mismatched Ta/Tb must Err (ARM SHRN Ta in {8H,4S,2D} with matching Tb; llvm-mc rejects shrn v0.8b, v0.2d, #1) at src/backend/arm/assembler/encoder/encode_neon_shrn_pbt.rs:451.
minimal failing input: rd = 0, rn = 0, tb = "8b", ta = "2d", shift = 1, is_high = false, opcode = 33
	successes: 0
	local rejects: 0
	global rejects: 0
```

### B3: encode_neon_shrn truncates i64 shift via `as u32`

**Formal:** ∀ rd,rn ∈ [0,31], Ta ∈ {8h,4s,2d}, is_high, opcode, shift ∉ [1, dest_esize(Ta)]. encode_neon_shrn = Err ∧ (shift in a llvm-mc-representable range ⇒ llvm-mc = Err)
**Contract evidence:** inferred (Operand::Imm is i64; ARM SHRN shift in [1, dest_esize]; llvm-mc "immediate must be an integer in range [1, 8]" for .8b; gas "immediate value out of range")
**Documentation conflict:** (none) — neon.rs:1444 range-checks after the truncation, so the author's check never sees the original i64
**Severity:** medium
**Counterexample:** encode_neon_shrn([v0.8b, v0.8h, #4294967297], opcode=0b100001, is_high=false)
**Expected / Actual:** Err / Ok(Word) of shift #1
**Impact:** An out-of-range immediate congruent to a legal shift modulo 2^32 is silently rewritten to that legal shift
**Root cause:** neon.rs:1440 truncates the i64 immediate to u32 before the range check at neon.rs:1444
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:1440`
```rust
    let shift = get_imm(operands, 2)? as u32;
```
**Suggested fix:** Range-check the i64 immediate before narrowing
```rust
    let shift_i = get_imm(operands, 2)?;
    if shift_i < 1 || shift_i > half_bits as i64 {
        return Err(format!("shrn: shift {} out of range", shift_i));
    }
    let shift = shift_i as u32;
```
**Bug report:** bug_reports/encode_neon_shrn_shift_i64_trunc.md
**Repro seed:** cc 516b13a6d692f623bcbabff1e554424250a9347e6bd21b311d26a1d62aa0c43e
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_neon_shrn_pbt::encode_neon_shrn_neg_shift_oob' (2365184) panicked at src/backend/arm/assembler/encoder/encode_neon_shrn_pbt.rs:459:1:
Test failed: shift 4294967297 not in [1, 8] must Err (llvm-mc rejects shrn v0.8b, v0.8h, #4294967297) at src/backend/arm/assembler/encoder/encode_neon_shrn_pbt.rs:482.
minimal failing input: rd = 0, rn = 0, ta_shift = (
    "8h",
    4294967297,
), is_high = false, opcode = 33
	successes: 1
	local rejects: 0
	global rejects: 0
```

### B4: encode_neon_shrn accepts a bare V dest without arrangement

**Formal:** ∀ rd,rn ∈ [0,31], kind ∈ {bare dest, bare src, GPR dest arrangement, GPR src, scalar dest}. encode_neon_shrn(kind) = Err ∧ llvm-mc(kind) = Err
**Contract evidence:** inferred (ARM SHRN dest is Vd.Tb; README.md:12 gas compatibility; llvm-mc rejects `shrn v0, v0.8h, #1`)
**Documentation conflict:** (none)
**Severity:** medium
**Counterexample:** encode_neon_shrn([Reg("v0"), v0.8h, #1], opcode=0b100001, is_high=false)
**Expected / Actual:** Err / Ok(Word) same as `shrn v0.8b, v0.8h, #1`
**Impact:** Bare-V or GPR-prefixed dest is silently encoded as a NEON vector register
**Root cause:** get_neon_reg (neon.rs:14-17) accepts Operand::Reg with an empty arrangement, and encode_neon_shrn (neon.rs:1438) discards dest arrangement
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:14`
```rust
        Some(Operand::Reg(name)) => {
            let num = parse_reg_num(name)
                .ok_or_else(|| format!("invalid register: {}", name))?;
            Ok((num, String::new()))
        }
```
**Suggested fix:** Require RegArrangement at the dest slot
```rust
    let (rd, arr_d) = match &operands[0] {
        Operand::RegArrangement { reg, arrangement } => {
            (parse_reg_num(reg).ok_or_else(|| format!("invalid NEON register: {}", reg))?, arrangement.clone())
        }
        other => return Err(format!("expected NEON register with arrangement at operand 0, got {:?}", other)),
    };
```
**Bug report:** bug_reports/encode_neon_shrn_bare_dest.md
**Repro seed:** (none — deterministic regression)
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_neon_shrn_pbt::encode_neon_shrn_neg_gpr_or_bare' (2365156) panicked at src/backend/arm/assembler/encoder/encode_neon_shrn_pbt.rs:492:1:
Test failed: GPR/bare/non-arrangement kind=2 must Err (llvm-mc rejects shrn v0, v0.8h, #1) at src/backend/arm/assembler/encoder/encode_neon_shrn_pbt.rs:552.
minimal failing input: rd = 0, rn = 0, kind = 2, fp_prefix = "x"
	successes: 0
	local rejects: 0
	global rejects: 0
```

## Design Caveats

(none)

## Test Files Created

| File | Tests |
|------|-------|
| src/backend/arm/assembler/encoder/encode_neon_shrn_pbt.rs | 11 properties + 1 KAT + 4 regression witnesses |
| src/backend/arm/assembler/encoder/mod.rs | one-line `#[cfg(test)] mod encode_neon_shrn_pbt` |

## Reproduction

Whole suite:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_neon_shrn -- --test-threads=1
```

B1 extra operand:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_shrn_regression_extra_operand -- --test-threads=1 --exact
```

B2 mismatched dest Tb:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_shrn_regression_mismatched_dest_tb -- --test-threads=1 --exact
```

B3 i64 shift trunc:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_shrn_regression_shift_i64_trunc -- --test-threads=1 --exact
```

B4 bare dest:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_shrn_regression_bare_dest -- --test-threads=1 --exact
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
- pbt-out/bug_reports/encode_neon_shrn_extra_operand.md
- pbt-out/bug_reports/encode_neon_shrn_extra_operand.html
- pbt-out/bug_reports/encode_neon_shrn_mismatched_dest_tb.md
- pbt-out/bug_reports/encode_neon_shrn_mismatched_dest_tb.html
- pbt-out/bug_reports/encode_neon_shrn_shift_i64_trunc.md
- pbt-out/bug_reports/encode_neon_shrn_shift_i64_trunc.html
- pbt-out/bug_reports/encode_neon_shrn_bare_dest.md
- pbt-out/bug_reports/encode_neon_shrn_bare_dest.html
- pbt-out/CHANGE_SURFACE.md
- pbt-out/build.log

## Coverage Report

# PBT Coverage Status

> Last updated: 2026-10-05 15:57 (campaign: coverage)
> Files: 10/10 scanned (100%) | Functions: 126/289 total | PBT candidates: 126 | Tested: 126 (100%) | 0 pass, 126 fail

## Summary

| Metric | Value |
|--------|-------|
| Total source files | 10 |
| Files scanned | 10 / 10 (100%) |
| Total functions (all files) | 289 |
| PBT candidates (from FUNCTION_INDEX) | 126 |
| **Tested (of PBT candidates)** | **126 / 126 (100%)** |
| &nbsp;&nbsp;↳ Pass / Fail / Other | 0 / 126 / 0 |
| **Overall (tested / all functions)** | **126 / 289 (44%)** |
| Untested | 0 |
| Skipped | 0 |

## Module Breakdown

| Module | Scanned | Tested | Skipped | Coverage |
|--------|---------|--------|---------|----------|
|  | 126 | 126 | 0 | 100% |

## Oracle Type Distribution

| Oracle Type | Total | Covered | Skipped | Coverage |
|-------------|-------|---------|---------|----------|
| unknown | 126 | 126 | 0 | 100% |

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
| neon.rs | 68 | 41 | 41 | 100% | covered |
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
