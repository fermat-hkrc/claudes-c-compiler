# PBT Campaign Report: encode_neon_eor3

## Summary

**Verdict:** 4 medium: encode_neon_eor3 silently encodes a fifth operand, non-.16B arrangements, mismatched T, and GPR/bare-V dest as a valid 16B EOR3 word, so invalid SHA3 assembly becomes machine code instead of an error.
**Date:** 2026-10-05
**Repository:** /home/toan/github/claudes-c-compiler
**Modules tested:** encode_neon_eor3
**Tests:** 10 properties (plus 1 KAT + 4 failing regression witnesses)
**Result:** 6 passing, 4 bugs
**Change surface:** 1 changed function, 1 with a property, 0 error-handling changes
**Coverage evidence:** file-level (symbol presence) — coverage_gaps reported no .gcda/.profraw and listed encode_neon_eor3 as NOT LINKED in unrelated C++ pbt binaries. The cargo lib-test binary executed encode_neon_eor3 (7 passing + 8 failing unit tests including KAT and regressions). Sweep: manual arm audit of the four-statement body plus alt-spellings and non-register error-path properties.
**Tier:** standard

## Modules Tested

| Module | Tests | Bugs | Oracles Used |
|--------|-------|------|-------------|
| encode_neon_eor3 | 10 | 4 | differential, algebraic.metamorphic, algebraic.invariant, negative_error |

## Bugs Found

### B1: encode_neon_eor3 ignores a fifth operand

**Formal:** ∀ rd,rn,rm,rk,extra ∈ {0..31}. encode_neon_eor3([Vrd.16b, Vrn.16b, Vrm.16b, Vrk.16b, Vextra.16b]) = Err
**Contract evidence:** documented neon.rs:1114 "eor3 requires 4 operands" plus inferred (README.md:12 gas-compatible; llvm-mc rejects a fifth operand)
**Documentation conflict:** neon.rs:1114 "eor3 requires 4 operands" states the arity; the check is `len < 4`, so extras are accepted. The comment does not declare extra operands valid.
**Severity:** medium
**Counterexample:** encode_neon_eor3([v0.16b, v0.16b, v0.16b, v0.16b, v0.16b])
**Expected / Actual:** Err / Ok(Word(0xce000000))
**Impact:** A fifth operand is dropped; the assembler emits a 16B EOR3 instead of diagnosing invalid assembly
**Root cause:** neon.rs:1113 uses `operands.len() < 4`, so operands after the first four are ignored
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:1113`
```rust
    if operands.len() < 4 {
        return Err("eor3 requires 4 operands".to_string());
    }
```
**Suggested fix:** Reject arity other than 4
```rust
    if operands.len() != 4 {
        return Err("eor3 requires 4 operands".to_string());
    }
```
**Bug report:** bug_reports/encode_neon_eor3_extra_operand.md
**Repro seed:** cc 2e291ddd3d73e6cec4ef9a65222204f10403c2176767690106d56cdb58fb402e
**Raw output:**
```text
Test failed: 5 operands must Err (llvm-mc rejects eor3 v0.16b, v0.16b, v0.16b, v0.16b, v0.16b)
minimal failing input: rd = 0, rn = 0, rm = 0, rk = 0, extra = 0
```

### B2: encode_neon_eor3 encodes arrangements other than .16B

**Formal:** ∀ rd,rn,rm,rk ∈ {0..31}. ∀ t ∈ {8b,4h,8h,2s,4s,2d,1d}. encode_neon_eor3([Vrd.t, Vrn.t, Vrm.t, Vrk.t]) = Err
**Contract evidence:** documented neon.rs:1111 "Encode NEON EOR3 (three-way XOR, SHA3 extension): EOR3 Vd.16b, Vn.16b, Vm.16b, Vk.16b"
**Documentation conflict:** neon.rs:1111 states the instruction form is Vd.16b; the body discards arrangement and always emits the 16B word. The comment asserts the behavior is .16B-only, which the code violates.
**Severity:** medium
**Counterexample:** encode_neon_eor3([v0.8b, v0.8b, v0.8b, v0.8b])
**Expected / Actual:** Err / Ok(Word(0xce000000))
**Impact:** Illegal SHA3 assembly such as `eor3 v0.8b, ...` becomes a valid 16B EOR3 encoding
**Root cause:** neon.rs:1115-1118 discards every arrangement (`let (rd, _)`) then always packs the SHA3 16B word
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:1115`
```rust
    let (rd, _) = get_neon_reg(operands, 0)?;
    let (rn, _) = get_neon_reg(operands, 1)?;
    let (rm, _) = get_neon_reg(operands, 2)?;
    let (rk, _) = get_neon_reg(operands, 3)?;
```
**Suggested fix:** Require arrangement `16b` on every operand
```rust
    let (rd, arr_d) = get_neon_reg(operands, 0)?;
    let (rn, arr_n) = get_neon_reg(operands, 1)?;
    let (rm, arr_m) = get_neon_reg(operands, 2)?;
    let (rk, arr_k) = get_neon_reg(operands, 3)?;
    if arr_d != "16b" || arr_n != "16b" || arr_m != "16b" || arr_k != "16b" {
        return Err("eor3 requires .16b arrangement".to_string());
    }
```
**Bug report:** bug_reports/encode_neon_eor3_invalid_t.md
**Repro seed:** (deterministic after shrink; t = "8b", rd=rn=rm=rk=0)
**Raw output:**
```text
Test failed: T=8b must Err (ARM SHA3 EOR3 is 16B only; llvm-mc rejects eor3 v0.8b, v0.8b, v0.8b, v0.8b)
minimal failing input: rd = 0, rn = 0, rm = 0, rk = 0, t = "8b"
```

