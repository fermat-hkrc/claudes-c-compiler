# PBT Campaign Report: encode_neon_logical

## Summary

**Verdict:** 5 medium: encode_neon_logical silently accepts extra operands, mismatched/invalid arrangements, GPR sources, and ANDS (encoded as EOR), so vector AND/ORR/EOR typos assemble as the wrong instruction instead of failing like gas/llvm-mc.
**Date:** 2026-10-05
**Repository:** /home/toan/github/claudes-c-compiler
**Modules tested:** encode_neon_logical
**Tests:** 12
**Result:** 7 passing, 5 bugs
**Change surface:** 1 changed function, 1 with a property, 0 error-handling changes with a failure-path property
**Coverage evidence:** file-level (symbol presence) — coverage_gaps had no LLVM profraw (C++ reporter listed unrelated binaries and claimed NOT LINKED). Cargo tests execute encode_neon_logical (8 KAT + 12 properties). Sweep round 1/1 spent: ANDS, arity, unsupported opc, uppercase V.
**Tier:** standard

## Modules Tested

| Module | Tests | Bugs | Oracles Used |
|--------|-------|------|-------------|
| encode_neon_logical | 12 | 5 | differential, algebraic.metamorphic, algebraic.invariant, negative_error |

## Bugs Found

### B1: encode_neon_logical ignores a fourth operand

**Formal:** ∀ rd,rn,rm,extra ∈ {0..31}, T ∈ {8b,16b}, opc ∈ {0b00,0b01,0b10}. llvm-mc rejects mnemonic Vd.T,Vn.T,Vm.T,Vextra.T ⇒ encode_neon_logical([Vd.T,Vn.T,Vm.T,Vextra.T], opc) is Err
**Contract evidence:** documented neon.rs:296 "Encode NEON logical operations: ORR/AND/EOR Vd.T, Vn.T, Vm.T" (three operands); inferred (README.md:12 gas/llvm-mc reject a fourth operand)
**Documentation conflict:** (none)
**Severity:** medium
**Counterexample:** encode_neon_logical([v0.8b, v0.8b, v0.8b, v0.8b], opc=0)
**Expected / Actual:** Err / Ok(Word) of `and v0.8b, v0.8b, v0.8b`
**Impact:** A typo such as `and v0.8b, v1.8b, v2.8b, v3.8b` assembles instead of failing like gas/llvm-mc
**Root cause:** neon.rs:297-318 never checks operands.len(); it reads indices 0..2 and returns Word
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:298`
```rust
    let (rd, arr_d) = get_neon_reg(operands, 0)?;
```
**Suggested fix:** Reject anything other than exactly three operands
```rust
if operands.len() != 3 {
    return Err("NEON logical requires 3 operands".to_string());
}
```
**Bug report:** bug_reports/encode_neon_logical_extra_operand.md
**Repro seed:** cc 0ee0f868bfa6335bd9456d319a52171a5d81f957cd7251d5263c2b08167d79d9
**Raw output:** Test failed: extra operand must Err (llvm-mc rejects and v0.8b, v0.8b, v0.8b, v0.8b). minimal failing input: rd = 0, rn = 0, rm = 0, extra = 0, t = "8b", opc = 0

### B2: encode_neon_logical ignores mismatched source arrangements

**Formal:** ∀ rd,rn,rm ∈ {0..31}, Td,Tn,Tm ∈ {8b,16b}, opc ∈ {0b00,0b01,0b10}. ¬(Td=Tn=Tm) ⇒ encode_neon_logical([Vd.Td,Vn.Tn,Vm.Tm], opc) is Err
**Contract evidence:** documented neon.rs:296 "Encode NEON logical operations: ORR/AND/EOR Vd.T, Vn.T, Vm.T" (same T)
**Documentation conflict:** (none)
**Severity:** medium
**Counterexample:** encode_neon_logical([v0.8b, v0.8b, v0.16b], opc=0)
**Expected / Actual:** Err / Ok(Word) of `and v0.8b, v0.8b, v0.8b`
**Impact:** Mixed-width typos assemble as dest-Q AND/ORR/EOR
**Root cause:** neon.rs:299-300 bind `_arr_n` / `_arr_m` and never compare them to dest T
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:299`
```rust
    let (rn, _arr_n) = get_neon_reg(operands, 1)?;
```
**Suggested fix:** Require dest, Rn, and Rm arrangements to match
```rust
    let (rn, arr_n) = get_neon_reg(operands, 1)?;
    let (rm, arr_m) = get_neon_reg(operands, 2)?;
    if arr_d != arr_n || arr_n != arr_m {
        return Err(format!("NEON logical arrangement mismatch: .{arr_d}, .{arr_n}, .{arr_m}"));
    }
```
**Bug report:** bug_reports/encode_neon_logical_mismatch_t.md
**Repro seed:** (deterministic regression)
**Raw output:** Test failed: mismatched T must Err (llvm-mc rejects and v0.8b, v0.8b, v0.16b). minimal failing input: rd = 0, rn = 0, rm = 0, td = "8b", tn = "8b", tm = "16b", opc = 0

