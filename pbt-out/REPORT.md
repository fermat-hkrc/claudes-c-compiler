# PBT Campaign Report: encode_neon_float_elem

## Summary

**Verdict:** 1 high: encode_neon_float_elem leaves ARM size bit 23 clear, so every FMUL/FMLA/FMLS by-element word disagrees with llvm-mc/gas (0x0f009000 vs 0x0f809000 for `fmul v0.2s, v0.2s, v0.s[0]`); plus 5 medium: extra operand, mismatched T, X-prefix dest, index OOB, mismatched lane size.
**Date:** 2026-10-05
**Repository:** /home/toan/github/claudes-c-compiler
**Modules tested:** encode_neon_float_elem
**Tests:** 11
**Result:** 4 passing, 7 bugs (6 unique defects; B1 and B7 are the same size-bit encoding)
**Change surface:** 1 changed function (encode_neon_float_elem), 1 with a property, 0 error-handling changes
**Coverage evidence:** file-level (symbol presence) — coverage_gaps reported no .gcda/.profraw (C++ reporter; claimed NOT LINKED). The SUT ran under `cargo test --lib encode_neon_float_elem`. Sweep round 1/1 spent.
**Effort tier:** standard

## Modules Tested

| Module | Tests | Bugs | Oracles Used |
|--------|-------|------|-------------|
| encode_neon_float_elem | 11 | 7 | differential, algebraic.metamorphic, algebraic.invariant, negative_error |

## Bugs Found

### B1: encode_neon_float_elem leaves ARM size bit 23 clear

**Formal:** ∀ rd,rn,rm ∈ {0..31}, T ∈ {2s,4s,2d}, idx ∈ {0..imax(T)}, (U,opc,mnem) ∈ {(0,0b1001,fmul),(0,0b0001,fmla),(0,0b0101,fmls),(1,0b1001,fmulx)}. encode_neon_float_elem([Vd.T, Vn.T, Vm.Ts[idx]], U, opc) = llvm-mc("-triple=aarch64 -show-encoding", "mnem Vd.T, Vn.T, Vm.Ts[idx]") where imax(2s)=imax(4s)=3, imax(2d)=1, Ts is s for 2s/4s and d for 2d
**Contract evidence:** inferred (README.md:12 gas compatibility; ARM Advanced SIMD vector x indexed element size=10/11; llvm-mc 15.0.6)
**Documentation conflict:** (none)
**Severity:** high
**Counterexample:** encode_neon_float_elem([v0.2s, v0.2s, v0.s[0]], u_bit=0, opcode=0b1001)
**Expected / Actual:** 0x0f809000 / 0x0f009000
**Impact:** Every NEON FP by-element instruction the assembler emits is the wrong 32-bit word (reserved size 00/01). Linked objects execute a different instruction than the assembly text
**Root cause:** neon.rs:1633 shifts `sz` (0 for S, 1 for D) to bit 22 only, leaving bit 23 = 0. ARM/llvm-mc encode size[1:0] as 10 (S) / 11 (D)
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:1633`
```rust
    let word = (q << 30) | (u_bit << 29) | (0b01111 << 24) | (sz << 22)
```
**Suggested fix:** OR the high size bit so S=0b10 and D=0b11
```rust
    let word = (q << 30) | (u_bit << 29) | (0b01111 << 24) | ((0b10 | sz) << 22)
```
**Bug report:** bug_reports/encode_neon_float_elem_size_bit23.md
**Repro seed:** cc c99eaf81cfd8b5362f14e6432377b70fa338f321e7658ecdb5613b7d2b65b85b
**Raw output:**
```text
Test failed: assertion failed: `(left == right)`
  left: `251695104`,
 right: `260083712`: SUT vs llvm-mc for fmul v0.2s, v0.2s, v0.s[0]