### B3: encode_neon_eor3 ignores mismatched arrangements

**Formal:** ∀ rd,rn,rm,rk ∈ {0..31}. ∀ td,tn,tm,tk ∈ {8b,16b,4h,8h,2s,4s,2d}. (∃ T=16b) ∧ (∃ T≠16b) ⇒ encode_neon_eor3([Vrd.td, Vrn.tn, Vrm.tm, Vrk.tk]) = Err
**Contract evidence:** documented neon.rs:1111 "Encode NEON EOR3 (three-way XOR, SHA3 extension): EOR3 Vd.16b, Vn.16b, Vm.16b, Vk.16b"
**Documentation conflict:** neon.rs:1111 names .16b on all four operands; mixed T is accepted because arrangements are discarded
**Severity:** medium
**Counterexample:** encode_neon_eor3([v0.16b, v0.8b, v0.8b, v0.8b])
**Expected / Actual:** Err / Ok(Word(0xce000000))
**Impact:** A source/dest arrangement typo encodes as 16B EOR3 instead of an error
**Root cause:** neon.rs:1115-1118 discards every arrangement, so mixed T never fails
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:1115`
```rust
    let (rd, _) = get_neon_reg(operands, 0)?;
    let (rn, _) = get_neon_reg(operands, 1)?;
    let (rm, _) = get_neon_reg(operands, 2)?;
    let (rk, _) = get_neon_reg(operands, 3)?;
```
**Suggested fix:** Require arrangement `16b` on every operand
```rust
    let (rd, arr_d) = get_neon_reg(operands, 0)?;
    let (rn, arr_n) = get_neon_reg(operands, 1)?;
    let (rm, arr_m) = get_neon_reg(operands, 2)?;
    let (rk, arr_k) = get_neon_reg(operands, 3)?;
    if arr_d != "16b" || arr_n != "16b" || arr_m != "16b" || arr_k != "16b" {
        return Err("eor3 requires .16b arrangement".to_string());
    }
```
**Bug report:** bug_reports/encode_neon_eor3_mismatched_t.md
**Repro seed:** cc ef02d57cdc9ac2da69a9e061d34368fce029823dfac1c3ba1c0154e50fff7822
**Raw output:**
```text
Test failed: mismatched T must Err (llvm-mc/gas reject eor3 v0.16b, v0.8b, v0.8b, v0.8b)
minimal failing input: rd = 0, rn = 0, rm = 0, rk = 0, td = "16b", tn = "8b", tm = "8b", tk = "8b"
```

### B4: encode_neon_eor3 accepts GPR, bare V, and non-arrangement operands

**Formal:** ∀ rd,rn,rm,rk ∈ {0..31}. ∀ kind ∈ GPR-dest | bare-V-src | X-src | bare-V-dest | X.16b-dest. encode_neon_eor3(ops(kind)) = Err
**Contract evidence:** documented neon.rs:1111 "Encode NEON EOR3 (three-way XOR, SHA3 extension): EOR3 Vd.16b, Vn.16b, Vm.16b, Vk.16b"
**Documentation conflict:** neon.rs:1111 states Vd.16b form; get_neon_reg's Operand::Reg arm accepts x/w/d/s/q/v/h/b/sp and EOR3 ignores the empty arrangement
**Severity:** medium
**Counterexample:** encode_neon_eor3([Reg("x0"), v0.16b, v0.16b, v0.16b])
**Expected / Actual:** Err / Ok(Word(0xce000000))
**Impact:** `eor3 x0, v0.16b, v0.16b, v0.16b` encodes as `eor3 v0.16b, ...` because parse_reg_num maps x0 to 0
**Root cause:** neon.rs:1115 calls get_neon_reg, whose Operand::Reg arm (neon.rs:14-17) accepts any parse_reg_num name and returns an empty arrangement that EOR3 ignores
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:1115`
```rust
    let (rd, _) = get_neon_reg(operands, 0)?;
```
**Suggested fix:** Require a V-prefixed RegArrangement with arrangement 16b
```rust
    let (rd, arr_d) = get_neon_reg(operands, 0)?;
    if !matches!(&operands[0], Operand::RegArrangement { reg, .. } if reg.starts_with('v') || reg.starts_with('V'))
        || arr_d != "16b"
    {
        return Err("eor3 requires Vd.16b".to_string());
    }
```
**Bug report:** bug_reports/encode_neon_eor3_gpr_or_bare.md
**Repro seed:** (deterministic after shrink; kind=0, fp_prefix="x", rd=rn=rm=rk=0)
**Raw output:**
```text
Test failed: GPR/bare/non-arrangement kind=0 must Err (llvm-mc rejects eor3 x0, v0.16b, v0.16b, v0.16b)
minimal failing input: rd = 0, rn = 0, rm = 0, rk = 0, kind = 0, fp_prefix = "x"
```

