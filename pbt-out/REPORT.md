# PBT Campaign Report: encode_neon_cmp_zero

## Summary

**Verdict:** 4 medium: encode_neon_cmp_zero silently encodes assembly gas/llvm-mc reject — extra operand, mismatched T, reserved .1d, and GPR/non-V names as V registers.
**Date:** 2026-10-05
**Repository:** /home/toan/github/claudes-c-compiler
**Modules tested:** encode_neon_cmp_zero
**Tests:** 11 properties (plus 4 KAT + 5 regression witnesses)
**Result:** 7 passing, 4 failing properties, 4 bugs
**Change surface:** 1 changed function (encode_neon_cmp_zero), 1 with a property, 0 error-handling changes
**Coverage evidence:** file-level (symbol presence) — coverage_gaps returned no .gcda/.profraw (C++ reporter listed unrelated binaries and claimed NOT LINKED). Rust `cargo test --lib encode_neon_cmp_zero` executed the production symbol (4 KAT + 11 properties). Sweep was a manual arm audit plus invalid-T / non-register / uppercase-V properties.
**Tier:** standard

## Modules Tested

| Module | Tests | Bugs | Oracles Used |
|--------|-------|------|-------------|
| encode_neon_cmp_zero | 11 | 4 | differential, algebraic.metamorphic, algebraic.invariant, negative_error |

## Bugs Found

### B1: encode_neon_cmp_zero ignores a fourth operand

