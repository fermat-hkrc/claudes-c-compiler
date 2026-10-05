# PBT Campaign Report: encode_neon_two_misc_narrow

## Summary

**Verdict:** 1 high, 2 medium: encode_neon_two_misc_narrow discards the dest arrangement so `xtn v0.8h, v0.4s` encodes as XTN Vd.4H, silently accepts a third operand, and encodes a bare/GPR dest as Vd.8B — gas/llvm-mc reject all three.
**Date:** 2026-10-05
**Repository:** /home/toan/github/claudes-c-compiler
**Modules tested:** encode_neon_two_misc_narrow
**Tests:** 9
**Result:** 6 passing, 3 bugs
**Change surface:** 1 changed function, 1 with a property, 0 error-handling changes with a failure-path property
**Coverage evidence:** file-level (symbol presence) — coverage_gaps reported no .gcda/.profraw and listed unrelated C++ binaries as NOT LINKED; the SUT ran inside `cargo test --lib encode_neon_two_misc_narrow` (KAT + 1000-case properties).
**Tier:** standard

## Modules Tested

| Module | Tests | Bugs | Oracles Used |
|--------|-------|------|-------------|
| encode_neon_two_misc_narrow | 9 | 3 | differential, algebraic.metamorphic, algebraic.invariant, negative_error |

## Bugs Found

### B1: encode_neon_two_misc_narrow silently encodes a third operand

**Formal:** ∀ rd,rn,extra ∈ {0..31}, ∀ valid (tb,ta,is_high,u,opc,mnem). llvm-mc rejects mnem||suffix || " v"||rd||"."||tb||", v"||rn||"."||ta||", v"||extra||"."||tb ∧ encode_neon_two_misc_narrow([Vd,Vn,Vextra], u, opc, is_high) is Err
**Contract evidence:** documented neon.rs:207 "NEON two-reg narrow requires 2 operands"
**Documentation conflict:** neon.rs:207 "NEON two-reg narrow requires 2 operands" states two operands are required; the body only rejects `len < 2`, so arity 3+ is accepted. The comment states the behavior IS handled (exactly 2), which the code violates.
**Severity:** medium
**Counterexample:** encode_neon_two_misc_narrow([v0.8b, v0.8h, v0.8b], u_bit=0, opcode=0b10010, is_high=false)
**Expected / Actual:** Err / Ok(Word(0x0e212800))
**Impact:** Invalid assembly such as `xtn v0.8b, v0.8h, v0.8b` is assembled into an XTN word instead of an error, so the GNU-style assembler emits machine code that gas/llvm-mc refuse.
**Root cause:** neon.rs:207 checks only `operands.len() < 2`, so arity 3+ is treated as a 2-operand encode using the first two operands
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:207`
```rust
    if operands.len() < 2 {
        return Err("NEON two-reg narrow requires 2 operands".to_string());
    }
```
**Suggested fix:** Reject any arity other than 2
```rust
    if operands.len() != 2 {
        return Err("NEON two-reg narrow requires 2 operands".to_string());
    }
```
**Bug report:** bug_reports/encode_neon_two_misc_narrow_extra_operand.md
**Repro seed:** cc 4e956affd40e557380117bed6fa5827bbe3fc738be9282a288f7b19b8e680b41
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_neon_two_misc_narrow_pbt::encode_neon_two_misc_narrow_neg_extra_operand' (2513865) panicked at src/backend/arm/assembler/encoder/encode_neon_two_misc_narrow_pbt.rs:423:1:
Test failed: 3 operands must Err (llvm-mc rejects xtn v0.8b, v0.8h, v0.8b) at src/backend/arm/assembler/encoder/encode_neon_two_misc_narrow_pbt.rs:444.
minimal failing input: rd = 0, rn = 0, extra = 0, pair = (
    "8b",
    "8h",
    false,
), fam = (
    "xtn",
    0,
    18,
)
	successes: 0
	local rejects: 0
	global rejects: 0
```

### B2: encode_neon_two_misc_narrow accepts dest 8H with source 4S on XTN (Q=0)

