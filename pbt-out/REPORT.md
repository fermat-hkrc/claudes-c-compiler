# PBT Campaign Report: encode_ldr_str_auto

## Summary

**Verdict:** 1 high: encode_ldr_str_auto maps Bt/Ht to 64-bit D-form (ldr b0,[x1] emits 0xfd400020 instead of 0x3d400020), so SIMD byte/halfword loads assemble as 8-byte transfers; 1 high: SP dest is accepted and encoded as D31; plus 2 medium (bare Vn silently becomes D-form; GNU `fp` alias is rejected).
**Date:** 2026-10-06
**Repository:** /home/toan/github/claudes-c-compiler
**Modules tested:** encode_ldr_str_auto
**Tests:** 11
**Result:** 6 passing, 5 failing properties, 4 bugs
**Change surface:** 1 changed function, 1 with a property, 0 error-handling changes (the documented non-Reg first-operand Err path has encode_ldr_str_auto_neg_first_operand)
**Coverage evidence:** file-level (symbol presence) — coverage_gaps reported no .gcda/.profraw (Rust cargo test, C++ reporter listed unrelated binaries and claimed NOT LINKED). Sweep was a manual arm audit of the 25-line body plus encode_ldr_str_auto_neg_v_reg / encode_ldr_str_auto_diff_fp_alias.
**Tier:** standard

## Modules Tested

| Module | Tests | Bugs | Oracles Used |
|--------|-------|------|-------------|
| encode_ldr_str_auto | 11 | 4 | differential, algebraic.invariant, algebraic.metamorphic, negative_error |

## Bugs Found

### B1: encode_ldr_str_auto encodes Bt/Ht as 64-bit D-form

**Formal:** ∀ is_load ∈ {false,true}, rt ∈ [0,31], rn ∈ [0,31], imm12 ∈ [0,4095], pref ∈ {b,h}. encode_ldr_str_auto([Reg(pref+rt), Mem{Xn|SP, imm12*(1<<shift)}], is_load) = Word(llvm-mc(`ldr|str PrefRt, [Xn|SP, #pimm]`)) where (b→size=00 shift=0), (h→size=01 shift=1)
**Contract evidence:** documented README.md:12 "It accepts the same textual assembly that GCC's gas would consume"; ARM SIMD&FP LDR/STR Bt size=00 / Ht size=01 V=1. load_store.rs:9 names S/D/Q but does not declare B/H invalid.
**Documentation conflict:** load_store.rs:8-9 lists W/X/S/D/Q only — it does not declare Bt/Ht invalid (not an input-domain restriction). parse_reg_num and is_fp_reg accept b/h. Classified as undocumented gap on an accepted input, not a documented limitation.
**Severity:** high
**Counterexample:** encode_ldr_str_auto([Reg("b0"), Mem{base:"x0", offset:0}], is_load=false) then compare to llvm-mc `str b0, [x0]`
**Expected / Actual:** 0x3d000000 / 0xfd000000
**Impact:** SIMD byte and halfword load/store instructions assemble as 64-bit FP D-form; wrong transfer size and scale.
**Root cause:** load_store.rs:26 unknown prefixes including `b`/`h` take `else { 0b11 }`, so Bt/Ht inherit D-form size=11 V=1.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/load_store.rs:26`
```rust
    } else {
        0b11 // default 64-bit
    };
```
**Suggested fix:** Map `h`→size=01 and `b`→size=00 before the default; keep `is_128bit` only for `q`.
```rust
    } else if reg_name.starts_with('h') {
        0b01
    } else if reg_name.starts_with('b') {
        0b00
    } else {
        0b11
    };
```
**Bug report:** bug_reports/encode_ldr_str_auto_byte_half_size.md
**Repro seed:** is_load=false, rt=0, rn=0, imm12=0, is_h=false (proptest cc 9180621a6e20b55ab1bd167fa93b0c060a965641616fcc60a9895cc3b3072241)
**Raw output:**
```text
Test failed: assertion failed: `(left == right)`
  left: `4244635648`,
 right: `1023410176`: SUT vs llvm-mc mismatch for str b0, [x0]
