# PBT Campaign Report: encode_neon_elem_long

## Summary

**Verdict:** 1 high: encode_neon_elem_long silently encodes `smull … v16.h[0]` as `v0.h[0]` (Rm truncated); plus 4 medium: extra operand ignored, dest arrangement ignored, GPR dest accepted, lane elem_size ignored.
**Date:** 2026-10-05
**Repository:** /home/toan/github/claudes-c-compiler
**Modules tested:** encode_neon_elem_long
**Tests:** 12
**Result:** 7 passing, 5 bugs
**Change surface:** 1 changed function, 1 with a property, 0 error-handling changes with a failure-path property
**Coverage evidence:** file-level (symbol presence) — coverage_gaps reported no .gcda/.profraw and claimed NOT LINKED against unrelated C++ binaries; cargo test --lib linked and executed encode_neon_elem_long (4 KAT + 7 passing properties).
**Tier:** standard

## Modules Tested

| Module | Tests | Bugs | Oracles Used |
|--------|-------|------|-------------|
| encode_neon_elem_long | 12 | 5 | differential, algebraic.metamorphic, algebraic.invariant, negative_error |

## Bugs Found

### B1: encode_neon_elem_long ignores a fourth operand

**Formal:** ∀ rd,rn,rm ∈ {0..31}, valid (ta,tb,elem,idx,hi), extra operand. llvm-mc rejects mnem Vd.ta, Vn.tb, Vm.elem[idx], extra ⇒ encode_neon_elem_long([Vd.ta,Vn.tb,Vm.elem[idx],extra],...) = Err
**Contract evidence:** inferred (README.md:12 GNU-style assembly; llvm-mc rejects a fourth operand)
**Documentation conflict:** (none) — neon.rs:237 states a minimum of 3 operands and does not declare a maximum
**Severity:** medium
**Counterexample:** encode_neon_elem_long([v0.4s, v0.4h, v0.h[0], v0.4s], u=0, opcode=0b1010, is_high=false)
**Expected / Actual:** Err / Ok(Word) (fourth operand ignored)
**Impact:** Illegal four-operand long by-element assembly is assembled as the three-operand form
**Root cause:** neon.rs:236 checks only `operands.len() < 3`, then encodes
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:236`
```rust
    if operands.len() < 3 {
        return Err("NEON elem-long requires 3 operands".to_string());
    }
```
**Suggested fix:** Reject when the operand count is not exactly 3
```rust
    if operands.len() != 3 {
        return Err("NEON elem-long requires 3 operands".to_string());
    }
```
**Bug report:** bug_reports/encode_neon_elem_long_extra_operand.md
**Repro seed:** cc 1e40101823349a70e495bafb7ead80283ab80286b71bc24dcfa82aead9bcb594
**Raw output:**
```text
Test failed: extra operand must Err (llvm-mc rejects smull v0.4s, v0.4h, v0.h[0], v0.4s)
minimal failing input: rd = 0, rn = 0, rm_raw = 0, extra = 0, idx_raw = 0, shape = ("4h", "4s", "h", 7, 15, false), insn = (0, 10, "smull")
```

### B2: encode_neon_elem_long ignores destination arrangement

**Formal:** ∀ rd,rn,rm, tb ∈ {4h,8h,2s,4s}, ta' ≠ mandated_ta(tb), idx in-range. llvm-mc rejects smull Vd.ta', Vn.tb, Vm.elem[idx] ⇒ encode_neon_elem_long([Vd.ta',Vn.tb,Vm.elem[idx]], 0, 0b1010, hi(tb)) = Err
**Contract evidence:** inferred (ARM long by-element dest is the widened source; llvm-mc rejects mismatched Ta)
**Documentation conflict:** (none)
**Severity:** medium
**Counterexample:** encode_neon_elem_long([v0.8b, v0.4h, v0.h[0]], u=0, opcode=0b1010, is_high=false)
**Expected / Actual:** Err / Ok(Word) (dest arrangement discarded)
**Impact:** Illegal dest arrangement is encoded as if dest were the mandated .4s/.2d
**Root cause:** neon.rs:239 binds dest arrangement as `_arr_d` and never checks it
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:239`
```rust
    let (rd, _arr_d) = get_neon_reg(operands, 0)?;
```
**Suggested fix:** Require dest arrangement to be the widened form of the source (4h/8h → 4s, 2s/4s → 2d)
```rust
    let (rd, arr_d) = get_neon_reg(operands, 0)?;
    let mandated = match arr_n.as_str() {
        "4h" | "8h" => "4s",
        "2s" | "4s" => "2d",
        _ => unreachable!(),
    };
    if arr_d != mandated {
        return Err(format!("elem-long dest arrangement {} does not match source {}", arr_d, arr_n));
    }
```
**Bug report:** bug_reports/encode_neon_elem_long_mismatch_ta.md
**Repro seed:** (deterministic regression)
**Raw output:**
```text
Test failed: mismatched dest Ta must Err (llvm-mc rejects smull v0.8b, v0.4h, v0.h[0]; mandated Ta=4s)
minimal failing input: rd = 0, rn = 0, rm_raw = 0, idx_raw = 0, shape = ("4h", "4s", "h", 7, 15, false), ta_wrong = "8b"
```

