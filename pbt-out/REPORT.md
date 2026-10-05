# PBT Campaign Report: encode_neon_faddp

## Summary

**Verdict:** 3 medium: encode_neon_faddp silently encodes a fourth operand, mismatched source arrangements, and scalar dest/source size mismatches that llvm-mc/gas reject, so invalid FADDP text becomes a 32-bit instruction.
**Date:** 2026-10-05
**Repository:** /home/toan/github/claudes-c-compiler
**Modules tested:** encode_neon_faddp
**Tests:** 9
**Result:** 6 passing, 3 bugs
**Change surface:** 1 changed function, 1 with a property, 0 error-handling changes
**Coverage evidence:** file-level (symbol presence) — coverage_gaps reported no .gcda/.profraw (C++ reporter listed unrelated binaries and claimed NOT LINKED); the cargo lib tests did execute encode_neon_faddp. Tier: standard.

## Modules Tested

| Module | Tests | Bugs | Oracles Used |
|--------|-------|------|-------------|
| encode_neon_faddp | 9 | 3 | differential, algebraic.metamorphic, algebraic.invariant, negative_error |

## Bugs Found

### B1: encode_neon_faddp silently encodes a fourth operand

**Formal:** ∀ rd,rn,rm,extra ∈ {0..31}, T ∈ {2s,4s,2d}. llvm-mc rejects "faddp Vd.T, Vn.T, Vm.T, Vextra.T" ⇒ encode_neon_faddp([Vd.T,Vn.T,Vm.T,Vextra.T]) is Err. Also ∀ n ∈ {0,1}. encode_neon_faddp(ops with n operands) is Err
**Contract evidence:** documented neon.rs:1718 "faddp requires 2 or 3 operands"; README.md:12 gas-compatible assembly (llvm-mc rejects a fourth operand)
**Documentation conflict:** neon.rs:1718 `"faddp requires 2 or 3 operands"` states the behavior IS handled (arity not in {2,3} is an error). The code contradicts it for arity ≥ 4. (not independently verified)
**Severity:** medium
**Counterexample:** encode_neon_faddp([v0.2s, v0.2s, v0.2s, v0.2s])
**Expected / Actual:** Err / Ok(Word) encoding vector FADDP v0.2s, v0.2s, v0.2s
**Impact:** Invalid assembly is assembled instead of rejected, so gas/llvm-mc-incompatible source produces a silent encoding
**Root cause:** neon.rs:1687 uses `operands.len() >= 3` for the vector form, so arity 4+ is treated as a 3-operand vector encode; the "requires 2 or 3" error only fires for arity < 2
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:1687`
```rust
    if operands.len() >= 3 {
```
**Suggested fix:** Dispatch on exact arity 3 (vector) or 2 (scalar); reject anything else
```rust
    match operands.len() {
        3 => { /* vector form */ }
        2 => { /* scalar form */ }
        _ => Err("faddp requires 2 or 3 operands".to_string()),
    }
```
**Bug report:** bug_reports/encode_neon_faddp_extra_operand.md
**Repro seed:** cc a409152dfb73ee7721e22249c08d6ed4182c4e7645f5f6f1b794a8c53a3cdfdc
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_neon_faddp_pbt::encode_neon_faddp_neg_extra' (2471457) panicked at src/backend/arm/assembler/encoder/encode_neon_faddp_pbt.rs:217:1:
Test failed: extra operand must Err (llvm-mc rejects faddp v0.2s, v0.2s, v0.2s, v0.2s) at src/backend/arm/assembler/encoder/encode_neon_faddp_pbt.rs:347.
minimal failing input: rd = 0, rn = 0, rm = 0, extra = 0, t = "2s", n = 0
	successes: 0
	local rejects: 0
	global rejects: 0
```

### B2: encode_neon_faddp ignores source arrangements

**Formal:** ∀ rd,rn,rm ∈ {0..31}, Td,Tn,Tm ∈ {2s,4s,2d}. ¬(Td=Tn=Tm) ∧ llvm-mc rejects "faddp Vd.Td, Vn.Tn, Vm.Tm" ⇒ encode_neon_faddp([Vd.Td,Vn.Tn,Vm.Tm]) is Err
**Contract evidence:** documented neon.rs:1682 "Vector form: FADDP Vd.T, Vn.T, Vm.T"; README.md:12 gas-compatible assembly
**Documentation conflict:** neon.rs:1682 `"Vector form: FADDP Vd.T, Vn.T, Vm.T"` states one T for all three operands. The code contradicts it by encoding from dest T only. (not independently verified)
**Severity:** medium
**Counterexample:** encode_neon_faddp([v0.2d, v0.2s, v0.2s])
**Expected / Actual:** Err / Ok(Word) encoding FADDP v0.2d (Q=1, sz=1 from dest T)
**Impact:** Mismatched assembly is encoded as dest-T FADDP using source register numbers, dropping Vn/Vm arrangements
**Root cause:** neon.rs:1691-1692 bind source arrangements as `_` and derive Q/sz only from dest T
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:1691`
```rust
        let (rn, _) = get_neon_reg(operands, 1)?;
        let (rm, _) = get_neon_reg(operands, 2)?;
```
**Suggested fix:** Require Vn and Vm arrangements to equal Vd.T
```rust
        let (rn, arr_n) = get_neon_reg(operands, 1)?;
        let (rm, arr_m) = get_neon_reg(operands, 2)?;
        if arr_n != arr_d || arr_m != arr_d {
            return Err(format!("faddp: mismatched arrangement: {arr_d} vs {arr_n} vs {arr_m}"));
        }
```
**Bug report:** bug_reports/encode_neon_faddp_mismatch_t.md
**Repro seed:** (none — deterministic regression)
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_neon_faddp_pbt::encode_neon_faddp_neg_mismatch_t' (2472509) panicked at src/backend/arm/assembler/encoder/encode_neon_faddp_pbt.rs:217:1:
Test failed: mismatched T must Err (llvm-mc rejects faddp v0.2d, v0.2s, v0.2s) at src/backend/arm/assembler/encoder/encode_neon_faddp_pbt.rs:392.
minimal failing input: rd = 0, rn = 0, rm = 0, td = "2d", tn = "2s", tm = "2s"
	successes: 0
	local rejects: 0
	global rejects: 0
```

### B3: encode_neon_faddp scalar form does not check dest size against source T

**Formal:** ∀ inputs in {GPR triple, W dest + vector, SP dest, bare V triple, D dest + Vn.2S, S dest + Vn.2D, X dest + Vn.2S, S dest + Vn.4S}. llvm-mc rejects asm ⇒ encode_neon_faddp(ops) is Err
**Contract evidence:** documented neon.rs:1684 "Scalar form: FADDP Sd, Vn.2S  or FADDP Dd, Vn.2D"; README.md:12 gas-compatible assembly
**Documentation conflict:** neon.rs:1684 `"Scalar form: FADDP Sd, Vn.2S  or FADDP Dd, Vn.2D"` states dest size must match source T. The code contradicts it. (not independently verified)
**Severity:** medium
**Counterexample:** encode_neon_faddp([Reg("d0"), v0.2s])
**Expected / Actual:** Err / Ok(Word) encoding SISD FADDP sz=0 Rd=0 (same as faddp s0, v0.2s)
**Impact:** Dest D with .2S (and S with .2D, X with .2S) assembles as a different SISD FADDP than the text
**Root cause:** neon.rs:1704 accepts any Operand::Reg whose parse_reg_num succeeds and sets sz only from the source arrangement
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:1704`
```rust
            Operand::Reg(r) => parse_reg_num(r).ok_or("invalid dest reg")?,
```
**Suggested fix:** Require dest prefix s with source .2s, or dest prefix d with source .2d
```rust
        let dest = match &operands[0] {
            Operand::Reg(r) => r,
            _ => return Err("faddp scalar: expected register".to_string()),
        };
        let rd = parse_reg_num(dest).ok_or("invalid dest reg")?;
        let prefix = dest.chars().next().unwrap_or(' ').to_ascii_lowercase();
        let (rn, arr_n) = get_neon_reg(operands, 1)?;
        let sz = match (prefix, arr_n.as_str()) {
            ('s', "2s") => 0u32,
            ('d', "2d") => 1,
            _ => return Err(format!("faddp scalar: dest {dest} incompatible with source {arr_n}")),
        };
```
**Bug report:** bug_reports/encode_neon_faddp_scalar_dest_size.md
**Repro seed:** (none — deterministic regression)
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_neon_faddp_pbt::encode_neon_faddp_neg_gpr_bare_scalar_dest' (2471482) panicked at src/backend/arm/assembler/encoder/encode_neon_faddp_pbt.rs:217:1:
Test failed: non-arranged NEON / GPR / SP / dest-size mismatch must Err (llvm-mc rejects faddp d0, v0.2s) at src/backend/arm/assembler/encoder/encode_neon_faddp_pbt.rs:453.
minimal failing input: rd = 0, rn = 0, rm = 0, kind = 4
	successes: 0
	local rejects: 0
	global rejects: 0
```

## Design Caveats

(none)

## Test Files Created

| File | Tests |
|------|-------|
| src/backend/arm/assembler/encoder/encode_neon_faddp_pbt.rs | 9 properties + 6 KAT + 3 regression witnesses |

## Reproduction

Valid-domain suite (passing KAT / differential / metamorphic / invariant / invalid-T / alt-spellings):
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_neon_faddp -- --test-threads=1
```

B1 extra operand:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_neon_faddp_neg_extra -- --test-threads=1
```

B2 mismatched T:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_neon_faddp_neg_mismatch_t -- --test-threads=1
```

B3 scalar dest size:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_neon_faddp_neg_gpr_bare_scalar_dest -- --test-threads=1
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
- pbt-out/bug_reports/encode_neon_faddp_extra_operand.md
- pbt-out/bug_reports/encode_neon_faddp_extra_operand.html
- pbt-out/bug_reports/encode_neon_faddp_mismatch_t.md
- pbt-out/bug_reports/encode_neon_faddp_mismatch_t.html
- pbt-out/bug_reports/encode_neon_faddp_scalar_dest_size.md
- pbt-out/bug_reports/encode_neon_faddp_scalar_dest_size.html
- pbt-out/run/encode_neon_faddp.log
- pbt-out/run/encode_neon_faddp_regression.log
- pbt-out/run/encode_neon_faddp_alt.log

## Coverage Report

# PBT Coverage Status

> Last updated: 2026-10-05 21:28 (campaign: coverage)
> Files: 10/10 scanned (100%) | Functions: 143/289 total | PBT candidates: 143 | Tested: 143 (100%) | 0 pass, 143 fail

## Summary

| Metric | Value |
|--------|-------|
| Total source files | 10 |
| Files scanned | 10 / 10 (100%) |
| Total functions (all files) | 289 |
| PBT candidates (from FUNCTION_INDEX) | 143 |
| **Tested (of PBT candidates)** | **143 / 143 (100%)** |
| &nbsp;&nbsp;↳ Pass / Fail / Other | 0 / 143 / 0 |
| **Overall (tested / all functions)** | **143 / 289 (49%)** |
| Untested | 0 |
| Skipped | 0 |

## Module Breakdown

| Module | Scanned | Tested | Skipped | Coverage |
|--------|---------|--------|---------|----------|
|  | 143 | 143 | 0 | 100% |

## Oracle Type Distribution

| Oracle Type | Total | Covered | Skipped | Coverage |
|-------------|-------|---------|---------|----------|
| unknown | 143 | 143 | 0 | 100% |

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
| neon.rs | 68 | 58 | 58 | 100% | covered |
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