minimal failing input: rd = 0, rn = 0, rm = 0, idx_raw = 0, shape = ("2s", "s", 3), insn = (0, 9, "fmul")
```

### B7: encode_neon_float_elem ARM size field is 00/01 instead of 10/11

**Formal:** ∀ rd,rn,rm ∈ {0..31}, T ∈ {2s,4s,2d}, idx ∈ {0..imax(T)}, U ∈ {0,1}, opc ∈ {0b0001,0b0101,0b1001}. let w=encode_neon_float_elem([Vd.T,Vn.T,Vm.Ts[idx]],U,opc). w[23:22]=size(T) where size(2s)=size(4s)=0b10, size(2d)=0b11
**Contract evidence:** inferred (ARM Advanced SIMD vector x indexed element size=10/11)
**Documentation conflict:** (none)
**Severity:** high
**Counterexample:** encode_neon_float_elem([v0.2s, v0.2s, v0.s[0]], U=0, opc=0b0001) has bits[23:22]=00, ARM size for S is 10
**Expected / Actual:** bits[23:22]=10 / bits[23:22]=00
**Impact:** Same encoding defect as B1, observed via the ARM layout invariant
**Root cause:** neon.rs:1633 shifts sz to bit 22 only, leaving bit 23 clear
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:1633`
```rust
    let word = (q << 30) | (u_bit << 29) | (0b01111 << 24) | (sz << 22)
```
**Suggested fix:** OR the high size bit so S=0b10 and D=0b11
```rust
    let word = (q << 30) | (u_bit << 29) | (0b01111 << 24) | ((0b10 | sz) << 22)
```
**Bug report:** bug_reports/encode_neon_float_elem_size_layout.md
**Repro seed:** (deterministic)
**Raw output:**
```text
Test failed: assertion failed: `(left == right)`
  left: `0`,
 right: `2`: size must be 10 (S) or 11 (D)
minimal failing input: rd = 0, rn = 0, rm = 0, idx_raw = 0, shape = ("2s", "s", 3), u = 0, opcode = 1
```

### B2: encode_neon_float_elem ignores a fourth operand

**Formal:** ∀ valid (rd,rn,rm,T,idx,U,opc,mnem) and extra ∈ {0..31}. llvm-mc(mnem Vd.T, Vn.T, Vm.Ts[idx], Ve.T) is Err ∧ encode_neon_float_elem([Vd.T,Vn.T,Vm.Ts[idx],Ve.T], U, opc) is Err
**Contract evidence:** inferred (README.md:12 gas compatibility; llvm-mc rejects a fourth operand)
**Documentation conflict:** neon.rs:1615 "NEON float by-element requires 3 operands" — domain-restriction of a minimum of 3, not a maximum; it does not declare extras valid
**Severity:** medium
**Counterexample:** encode_neon_float_elem([v0.2s, v0.2s, v0.s[0], v0.2s], u_bit=0, opcode=0b1001)
**Expected / Actual:** Err / Ok(Word)
**Impact:** Trailing junk after a by-element FMUL is silently dropped
**Root cause:** neon.rs:1615 checks only `operands.len() < 3`
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:1615`
```rust
    if operands.len() < 3 { return Err("NEON float by-element requires 3 operands".to_string()); }
```
**Suggested fix:** Reject anything other than exactly 3 operands
```rust
    if operands.len() != 3 { return Err("NEON float by-element requires 3 operands".to_string()); }
```
**Bug report:** bug_reports/encode_neon_float_elem_extra_operand.md
**Repro seed:** (deterministic; first extra case)
**Raw output:**
```text
Test failed: extra operand must Err (llvm-mc rejects fmul v0.2s, v0.2s, v0.s[0], v0.2s)
minimal failing input: rd = 0, rn = 0, rm = 0, extra = 0, idx_raw = 0, shape = ("2s", "s", 3), insn = (0, 9, "fmul")
```

### B3: encode_neon_float_elem ignores a mismatched source arrangement

**Formal:** ∀ rd,rn,rm ∈ {0..31}, T ∈ {2s,4s,2d}, T' ≠ T, idx ∈ {0..imax(T)}. llvm-mc("fmul Vd.T, Vn.T', Vm.Ts[idx]") is Err ∧ encode_neon_float_elem([Vd.T, Vn.T', Vm.Ts[idx]], 0, 0b1001) is Err
**Contract evidence:** inferred (README.md:12; llvm-mc invalid operand for mismatched T)
**Documentation conflict:** (none)
**Severity:** medium
**Counterexample:** encode_neon_float_elem([v0.4s, v0.8b, v0.s[0]], u_bit=0, opcode=0b1001)
**Expected / Actual:** Err / Ok(Word)
**Impact:** `fmul v0.4s, v0.8b, v0.s[0]` encodes as if both were .4s
**Root cause:** neon.rs:1617 discards the source arrangement
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:1617`
```rust
    let (rn, _) = get_neon_reg(operands, 1)?;
```
**Suggested fix:** Require the source arrangement to equal dest
```rust
    let (rn, arr_n) = get_neon_reg(operands, 1)?;
    if arr_n != arr_d {
        return Err(format!("float by-element: mismatched arrangement {} vs {}", arr_d, arr_n));
    }
```
**Bug report:** bug_reports/encode_neon_float_elem_mismatch_t.md
**Repro seed:** (deterministic; v0.4s vs v0.8b)
**Raw output:**
```text
Test failed: mismatched source T must Err (llvm-mc rejects fmul v0.4s, v0.8b, v0.s[0])
minimal failing input: rd = 0, rn = 0, rm = 0, idx_raw = 0, shape = ("4s", "s", 3), t_wrong = "8b"
```