### B3: encode_neon_elem_long silently truncates H-lane Rm v16–v31 to v0–v15

**Formal:** ∀ rd,rn ∈ {0..31}, rm ∈ {16..31}, idx ∈ {0..7}, tb ∈ {4h,8h}. llvm-mc rejects smull Vd.4s, Vn.tb, Vm.h[idx] ⇒ encode_neon_elem_long(...) = Err
**Contract evidence:** inferred (ARM size=01 Rm is 4 bits / v0-v15; llvm-mc rejects v16.h[0]; README.md:12 GNU-style)
**Documentation conflict:** neon.rs:289 "Limit Rm for half-word indexing (only v0-v15)" is the producing mask comment, not an input-domain restriction and not an admission that v16-v31 may be passed. Does not retire the fail.
**Severity:** high
**Counterexample:** encode_neon_elem_long([v0.4s, v0.4h, v16.h[0]], u=0, opcode=0b1010, is_high=false)
**Expected / Actual:** Err / Ok(Word) with Rm field 0 (v16 & 0xF)
**Impact:** `smull v0.4s, v0.4h, v16.h[0]` is assembled as `smull v0.4s, v0.4h, v0.h[0]` — wrong register, no error
**Root cause:** neon.rs:287 masks `rm & 0xF` for half-word instead of rejecting rm > 15
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:287`
```rust
    let rm_enc = if size == 0b01 { rm & 0xF } else { rm & 0x1F };
```
**Suggested fix:** Return Err when size=01 and rm > 15
```rust
    if size == 0b01 && rm > 15 {
        return Err(format!("H-lane Rm v{} out of range (v0-v15)", rm));
    }
    let rm_enc = if size == 0b01 { rm & 0xF } else { rm & 0x1F };
```
**Bug report:** bug_reports/encode_neon_elem_long_h_rm_hi.md
**Repro seed:** (deterministic regression)
**Raw output:**
```text
Test failed: H-lane Rm v16-v31 must Err (ARM size=01 Rm v0-v15; llvm-mc rejects smull v0.4s, v0.4h, v16.h[0])
minimal failing input: rd = 0, rn = 0, rm = 16, idx = 0, tb = "4h"
```

### B4: encode_neon_elem_long accepts a GPR destination as a NEON register

**Formal:** ∀ kind ∈ {x-dest, w-dest, sp-dest, bare-v, x-arranged, non-lane-third, s-dest}. llvm-mc rejects the corresponding asm ⇒ encode_neon_elem_long(ops, 0, 0b1010, false) = Err
**Contract evidence:** inferred (README.md:12 GNU-style; llvm-mc requires Vd.Ta)
**Documentation conflict:** (none)
**Severity:** medium
**Counterexample:** encode_neon_elem_long([x0, v0.4h, v0.h[0]], u=0, opcode=0b1010, is_high=false)
**Expected / Actual:** Err / Ok(Word) with Rd=0
**Impact:** `smull x0, v0.4h, v0.h[0]` is encoded as if dest were v0
**Root cause:** neon.rs:239 extracts dest via get_neon_reg, which accepts Operand::Reg and x/w/s/sp prefixes
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:239`
```rust
    let (rd, _arr_d) = get_neon_reg(operands, 0)?;
```
**Suggested fix:** Require dest to be Operand::RegArrangement with a `v` prefix
```rust
    let (rd, arr_d) = match operands.get(0) {
        Some(Operand::RegArrangement { reg, arrangement }) if reg.to_lowercase().starts_with('v') => {
            let num = parse_reg_num(reg).ok_or_else(|| format!("invalid NEON register: {}", reg))?;
            (num, arrangement.clone())
        }
        other => return Err(format!("expected NEON Vd.Ta at operand 0, got {:?}", other)),
    };
```
**Bug report:** bug_reports/encode_neon_elem_long_gpr_dest.md
**Repro seed:** (deterministic regression)
**Raw output:**
```text
Test failed: non-arranged NEON / GPR / SP / non-V prefix / non-lane third must Err (llvm-mc rejects smull x0, v0.4h, v0.h[0])
minimal failing input: rd = 0, rn = 0, rm = 0, idx = 0, kind = 0
```

### B5: encode_neon_elem_long ignores the lane element size