### B3: encode_neon_logical accepts arrangements other than .8b/.16b

**Formal:** ∀ rd,rn,rm ∈ {0..31}, T ∈ {4h,8h,2s,4s,2d,1d,4b,8d}, opc ∈ {0b00,0b01,0b10}. encode_neon_logical([Vd.T,Vn.T,Vm.T], opc) is Err
**Contract evidence:** documented neon.rs:296 "Encode NEON logical operations: ORR/AND/EOR Vd.T, Vn.T, Vm.T"; ARM ARM AND/ORR/EOR T in {8B,16B}
**Documentation conflict:** (none)
**Severity:** medium
**Counterexample:** encode_neon_logical([v0.4h, v0.4h, v0.4h], opc=0)
**Expected / Actual:** Err / Ok(Word) of `and v0.8b, v0.8b, v0.8b` (Q=0)
**Impact:** Invalid SIMD shapes assemble as the wrong Q-width instruction
**Root cause:** neon.rs:302 sets Q=1 only when dest is exactly "16b", else Q=0, with no {8b,16b} allow-list
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:302`
```rust
    let q: u32 = if arr_d == "16b" { 1 } else { 0 };
```
**Suggested fix:** Accept only .8b and .16b
```rust
    let q: u32 = match arr_d.as_str() {
        "8b" => 0,
        "16b" => 1,
        other => return Err(format!("NEON logical requires .8b or .16b, got .{other}")),
    };
```
**Bug report:** bug_reports/encode_neon_logical_invalid_t.md
**Repro seed:** (deterministic regression)
**Raw output:** Test failed: invalid T must Err (only .8b/.16b; llvm-mc rejects and v0.4h, v0.4h, v0.4h). minimal failing input: rd = 0, rn = 0, rm = 0, t = "4h", opc = 0

### B4: encode_neon_logical encodes GPR/SP/bare/FP sources as NEON registers

**Formal:** ∀ rd,rn,rm ∈ {0..31}, T ∈ {8b,16b}, opc ∈ {0b00,0b01,0b10}, kind ∈ {x-src, w-src, sp-src, bare-v, d-src, s-src, q-src}. encode_neon_logical([Vd.T, non-arranged Vn/Vm], opc) is Err
**Contract evidence:** documented neon.rs:296 "Encode NEON logical operations: ORR/AND/EOR Vd.T, Vn.T, Vm.T"; inferred (encode_logical passes Vn/Vm through when dest is RegArrangement)
**Documentation conflict:** (none)
**Severity:** medium
**Counterexample:** encode_neon_logical([v0.8b, x0, x0], opc=0)
**Expected / Actual:** Err / Ok(Word) of `and v0.8b, v0.8b, v0.8b`
**Impact:** Mixed GPR/vector typos produce the wrong instruction
**Root cause:** neon.rs:299-300 call get_neon_reg, which accepts Operand::Reg; GPR numbers are packed into Rn/Rm
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:299`
```rust
    let (rn, _arr_n) = get_neon_reg(operands, 1)?;
```
**Suggested fix:** Require a non-empty arrangement on every operand
```rust
    if arr_d.is_empty() || arr_n.is_empty() || arr_m.is_empty() {
        return Err("NEON logical requires arranged vector registers".to_string());
    }
```
**Bug report:** bug_reports/encode_neon_logical_gpr_src.md
**Repro seed:** (deterministic regression)
**Raw output:** Test failed: non-arranged NEON / GPR / SP / FP source must Err (llvm-mc rejects and v0.8b, x0, x0). minimal failing input: rd = 0, rn = 0, rm = 0, t = "8b", opc = 0, kind = 0

### B5: encode_neon_logical encodes ANDS vector form as EOR

