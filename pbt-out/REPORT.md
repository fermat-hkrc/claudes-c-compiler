# PBT Campaign Report: encode_neon_add_sub

## Summary

**Verdict:** 3 medium: encode_neon_add_sub silently encodes extra operands, mismatched/reserved T, and bare/GPR sources that llvm-mc and gas reject, so invalid NEON ADD/SUB becomes wrong machine code.
**Date:** 2026-10-05
**Repository:** /home/toan/github/claudes-c-compiler
**Modules tested:** encode_neon_add_sub
**Tests:** 9
**Result:** 6 passing, 3 bugs
**Change surface:** 1 changed function, 1 with a property, 0 error-handling changes
**Coverage evidence:** file-level (symbol presence) — coverage_gaps reported no .gcda/.profraw (build tree not instrumented); C++ reporter listed unrelated binaries and claimed encode_neon_add_sub NOT LINKED. Cargo lib tests executed the symbol (KAT + 9 properties). Sweep round 1/1: manual arm audit plus alt-spellings and nonreg.

**Tier:** standard

## Modules Tested

| Module | Tests | Bugs | Oracles Used |
|--------|-------|------|-------------|
| encode_neon_add_sub | 9 | 3 | differential, algebraic.metamorphic, algebraic.invariant, negative_error |

## Bugs Found

### B1: encode_neon_add_sub ignores a fourth operand

