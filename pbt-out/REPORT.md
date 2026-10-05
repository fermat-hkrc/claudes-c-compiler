# PBT Campaign Report: encode_neon_scalar_three_same

## Summary

**Verdict:** 2 medium: encode_neon_scalar_three_same silently encodes a fourth operand (`add d0, d0, d0, d0`) and silently treats a non-D source as Dd (`add d0, s0, d0`), so invalid scalar ADD/SUB assembly becomes a 32-bit word instead of an error.
**Date:** 2026-10-05
**Repository:** /home/toan/github/claudes-c-compiler
**Modules tested:** encode_neon_scalar_three_same
**Tests:** 8
**Result:** 6 passing, 2 bugs
**Change surface:** 1 changed function, 1 with a property, 0 error-handling changes with a failure-path property
**Coverage evidence:** file-level (symbol presence) — coverage_gaps reported no .gcda/.profraw and listed unrelated C++ binaries as NOT LINKED; cargo test --lib executed the real symbol (KAT plus 6 passing properties).
**Tier:** standard

## Modules Tested

| Module | Tests | Bugs | Oracles Used |
|--------|-------|------|-------------|
| encode_neon_scalar_three_same | 8 | 2 | differential, algebraic.metamorphic, algebraic.invariant, negative_error |

## Bugs Found

### B1: encode_neon_scalar_three_same silently encodes a fourth operand

**Formal:** ∀ rd,rn,rm,extra ∈ {0..31}, is_sub ∈ {false,true}. llvm-mc rejects "{add|sub} Dd, Dn, Dm, Dx" ∧ encode_neon_scalar_three_same([Dd,Dn,Dm,Dx], U, 10000, 11) = Err
**Contract evidence:** documented neon.rs:1792 "scalar three-same requires 3 operands"
**Documentation conflict:** neon.rs:1792 "scalar three-same requires 3 operands" — states the arity IS 3; the code only rejects arity below 3, so extras are accepted. The comment/error is the contract, not an input-domain exclusion of extra operands.
**Severity:** medium
**Counterexample:** encode_neon_scalar_three_same([d0, d0, d0, d0], U=0, opcode=10000, size=11)
**Expected / Actual:** Err / Ok(Word(0x5ee08400)) encoding add d0, d0, d0
**Impact:** Invalid assembly such as add d0, d0, d0, d0 is assembled into a scalar ADD word instead of an error, so the GNU-style assembler emits machine code that gas/llvm-mc refuse
**Root cause:** neon.rs:1792 checks only operands.len() < 3, so arity 4+ is treated as a 3-operand encode using the first three registers
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:1792`
```rust
    if operands.len() < 3 { return Err("scalar three-same requires 3 operands".to_string()); }
```
**Suggested fix:** Reject any arity other than 3
```rust
    if operands.len() != 3 { return Err("scalar three-same requires 3 operands".to_string()); }
```
**Bug report:** bug_reports/encode_neon_scalar_three_same_extra_operand.md
**Repro seed:** cc 54721bbbff3ca73801a36fb9e9db3a6b84d46c6530d062249e840a9b5986026b
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_neon_scalar_three_same_pbt::encode_neon_scalar_three_same_neg_extra_operand' (2480918) panicked at src/backend/arm/assembler/encoder/encode_neon_scalar_three_same_pbt.rs:182:1:
Test failed: 4 operands must Err (llvm-mc rejects add d0, d0, d0, d0) at src/backend/arm/assembler/encoder/encode_neon_scalar_three_same_pbt.rs:321.
minimal failing input: rd = 0, rn = 0, rm = 0, extra = 0, is_sub = false
	successes: 0
	local rejects: 0
	global rejects: 0
```

### B2: encode_neon_scalar_three_same encodes non-D sources as Dd

