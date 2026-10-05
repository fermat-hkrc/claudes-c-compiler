# PBT Campaign Report: encode_neon_two_misc

## Summary

**Verdict:** 1 high, 3 medium: encode_neon_two_misc encodes legal SADDLP/UADDLP/SADALP/UADALP with size from dest T (so `saddlp v0.4h, v0.8b` becomes `saddlp v0.2s, v0.4h`), and silently accepts a third operand, mismatched T, and opcode-reserved arrangements that llvm-mc/gas reject.
**Date:** 2026-10-05
**Repository:** /home/toan/github/claudes-c-compiler
**Modules tested:** encode_neon_two_misc
**Tests:** 10
**Result:** 6 passing, 4 bugs
**Change surface:** 1 changed function (encode_neon_two_misc), 1 with a property, 0 error-handling changes
**Coverage evidence:** file-level (symbol presence) — `coverage_gaps` reported no .gcda/.profraw (C++ reporter listed unrelated binaries and claimed encode_neon_two_misc NOT LINKED). Manual arm audit of the function body plus two sweep properties (alt-spellings, nonreg).
**Tier:** standard

## Modules Tested

| Module | Tests | Bugs | Oracles Used |
|--------|-------|------|-------------|
| encode_neon_two_misc | 10 | 4 | differential, algebraic.metamorphic, algebraic.invariant, negative_error |

## Bugs Found

### B1: encode_neon_two_misc encodes pairwise-long size from dest T

**Formal:** ∀ rd,rn ∈ {0..31}, ∀ (mnem,u,opc,Tb,Ta) ∈ pairwise_domain. encode_neon_two_misc([Vd.Ta, Vn.Tb], u, opc) = llvm-mc("{mnem} Vd.Ta, Vn.Tb")
**Contract evidence:** inferred (ARM Advanced SIMD two-misc SADDLP/UADDLP/SADALP/UADALP size is source esize; README.md:12 gas compatibility; encoder/mod.rs:630-633 dispatches those mnemonics to this symbol; llvm-mc `saddlp v0.4h, v0.8b` = 0x0e202800)
**Documentation conflict:** (none)
**Severity:** high
**Counterexample:** encode_neon_two_misc([v0.4h, v0.8b], u_bit=0, opcode=0b00010)
**Expected / Actual:** 0x0e202800 (llvm-mc saddlp v0.4h, v0.8b) / 0x0e602800 (saddlp v0.2s, v0.4h)
**Impact:** Legal pairwise-long SIMD is assembled as a different element size, so compiler-emitted saddlp/uaddlp/sadalp/uadalp execute the wrong operation; dest 2d encodes reserved size=11
**Root cause:** neon.rs:1408-1410 takes Q and size from the destination arrangement and discards the source arrangement, but ARM pairwise-long size is the source esize
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:1408`
```rust
    let (rd, arr_d) = get_neon_reg(operands, 0)?;
    let (rn, _) = get_neon_reg(operands, 1)?;
    let (q, size) = neon_arr_to_q_size(&arr_d)?;
```
**Suggested fix:** For opcodes 00010/00110 derive size from source Tb and Q from dest Ta; require the ARM (Tb,Ta) pairing
```rust
    let (rd, arr_d) = get_neon_reg(operands, 0)?;
    let (rn, arr_n) = get_neon_reg(operands, 1)?;
    let (q, size) = if opcode == 0b00010 || opcode == 0b00110 {
        match (arr_n.as_str(), arr_d.as_str()) {
            ("8b", "4h") => (0u32, 0b00u32),
            ("16b", "8h") => (1, 0b00),
            ("4h", "2s") => (0, 0b01),
            ("8h", "4s") => (1, 0b01),
            ("2s", "1d") => (0, 0b10),
            ("4s", "2d") => (1, 0b10),
            _ => return Err(format!("pairwise-long: expected (Tb,Ta) pair, got {}, {}", arr_n, arr_d)),
        }
    } else {
        if arr_d != arr_n {
            return Err(format!("two-misc: arrangement mismatch {} vs {}", arr_d, arr_n));
        }
        neon_arr_to_q_size(&arr_d)?
    };
```
**Bug report:** bug_reports/encode_neon_two_misc_pairwise_size_from_dest.md
**Repro seed:** cc ecd3a9262a72ce1de119024b9ca513193459372fb4fbc1fc2992662970d2d229
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_neon_two_misc_pbt::encode_neon_two_misc_diff_llvm_mc_pairwise' (2372447) panicked at src/backend/arm/assembler/encoder/encode_neon_two_misc_pbt.rs:356:1:
Test failed: assertion failed: `(left == right)`
  left: `241182720`,
 right: `236988416`: SUT vs llvm-mc for saddlp v0.4h, v0.8b at src/backend/arm/assembler/encoder/encode_neon_two_misc_pbt.rs:384.
minimal failing input: rd = 0, rn = 0, mnem_case = (
    "saddlp",
    0,
    2,
), pair = (
    "8b",
    "4h",
)
	successes: 0
	local rejects: 0
	global rejects: 0
```