**Formal:** ∀ rd,rn,extra ∈ {0..31}, T ∈ valid_T, (U,opc,mnem) ∈ cmp_zero_table. llvm-mc(mnem Vd.T, Vn.T, #0, Vextra.T) = Err ∧ encode_neon_cmp_zero([Vd.T, Vn.T, Imm(0), Vextra.T], U, opc) = Err
**Contract evidence:** inferred (README.md:12 GNU-style/gas compatibility; llvm-mc rejects a fourth operand; dispatcher passes operands through)
**Documentation conflict:** (none) — neon.rs:191 states a minimum of 2 operands, not a maximum
**Severity:** medium
**Counterexample:** encode_neon_cmp_zero([v0.8b, v0.8b, Imm(0), v0.8b], u=0, opcode=0b01001) — `cmeq v0.8b, v0.8b, #0, v0.8b`
**Expected / Actual:** Err / Ok(Word) same as `cmeq v0.8b, v0.8b, #0`
**Impact:** A typo or extra operand is silently assembled instead of diagnosed.
**Root cause:** neon.rs:190 only rejects `operands.len() < 2`; operands at index 2+ are never inspected.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:190`
```rust
    if operands.len() < 2 {
        return Err("NEON compare-zero requires at least 2 operands".to_string());
    }
    let (rd, arr_d) = get_neon_reg(operands, 0)?;
    let (rn, _) = get_neon_reg(operands, 1)?;
```
**Suggested fix:** Reject more than two NEON registers; a third operand must be Imm(0).
```rust
    if operands.len() > 3 {
        return Err("NEON compare-zero: extra operand".to_string());
    }
    if operands.len() == 3 && !matches!(operands.get(2), Some(Operand::Imm(0))) {
        return Err("NEON compare-zero: expected #0".to_string());
    }
```
**Bug report:** bug_reports/encode_neon_cmp_zero_extra_operand.md
**Repro seed:** cc bbffe64bd42b91c8a80c4b97bd9b2f0f3f480f89b64b180b1b068fb4168f94fd
**Raw output:** Test failed: extra operand must Err (llvm-mc rejects cmeq v0.8b, v0.8b, #0, v0.8b). minimal failing input: rd = 0, rn = 0, extra = 0, t = "8b", insn = (0, 9, "cmeq")

### B2: encode_neon_cmp_zero ignores source arrangement mismatch

**Formal:** ∀ rd,rn ∈ {0..31}, Td ≠ Tn both in valid_T. llvm-mc(cmeq Vd.Td, Vn.Tn, #0) = Err ∧ encode_neon_cmp_zero([Vd.Td, Vn.Tn], 0, 0b01001) = Err
**Contract evidence:** inferred (ARM matching-T; README.md:12; llvm-mc invalid operand)
**Documentation conflict:** (none)
**Severity:** medium
**Counterexample:** encode_neon_cmp_zero([v0.16b, v0.8b], 0, 0b01001) — `cmeq v0.16b, v0.8b, #0`
**Expected / Actual:** Err / Ok(Word) encoded as if both were .16b
**Impact:** Mismatched lane arrangements assemble using only dest T.
**Root cause:** neon.rs:194 discards the source arrangement.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:194`
```rust
    let (rn, _) = get_neon_reg(operands, 1)?;
```
**Suggested fix:** Require arr_n == arr_d.
```rust
    let (rn, arr_n) = get_neon_reg(operands, 1)?;
    if arr_n != arr_d {
        return Err(format!("NEON compare-zero: arrangement mismatch {} vs {}", arr_d, arr_n));
    }
```
**Bug report:** bug_reports/encode_neon_cmp_zero_mismatch_t.md
**Repro seed:** (deterministic after shrink: td=16b, tn=8b)
**Raw output:** Test failed: mismatched T must Err (llvm-mc rejects cmeq v0.16b, v0.8b, #0). minimal failing input: rd = 0, rn = 0, td = "16b", tn = "8b"

### B3: encode_neon_cmp_zero encodes reserved .1d (size:Q=11:0)

**Formal:** ∀ rd,rn ∈ {0..31}, (U,opc) in cmp_zero_u_opc. llvm-mc(cmeq Vd.1d, Vn.1d, #0) = Err ∧ encode_neon_cmp_zero([Vd.1d, Vn.1d], U, opc) = Err
**Contract evidence:** inferred (ARM ARM reserved size:Q=11:0 for integer compare-to-zero; llvm-mc invalid operand)
**Documentation conflict:** (none) — neon_arr_to_q_size accepts 1d for other NEON uses; this instruction must still reject it
**Severity:** medium
**Counterexample:** encode_neon_cmp_zero([v0.1d, v0.1d], u=0, opcode=8) — `cmeq v0.1d, v0.1d, #0`
**Expected / Actual:** Err / Ok(Word) with Q=0, size=11
**Impact:** Reserved encoding is emitted for an arrangement llvm-mc/gas reject.
**Root cause:** neon.rs:195 uses neon_arr_to_q_size, which maps "1d" to (0, 0b11) at neon.rs:52, without rejecting that reserved pair.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:52`
```rust
        "1d" => Ok((0, 0b11)),
```
**Suggested fix:** Reject Q=0,size=11 inside encode_neon_cmp_zero.
```rust
    let (q, size) = neon_arr_to_q_size(&arr_d)?;
    if q == 0 && size == 0b11 {
        return Err("NEON compare-zero: .1d is reserved".to_string());
    }
```
**Bug report:** bug_reports/encode_neon_cmp_zero_reserved_1d.md
**Repro seed:** (deterministic after shrink: rd=0, rn=0, u=0, opcode=8)
**Raw output:** Test failed: reserved .1d must Err (ARM size:Q=11:0 reserved; llvm-mc rejects cmeq v0.1d, v0.1d, #0). minimal failing input: rd = 0, rn = 0, u = 0, opcode = 8

### B4: encode_neon_cmp_zero accepts GPR/non-V names as NEON registers

**Formal:** ∀ kind ∈ {x-dest, w-dest, sp-dest, bare-v, arranged-x-prefix, GPR-src, s-dest}. llvm-mc rejects ∧ encode_neon_cmp_zero(ops, 0, 0b01001) = Err
**Contract evidence:** inferred (README.md:12; llvm-mc requires Vd.T / Vn.T)
**Documentation conflict:** (none)
**Severity:** medium
**Counterexample:** encode_neon_cmp_zero([RegArrangement{reg:"x0", arrangement:"8b"}, v0.8b], 0, 0b01001) — `cmeq x0.8b, v0.8b, #0`
**Expected / Actual:** Err / Ok(Word) identical to `cmeq v0.8b, v0.8b, #0`
**Impact:** GPR names assemble as V registers. Additional witness: `cmeq v0.8b, x0, #0` also encodes.
**Root cause:** get_neon_reg feeds any register name through parse_reg_num, which accepts x/w/d/s/q/v/h/b prefixes.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:9`
```rust
            let num = parse_reg_num(reg)
                .ok_or_else(|| format!("invalid NEON register: {}", reg))?;
            Ok((num, arrangement.clone()))
```
**Suggested fix:** Require a V-prefixed RegArrangement on both operands.
```rust
            Some(Operand::RegArrangement { reg, arrangement })
                if reg.starts_with('v') || reg.starts_with('V') => { /* parse */ }
```
**Bug report:** bug_reports/encode_neon_cmp_zero_non_v_prefix.md
**Repro seed:** (deterministic after shrink: kind=4, rd=0, rn=0, t=8b)
**Raw output:** Test failed: non-arranged NEON / GPR / SP / non-V prefix must Err (llvm-mc rejects cmeq x0.8b, v0.8b, #0). minimal failing input: rd = 0, rn = 0, t = "8b", kind = 4

## Design Caveats

(none)

## Test Files Created

| File | Tests |
|------|-------|
| src/backend/arm/assembler/encoder/encode_neon_cmp_zero_pbt.rs | 11 properties + 4 KAT + 5 regression witnesses |
| src/backend/arm/assembler/encoder/mod.rs | one-line `#[cfg(test)] mod encode_neon_cmp_zero_pbt` |

## Reproduction

Whole suite:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_neon_cmp_zero -- --test-threads=1
```

B1 extra operand:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_neon_cmp_zero_neg_extra -- --test-threads=1
```

B2 mismatch T:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_neon_cmp_zero_neg_mismatch_t -- --test-threads=1
```

B3 reserved 1d:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_neon_cmp_zero_neg_reserved_1d -- --test-threads=1
```

B4 non-V prefix:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_neon_cmp_zero_neg_gpr_bare_nonv -- --test-threads=1
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
- pbt-out/bug_reports/encode_neon_cmp_zero_extra_operand.md
- pbt-out/bug_reports/encode_neon_cmp_zero_extra_operand.html
- pbt-out/bug_reports/encode_neon_cmp_zero_mismatch_t.md
- pbt-out/bug_reports/encode_neon_cmp_zero_mismatch_t.html
- pbt-out/bug_reports/encode_neon_cmp_zero_reserved_1d.md
- pbt-out/bug_reports/encode_neon_cmp_zero_reserved_1d.html
- pbt-out/bug_reports/encode_neon_cmp_zero_non_v_prefix.md
- pbt-out/bug_reports/encode_neon_cmp_zero_non_v_prefix.html
- pbt-out/run/encode_neon_cmp_zero_test.log
- pbt-out/run/encode_neon_cmp_zero_test2.log
- pbt-out/run/encode_neon_cmp_zero_test3.log

## Coverage Report

# PBT Coverage Status

> Last updated: 2026-10-05 19:12 (campaign: coverage)
> Files: 10/10 scanned (100%) | Functions: 136/289 total | PBT candidates: 136 | Tested: 136 (100%) | 0 pass, 136 fail

## Summary

| Metric | Value |
|--------|-------|
| Total source files | 10 |
| Files scanned | 10 / 10 (100%) |
| Total functions (all files) | 289 |
| PBT candidates (from FUNCTION_INDEX) | 136 |
| **Tested (of PBT candidates)** | **136 / 136 (100%)** |
| &nbsp;&nbsp;↳ Pass / Fail / Other | 0 / 136 / 0 |
| **Overall (tested / all functions)** | **136 / 289 (47%)** |
| Untested | 0 |
| Skipped | 0 |

## Module Breakdown

| Module | Scanned | Tested | Skipped | Coverage |
|--------|---------|--------|---------|----------|
|  | 136 | 136 | 0 | 100% |

## Oracle Type Distribution

| Oracle Type | Total | Covered | Skipped | Coverage |
|-------------|-------|---------|---------|----------|
| unknown | 136 | 136 | 0 | 100% |

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
| neon.rs | 68 | 51 | 51 | 100% | covered |
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
