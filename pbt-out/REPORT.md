# PBT Campaign Report: encode_neon_mvni

## Summary

**Verdict:** 2 high, 3 medium: encode_neon_mvni drops LSL #8 on .4h/.8h (silent wrong cmode), truncates immediates outside [0,255] to 8 bits, and accepts extra operands / illegal H shifts / LSR that llvm-mc and gas reject.
**Date:** 2026-10-05
**Repository:** /home/toan/github/claudes-c-compiler
**Modules tested:** encode_neon_mvni
**Tests:** 11 properties (plus 1 KAT + 5 regression witnesses)
**Result:** 6 passing, 5 failing properties (5 bugs)
**Change surface:** 1 changed function (encode_neon_mvni), 1 with a property, 0 error-handling changes
**Coverage evidence:** file-level (symbol presence) — coverage_gaps returned no LLVM profraw; C++ reporter listed unrelated binaries and claimed encode_neon_mvni NOT LINKED. Sweep was a manual arm audit of documented error paths.
**Tier:** standard

## Modules Tested

| Module | Tests | Bugs | Oracles Used |
|--------|-------|------|-------------|
| encode_neon_mvni | 11 properties (6 pass, 5 fail) + 1 KAT + 5 regressions | 5 | differential, algebraic.metamorphic, algebraic.invariant, negative_error |

## Bugs Found

### B1: encode_neon_mvni ignores LSL #8 on .4h/.8h