**Formal:** ∀ rd,rn,rm ∈ {0..31}, is_sub ∈ {false,true}, slot ∈ {1,2}, pfx ∈ {s,h,b,x,w,q,v,sp,xzr}. dest is Dd ∧ source slot is non-D ∧ llvm-mc rejects the asm ∧ encode_neon_scalar_three_same(ops, U, 10000, 11) = Err
**Contract evidence:** documented neon.rs:1789 "NEON scalar three-same: ADD/SUB Dd, Dn, Dm"
**Documentation conflict:** neon.rs:1789 "NEON scalar three-same: ADD/SUB Dd, Dn, Dm" — asserts D registers; the code accepts any Operand::Reg parse_reg_num understands. The comment states the behavior IS handled as Dd/Dn/Dm, so this is documented-and-violated, not an exclusion.
**Severity:** medium
**Counterexample:** encode_neon_scalar_three_same([d0, s0, d0], U=0, opcode=10000, size=11)
**Expected / Actual:** Err / Ok(Word(0x5ee08400)) encoding as if add d0, d0, d0
**Impact:** Invalid assembly such as add d0, s0, d0 is assembled into a scalar ADD word. encode() routes here whenever dest is a d-register, so mixed-class sources are caller-reachable
**Root cause:** neon.rs:1794 extracts Rn via parse_reg_num, which accepts s/h/b/x/w/q/v/sp prefixes and never checks that the register is a D register required by size=11 ADD/SUB
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:1794`
```rust
    let rn = match &operands[1] { Operand::Reg(r) => parse_reg_num(r).ok_or("invalid reg")?, _ => return Err("expected register".to_string()) };
```
**Suggested fix:** Require a D-register prefix on every operand before packing
```rust
    if !r.to_lowercase().starts_with('d') {
        return Err(format!("scalar three-same requires Dd, Dn, Dm, got {r}"));
    }
```
**Bug report:** bug_reports/encode_neon_scalar_three_same_wrong_reg_class.md
**Repro seed:** (none — deterministic shrunk input)
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_neon_scalar_three_same_pbt::encode_neon_scalar_three_same_neg_wrong_reg_class' (2481271) panicked at src/backend/arm/assembler/encoder/encode_neon_scalar_three_same_pbt.rs:182:1:
Test failed: non-D source slot=1 pfx=s must Err (llvm-mc rejects add d0, s0, d0) at src/backend/arm/assembler/encoder/encode_neon_scalar_three_same_pbt.rs:352.
minimal failing input: rd = 0, rn = 0, rm = 0, is_sub = false, slot = 1, pfx = "s"
	successes: 0
	local rejects: 0
	global rejects: 0
```

## Design Caveats

(none)

## Test Files Created

| File | Tests |
|------|-------|
| src/backend/arm/assembler/encoder/encode_neon_scalar_three_same_pbt.rs | 8 properties + 1 KAT + 2 regression witnesses |

## Reproduction

Whole suite (includes two expected SUT failures):
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_neon_scalar_three_same -- --test-threads=1
```

B1 extra operand:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_neon_scalar_three_same -- --test-threads=1
cargo test --lib encode_neon_scalar_three_same_neg_extra_operand -- --test-threads=1
```

B2 wrong register class:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_neon_scalar_three_same -- --test-threads=1
cargo test --lib encode_neon_scalar_three_same_neg_wrong_reg_class -- --test-threads=1
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
- pbt-out/bug_reports/encode_neon_scalar_three_same_extra_operand.md
- pbt-out/bug_reports/encode_neon_scalar_three_same_extra_operand.html
- pbt-out/bug_reports/encode_neon_scalar_three_same_wrong_reg_class.md
- pbt-out/bug_reports/encode_neon_scalar_three_same_wrong_reg_class.html
- pbt-out/run/encode_neon_scalar_three_same_round1.log
- pbt-out/run/encode_neon_scalar_three_same_round2.log

## Coverage Report

# PBT Coverage Status

> Last updated: 2026-10-05 21:48 (campaign: coverage)
> Files: 10/10 scanned (100%) | Functions: 144/289 total | PBT candidates: 144 | Tested: 144 (100%) | 0 pass, 144 fail

## Summary

| Metric | Value |
|--------|-------|
| Total source files | 10 |
| Files scanned | 10 / 10 (100%) |
| Total functions (all files) | 289 |
| PBT candidates (from FUNCTION_INDEX) | 144 |
| **Tested (of PBT candidates)** | **144 / 144 (100%)** |
| &nbsp;&nbsp;↳ Pass / Fail / Other | 0 / 144 / 0 |
| **Overall (tested / all functions)** | **144 / 289 (50%)** |
| Untested | 0 |
| Skipped | 0 |

## Module Breakdown

| Module | Scanned | Tested | Skipped | Coverage |
|--------|---------|--------|---------|----------|
|  | 144 | 144 | 0 | 100% |

## Oracle Type Distribution

| Oracle Type | Total | Covered | Skipped | Coverage |
|-------------|-------|---------|---------|----------|
| unknown | 144 | 144 | 0 | 100% |

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
| neon.rs | 68 | 59 | 59 | 100% | covered |
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