### B4: encode_neon_float_elem accepts an X-prefixed destination as Vd

**Formal:** ∀ kind ∈ {x-dest, w-dest, sp-dest, bare-V, x-prefix-arr, s-dest}. llvm-mc(asm(kind)) is Err ∧ encode_neon_float_elem(ops(kind), 0, 0b1001) is Err
**Contract evidence:** inferred (README.md:12; llvm-mc rejects `fmul x0.4s, ...`)
**Documentation conflict:** (none)
**Severity:** medium
**Counterexample:** encode_neon_float_elem([RegArrangement { reg: "x0", arrangement: "4s" }, v0.4s, v0.s[0]], u_bit=0, opcode=0b1001)
**Expected / Actual:** Err / Ok(Word) with Rd=0
**Impact:** `fmul x0.4s, v0.4s, v0.s[0]` encodes as `fmul v0.4s, ...`
**Root cause:** neon.rs:1616 uses get_neon_reg → parse_reg_num, which accepts the x prefix and returns 0
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:1616`
```rust
    let (rd, arr_d) = get_neon_reg(operands, 0)?;
```
**Suggested fix:** Require a V prefix on dest and source register names
```rust
    if !reg.to_ascii_lowercase().starts_with('v') {
        return Err(format!("expected NEON V register, got {}", reg));
    }
```
**Bug report:** bug_reports/encode_neon_float_elem_x_prefix.md
**Repro seed:** cc 9f450d3ead4f348f55eb12cfec1fd265045848b6dcde3eb4ab5bdb41e6f22fad
**Raw output:**
```text
Test failed: non-arranged NEON / GPR / SP / non-V prefix must Err (llvm-mc rejects fmul x0.4s, v0.4s, v0.s[0])
minimal failing input: rd = 0, rn = 0, rm = 0, idx = 0, kind = 4
```

### B5: encode_neon_float_elem accepts an out-of-range lane index

**Formal:** ∀ rd,rn,rm ∈ {0..31}, T ∈ {2s,4s,2d}, extra ∈ {1..8}. let idx = imax(T)+extra. llvm-mc("fmul Vd.T, Vn.T, Vm.Ts[idx]") is Err ∧ encode_neon_float_elem([Vd.T,Vn.T,Vm.Ts[idx]], 0, 0b1001) is Err
**Contract evidence:** inferred (llvm-mc "vector lane must be an integer in range [0, 3]" / "[0, 1]")
**Documentation conflict:** (none)
**Severity:** medium
**Counterexample:** encode_neon_float_elem([v0.2s, v0.2s, v0.s[4]], u_bit=0, opcode=0b1001)
**Expected / Actual:** Err / Ok(Word) with H:L from the low bits of 4
**Impact:** An out-of-range lane is silently turned into a different in-range lane
**Root cause:** neon.rs:1618 extracts index and uses only H/L bits; no range check
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:1618`
```rust
    let (rm, index) = match &operands[2] {
```
**Suggested fix:** Reject index above the ARM maximum for the dest size
```rust
    let imax = if sz == 0 { 3u32 } else { 1u32 };
    if index > imax {
        return Err(format!("float by-element: lane index {} out of range 0..{}", index, imax));
    }
```
**Bug report:** bug_reports/encode_neon_float_elem_index_oob.md
**Repro seed:** (deterministic; idx = imax+1)
**Raw output:**
```text
Test failed: index 4 out of range for .s must Err (llvm-mc rejects fmul v0.2s, v0.2s, v0.s[4])
minimal failing input: rd = 0, rn = 0, rm = 0, shape = ("2s", "s", 3), extra = 1
```

### B6: encode_neon_float_elem ignores a mismatched lane element size

**Formal:** ∀ rd,rn,rm ∈ {0..31}, T ∈ {2s,4s,2d}, wrong ≠ Ts(T), idx ∈ {0..imax(T)}. llvm-mc("fmul Vd.T, Vn.T, Vm.wrong[idx]") is Err ∧ encode_neon_float_elem([Vd.T, Vn.T, Vm.wrong[idx]], 0, 0b1001) is Err
**Contract evidence:** inferred (README.md:12; llvm-mc rejects `fmul v0.2d, v0.2d, v0.b[0]`)
**Documentation conflict:** (none)
**Severity:** medium
**Counterexample:** encode_neon_float_elem([v0.2d, v0.2d, v0.b[0]], u_bit=0, opcode=0b1001)
**Expected / Actual:** Err / Ok(Word)
**Impact:** A mistyped lane size is encoded as if it matched T
**Root cause:** neon.rs:1618 matches RegLane with `elem_size` in `..`
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:1619`
```rust
        Operand::RegLane { reg, index, .. } => (parse_reg_num(reg).ok_or("invalid reg")?, *index),