**Formal:** ∀ rd ∈ [0,31], T ∈ {4h,8h}, imm8 ∈ [0,255]. encode_neon_mvni([Vd.T, #imm8, lsl #8]) = llvm-mc("mvni Vd.T, #imm8, lsl #8")
**Contract evidence:** documented README.md:12 "It accepts the same textual assembly that GCC's gas would consume"; ARM AdvSIMD MVNI cmode=1010 for H LSL #8. neon.rs:1374 describes the no-shift encoding and does not declare LSL #8 invalid.
**Documentation conflict:** neon.rs:1374 "MVNI 16-bit: cmode=1000, op=1" — other (describes the no-shift encoding; does not declare LSL #8 invalid). (not independently verified) against ARM/llvm-mc, which accept LSL #8.
**Severity:** high
**Counterexample:** encode_neon_mvni([v0.4h, #0, lsl #8])
**Expected / Actual:** 0x2f00a400 / 0x2f008400
**Impact:** `mvni v0.4h, #0, lsl #8` is assembled as no-shift `mvni v0.4h, #0`, so the 8-bit immediate is placed in the wrong byte of each 16-bit lane.
**Root cause:** neon.rs:1378-1380 hard-codes cmode=1000 and never reads operands[2] on the 4h/8h path.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:1377`
```rust
            // MVNI 16-bit: cmode=1000, op=1
            let word = (q << 30) | (1 << 29) | (0b0111100 << 22)
                | (abc << 16) | (0b1000 << 12) | (0b01 << 10) | (defgh << 5) | rd;
            Ok(EncodeResult::Word(word))
```
**Suggested fix:** Decode optional LSL on 4h/8h: amount 0 → cmode=1000, amount 8 → cmode=1010, else Err.
```rust
            let cmode = if let Some(Operand::Shift { kind, amount }) = operands.get(2) {
                if kind.to_lowercase() == "lsl" {
                    match *amount {
                        0 => 0b1000u32,
                        8 => 0b1010,
                        _ => return Err(format!("mvni: unsupported shift amount: {}", amount)),
                    }
                } else {
                    return Err(format!("mvni: unsupported shift kind: {}", kind));
                }
            } else {
                0b1000
            };
```
**Bug report:** bug_reports/encode_neon_mvni_h_lsl8.md
**Repro seed:** cc 288b06d450d1dba90931f99133eea772a9dbe8baafbb610dc2a9d7456255f703
**Raw output:**
```text
Test failed: assertion failed: `(left == right)`
  left: `788562944`,
 right: `788571136`: mismatch for mvni v0.4h, #0, lsl #8
minimal failing input: rd = 0, t = "4h", imm8 = 0
```

### B2: encode_neon_mvni ignores extra operands

**Formal:** ∀ rd ∈ [0,31], T ∈ {4h,8h,2s,4s}, extra-or-illegal-shift operand. llvm-mc rejects asm ⇒ encode_neon_mvni(ops) is Err
**Contract evidence:** inferred (README.md:12 gas-compatible assembler; llvm-mc rejects a third non-shift operand). neon.rs:1335 documents a minimum of 2 operands, not a maximum.
**Documentation conflict:** (none)
**Severity:** medium
**Counterexample:** encode_neon_mvni([v0.4h, #0, v0.4h])
**Expected / Actual:** Err / Ok(Word) encoding mvni v0.4h, #0
**Impact:** Typos such as a leftover register after the immediate assemble silently as two-operand MVNI.
**Root cause:** neon.rs:1334 only rejects operands.len() < 2.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:1334`
```rust
    if operands.len() < 2 {
        return Err("mvni requires 2 operands".to_string());
    }
```
**Suggested fix:** Reject len > 3, and require operand 2 (when present) to be a legal Shift.
```rust
    if operands.len() > 3 {
        return Err("mvni: extra operand".to_string());
    }
    if operands.len() == 3 && !matches!(operands.get(2), Some(Operand::Shift { .. })) {
        return Err("mvni: expected optional shift".to_string());
    }
```
**Bug report:** bug_reports/encode_neon_mvni_extra_operand.md
**Repro seed:** cc 11b31a873604ce4cacbed0ab9218617b4cfac0f58137f7af23c045d6ccdd90cb
**Raw output:**
```text
Test failed: extra/illegal shift must Err (llvm-mc rejects mvni v0.4h, #0, v0.4h)
minimal failing input: rd = 0, extra = 0, t = "4h", imm8 = 0, kind = 0
```

### B3: encode_neon_mvni truncates out-of-range immediates

**Formal:** ∀ rd ∈ [0,31]. (imm ∉ [0,255] on a valid T, or T ∉ {4h,8h,2s,4s}). llvm-mc rejects asm ⇒ encode_neon_mvni(ops) is Err
**Contract evidence:** inferred (README.md:12 gas-compat; llvm-mc: "immediate must be an integer in range [0, 255]")
**Documentation conflict:** (none)
**Severity:** high
**Counterexample:** encode_neon_mvni([v0.4h, #256])
**Expected / Actual:** Err / Ok(Word) encoding #0 (256 as u32 & 0xFF)
**Impact:** `#256` becomes `#0` and `#-1` becomes `#255`; the assembler emits the wrong immediate with no error.
**Root cause:** neon.rs:1338 masks with `imm as u32 & 0xFF` instead of range-checking.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:1338`
```rust
    let imm8 = imm as u32 & 0xFF;
```
**Suggested fix:** Reject values outside [0, 255] before encoding.
```rust
    if imm < 0 || imm > 255 {
        return Err(format!("mvni: immediate {} out of range [0, 255]", imm));
    }
    let imm8 = imm as u32;
```
**Bug report:** bug_reports/encode_neon_mvni_imm_oor.md
**Repro seed:** (deterministic; first example kind=0, over=1)
**Raw output:**
```text
Test failed: OOR imm / invalid T must Err (llvm-mc rejects mvni v0.4h, #256)
minimal failing input: rd = 0, kind = 0, t_ok = "4h", t_bad = "8b", over = 1, neg = 1
```

### B4: encode_neon_mvni accepts illegal LSL amounts on .4h/.8h

**Formal:** ∀ rd ∈ [0,31], T ∈ {4h,8h,2s,4s}, extra-or-illegal-shift operand. llvm-mc rejects asm ⇒ encode_neon_mvni(ops) is Err
**Contract evidence:** inferred (README.md:12; llvm-mc rejects `mvni v0.4h, #0, lsl #32`)
**Documentation conflict:** (none)
**Severity:** medium
**Counterexample:** encode_neon_mvni([v0.4h, #0, lsl #32])
**Expected / Actual:** Err / Ok(Word) encoding no-shift MVNI
**Impact:** Illegal H shift amounts assemble as LSL #0 instead of failing.
**Root cause:** neon.rs:1378-1380 never inspects the optional shift on the 4h/8h path.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:1377`
```rust
            // MVNI 16-bit: cmode=1000, op=1
            let word = (q << 30) | (1 << 29) | (0b0111100 << 22)
                | (abc << 16) | (0b1000 << 12) | (0b01 << 10) | (defgh << 5) | rd;
            Ok(EncodeResult::Word(word))
```
**Suggested fix:** Parse optional LSL on 4h/8h and reject amounts other than 0 and 8.
```rust
                    match *amount {
                        0 => 0b1000u32,
                        8 => 0b1010,
                        _ => return Err(format!("mvni: unsupported shift amount: {}", amount)),
                    }
```
**Bug report:** bug_reports/encode_neon_mvni_illegal_h_shift.md
**Repro seed:** (deterministic regression)
**Raw output:**
```text
mvni v0.4h, #0, lsl #32 must Err (llvm-mc rejects illegal H LSL amount)
```

### B5: encode_neon_mvni treats LSR as no-shift

**Formal:** ∀ rd ∈ [0,31], T ∈ {4h,8h,2s,4s}, extra-or-illegal-shift operand. llvm-mc rejects asm ⇒ encode_neon_mvni(ops) is Err
**Contract evidence:** inferred (README.md:12; llvm-mc rejects `mvni v0.4s, #0, lsr #8`; ARM MVNI shift kinds are LSL and MSL only)
**Documentation conflict:** (none)
**Severity:** medium
**Counterexample:** encode_neon_mvni([v0.4s, #0, lsr #8])
**Expected / Actual:** Err / Ok(Word) encoding no-shift MVNI (cmode=0000)
**Impact:** `lsr` (and other non-lsl/non-msl kinds) assemble as no-shift instead of an error.
**Root cause:** neon.rs:1364-1366 maps any non-lsl/non-msl shift kind to cmode=0000.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:1364`
```rust
                } else {
                    0b0000
                }
```
**Suggested fix:** Reject unknown shift kinds.
```rust
                } else {
                    return Err(format!("mvni: unsupported shift kind: {}", kind));
                }
```
**Bug report:** bug_reports/encode_neon_mvni_lsr.md
**Repro seed:** (deterministic regression)
**Raw output:**
```text
mvni v0.4s, #0, lsr #8 must Err (llvm-mc rejects LSR on MVNI)
```

## Design Caveats

(none)

## Test Files Created

| File | Tests |
|------|-------|
| src/backend/arm/assembler/encoder/encode_neon_mvni_pbt.rs | 11 properties + 1 KAT + 5 regression witnesses |
| src/backend/arm/assembler/encoder/mod.rs | one-line `#[cfg(test)] mod encode_neon_mvni_pbt` registration |

## Reproduction

Whole suite:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_neon_mvni -- --test-threads=1
```

B1 (h_lsl8):
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_neon_mvni_diff_h_lsl8 -- --test-threads=1
```

B2 (extra operand):
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_neon_mvni_neg_extra_and_illegal_shift -- --test-threads=1
```

B3 (imm OOR):
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_neon_mvni_neg_imm_oor_invalid_t -- --test-threads=1
```

B4 (illegal H shift):
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_mvni_regression_illegal_h_shift -- --test-threads=1
```

B5 (LSR):
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_mvni_regression_lsr -- --test-threads=1
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
- pbt-out/run/encode_neon_mvni.log
- pbt-out/run/encode_neon_mvni_sweep.log
- pbt-out/bug_reports/encode_neon_mvni_h_lsl8.md
- pbt-out/bug_reports/encode_neon_mvni_h_lsl8.html
- pbt-out/bug_reports/encode_neon_mvni_extra_operand.md
- pbt-out/bug_reports/encode_neon_mvni_extra_operand.html
- pbt-out/bug_reports/encode_neon_mvni_imm_oor.md
- pbt-out/bug_reports/encode_neon_mvni_imm_oor.html
- pbt-out/bug_reports/encode_neon_mvni_illegal_h_shift.md
- pbt-out/bug_reports/encode_neon_mvni_illegal_h_shift.html
- pbt-out/bug_reports/encode_neon_mvni_lsr.md
- pbt-out/bug_reports/encode_neon_mvni_lsr.html

## Coverage Report

# PBT Coverage Status

> Last updated: 2026-10-05 11:14 (campaign: coverage)
> Files: 10/10 scanned (100%) | Functions: 111/289 total | PBT candidates: 111 | Tested: 111 (100%) | 0 pass, 111 fail

## Summary

| Metric | Value |
|--------|-------|
| Total source files | 10 |
| Files scanned | 10 / 10 (100%) |
| Total functions (all files) | 289 |
| PBT candidates (from FUNCTION_INDEX) | 111 |
| **Tested (of PBT candidates)** | **111 / 111 (100%)** |
| &nbsp;&nbsp;↳ Pass / Fail / Other | 0 / 111 / 0 |
| **Overall (tested / all functions)** | **111 / 289 (38%)** |
| Untested | 0 |
| Skipped | 0 |

## Module Breakdown

| Module | Scanned | Tested | Skipped | Coverage |
|--------|---------|--------|---------|----------|
|  | 111 | 111 | 0 | 100% |

## Oracle Type Distribution

| Oracle Type | Total | Covered | Skipped | Coverage |
|-------------|-------|---------|---------|----------|
| unknown | 111 | 111 | 0 | 100% |

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
| neon.rs | 68 | 26 | 26 | 100% | covered |
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