### B2: encode_neon_two_misc ignores a third operand

**Formal:** ∀ rd,rn,extra ∈ {0..31}, ∀ T ∈ matching_T(ABS). llvm-mc("abs Vd.T, Vn.T, Vextra.T") = Err ∧ encode_neon_two_misc([Vd.T,Vn.T,Vextra.T], 0, ABS_opc) = Err
**Contract evidence:** inferred (ARM two-misc is two-operand; README.md:12 gas compatibility; llvm-mc rejects a third operand)
**Documentation conflict:** (none)
**Severity:** medium
**Counterexample:** encode_neon_two_misc([v0.8b, v0.8b, v0.8b], u_bit=0, opcode=0b01011)
**Expected / Actual:** Err / Ok(Word) same as `abs v0.8b, v0.8b`
**Impact:** Invalid assembly with a trailing operand is silently encoded instead of diagnosed
**Root cause:** neon.rs:1408-1409 only reads operands[0] and operands[1]; there is no arity check
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:1408`
```rust
    let (rd, arr_d) = get_neon_reg(operands, 0)?;
    let (rn, _) = get_neon_reg(operands, 1)?;
```
**Suggested fix:** Reject arity other than 2
```rust
    if operands.len() != 2 {
        return Err("NEON two-misc requires 2 operands".to_string());
    }
```
**Bug report:** bug_reports/encode_neon_two_misc_extra_operand.md
**Repro seed:** (none — shrunk to rd=0, rn=0, extra=0, t="8b"; deterministic regression)
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_neon_two_misc_pbt::encode_neon_two_misc_neg_extra' (2372485) panicked at src/backend/arm/assembler/encoder/encode_neon_two_misc_pbt.rs:356:1:
Test failed: extra operand must Err (llvm-mc rejects abs v0.8b, v0.8b, v0.8b) at src/backend/arm/assembler/encoder/encode_neon_two_misc_pbt.rs:470.
minimal failing input: rd = 0, rn = 0, extra = 0, t = "8b"
	successes: 0
	local rejects: 0
	global rejects: 0
```

### B3: encode_neon_two_misc ignores source arrangement

**Formal:** ∀ rd,rn ∈ {0..31}, ∀ Td ≠ Tn ∈ abs_legal. llvm-mc("abs Vd.Td, Vn.Tn") = Err ∧ encode_neon_two_misc([Vd.Td,Vn.Tn], 0, ABS_opc) = Err
**Contract evidence:** inferred (ARM matching-T two-misc requires identical arrangements; README.md:12 gas compatibility; llvm-mc rejects `abs v0.8b, v0.16b`)
**Documentation conflict:** (none)
**Severity:** medium
**Counterexample:** encode_neon_two_misc([v0.8b, v0.16b], u_bit=0, opcode=0b01011)
**Expected / Actual:** Err / Ok(Word) encoded as `abs v0.8b, v0.8b`
**Impact:** Wrong-arrangement ABS/NEG/CLS is silently accepted and encoded as a different legal instruction
**Root cause:** neon.rs:1409 discards the source arrangement (`let (rn, _)`), so Q and size come only from dest
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:1409`
```rust
    let (rn, _) = get_neon_reg(operands, 1)?;
```
**Suggested fix:** Require dest and source arrangements to match for non-pairwise opcodes
```rust
    let (rn, arr_n) = get_neon_reg(operands, 1)?;
    if arr_d != arr_n {
        return Err(format!("two-misc: arrangement mismatch {} vs {}", arr_d, arr_n));
    }
```
**Bug report:** bug_reports/encode_neon_two_misc_mismatch_t.md
**Repro seed:** (none — shrunk to rd=0, rn=0, td="8b", tn="16b"; deterministic regression)
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_neon_two_misc_pbt::encode_neon_two_misc_neg_mismatch' (2372499) panicked at src/backend/arm/assembler/encoder/encode_neon_two_misc_pbt.rs:356:1:
Test failed: mismatched T must Err (llvm-mc rejects abs v0.8b, v0.16b) at src/backend/arm/assembler/encoder/encode_neon_two_misc_pbt.rs:492.
minimal failing input: rd = 0, rn = 0, td = "8b", tn = "16b"
	successes: 0
	local rejects: 0
	global rejects: 0
```

### B4: encode_neon_two_misc encodes opcode-reserved arrangements