## Design Caveats

(none)

## Test Files Created

| File | Tests |
|------|-------|
| src/backend/arm/assembler/encoder/encode_neon_eor3_pbt.rs | 10 properties + 1 KAT + 4 regression witnesses |

## Reproduction

Valid-domain suite (passing properties + KAT):
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_neon_eor3_diff_llvm_mc -- --test-threads=1
cargo test --lib encode_neon_eor3_kat_llvm_mc -- --test-threads=1 --exact
```

Whole-suite command (4 property failures + 4 regression failures expected):
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_neon_eor3 -- --test-threads=1
```

B1 extra operand:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_eor3_regression_extra_operand -- --test-threads=1 --exact
```

B2 invalid T:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_eor3_regression_invalid_t -- --test-threads=1 --exact
```

B3 mismatched T:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_eor3_regression_mismatched_t -- --test-threads=1 --exact
```

B4 GPR dest:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_eor3_regression_gpr_dest -- --test-threads=1 --exact
```

## Output Directories

- pbt-out/REPORT.md
- pbt-out/REPORT.html
- pbt-out/PROPERTIES.md
- pbt-out/PLAN.md
- pbt-out/COVERAGE.md
- pbt-out/COVERAGE_STATUS.md
- pbt-out/INVARIANTS.md
- pbt-out/report.json
- pbt-out/FUNCTION_INDEX.md
- pbt-out/bug_reports/encode_neon_eor3_extra_operand.md
- pbt-out/bug_reports/encode_neon_eor3_extra_operand.html
- pbt-out/bug_reports/encode_neon_eor3_invalid_t.md
- pbt-out/bug_reports/encode_neon_eor3_invalid_t.html
- pbt-out/bug_reports/encode_neon_eor3_mismatched_t.md
- pbt-out/bug_reports/encode_neon_eor3_mismatched_t.html
- pbt-out/bug_reports/encode_neon_eor3_gpr_or_bare.md
- pbt-out/bug_reports/encode_neon_eor3_gpr_or_bare.html
- pbt-out/run/encode_neon_eor3_test.log
- pbt-out/run/encode_neon_eor3_test_round2.log

## Coverage Report

# PBT Coverage Status

> Last updated: 2026-10-05 13:33 (campaign: coverage)
> Files: 10/10 scanned (100%) | Functions: 119/289 total | PBT candidates: 119 | Tested: 119 (100%) | 0 pass, 119 fail

## Summary

| Metric | Value |
|--------|-------|
| Total source files | 10 |
| Files scanned | 10 / 10 (100%) |
| Total functions (all files) | 289 |
| PBT candidates (from FUNCTION_INDEX) | 119 |
| **Tested (of PBT candidates)** | **119 / 119 (100%)** |
| &nbsp;&nbsp;↳ Pass / Fail / Other | 0 / 119 / 0 |
| **Overall (tested / all functions)** | **119 / 289 (41%)** |
| Untested | 0 |
| Skipped | 0 |

## Module Breakdown

| Module | Scanned | Tested | Skipped | Coverage |
|--------|---------|--------|---------|----------|
|  | 119 | 119 | 0 | 100% |

## Oracle Type Distribution

| Oracle Type | Total | Covered | Skipped | Coverage |
|-------------|-------|---------|---------|----------|
| unknown | 119 | 119 | 0 | 100% |

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
| neon.rs | 68 | 34 | 34 | 100% | covered |
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
