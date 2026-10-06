# PBT Campaign Report: encode_adrp

## Summary

**Verdict:** 3 medium: encode_adrp silently encodes W/SP/FP destinations as Xd/XZR, ignores extra operands, and rejects valid `:got:sym+addend` (ModifierOffset) that gas and llvm-mc assemble as R_AARCH64_ADR_GOT_PAGE.
**Date:** 2026-10-06
**Repository:** /home/toan/github/claudes-c-compiler
**Modules tested:** encode_adrp
**Tests:** 9
**Result:** 6 passing, 3 bugs
**Change surface:** 1 changed function (encode_adrp), 1 with a property, 0 error-handling changes
**Coverage evidence:** file-level (symbol presence) — coverage_gaps reported no .gcda/.profraw (C++ reporter listed unrelated binaries and claimed NOT LINKED). Manual arm audit of encode_adrp plus encode_adrp_diff_alt_spellings.

## Modules Tested

| Module | Tests | Bugs | Oracles Used |
|--------|-------|------|-------------|
| encode_adrp | 9 | 3 | differential, algebraic.invariant, algebraic.metamorphic, negative_error |

## Bugs Found

### B1: encode_adrp accepts W, SP, and FP/SIMD destinations

**Formal:** ∀ dest ∈ W-regs ∪ {sp,wsp} ∪ FP/SIMD, ∀ op1 ∈ valid-symbol-ops. encode_adrp([Reg(dest), op1]) = Err
**Contract evidence:** inferred (ARM ADRP <Xd>; README.md:12 gas agreement; llvm-mc/gas reject `adrp w0, foo` / `adrp sp, foo` / `adrp d0, foo`)
**Documentation conflict:** (none)
**Severity:** medium
**Counterexample:** encode_adrp([Reg("w0"), Symbol("s0")])
**Expected / Actual:** Err / Ok(WordWithReloc { word: 0x90000000, AdrpPage21, "s0", 0 })
**Impact:** Invalid GNU assembly is silently rewritten as `adrp x0` / `adrp xzr`, so a W or FP dest never fails the assembler
**Root cause:** load_store.rs:654 discards get_reg's is_64 flag and does not reject SP (num=31, is_64=true) or FP prefixes parse_reg_num accepts
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/load_store.rs:654`
```rust
    let (rd, _) = get_reg(operands, 0)?;
```
**Suggested fix:** Require a 64-bit X register that is not SP
```rust
    let (rd, is_64) = get_reg(operands, 0)?;
    let name = match operands.get(0) {
        Some(Operand::Reg(n)) => n.as_str(),
        _ => return Err("adrp needs Xd destination".into()),
    };
    let lower = name.to_ascii_lowercase();
    if !is_64 || lower == "sp" || matches!(lower.chars().next(), Some('d' | 's' | 'q' | 'v' | 'h' | 'b')) {
        return Err(format!("adrp destination must be Xd, got {name}"));
    }
```
**Bug report:** bug_reports/encode_adrp_w_sp_fp.md
**Repro seed:** kind = 0, n = 0, suffix = 0
**Raw output:** Test failed: ADRP takes Xd only; w0 must Err (gas/llvm-mc reject it)

### B2: encode_adrp ignores extra operands

**Formal:** ∀ rd ∈ {0..30}, ∀ kind ∈ {empty, missing-op1, extra, lo12, got_lo12, Imm, Mem}. encode_adrp(ops(kind)) = Err
**Contract evidence:** inferred (README.md:12 gas contract; gas "unexpected characters following instruction"; llvm-mc "invalid operand")
**Documentation conflict:** (none)
**Severity:** medium
**Counterexample:** encode_adrp([Reg("x0"), Symbol("foo"), Reg("x0")])
**Expected / Actual:** Err / Ok(WordWithReloc { word: 0x90000000, AdrpPage21, "foo", 0 })
**Impact:** `adrp x0, foo, x1` is assembled as `adrp x0, foo` instead of being rejected
**Root cause:** load_store.rs:653–690 never checks operands.len(); only operands[0] and [1] are read
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/load_store.rs:653`
```rust
pub(crate) fn encode_adrp(operands: &[Operand]) -> Result<EncodeResult, String> {
```
**Suggested fix:** Reject anything other than exactly two operands
```rust
    if operands.len() != 2 {
        return Err(format!("adrp takes 2 operands, got {}", operands.len()));
    }
```
**Bug report:** bug_reports/encode_adrp_extra_operand.md
**Repro seed:** kind = 2, rd = 0, extra = Reg("x0")
**Raw output:** Test failed: invalid ADRP arity/kind=2 must Err, got Ok(WordWithReloc { word: 2415919104, ... })

### B3: encode_adrp rejects :got:symbol+addend

