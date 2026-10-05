# PBT Campaign Report: encode_neon_scalar_addp

## Summary

**Verdict:** 2 medium: encode_neon_scalar_addp ignores a third operand and encodes non-D destinations / non-V source prefixes as scalar ADDP, so GNU-style `addp` that gas/llvm-mc reject still becomes a 32-bit word.
**Date:** 2026-10-05
**Repository:** /home/toan/github/claudes-c-compiler
**Modules tested:** encode_neon_scalar_addp
**Tests:** 9
**Result:** 7 passing, 2 bugs
**Change surface:** 1 changed function, 1 with a property, 0 error-handling changes
**Coverage evidence:** file-level (symbol presence) — coverage_gaps returned no .gcda/.profraw (Rust cargo test is not the C++ reporter); it listed unrelated host binaries and claimed encode_neon_scalar_addp NOT LINKED. Cargo tests executed the production symbol (KAT + 9 properties). Sweep: manual arm audit plus diff_alt_spellings.
**Tier:** standard

## Modules Tested

| Module | Tests | Bugs | Oracles Used |
|--------|-------|------|-------------|
| encode_neon_scalar_addp | 9 | 2 | differential, algebraic.metamorphic, algebraic.invariant, negative_error |

## Bugs Found

### B1: encode_neon_scalar_addp silently encodes a third operand

**Formal:** ∀ rd,rn,extra ∈ {0..31}. llvm-mc("addp d{rd}, v{rn}.2d, d{extra}") fails ∧ encode_neon_scalar_addp([Dd, Vn.2d, extra]) = Err
**Contract evidence:** documented neon.rs:1803 "scalar addp requires 2 operands"
**Documentation conflict:** neon.rs:1803 "scalar addp requires 2 operands" — the comment states the arity IS two operands; the code implements only `len < 2`, so extra operands violate the documented contract rather than declaring extra invalid. (not independently verified)
**Severity:** medium
**Counterexample:** encode_neon_scalar_addp([Reg("d0"), RegArrangement { reg: "v0", arrangement: "2d" }, Reg("d0")])
**Expected / Actual:** Err ("scalar addp requires 2 operands") / Ok(Word(0x5ef1b800))
**Impact:** Invalid assembly such as `addp d0, v0.2d, d0` is assembled into a scalar ADDP word instead of an error. encode() currently routes only arity==2 into this helper.
**Root cause:** neon.rs:1803 checks only `operands.len() < 2`, so arity 3+ is treated as a 2-operand encode using the first two operands
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:1803`
```rust
    if operands.len() < 2 { return Err("scalar addp requires 2 operands".to_string()); }
```
**Suggested fix:** Reject any arity other than 2
```rust
    if operands.len() != 2 { return Err("scalar addp requires 2 operands".to_string()); }
```
**Bug report:** bug_reports/encode_neon_scalar_addp_extra_operand.md
**Repro seed:** cc 94cb64af59dacd9676418d6d37ebce4c2b4d5f5cfd85fcd686f0f08be1f94bd4
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_neon_scalar_addp_pbt::encode_neon_scalar_addp_neg_extra_operand' (2493589) panicked at src/backend/arm/assembler/encoder/encode_neon_scalar_addp_pbt.rs:170:1:
Test failed: 3 operands must Err (llvm-mc rejects addp d0, v0.2d, d0) at src/backend/arm/assembler/encoder/encode_neon_scalar_addp_pbt.rs:278.
minimal failing input: rd = 0, rn = 0, extra = 0
	successes: 0
	local rejects: 0
	global rejects: 0
```

### B2: encode_neon_scalar_addp encodes non-D dest and non-V source as scalar ADDP

**Formal:** ∀ rd,rn ∈ {0..31}, pfx ∈ {s,h,b,x,w,q,v,sp,xzr,wsp,wzr,lr}. llvm-mc rejects addp with dest pfx (or source pfx.2d) ∧ encode_neon_scalar_addp on that operand vector = Err
**Contract evidence:** documented neon.rs:1801 "NEON scalar ADDP: addp Dd, Vn.2d"; neon.rs:1805 "expected d register"
**Documentation conflict:** neon.rs:1801 "NEON scalar ADDP: addp Dd, Vn.2d" states the form IS Dd, Vn.2d; neon.rs:1805 "expected d register" states dest is a d register. Neither declares s/x prefixes invalid as an input-domain restriction on parse_reg_num — they assert the required form, which the code violates. (not independently verified)
**Severity:** medium
**Counterexample:** encode_neon_scalar_addp([Reg("s0"), RegArrangement { reg: "v0", arrangement: "2d" }]); also encode_neon_scalar_addp([Reg("d0"), RegArrangement { reg: "x0", arrangement: "2d" }])
**Expected / Actual:** Err (non-D dest / non-V source) / Ok(Word(0x5ef1b800))
**Impact:** Invalid assembly such as `addp s0, v0.2d` or caller-reachable `addp d0, x0.2d` is assembled into a scalar ADDP word. encode() sanitizes non-D dest but passes a non-V .2d source through.
**Root cause:** neon.rs:1805 extracts Rd via parse_reg_num without requiring a D prefix; neon.rs:1808 extracts Rn via parse_reg_num on the arrangement register without requiring a V prefix
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:1805`
```rust
    let rd = match &operands[0] { Operand::Reg(r) => parse_reg_num(r).ok_or("invalid reg")?, _ => return Err("expected d register".to_string()) };
