# PBT Campaign Report: encode_neon_three_diff

## Summary

**Verdict:** 1 high: encode_neon_three_diff encodes WIDE SADDW/UADDW/SSUBW/USUBW with size/Q taken from wide Vn instead of narrow Vm, so `saddw v0.8h, v0.8h, v0.8b` becomes 0x4e601000 instead of llvm-mc's 0x0e201000; plus 4 medium validation holes (extra operand, dest Ta, GPR Rm, Rm Tb).
**Date:** 2026-10-05
**Repository:** /home/toan/github/claudes-c-compiler
**Modules tested:** encode_neon_three_diff
**Tests:** 10
**Result:** 5 passing, 5 bugs
**Change surface:** 1 changed function, 1 with a property, 0 error-handling changes
**Coverage evidence:** file-level (symbol presence) — coverage_gaps returned no .gcda/.profraw (Rust cargo test is not the C++ reporter) and listed unrelated C++ binaries as NOT LINKED; the cargo lib test binary did execute encode_neon_three_diff (KAT + 1000-case properties). Sweep round 1/1: added unsupported-src (passing) and Rm Tb mismatch (failing bug).
**Tier:** standard

## Modules Tested

| Module | Tests | Bugs | Oracles Used |
|--------|-------|------|-------------|
| encode_neon_three_diff | 10 | 5 | differential, algebraic.invariant, algebraic.metamorphic, negative_error |

## Bugs Found

### B1: WIDE size/Q taken from Vn instead of narrow Vm