**Formal:** ∀ rd,rn ∈ {0..31}, ∀ tb,ta ∈ arrangements, ∀ is_high ∈ {false,true}, ∀ family. ¬valid_pair(tb,ta,is_high) ⇒ llvm-mc rejects the asm ∧ encode_neon_two_misc_narrow([Vd.tb, Vn.ta], u, opc, is_high) is Err
**Contract evidence:** inferred (ARM XTN{2} Vd.Tb, Vn.Ta with Tb matching Ta and Q; neon.rs:202 names XTN/SQXTN/UQXTN; encode() at encoder/mod.rs:941-946 passes operands through)
**Documentation conflict:** (none) — neon.rs:215 documents unsupported *source* arrangements only; dest arrangement is bound as `_arr_d` with no comment declaring dest unchecked
**Severity:** high
**Counterexample:** encode_neon_two_misc_narrow([v0.8h, v0.4s], u_bit=0, opcode=0b10010, is_high=false)
**Expected / Actual:** Err / Ok(Word(0x0e612800))
**Impact:** A mistyped dest arrangement is silently rewritten: `xtn v0.8h, v0.4s` encodes as XTN Vd.4H, Vn.4S instead of an assemble error.
**Root cause:** neon.rs:210 binds dest arrangement as `_arr_d` and never checks it against Ta or is_high; size is taken only from the source arrangement
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:210`
```rust
    let (rd, _arr_d) = get_neon_reg(operands, 0)?;
    let (rn, arr_n) = get_neon_reg(operands, 1)?;
```
**Suggested fix:** Require the ARM-mandated dest arrangement for (Ta, is_high)
```rust
    let (rd, arr_d) = get_neon_reg(operands, 0)?;
    let (rn, arr_n) = get_neon_reg(operands, 1)?;
    let expected_tb = match (arr_n.as_str(), is_high) {
        ("8h", false) => "8b",
        ("8h", true) => "16b",
        ("4s", false) => "4h",
        ("4s", true) => "8h",
        ("2d", false) => "2s",
        ("2d", true) => "4s",
        _ => return Err(format!("unsupported source arrangement for narrow: {}", arr_n)),
    };
    if arr_d != expected_tb {
        return Err(format!("narrow: source {} requires dest {}", arr_n, expected_tb));
    }
```
**Bug report:** bug_reports/encode_neon_two_misc_narrow_mismatched_tb_ta.md
**Repro seed:** cc e2fc582b4808de07089af20711cca817c2706965e85a706f2572dfe348065989
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_neon_two_misc_narrow_pbt::encode_neon_two_misc_narrow_neg_mismatched_tb_ta' (2513885) panicked at src/backend/arm/assembler/encoder/encode_neon_two_misc_narrow_pbt.rs:452:1:
Test failed: invalid/mismatched Tb/Ta must Err (ARM XTN Ta in {8H,4S,2D} with matching Tb; llvm-mc rejects xtn v0.8h, v0.4s) at src/backend/arm/assembler/encoder/encode_neon_two_misc_narrow_pbt.rs:473.
minimal failing input: rd = 0, rn = 0, tb = "8h", ta = "4s", is_high = false, fam = (
    "xtn",
    0,
    18,
)
	successes: 3
	local rejects: 0
	global rejects: 1
		1 times at src/backend/arm/assembler/encoder/encode_neon_two_misc_narrow_pbt.rs:464:9: !is_valid_pair(tb, ta, is_high)
```

### B3: encode_neon_two_misc_narrow encodes a bare V dest as XTN Vd.8B

**Formal:** ∀ rd,rn ∈ {0..31}, ∀ kind ∈ {gpr-dest, bare-src, bare-dest, x-arrangement-dest, gpr-src}. llvm-mc rejects the asm ∧ encode_neon_two_misc_narrow(ops(kind), 0, 0b10010, false) is Err
**Contract evidence:** inferred (ARM XTN dest is Vd.Tb; neon.rs:202 names XTN; llvm-mc/gas reject `xtn v0, v0.8h` and `xtn x0, v0.8h`; encode() passes operands through)
**Documentation conflict:** (none)
**Severity:** medium
**Counterexample:** encode_neon_two_misc_narrow([Reg("v0"), v0.8h], u_bit=0, opcode=0b10010, is_high=false)
**Expected / Actual:** Err / Ok(Word(0x0e212800))
**Impact:** A dest without arrangement (or with a GPR prefix) is encoded as the corresponding V register, so a mistyped `xtn v0, v0.8h` silently becomes `xtn v0.8b, v0.8h`.
**Root cause:** neon.rs:210 calls get_neon_reg which accepts Operand::Reg, then discards dest arrangement; encode_neon_two_misc_narrow never requires a V-prefixed arrangement dest
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:210`
```rust
    let (rd, _arr_d) = get_neon_reg(operands, 0)?;