```
**Suggested fix:** Require a D dest prefix and a V source prefix before packing
```rust
    let rd_name = r.to_lowercase();
    if !rd_name.starts_with('d') {
        return Err(format!("scalar addp requires Dd dest, got {r}"));
    }
    let rd = parse_reg_num(r).ok_or("invalid reg")?;
    if !reg.to_lowercase().starts_with('v') {
        return Err(format!("scalar addp requires Vn.2d source, got {reg}"));
    }
```
**Bug report:** bug_reports/encode_neon_scalar_addp_wrong_reg_class.md
**Repro seed:** (shrunk by proptest to rd=0, rn=0, which=0, dest_pfx="s")
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_neon_scalar_addp_pbt::encode_neon_scalar_addp_neg_wrong_reg_class' (2494598) panicked at src/backend/arm/assembler/encoder/encode_neon_scalar_addp_pbt.rs:170:1:
Test failed: wrong register class which=0 must Err (llvm-mc rejects addp s0, v0.2d) at src/backend/arm/assembler/encoder/encode_neon_scalar_addp_pbt.rs:324.
minimal failing input: rd = 0, rn = 0, which = 0, dest_pfx = "s", src_pfx = "s"
	successes: 0
	local rejects: 0
	global rejects: 0
```

## Design Caveats

(none)

## Test Files Created

| File | Tests |
|------|-------|
| src/backend/arm/assembler/encoder/encode_neon_scalar_addp_pbt.rs | 9 properties + 1 KAT + 3 regression witnesses |

## Reproduction

Whole suite:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_neon_scalar_addp -- --test-threads=1
```

B1 extra operand:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_neon_scalar_addp_neg_extra_operand -- --test-threads=1
```

B2 wrong register class:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_neon_scalar_addp_neg_wrong_reg_class -- --test-threads=1
```

## Output Directories

- pbt-out/REPORT.md
- pbt-out/REPORT.html
- pbt-out/PROPERTIES.md
- pbt-out/PLAN.md
- pbt-out/COVERAGE.md
- pbt-out/COVERAGE_STATUS.md
- pbt-out/report.json
- pbt-out/FUNCTION_INDEX.md
- pbt-out/INVARIANTS.md
- pbt-out/bug_reports/encode_neon_scalar_addp_extra_operand.md
- pbt-out/bug_reports/encode_neon_scalar_addp_extra_operand.html
- pbt-out/bug_reports/encode_neon_scalar_addp_wrong_reg_class.md
- pbt-out/bug_reports/encode_neon_scalar_addp_wrong_reg_class.html

## Coverage Report

# PBT Coverage Status

> Last updated: 2026-10-05 22:07 (campaign: coverage)
> Files: 10/10 scanned (100%) | Functions: 145/289 total | PBT candidates: 145 | Tested: 145 (100%) | 0 pass, 145 fail

## Summary

| Metric | Value |
|--------|-------|
| Total source files | 10 |
| Files scanned | 10 / 10 (100%) |
| Total functions (all files) | 289 |
| PBT candidates (from FUNCTION_INDEX) | 145 |
| **Tested (of PBT candidates)** | **145 / 145 (100%)** |
| &nbsp;&nbsp;↳ Pass / Fail / Other | 0 / 145 / 0 |
| **Overall (tested / all functions)** | **145 / 289 (50%)** |
| Untested | 0 |
| Skipped | 0 |

## Module Breakdown

| Module | Scanned | Tested | Skipped | Coverage |
|--------|---------|--------|---------|----------|
|  | 145 | 145 | 0 | 100% |

## Oracle Type Distribution

| Oracle Type | Total | Covered | Skipped | Coverage |
|-------------|-------|---------|---------|----------|
| unknown | 145 | 145 | 0 | 100% |

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
| neon.rs | 68 | 60 | 60 | 100% | covered |
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