**Formal:** ∀ rd,rn,rm ∈ {0..31}, T ∈ {8b,16b}. encode_neon_logical([Vd.T,Vn.T,Vm.T], 0b11) is Err
**Contract evidence:** documented limitation neon.rs:313 "ANDS - not valid for NEON, fall back"
**Documentation conflict:** neon.rs:313 "ANDS - not valid for NEON, fall back" — admits a gap on an input the API accepts (encode_logical routes "ands" with opc=0b11). Known limitation, not an input-domain exclusion.
**Severity:** medium (documented by the author)
**Counterexample:** encode_neon_logical([v0.8b, v0.8b, v0.8b], opc=0b11)
**Expected / Actual:** Err / Ok(Word) of `eor v0.8b, v0.8b, v0.8b`
**Impact:** `ands v0.16b, v1.16b, v2.16b` emits EOR instead of failing
**Root cause:** neon.rs:313 maps opc=0b11 to (U=1, size=00), the EOR encoding
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:313`
```rust
        0b11 => (1, 0b00),  // ANDS - not valid for NEON, fall back
```
**Suggested fix:** Return Err for opc=0b11
```rust
        0b11 => return Err("ANDS is not a NEON instruction".to_string()),
```
**Bug report:** bug_reports/encode_neon_logical_ands.md
**Repro seed:** (deterministic regression)
**Raw output:** Test failed: ANDS is not a NEON instruction (neon.rs:313); must Err (llvm-mc rejects ands v0.8b, v0.8b, v0.8b). minimal failing input: rd = 0, rn = 0, rm = 0, t = "8b"

## Design Caveats

(none)

## Test Files Created

| File | Tests |
|------|-------|
| src/backend/arm/assembler/encoder/encode_neon_logical_pbt.rs | 12 properties + 8 KAT + 5 regression witnesses |

## Reproduction

Whole suite:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_neon_logical -- --test-threads=1
```

B1 extra operand:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_logical_regression_extra_operand -- --test-threads=1
```

B2 mismatch T:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_logical_regression_mismatch_t -- --test-threads=1
```

B3 invalid T:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_logical_regression_invalid_t -- --test-threads=1
```

B4 GPR source:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_logical_regression_gpr_src -- --test-threads=1
```

B5 ANDS as EOR:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_logical_regression_ands -- --test-threads=1
```

## Output Directories

- pbt-out/PLAN.md
- pbt-out/PROPERTIES.md
- pbt-out/REPORT.md
- pbt-out/REPORT.html
- pbt-out/report.json
- pbt-out/COVERAGE.md
- pbt-out/COVERAGE_STATUS.md
- pbt-out/FUNCTION_INDEX.md
- pbt-out/INVARIANTS.md
- pbt-out/CHANGE_SURFACE.md
- pbt-out/run/encode_neon_logical_pbt.log
- pbt-out/run/encode_neon_logical_pbt_round2.log
- pbt-out/bug_reports/encode_neon_logical_extra_operand.md
- pbt-out/bug_reports/encode_neon_logical_extra_operand.html
- pbt-out/bug_reports/encode_neon_logical_mismatch_t.md
- pbt-out/bug_reports/encode_neon_logical_mismatch_t.html
- pbt-out/bug_reports/encode_neon_logical_invalid_t.md
- pbt-out/bug_reports/encode_neon_logical_invalid_t.html
- pbt-out/bug_reports/encode_neon_logical_gpr_src.md
- pbt-out/bug_reports/encode_neon_logical_gpr_src.html
- pbt-out/bug_reports/encode_neon_logical_ands.md
- pbt-out/bug_reports/encode_neon_logical_ands.html

## Coverage Report

# PBT Coverage Status

> Last updated: 2026-10-05 18:52 (campaign: coverage)
> Files: 10/10 scanned (100%) | Functions: 135/289 total | PBT candidates: 135 | Tested: 135 (100%) | 0 pass, 135 fail

## Summary

| Metric | Value |
|--------|-------|
| Total source files | 10 |
| Files scanned | 10 / 10 (100%) |
| Total functions (all files) | 289 |
| PBT candidates (from FUNCTION_INDEX) | 135 |
| **Tested (of PBT candidates)** | **135 / 135 (100%)** |
| &nbsp;&nbsp;↳ Pass / Fail / Other | 0 / 135 / 0 |
| **Overall (tested / all functions)** | **135 / 289 (47%)** |
| Untested | 0 |
| Skipped | 0 |

## Module Breakdown

| Module | Scanned | Tested | Skipped | Coverage |
|--------|---------|--------|---------|----------|
|  | 135 | 135 | 0 | 100% |

## Oracle Type Distribution

| Oracle Type | Total | Covered | Skipped | Coverage |
|-------------|-------|---------|---------|----------|
| unknown | 135 | 135 | 0 | 100% |

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
| neon.rs | 68 | 50 | 50 | 100% | covered |
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