```
**Suggested fix:** Bind elem_size and, after sz is known, require it to match dest T
```rust
    let expect = if sz == 0 { "s" } else { "d" };
    if elem_size != expect {
        return Err(format!("float by-element: lane size {} does not match {}", elem_size, expect));
    }
```
**Bug report:** bug_reports/encode_neon_float_elem_lane_elem.md
**Repro seed:** cc d7758af9b506bbd4e8df324fdb19055235d90fe62b37ac1ae9d07d222708122a
**Raw output:**
```text
Test failed: lane elem_size b must match arrangement d (llvm-mc rejects fmul v0.2d, v0.2d, v0.b[0])
minimal failing input: rd = 0, rn = 0, rm = 0, idx_raw = 0, shape = ("2d", "d", 1), wrong = "b"
```

## Design Caveats

(none)

## Test Files Created

| File | Tests |
|------|-------|
| src/backend/arm/assembler/encoder/encode_neon_float_elem_pbt.rs | 11 properties + 2 KAT + 6 regression witnesses |
| src/backend/arm/assembler/encoder/mod.rs | one-line `mod encode_neon_float_elem_pbt` registration |

## Reproduction

```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_neon_float_elem -- --test-threads=1
```

Whole-suite command (same as the build contract with the test target swapped):

```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_neon_float_elem -- --test-threads=1
```

Per-bug:

```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_neon_float_elem_diff_llvm_mc -- --test-threads=1
cargo test --lib encode_neon_float_elem_neg_extra -- --test-threads=1
cargo test --lib encode_neon_float_elem_neg_mismatch_t -- --test-threads=1
cargo test --lib encode_neon_float_elem_neg_gpr_bare_nonv -- --test-threads=1
cargo test --lib encode_neon_float_elem_neg_index_oob -- --test-threads=1
cargo test --lib encode_neon_float_elem_neg_lane_elem_mismatch -- --test-threads=1
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
- pbt-out/bug_reports/encode_neon_float_elem_size_bit23.md
- pbt-out/bug_reports/encode_neon_float_elem_size_bit23.html
- pbt-out/bug_reports/encode_neon_float_elem_size_layout.md
- pbt-out/bug_reports/encode_neon_float_elem_size_layout.html
- pbt-out/bug_reports/encode_neon_float_elem_extra_operand.md
- pbt-out/bug_reports/encode_neon_float_elem_extra_operand.html
- pbt-out/bug_reports/encode_neon_float_elem_mismatch_t.md
- pbt-out/bug_reports/encode_neon_float_elem_mismatch_t.html
- pbt-out/bug_reports/encode_neon_float_elem_x_prefix.md
- pbt-out/bug_reports/encode_neon_float_elem_x_prefix.html
- pbt-out/bug_reports/encode_neon_float_elem_index_oob.md
- pbt-out/bug_reports/encode_neon_float_elem_index_oob.html
- pbt-out/bug_reports/encode_neon_float_elem_lane_elem.md
- pbt-out/bug_reports/encode_neon_float_elem_lane_elem.html
- pbt-out/run/encode_neon_float_elem_pbt.log
- src/backend/arm/assembler/encoder/encode_neon_float_elem_pbt.rs

## Coverage Report

# PBT Coverage Status

> Last updated: 2026-10-05 20:20 (campaign: coverage)
> Files: 10/10 scanned (100%) | Functions: 139/289 total | PBT candidates: 139 | Tested: 139 (100%) | 0 pass, 139 fail

## Summary

| Metric | Value |
|--------|-------|
| Total source files | 10 |
| Files scanned | 10 / 10 (100%) |
| Total functions (all files) | 289 |
| PBT candidates (from FUNCTION_INDEX) | 139 |
| **Tested (of PBT candidates)** | **139 / 139 (100%)** |
| &nbsp;&nbsp;↳ Pass / Fail / Other | 0 / 139 / 0 |
| **Overall (tested / all functions)** | **139 / 289 (48%)** |
| Untested | 0 |
| Skipped | 0 |

## Module Breakdown

| Module | Scanned | Tested | Skipped | Coverage |
|--------|---------|--------|---------|----------|
|  | 139 | 139 | 0 | 100% |

## Oracle Type Distribution

| Oracle Type | Total | Covered | Skipped | Coverage |
|-------------|-------|---------|---------|----------|
| unknown | 139 | 139 | 0 | 100% |

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
| neon.rs | 68 | 54 | 54 | 100% | covered |
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