```
**Suggested fix:** Reject a destination that is not a V-prefixed RegArrangement
```rust
    let (rd, arr_d) = get_neon_reg(operands, 0)?;
    match &operands[0] {
        Operand::RegArrangement { reg, .. } if reg.to_ascii_lowercase().starts_with('v') => {}
        _ => return Err("narrow: destination must be a V register with arrangement".to_string()),
    }
    let _ = arr_d;
```
**Bug report:** bug_reports/encode_neon_two_misc_narrow_bare_dest.md
**Repro seed:** cc ea0e467b95a3a37895fdc601c1dfe696818f53261cde18f44bbf4347db492367
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_neon_two_misc_narrow_pbt::encode_neon_two_misc_narrow_neg_gpr_or_bare' (2513879) panicked at src/backend/arm/assembler/encoder/encode_neon_two_misc_narrow_pbt.rs:481:1:
Test failed: GPR/bare/non-arrangement kind=2 must Err (llvm-mc rejects xtn v0, v0.8h) at src/backend/arm/assembler/encoder/encode_neon_two_misc_narrow_pbt.rs:536.
minimal failing input: rd = 0, rn = 0, kind = 2, fp_prefix = "x"
	successes: 0
	local rejects: 0
	global rejects: 0
```

## Design Caveats

(none)

## Test Files Created

| File | Tests |
|------|-------|
| src/backend/arm/assembler/encoder/encode_neon_two_misc_narrow_pbt.rs | 9 properties + 2 KAT + 5 regression witnesses |

## Reproduction

Whole suite:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_neon_two_misc_narrow -- --test-threads=1
```

B1 extra operand:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_neon_two_misc_narrow_neg_extra_operand -- --test-threads=1
cargo test --lib test_encode_neon_two_misc_narrow_regression_extra_operand -- --test-threads=1
```

B2 mismatched Tb/Ta:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_neon_two_misc_narrow_neg_mismatched_tb_ta -- --test-threads=1
cargo test --lib test_encode_neon_two_misc_narrow_regression_mismatched_tb_ta -- --test-threads=1
```

B3 bare dest:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_neon_two_misc_narrow_neg_gpr_or_bare -- --test-threads=1
cargo test --lib test_encode_neon_two_misc_narrow_regression_bare_dest -- --test-threads=1
```

## Output Directories

- pbt-out/REPORT.md
- pbt-out/REPORT.html
- pbt-out/PROPERTIES.md
- pbt-out/PLAN.md
- pbt-out/COVERAGE.md
- pbt-out/COVERAGE_STATUS.md
- pbt-out/FUNCTION_INDEX.md
- pbt-out/INVARIANTS.md
- pbt-out/report.json
- pbt-out/bug_reports/encode_neon_two_misc_narrow_extra_operand.md
- pbt-out/bug_reports/encode_neon_two_misc_narrow_extra_operand.html
- pbt-out/bug_reports/encode_neon_two_misc_narrow_mismatched_tb_ta.md
- pbt-out/bug_reports/encode_neon_two_misc_narrow_mismatched_tb_ta.html
- pbt-out/bug_reports/encode_neon_two_misc_narrow_bare_dest.md
- pbt-out/bug_reports/encode_neon_two_misc_narrow_bare_dest.html
- pbt-out/run/encode_neon_two_misc_narrow.log

## Coverage Report

# PBT Coverage Status

> Last updated: 2026-10-05 23:20 (campaign: coverage)
> Files: 10/10 scanned (100%) | Functions: 148/289 total | PBT candidates: 148 | Tested: 148 (100%) | 0 pass, 148 fail

## Summary

| Metric | Value |
|--------|-------|
| Total source files | 10 |
| Files scanned | 10 / 10 (100%) |
| Total functions (all files) | 289 |
| PBT candidates (from FUNCTION_INDEX) | 148 |
| **Tested (of PBT candidates)** | **148 / 148 (100%)** |
| &nbsp;&nbsp;↳ Pass / Fail / Other | 0 / 148 / 0 |
| **Overall (tested / all functions)** | **148 / 289 (51%)** |
| Untested | 0 |
| Skipped | 0 |

## Module Breakdown

| Module | Scanned | Tested | Skipped | Coverage |
|--------|---------|--------|---------|----------|
|  | 148 | 148 | 0 | 100% |

## Oracle Type Distribution

| Oracle Type | Total | Covered | Skipped | Coverage |
|-------------|-------|---------|---------|----------|
| unknown | 148 | 148 | 0 | 100% |

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