minimal failing input: is_load = false, rt = 0, rn = 0, imm12 = 0, is_h = false
```

### B2: encode_ldr_str_auto encodes bare Vn as D-form instead of rejecting it

**Formal:** ∀ is_load ∈ {false,true}, rt ∈ [0,31], rn ∈ [0,31]. encode_ldr_str_auto([Reg("v"+rt), Mem{Xn|SP, 0}], is_load) = Err
**Contract evidence:** documented README.md:12 gas contract; llvm-mc rejects `ldr v0, [x1]` as invalid operand. Operand::Reg documents v0-v31 so the name is in-domain for the parser.
**Documentation conflict:** (none) — the size-detect comment does not mention V; the else default is the producing statement, not an exclusion.
**Severity:** medium
**Counterexample:** encode_ldr_str_auto([Reg("v0"), Mem{base:"x0", offset:0}], is_load=false)
**Expected / Actual:** Err / Ok(Word(0xfd000000))
**Impact:** Bare V-form LDR/STR silently become 64-bit FP D-form with no diagnostic.
**Root cause:** load_store.rs:26 `v` falls into default size=11; is_fp_reg('v') sets V=1, producing D-form.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/load_store.rs:26`
```rust
    } else {
        0b11 // default 64-bit
    };
```
**Suggested fix:** Return Err for a `v` prefix (bare Vn is not a scalar LDR/STR Rt).
```rust
    } else if reg_name.starts_with('v') {
        return Err("ldr/str: bare Vn is not a valid scalar Rt".to_string());
    } else {
        0b11
    };
```
**Bug report:** bug_reports/encode_ldr_str_auto_bare_v.md
**Repro seed:** is_load=false, rt=0, rn=0
**Raw output:**
```text
Test failed: bare Vn must Err (llvm-mc: invalid operand); got Ok(Word(4244635648))
minimal failing input: is_load = false, rt = 0, rn = 0
```

### B3: encode_ldr_str_auto rejects the GNU fp alias of X29

**Formal:** ∀ is_load ∈ {false,true}, rn ∈ [0,30]. encode_ldr_str_auto([Reg("fp"), Mem{Xn, 0}], is_load) = Word(llvm-mc(`ldr|str fp, [Xn]`))
**Contract evidence:** documented README.md:12 "It accepts the same textual assembly that GCC's gas would consume"; llvm-mc `ldr fp, [x1]` = 0xf940003d = `ldr x29, [x1]`. lr is already special-cased in this function.
**Documentation conflict:** (none) — parse_reg_num lists lr/sp/xzr/wzr but not fp; that is an omission, not an exclusion of fp.
**Severity:** medium
**Counterexample:** encode_ldr_str_auto([Reg("fp"), Mem{base:"x0", offset:0}], is_load=false)
**Expected / Actual:** Word(llvm-mc `str fp, [x0]`) / Err("invalid register: fp")
**Impact:** GNU assembly using `fp` as an LDR/STR data register fails to assemble.
**Root cause:** parse_reg_num (encoder/mod.rs:294) has no `fp => 29` alias; get_reg then fails inside encode_ldr_str. encode_ldr_str_auto's default 64-bit size would have been correct.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/mod.rs:294`
```rust
        "sp" | "wsp" => Some(31),
        "xzr" | "wzr" => Some(31),
        "lr" => Some(30),
        _ => {
```
**Suggested fix:** Alias `fp` to 29 next to `lr`.
```rust
        "lr" => Some(30),
        "fp" => Some(29),
```
**Bug report:** bug_reports/encode_ldr_str_auto_fp_alias.md
**Repro seed:** is_load=false, rn=0
**Raw output:**
```text
Test failed: SUT rejected valid str fp, [x0]: invalid register: fp.
minimal failing input: is_load = false, rn = 0
```