**Formal:** ∀ rd,rn,rm,extra ∈ {0..31}, T ∈ {8b,16b,4h,8h,2s,4s,2d}, is_sub ∈ {false,true}. llvm-mc rejects 4-operand ADD/SUB ⇒ encode_neon_add_sub([Vd.T,Vn.T,Vm.T,Vextra.T], is_sub) = Err
**Contract evidence:** documented neon.rs:1163 "Encode NEON ADD/SUB (vector integer): ADD/SUB Vd.T, Vn.T, Vm.T"
**Documentation conflict:** neon.rs:1163 states the three-operand form ADD/SUB Vd.T, Vn.T, Vm.T — the comment is the contract the code violates by accepting a fourth operand. (not independently verified)
**Severity:** medium
**Counterexample:** encode_neon_add_sub([v0.8b, v0.8b, v0.8b, v0.8b], is_sub=false)
**Expected / Actual:** Err / Ok(Word(0x0e208400))
**Impact:** Invalid assembly with a trailing operand is assembled as the three-operand form instead of diagnosed
**Root cause:** neon.rs:1164-1167 reads only operands 0..2 via get_neon_reg and never checks operands.len()
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:1164`
```rust
pub(crate) fn encode_neon_add_sub(operands: &[Operand], is_sub: bool) -> Result<EncodeResult, String> {
    let (rd, arr_d) = get_neon_reg(operands, 0)?;
    let (rn, _) = get_neon_reg(operands, 1)?;
    let (rm, _) = get_neon_reg(operands, 2)?;
```
**Suggested fix:** Reject arity other than 3
```rust
    if operands.len() != 3 {
        return Err("add/sub requires 3 operands".to_string());
    }
```
**Bug report:** bug_reports/encode_neon_add_sub_extra_operand.md
**Repro seed:** cc 34296dadfc5978db9ff90f0747750242b22fcd62b9ed2e59e79338abbe5f4e20
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_neon_add_sub_pbt::encode_neon_add_sub_neg_extra_operand' (2332177) panicked at src/backend/arm/assembler/encoder/encode_neon_add_sub_pbt.rs:221:1:
Test failed: 4 operands must Err (llvm-mc rejects add v0.8b, v0.8b, v0.8b, v0.8b) at src/backend/arm/assembler/encoder/encode_neon_add_sub_pbt.rs:341.
minimal failing input: rd = 0, rn = 0, rm = 0, extra = 0, t = "8b", is_sub = false
	successes: 0
	local rejects: 0
	global rejects: 0
```

### B2: encode_neon_add_sub encodes mismatched and reserved arrangements

**Formal:** ∀ rd,rn,rm ∈ {0..31}, Td,Tn,Tm ∈ {8b,16b,4h,8h,2s,4s,1d,2d,1q}, is_sub ∈ {false,true}. ¬valid(Td,Tn,Tm) ∧ llvm-mc rejects ⇒ encode_neon_add_sub([Vd.Td,Vn.Tn,Vm.Tm], is_sub) = Err. valid iff Td=Tn=Tm ∈ {8b,16b,4h,8h,2s,4s,2d}
**Contract evidence:** documented neon.rs:1163 "Encode NEON ADD/SUB (vector integer): ADD/SUB Vd.T, Vn.T, Vm.T"
**Documentation conflict:** neon.rs:1163 states ADD/SUB Vd.T, Vn.T, Vm.T (same T) — the comment is the contract; reserved 1D is ARM-reserved, not excluded by an input-domain comment. (not independently verified)
**Severity:** medium
**Counterexample:** encode_neon_add_sub([v0.8b, v0.8b, v0.16b], is_sub=false)
**Expected / Actual:** Err / Ok(Word(0x0e208400))
**Impact:** Mismatched source T is ignored (encoded from dest T only); reserved .1d encodes size:Q=11:0
**Root cause:** neon.rs:1166-1168 discards source arrangements and neon_arr_to_q_size accepts 1d
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:1166`
```rust
    let (rn, _) = get_neon_reg(operands, 1)?;
    let (rm, _) = get_neon_reg(operands, 2)?;
    let (q, size) = neon_arr_to_q_size(&arr_d)?;
```
**Suggested fix:** Require matching T in {8b,16b,4h,8h,2s,4s,2d}
```rust
    let (rn, arr_n) = get_neon_reg(operands, 1)?;
    let (rm, arr_m) = get_neon_reg(operands, 2)?;
    if arr_d != arr_n || arr_d != arr_m {
        return Err(format!("add/sub arrangement mismatch: .{arr_d}, .{arr_n}, .{arr_m}"));
    }
    if !matches!(arr_d.as_str(), "8b" | "16b" | "4h" | "8h" | "2s" | "4s" | "2d") {
        return Err(format!("unsupported add/sub arrangement: {arr_d}"));
    }
```
**Bug report:** bug_reports/encode_neon_add_sub_invalid_t.md
**Repro seed:** (none saved; shrunk to td=8b, tn=8b, tm=16b)
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_neon_add_sub_pbt::encode_neon_add_sub_neg_invalid_t' (2332199) panicked at src/backend/arm/assembler/encoder/encode_neon_add_sub_pbt.rs:221:1:
Test failed: invalid/mismatched/reserved T must Err (ARM ADD/SUB T in {8B,16B,4H,8H,2S,4S,2D} matching; llvm-mc rejects add v0.8b, v0.8b, v0.16b) at src/backend/arm/assembler/encoder/encode_neon_add_sub_pbt.rs:367.
minimal failing input: rd = 0, rn = 0, rm = 0, is_sub = false, td = "8b", tn = "8b", tm = "16b"
	successes: 0
	local rejects: 0
	global rejects: 0
```

### B3: encode_neon_add_sub encodes bare V and GPR operands as NEON registers

**Formal:** ∀ rd,rn,rm ∈ {0..31}, is_sub ∈ {false,true}, kind ∈ {gpr_dest, bare_vn, x_rm, bare_vd, x_vd_arr}. llvm-mc rejects the corresponding asm ⇒ encode_neon_add_sub(ops(kind), is_sub) = Err
**Contract evidence:** documented neon.rs:1163 "Encode NEON ADD/SUB (vector integer): ADD/SUB Vd.T, Vn.T, Vm.T"
**Documentation conflict:** neon.rs:1163 states Vd.T, Vn.T, Vm.T — the comment is the contract the code violates by accepting Operand::Reg and xN.T. (not independently verified)
**Severity:** medium
**Counterexample:** encode_neon_add_sub([v0.8b, Reg("v0"), v0.8b], is_sub=false)
**Expected / Actual:** Err / Ok(Word(0x0e208400))
**Impact:** Bare V sources and GPR-prefixed arrangements encode as vN, so invalid assembly becomes a NEON ADD
**Root cause:** get_neon_reg at neon.rs:14 accepts Operand::Reg and parse_reg_num maps x/w/d/s/q/v/h/b to 0–31; source arrangement is discarded
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:14`
```rust
        Some(Operand::Reg(name)) => {
            let num = parse_reg_num(name)
                .ok_or_else(|| format!("invalid register: {}", name))?;
            Ok((num, String::new()))
        }
```
**Suggested fix:** Require RegArrangement with a V register on every operand
```rust
        Some(Operand::RegArrangement { reg, arrangement }) => {
            if !reg.to_lowercase().starts_with('v') {
                return Err(format!("expected NEON V register, got {reg}"));
            }
            let num = parse_reg_num(reg)
                .ok_or_else(|| format!("invalid NEON register: {}", reg))?;
            Ok((num, arrangement.clone()))
        }
        other => Err(format!("expected NEON register at operand {}, got {:?}", idx, other)),
```
**Bug report:** bug_reports/encode_neon_add_sub_bare_src.md
**Repro seed:** cc f275616237b2718fb365df4cb6a6736d41af3e079c7f017a0f5393d9228384ea
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_neon_add_sub_pbt::encode_neon_add_sub_neg_gpr_or_bare' (2332187) panicked at src/backend/arm/assembler/encoder/encode_neon_add_sub_pbt.rs:221:1:
Test failed: GPR/bare/non-arrangement kind=1 must Err (llvm-mc rejects add v0.8b, v0, v0.8b) at src/backend/arm/assembler/encoder/encode_neon_add_sub_pbt.rs:434.
minimal failing input: rd = 0, rn = 0, rm = 0, is_sub = false, kind = 1, fp_prefix = "x"
	successes: 0
	local rejects: 0
	global rejects: 0
```

## Design Caveats

(none)

## Test Files Created

| File | Tests |
|------|-------|
| src/backend/arm/assembler/encoder/encode_neon_add_sub_pbt.rs | 9 properties + 1 KAT + 5 regression witnesses |
| src/backend/arm/assembler/encoder/mod.rs | one-line `mod encode_neon_add_sub_pbt` registration |

## Reproduction

Valid-domain suite (KAT + passing properties):
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_neon_add_sub -- --test-threads=1
```

B1 extra operand:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_add_sub_regression_extra_operand -- --test-threads=1 --exact
```

B2 mismatched T:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_add_sub_regression_mismatched_t -- --test-threads=1 --exact
```

B3 bare source:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_add_sub_regression_bare_src -- --test-threads=1 --exact
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
- pbt-out/bug_reports/encode_neon_add_sub_extra_operand.md
- pbt-out/bug_reports/encode_neon_add_sub_extra_operand.html
- pbt-out/bug_reports/encode_neon_add_sub_invalid_t.md
- pbt-out/bug_reports/encode_neon_add_sub_invalid_t.html
- pbt-out/bug_reports/encode_neon_add_sub_bare_src.md
- pbt-out/bug_reports/encode_neon_add_sub_bare_src.html
- pbt-out/run/encode_neon_add_sub_test.log
- pbt-out/run/encode_neon_add_sub_regression.log

## Coverage Report

# PBT Coverage Status

> Last updated: 2026-10-05 14:13 (campaign: coverage)
> Files: 10/10 scanned (100%) | Functions: 121/289 total | PBT candidates: 121 | Tested: 121 (100%) | 0 pass, 121 fail

## Summary

| Metric | Value |
|--------|-------|
| Total source files | 10 |
| Files scanned | 10 / 10 (100%) |
| Total functions (all files) | 289 |
| PBT candidates (from FUNCTION_INDEX) | 121 |
| **Tested (of PBT candidates)** | **121 / 121 (100%)** |
| &nbsp;&nbsp;↳ Pass / Fail / Other | 0 / 121 / 0 |
| **Overall (tested / all functions)** | **121 / 289 (42%)** |
| Untested | 0 |
| Skipped | 0 |

## Module Breakdown

| Module | Scanned | Tested | Skipped | Coverage |
|--------|---------|--------|---------|----------|
|  | 121 | 121 | 0 | 100% |

## Oracle Type Distribution

| Oracle Type | Total | Covered | Skipped | Coverage |
|-------------|-------|---------|---------|----------|
| unknown | 121 | 121 | 0 | 100% |

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
| neon.rs | 68 | 36 | 36 | 100% | covered |
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