**Formal:** ∀ rd ∈ {0..31}, ∀ suffix, ∀ addend. encode_adrp([Xrd, Modifier{got,s}]) = WordWithReloc{0x90000000|rd, AdrGotPage21, s, 0} ∧ encode_adrp([Xrd, ModifierOffset{got,s,addend}]) = WordWithReloc{…, AdrGotPage21, s, addend}
**Contract evidence:** inferred (GNU as / llvm-mc accept `adrp x0, :got:foo+8` as R_AARCH64_ADR_GOT_PAGE with addend 8; README.md:248 `:got:variable`)
**Documentation conflict:** load_store.rs:659 "adrp x0, :got:symbol" documents the zero-addend Modifier form, not ModifierOffset — it does not declare `:got:sym+addend` invalid
**Severity:** medium
**Counterexample:** encode_adrp([Reg("x0"), ModifierOffset { kind: "got", symbol: "g0", offset: 0 }])
**Expected / Actual:** Ok(WordWithReloc AdrGotPage21 addend 0) / Err("adrp needs symbol operand, got Some(ModifierOffset { kind: \"got\", symbol: \"g0\", offset: 0 })")
**Impact:** Parser output for `:got:foo+8` cannot be encoded; GOT-relative ADRP with a non-zero addend fails to assemble
**Root cause:** load_store.rs:657 matches only Operand::Modifier { kind == "got" } and falls through to Err for ModifierOffset
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/load_store.rs:657`
```rust
        Some(Operand::Modifier { kind, symbol }) if kind == "got" => {
```
**Suggested fix:** Accept ModifierOffset with kind "got" and pass the offset through as the reloc addend
```rust
        Some(Operand::ModifierOffset { kind, symbol, offset }) if kind == "got" => {
            (symbol.clone(), *offset, RelocType::AdrGotPage21)
        }
```
**Bug report:** bug_reports/encode_adrp_got_modifier_offset.md
**Repro seed:** rd = 0, suffix = 0, addend = 0
**Raw output:** Test failed: ModifierOffset-got expected WordWithReloc AdrGotPage21, got Err("adrp needs symbol operand, ...")

## Design Caveats

(none)

## Test Files Created

| File | Tests |
|------|-------|
| src/backend/arm/assembler/encoder/encode_adrp_pbt.rs | 9 properties + 3 KAT + 5 regression witnesses |

## Reproduction

```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_adrp -- --test-threads=1
```

B1:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_adrp_regression_w_reg -- --test-threads=1
```

B2:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_adrp_regression_extra_operand -- --test-threads=1
```

B3:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_adrp_regression_got_modifier_offset -- --test-threads=1
```

## Output Directories

pbt-out/REPORT.md, pbt-out/REPORT.html, pbt-out/PROPERTIES.md, pbt-out/PLAN.md, pbt-out/COVERAGE.md, pbt-out/COVERAGE_STATUS.md, pbt-out/report.json, pbt-out/INVARIANTS.md, pbt-out/FUNCTION_INDEX.md, pbt-out/bug_reports/encode_adrp_w_sp_fp.md, pbt-out/bug_reports/encode_adrp_w_sp_fp.html, pbt-out/bug_reports/encode_adrp_extra_operand.md, pbt-out/bug_reports/encode_adrp_extra_operand.html, pbt-out/bug_reports/encode_adrp_got_modifier_offset.md, pbt-out/bug_reports/encode_adrp_got_modifier_offset.html

## Coverage Report

# PBT Coverage Status

> Last updated: 2026-10-06 10:34 (campaign: coverage)
> Files: 11/11 scanned (100%) | Functions: 178/307 total | PBT candidates: 178 | Tested: 178 (100%) | 1 pass, 178 fail

## Summary

| Metric | Value |
|--------|-------|
| Total source files | 11 |
| Files scanned | 11 / 11 (100%) |
| Total functions (all files) | 307 |
| PBT candidates (from FUNCTION_INDEX) | 178 |
| **Tested (of PBT candidates)** | **178 / 178 (100%)** |
| &nbsp;&nbsp;↳ Pass / Fail / Other | 1 / 178 / -1 |
| **Overall (tested / all functions)** | **178 / 307 (58%)** |
| Untested | 0 |
| Skipped | 0 |

## Module Breakdown

| Module | Scanned | Tested | Skipped | Coverage |
|--------|---------|--------|---------|----------|
|  | 178 | 178 | 0 | 100% |

## Oracle Type Distribution

| Oracle Type | Total | Covered | Skipped | Coverage |
|-------------|-------|---------|---------|----------|
| unknown | 178 | 178 | 0 | 100% |

## File Coverage

| Source File | Funcs | Candidates | Tested | Coverage | Status |
|-------------|-------|------------|--------|----------|--------|
| cast.rs | 6 | 1 | 1 | 100% | covered |
| compare_branch.rs | 21 | 20 | 20 | 100% | covered |
| constants.rs | 34 | 1 | 1 | 100% | covered |
| data_processing.rs | 36 | 30 | 30 | 100% | covered |
| fp_scalar.rs | 13 | 11 | 12 | 109% | covered |
| gp_integer.rs | 29 | 1 | 1 | 100% | covered |
| load_store.rs | 20 | 18 | 18 | 100% | covered |
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