**Formal:** ∀ valid (ta,tb,elem,idx), wrong ≠ elem. llvm-mc rejects mnem Vd.ta, Vn.tb, Vm.wrong[idx] ⇒ encode_neon_elem_long(...) = Err
**Contract evidence:** inferred (neon.rs:234 lane example v0.h[2]; llvm-mc requires lane elem size to match the source)
**Documentation conflict:** (none) — `elem_size: _` is the producing bind, not a documented exclusion
**Severity:** medium
**Counterexample:** encode_neon_elem_long([v0.2d, v0.2s, v0.b[0]], u=0, opcode=0b1010, is_high=false)
**Expected / Actual:** Err / Ok(Word) encoded as if the lane were v0.s[0]
**Impact:** `smull v0.2d, v0.2s, v0.b[0]` is assembled as a .s-lane form
**Root cause:** neon.rs:244 binds `elem_size: _` and never checks it against the source element size
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:244`
```rust
        Operand::RegLane { reg, elem_size: _, index } => {
```
**Suggested fix:** Require `elem_size` to be `"h"` when size=01 and `"s"` when size=10
```rust
        Operand::RegLane { reg, elem_size, index } => {
            let rm = parse_reg_num(reg).ok_or("invalid NEON register")?;
            (rm, elem_size.clone(), *index)
        }
    let want = if size == 0b01 { "h" } else { "s" };
    if elem_size != want {
        return Err(format!("elem-long lane size .{} does not match source", elem_size));
    }
```
**Bug report:** bug_reports/encode_neon_elem_long_lane_elem_mismatch.md
**Repro seed:** (deterministic regression)
**Raw output:**
```text
Test failed: lane elem_size b must match source s (llvm-mc rejects smull v0.2d, v0.2s, v0.b[0])
minimal failing input: rd = 0, rn = 0, rm_raw = 0, idx_raw = 0, shape = ("2s", "2d", "s", 3, 31, false), wrong = "b"
```

## Design Caveats

(none)

## Test Files Created

| File | Tests |
|------|-------|
| src/backend/arm/assembler/encoder/encode_neon_elem_long_pbt.rs | 12 properties + 4 KAT + 5 regression witnesses |
| src/backend/arm/assembler/encoder/mod.rs | one-line `#[cfg(test)] mod encode_neon_elem_long_pbt;` |

## Reproduction

Whole suite:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_neon_elem_long -- --test-threads=1
```

B1 extra operand:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_elem_long_regression_extra_operand -- --test-threads=1
```

B2 dest arrangement:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_elem_long_regression_mismatch_ta -- --test-threads=1
```

B3 H-lane Rm:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_elem_long_regression_h_rm_hi -- --test-threads=1
```

B4 GPR dest:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_elem_long_regression_gpr_dest -- --test-threads=1
```

B5 lane elem_size:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_elem_long_regression_lane_elem_mismatch -- --test-threads=1
```

## Output Directories

- pbt-out/REPORT.md
- pbt-out/REPORT.html
- pbt-out/PROPERTIES.md
- pbt-out/PLAN.md
- pbt-out/COVERAGE.md
- pbt-out/COVERAGE_STATUS.md
- pbt-out/INVARIANTS.md
- pbt-out/FUNCTION_INDEX.md
- pbt-out/report.json
- pbt-out/bug_reports/encode_neon_elem_long_extra_operand.md
- pbt-out/bug_reports/encode_neon_elem_long_extra_operand.html
- pbt-out/bug_reports/encode_neon_elem_long_mismatch_ta.md
- pbt-out/bug_reports/encode_neon_elem_long_mismatch_ta.html
- pbt-out/bug_reports/encode_neon_elem_long_h_rm_hi.md
- pbt-out/bug_reports/encode_neon_elem_long_h_rm_hi.html
- pbt-out/bug_reports/encode_neon_elem_long_gpr_dest.md
- pbt-out/bug_reports/encode_neon_elem_long_gpr_dest.html
- pbt-out/bug_reports/encode_neon_elem_long_lane_elem_mismatch.md
- pbt-out/bug_reports/encode_neon_elem_long_lane_elem_mismatch.html
- pbt-out/run/ (cargo test CWD)

## Coverage Report

# PBT Coverage Status

> Last updated: 2026-10-05 19:35 (campaign: coverage)
> Files: 10/10 scanned (100%) | Functions: 137/289 total | PBT candidates: 137 | Tested: 137 (100%) | 0 pass, 137 fail

## Summary

| Metric | Value |
|--------|-------|
| Total source files | 10 |
| Files scanned | 10 / 10 (100%) |
| Total functions (all files) | 289 |
| PBT candidates (from FUNCTION_INDEX) | 137 |
| **Tested (of PBT candidates)** | **137 / 137 (100%)** |
| &nbsp;&nbsp;↳ Pass / Fail / Other | 0 / 137 / 0 |
| **Overall (tested / all functions)** | **137 / 289 (47%)** |
| Untested | 0 |
| Skipped | 0 |

## Module Breakdown

| Module | Scanned | Tested | Skipped | Coverage |
|--------|---------|--------|---------|----------|
|  | 137 | 137 | 0 | 100% |

## Oracle Type Distribution

| Oracle Type | Total | Covered | Skipped | Coverage |
|-------------|-------|---------|---------|----------|
| unknown | 137 | 137 | 0 | 100% |

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
| neon.rs | 68 | 52 | 52 | 100% | covered |
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