### B4: encode_ldr_str_auto accepts SP as LDR/STR Rt and encodes it as D31

**Formal:** ∀ is_load ∈ {false,true}, rn ∈ [0,31]. encode_ldr_str_auto([Reg("sp"), Mem{Xn|SP, 0}], is_load) = Err
**Contract evidence:** documented README.md:12 gas contract; llvm-mc rejects `ldr sp, [x0]`; ARM Rt=31 is ZR not SP.
**Documentation conflict:** load_store.rs:17 names `sp` as 64-bit for size detection — that is a size map for the first operand, not a declaration that SP is a valid Rt. llvm-mc/gas reject SP dest.
**Severity:** high
**Counterexample:** encode_ldr_str_auto([Reg("sp"), Mem{base:"x0", offset:0}], is_load=false)
**Expected / Actual:** Err / Ok(Word(0xfd00001f)) (`str d31, [x0]`)
**Impact:** `ldr/str sp, [Xn]` assembles as a 64-bit FP store of D31 instead of being rejected.
**Root cause:** load_store.rs:17 maps `sp` to size=11 then encode_ldr_str does not reject SP as Rt; is_fp_reg("sp") is true (prefix s) so V=1.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/load_store.rs:17`
```rust
    } else if reg_name.starts_with('x') || reg_name == "sp" || reg_name == "xzr" || reg_name == "lr" {
        0b11 // 64-bit
```
**Suggested fix:** Reject SP/WSP as the data register before delegating.
```rust
    if reg_name == "sp" || reg_name == "wsp" {
        return Err("ldr/str: SP is not a valid Rt".to_string());
    }
```
**Bug report:** bug_reports/encode_ldr_str_auto_sp_dest.md
**Repro seed:** is_load=false, rn=0
**Raw output:**
```text
Test failed: SP dest must Err (llvm-mc: invalid operand); got Ok(Word(4244635679))
minimal failing input: is_load = false, rn = 0
```

## Design Caveats

(none)

## Test Files Created

| File | Tests |
|------|-------|
| src/backend/arm/assembler/encoder/encode_ldr_str_auto_pbt.rs | 11 properties + 2 passing KAT + 6 failing regression witnesses |

## Reproduction

Whole suite (serial, as run):
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_ldr_str_auto -- --test-threads=1
```

B1:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_ldr_str_auto_regression_byte_reg -- --test-threads=1
cargo test --lib encode_ldr_str_auto_diff_fp_bh -- --test-threads=1
```

B2:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_ldr_str_auto_regression_bare_v -- --test-threads=1
```

B3:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_ldr_str_auto_regression_fp_alias -- --test-threads=1
```

B4:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_ldr_str_auto_regression_sp_dest -- --test-threads=1
```

Build contract reused: `cargo test --lib encode_ldr_str_auto -- --test-threads=1` (target swapped from `encode_ldrsw_kat_llvm_mc_x0_x1`).

## Output Directories

- pbt-out/PLAN.md
- pbt-out/PROPERTIES.md
- pbt-out/REPORT.md
- pbt-out/REPORT.html (rendered from report.json)
- pbt-out/report.json
- pbt-out/COVERAGE.md
- pbt-out/COVERAGE_STATUS.md
- pbt-out/FUNCTION_INDEX.md
- pbt-out/INVARIANTS.md
- pbt-out/CHANGE_SURFACE.md
- pbt-out/run/encode_ldr_str_auto.log
- pbt-out/bug_reports/encode_ldr_str_auto_byte_half_size.md
- pbt-out/bug_reports/encode_ldr_str_auto_byte_half_size.html
- pbt-out/bug_reports/encode_ldr_str_auto_bare_v.md
- pbt-out/bug_reports/encode_ldr_str_auto_bare_v.html
- pbt-out/bug_reports/encode_ldr_str_auto_fp_alias.md
- pbt-out/bug_reports/encode_ldr_str_auto_fp_alias.html
- pbt-out/bug_reports/encode_ldr_str_auto_sp_dest.md
- pbt-out/bug_reports/encode_ldr_str_auto_sp_dest.html
- proptest-regressions/backend/arm/assembler/encoder/encode_ldr_str_auto_pbt.txt (framework shrunk witnesses)

## Coverage Report

# PBT Coverage Status

> Last updated: 2026-10-06 12:02 (campaign: coverage)
> Files: 11/11 scanned (100%) | Functions: 181/307 total | PBT candidates: 181 | Tested: 181 (100%) | 1 pass, 181 fail

## Summary

| Metric | Value |
|--------|-------|
| Total source files | 11 |
| Files scanned | 11 / 11 (100%) |
| Total functions (all files) | 307 |
| PBT candidates (from FUNCTION_INDEX) | 181 |
| **Tested (of PBT candidates)** | **181 / 181 (100%)** |
| &nbsp;&nbsp;↳ Pass / Fail / Other | 1 / 181 / -1 |
| **Overall (tested / all functions)** | **181 / 307 (59%)** |
| Untested | 0 |
| Skipped | 0 |

## Module Breakdown

| Module | Scanned | Tested | Skipped | Coverage |
|--------|---------|--------|---------|----------|
|  | 181 | 181 | 0 | 100% |

## Oracle Type Distribution

| Oracle Type | Total | Covered | Skipped | Coverage |
|-------------|-------|---------|---------|----------|
| unknown | 181 | 181 | 0 | 100% |

## File Coverage

| Source File | Funcs | Candidates | Tested | Coverage | Status |
|-------------|-------|------------|--------|----------|--------|
| cast.rs | 6 | 1 | 1 | 100% | covered |
| compare_branch.rs | 21 | 21 | 21 | 100% | covered |
| constants.rs | 34 | 1 | 1 | 100% | covered |
| data_processing.rs | 36 | 31 | 31 | 100% | covered |
| fp_scalar.rs | 13 | 11 | 12 | 109% | covered |
| gp_integer.rs | 29 | 1 | 1 | 100% | covered |
| load_store.rs | 20 | 19 | 19 | 100% | covered |
| neon.rs | 68 | 63 | 63 | 100% | covered |
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
| encode_fnmadd_fnmsub | fp_scalar.rs |
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
| encode_neon_scalar_two_misc | neon.rs |
| encode_neon_scalar_qshrn | neon.rs |
| encode_neon_two_misc_narrow | neon.rs |
| encode_dmb | system.rs |
| encode_dsb | system.rs |
| encode_mrs | system.rs |
| encode_msr | system.rs |
| encode_svc | system.rs |
| encode_hvc | system.rs |
| encode_brk | system.rs |
| encode_hint | system.rs |
| encode_bti | system.rs |
| encode_ic | system.rs |
| encode_dc | system.rs |
| encode_sys | system.rs |
| encode_at | system.rs |
| encode_tlbi | system.rs |
| encode_swp | load_store.rs |
| encode_ldop | load_store.rs |
| encode_stop | load_store.rs |
| encode_tst | compare_branch.rs |
| encode_tbz | compare_branch.rs |
| encode_crc32 | bitfield.rs |
| encode_smaddl | data_processing.rs |
| encode_mneg | data_processing.rs |
| encode_sxtb | data_processing.rs |
| encode_uxth | data_processing.rs |
| encode_uxtb | data_processing.rs |
| encode_ldr_str | load_store.rs |
| encode_ldp_stp | load_store.rs |
| encode_ldnp_stnp | load_store.rs |
| encode_adrp | load_store.rs |
| encode_mov | data_processing.rs |
| encode_cond_branch | compare_branch.rs |
| encode_ldr_str_auto | load_store.rs |