**Formal:** ∀ rd,rn,rm ∈ {0..31}, (Ta,Tb,is_high) ∈ WIDE_PAIRS, (U,opc,mnem) ∈ WIDE_TABLE. encode_neon_three_diff([Vd.Ta, Vn.Ta, Vm.Tb], U, opc, is_high) = llvm-mc("mnem{2} Vd.Ta, Vn.Ta, Vm.Tb")
**Contract evidence:** documented neon.rs:97 "Size is determined from the source (narrow) arrangement"
**Documentation conflict:** neon.rs:97 "Size is determined from the source (narrow) arrangement" states size comes from the narrow source; the code matches Vn, which is wide for SADDW. The comment is the contract the code violates. (not independently verified)
**Severity:** high
**Counterexample:** encode_neon_three_diff([v0.8h, v0.8h, v0.8b], u_bit=0, opcode=0b0001, is_high=false)
**Expected / Actual:** 0x0e201000 / 0x4e601000
**Impact:** Valid WIDE three-different instructions assemble to the wrong 32-bit word; `saddw v0.2d, …` is rejected because 2d is not in the Vn match.
**Root cause:** neon.rs:98 matches arr_n (Vn). For WIDE, Vn is 8h/4s/2d, so Q/size are those of a LONG *2 form rather than of narrow Vm.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:98`
```rust
    let (q, size) = match arr_n.as_str() {
```
**Suggested fix:** For WIDE opcodes 0001/0011 take size/Q from Vm; keep Vn for LONG.
```rust
    let arr_narrow = if opcode == 0b0001 || opcode == 0b0011 { &arr_m } else { &arr_n };
    let (q, size) = match arr_narrow.as_str() {
```
**Bug report:** bug_reports/encode_neon_three_diff_wide_vn_size.md
**Repro seed:** cc b6cdc5885ea4f946c082c808606b2f96b160a5285dd4918b6f30ec1201dc2afd
**Raw output:** Test failed: assertion failed: `(left == right)` left: `1314918400`, right: `236982272`: mismatch for saddw v0.8h, v0.8h, v0.8b

### B2: Extra operand ignored

**Formal:** ∀ rd,rn,rm,extra ∈ {0..31}, Tb ∈ {8b,16b,4h,8h,2s,4s}, (U,opc,mnem) ∈ LONG_TABLE. llvm-mc(mnem Vd.Ta, Vn.Tb, Vm.Tb, Vextra.Tb) is Err ∧ encode_neon_three_diff([Vd.Ta, Vn.Tb, Vm.Tb, Vextra.Tb], U, opc, is_high(Tb)) is Err
**Contract evidence:** documented neon.rs:91 "NEON three-different requires 3 operands"
**Documentation conflict:** neon.rs:91 "NEON three-different requires 3 operands" states the arity; the check is `len() < 3`, so four operands are accepted. The comment is the contract the code violates. (not independently verified)
**Severity:** medium
**Counterexample:** encode_neon_three_diff([v0.8h, v0.8b, v0.8b, v0.8b], u_bit=0, opcode=0, is_high=false)
**Expected / Actual:** Err / Ok(Word)
**Impact:** Trailing garbage is assembled as a valid 3-operand SADDL; GNU gas rejects it.
**Root cause:** neon.rs:90 checks `operands.len() < 3` only.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:90`
```rust
    if operands.len() < 3 {
```
**Suggested fix:** Reject any arity other than 3.
```rust
    if operands.len() != 3 {
```
**Bug report:** bug_reports/encode_neon_three_diff_extra_operand.md
**Repro seed:** (deterministic; rd=rn=rm=extra=0, tb=8b, saddl)
**Raw output:** Test failed: 4 operands must Err (llvm-mc rejects saddl v0.8h, v0.8b, v0.8b, v0.8b)

### B3: Destination arrangement discarded

**Formal:** ∀ rd,rn,rm ∈ {0..31}, Tb ∈ {8b,16b,4h,8h,2s,4s}, Td ∈ ARR, Td ≠ widen(Tb), (U,opc,mnem) ∈ LONG_TABLE. llvm-mc(mnem Vd.Td, Vn.Tb, Vm.Tb) is Err ∧ encode_neon_three_diff([Vd.Td, Vn.Tb, Vm.Tb], U, opc, is_high(Tb)) is Err
**Contract evidence:** documented neon.rs:83 "These instructions have wider destination than source operands."
**Documentation conflict:** neon.rs:83 "These instructions have wider destination than source operands." states dest is wider; dest arrangement is bound to `_arr_d` and unused. The comment is the contract the code violates. (not independently verified)
**Severity:** medium
**Counterexample:** encode_neon_three_diff([v0.8b, v0.8b, v0.8b], u_bit=0, opcode=0, is_high=false)
**Expected / Actual:** Err / Ok(Word)
**Impact:** `saddl v0.8b, v0.8b, v0.8b` encodes as if dest were 8h.
**Root cause:** neon.rs:93 discards dest arrangement.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:93`
```rust
    let (rd, _arr_d) = get_neon_reg(operands, 0)?;
```
**Suggested fix:** Require dest Ta to be the widening of source Tb.
```rust
    let (rd, arr_d) = get_neon_reg(operands, 0)?;
    if arr_d != expected_ta { return Err(...); }
```
**Bug report:** bug_reports/encode_neon_three_diff_dest_ta_mismatch.md
**Repro seed:** (deterministic; rd=rn=rm=0, tb=8b, td=8b, saddl)
**Raw output:** Test failed: dest Ta must be widen(Tb); llvm-mc rejects saddl v0.8b, v0.8b, v0.8b

### B4: GPR accepted as Vm

**Formal:** ∀ rd,rn,rm ∈ {0..31}, kind ∈ {gpr_dest, bare_dest, gpr_arr_dest, gpr_src}, (U,opc,mnem) ∈ LONG_TABLE. llvm-mc(asm(kind)) is Err ∧ encode_neon_three_diff(ops(kind), U, opc, false) is Err
**Contract evidence:** inferred (README.md:12 GNU-style assembly; NEON three-diff operands are Vd.Ta / Vn.Tb / Vm.Tb)
**Documentation conflict:** (none)
**Severity:** medium
**Counterexample:** encode_neon_three_diff([v0.8h, v0.8b, Operand::Reg("x0")], u_bit=0, opcode=0, is_high=false)
**Expected / Actual:** Err / Ok(Word) with Rm=0
**Impact:** `saddl v0.8h, v0.8b, x0` encodes; dest GPR and bare V dest have the same hole.
**Root cause:** neon.rs:95 calls get_neon_reg, which accepts Operand::Reg; parse_reg_num maps x0 to 0.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:95`
```rust
    let (rm, _arr_m) = get_neon_reg(operands, 2)?;
```
**Suggested fix:** Require Operand::RegArrangement with a v prefix on every operand.
```rust
    match operands.get(idx) {
        Some(Operand::RegArrangement { reg, arrangement }) if reg.to_ascii_lowercase().starts_with('v') => { ... }
        other => Err(format!("expected NEON Vn.T at operand {}, got {:?}", idx, other)),
    }
```
**Bug report:** bug_reports/encode_neon_three_diff_gpr_or_bare.md
**Repro seed:** (deterministic; kind=2, fp_prefix=x, saddl v0.8h, v0.8b, x0)
**Raw output:** Test failed: GPR/bare/non-arrangement kind=2 must Err (llvm-mc rejects saddl v0.8h, v0.8b, x0)

### B5: Vm arrangement ignored

**Formal:** ∀ rd,rn,rm ∈ {0..31}, Tb ∈ {8b,16b,4h,8h,2s,4s}, Tm ≠ Tb, (U,opc,mnem) ∈ LONG_TABLE. llvm-mc(mnem Vd.Ta, Vn.Tb, Vm.Tm) is Err ∧ encode_neon_three_diff([Vd.Ta, Vn.Tb, Vm.Tm], U, opc, is_high(Tb)) is Err
**Contract evidence:** inferred (ARM LONG form requires matching Tb; llvm-mc rejects saddl v0.8h, v0.8b, v0.16b)
**Documentation conflict:** (none)
**Severity:** medium
**Counterexample:** encode_neon_three_diff([v0.8h, v0.8b, v0.16b], u_bit=0, opcode=0, is_high=false)
**Expected / Actual:** Err / Ok(Word)
**Impact:** Mismatched Vm arrangement is assembled as if it matched Vn.
**Root cause:** neon.rs:95 binds Rm arrangement to `_arr_m` and never compares it to Vn.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:95`
```rust
    let (rm, _arr_m) = get_neon_reg(operands, 2)?;
```
**Suggested fix:** Require Vm arrangement to equal Vn arrangement for LONG forms.
```rust
    let (rm, arr_m) = get_neon_reg(operands, 2)?;
    if arr_m != arr_n {
        return Err(format!("source arrangements must match: Vn.{} vs Vm.{}", arr_n, arr_m));
    }
```
**Bug report:** bug_reports/encode_neon_three_diff_rm_tb_mismatch.md
**Repro seed:** cc 95f2bdc5b5994d8ac8fdc350e8ab3c55b3b597eef5f9fd1d11969f806ce28098
**Raw output:** Test failed: Rm Tb must match Vn Tb; llvm-mc rejects saddl v0.8h, v0.8b, v0.16b

## Design Caveats

(none)

## Test Files Created

| File | Tests |
|------|-------|
| src/backend/arm/assembler/encoder/encode_neon_three_diff_pbt.rs | 10 properties + 1 KAT + 5 regression witnesses |
| src/backend/arm/assembler/encoder/mod.rs | one-line `mod encode_neon_three_diff_pbt` registration |

## Reproduction

```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_neon_three_diff_pbt -- --test-threads=1
```

WIDE bug:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_neon_three_diff_diff_llvm_mc_wide -- --test-threads=1
```

Extra operand:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_neon_three_diff_neg_extra_operand -- --test-threads=1
```

Dest Ta:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_neon_three_diff_neg_dest_ta_mismatch -- --test-threads=1
```

GPR Rm:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_neon_three_diff_neg_gpr_or_bare -- --test-threads=1
```

Rm Tb:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_neon_three_diff_neg_rm_tb_mismatch -- --test-threads=1
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
- pbt-out/bug_reports/encode_neon_three_diff_wide_vn_size.md
- pbt-out/bug_reports/encode_neon_three_diff_wide_vn_size.html
- pbt-out/bug_reports/encode_neon_three_diff_extra_operand.md
- pbt-out/bug_reports/encode_neon_three_diff_extra_operand.html
- pbt-out/bug_reports/encode_neon_three_diff_dest_ta_mismatch.md
- pbt-out/bug_reports/encode_neon_three_diff_dest_ta_mismatch.html
- pbt-out/bug_reports/encode_neon_three_diff_gpr_or_bare.md
- pbt-out/bug_reports/encode_neon_three_diff_gpr_or_bare.html
- pbt-out/bug_reports/encode_neon_three_diff_rm_tb_mismatch.md
- pbt-out/bug_reports/encode_neon_three_diff_rm_tb_mismatch.html
- pbt-out/run/encode_neon_three_diff_pbt.log
- proptest-regressions/backend/arm/assembler/encoder/encode_neon_three_diff_pbt.txt (proptest shrink seeds)

## Coverage Report

# PBT Coverage Status

> Last updated: 2026-10-05 18:33 (campaign: coverage)
> Files: 10/10 scanned (100%) | Functions: 134/289 total | PBT candidates: 134 | Tested: 134 (100%) | 0 pass, 134 fail

## Summary

| Metric | Value |
|--------|-------|
| Total source files | 10 |
| Files scanned | 10 / 10 (100%) |
| Total functions (all files) | 289 |
| PBT candidates (from FUNCTION_INDEX) | 134 |
| **Tested (of PBT candidates)** | **134 / 134 (100%)** |
| &nbsp;&nbsp;↳ Pass / Fail / Other | 0 / 134 / 0 |
| **Overall (tested / all functions)** | **134 / 289 (46%)** |
| Untested | 0 |
| Skipped | 0 |

## Module Breakdown

| Module | Scanned | Tested | Skipped | Coverage |
|--------|---------|--------|---------|----------|
|  | 134 | 134 | 0 | 100% |

## Oracle Type Distribution

| Oracle Type | Total | Covered | Skipped | Coverage |
|-------------|-------|---------|---------|----------|
| unknown | 134 | 134 | 0 | 100% |

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
| neon.rs | 68 | 49 | 49 | 100% | covered |
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