**Formal:** ∀ rd,rn ∈ {0..31}, ∀ (mnem,u,opc,T) ∈ reserved_T_domain. llvm-mc("{mnem} Vd.T, Vn.T") = Err ∧ encode_neon_two_misc([Vd.T,Vn.T], u, opc) = Err
**Contract evidence:** inferred (ARM two-misc reserves ABS/NEG/SQABS/SQNEG 1D, CLS/CLZ 2D, REV16 not-byte, REV32 not-byte/half; README.md:12 gas compatibility; llvm-mc rejects `abs v0.1d, v0.1d` and `cls v0.2d, v0.2d`)
**Documentation conflict:** (none)
**Severity:** medium
**Counterexample:** encode_neon_two_misc([v0.1d, v0.1d], u_bit=0, opcode=0b01011)
**Expected / Actual:** Err / Ok(Word) with Q=0 size=11
**Impact:** Reserved encodings are emitted for assembly llvm-mc/gas reject, so invalid vector ABS .1d / CLS .2d assemble instead of diagnosing
**Root cause:** neon.rs:1410 uses neon_arr_to_q_size, which maps 1d/2d, with no opcode-specific arrangement check
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:1410`
```rust
    let (q, size) = neon_arr_to_q_size(&arr_d)?;
```
**Suggested fix:** Reject arrangements that ARM reserves for the given opcode
```rust
    let (q, size) = neon_arr_to_q_size(&arr_d)?;
    let reserved = match (opcode, u_bit, arr_d.as_str()) {
        (0b01011, _, "1d") => true,
        (0b00111, _, "1d") => true,
        (0b00100, _, "1d" | "2d") => true,
        (0b00001, 0, t) if t != "8b" && t != "16b" => true,
        (0b00000, 1, "2s" | "4s" | "1d" | "2d") => true,
        _ => false,
    };
    if reserved {
        return Err(format!("two-misc: reserved arrangement {} for opcode {:05b}", arr_d, opcode));
    }
```
**Bug report:** bug_reports/encode_neon_two_misc_reserved_t.md
**Repro seed:** (none — shrunk to abs v0.1d, v0.1d; deterministic regression)
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_neon_two_misc_pbt::encode_neon_two_misc_neg_reserved_t' (2372514) panicked at src/backend/arm/assembler/encoder/encode_neon_two_misc_pbt.rs:356:1:
Test failed: reserved T must Err (llvm-mc rejects abs v0.1d, v0.1d) at src/backend/arm/assembler/encoder/encode_neon_two_misc_pbt.rs:513.
minimal failing input: rd = 0, rn = 0, case = (
    "abs",
    0,
    11,
    "1d",
)
	successes: 0
	local rejects: 0
	global rejects: 0
```

## Design Caveats

(none)

## Test Files Created

| File | Tests |
|------|-------|
| src/backend/arm/assembler/encoder/encode_neon_two_misc_pbt.rs | 10 properties + 11 KAT + 6 regression witnesses |
| src/backend/arm/assembler/encoder/mod.rs | one-line `#[cfg(test)] mod encode_neon_two_misc_pbt` |

## Reproduction

```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_neon_two_misc -- --test-threads=1
```

B1 pairwise:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_two_misc_regression_pairwise_size_from_dest -- --test-threads=1 --exact
```

B2 extra:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_two_misc_regression_extra_operand -- --test-threads=1 --exact
```

B3 mismatch:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_two_misc_regression_mismatch_t -- --test-threads=1 --exact
```

B4 reserved T:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_two_misc_regression_reserved_1d -- --test-threads=1 --exact
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
- pbt-out/bug_reports/encode_neon_two_misc_pairwise_size_from_dest.md
- pbt-out/bug_reports/encode_neon_two_misc_pairwise_size_from_dest.html
- pbt-out/bug_reports/encode_neon_two_misc_extra_operand.md
- pbt-out/bug_reports/encode_neon_two_misc_extra_operand.html
- pbt-out/bug_reports/encode_neon_two_misc_mismatch_t.md
- pbt-out/bug_reports/encode_neon_two_misc_mismatch_t.html
- pbt-out/bug_reports/encode_neon_two_misc_reserved_t.md
- pbt-out/bug_reports/encode_neon_two_misc_reserved_t.html
- pbt-out/run/encode_neon_two_misc.log
- pbt-out/run/encode_neon_two_misc_round2.log

## Coverage Report

# PBT Coverage Status

> Last updated: 2026-10-05 16:17 (campaign: coverage)
> Files: 10/10 scanned (100%) | Functions: 127/289 total | PBT candidates: 127 | Tested: 127 (100%) | 0 pass, 127 fail

## Summary

| Metric | Value |
|--------|-------|
| Total source files | 10 |
| Files scanned | 10 / 10 (100%) |
| Total functions (all files) | 289 |
| PBT candidates (from FUNCTION_INDEX) | 127 |
| **Tested (of PBT candidates)** | **127 / 127 (100%)** |
| &nbsp;&nbsp;↳ Pass / Fail / Other | 0 / 127 / 0 |
| **Overall (tested / all functions)** | **127 / 289 (44%)** |
| Untested | 0 |
| Skipped | 0 |

## Module Breakdown

| Module | Scanned | Tested | Skipped | Coverage |
|--------|---------|--------|---------|----------|
|  | 127 | 127 | 0 | 100% |

## Oracle Type Distribution

| Oracle Type | Total | Covered | Skipped | Coverage |
|-------------|-------|---------|---------|----------|
| unknown | 127 | 127 | 0 | 100% |

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
| neon.rs | 68 | 42 | 42 | 100% | covered |
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
