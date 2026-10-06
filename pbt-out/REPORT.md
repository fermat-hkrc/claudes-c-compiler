# PBT Campaign Report: encode_vsetivli

## Summary

**Verdict:** 5 medium: encode_vsetivli disagrees with llvm-mc on extra operands, missing vtypei, wide SEW e128+, AVL outside 0..=31, and raw vtypei ≥ 1024 — each case silently encodes the wrong word or rejects a valid RVV instruction.
**Date:** 2026-10-06
**Repository:** /home/toan/github/claudes-c-compiler
**Modules tested:** encode_vsetivli
**Tests:** 11
**Result:** 6 passing, 5 bugs
**Change surface:** 1 changed function, 1 with a property, 0 error-handling changes
**Coverage evidence:** file-level (symbol presence) — coverage_gaps reported no .gcda/.profraw and encode_vsetivli NOT LINKED in C++ reporter binaries. Rust `cargo test --lib encode_vsetivli` executed the production symbol (KAT + 1000-case properties).
**Tier:** standard

## Modules Tested

| Module | Tests | Bugs | Oracles Used |
|--------|-------|------|-------------|
| encode_vsetivli | 11 | 5 | differential, algebraic.invariant, algebraic.metamorphic, negative_error |

## Bugs Found

### B1: encode_vsetivli rejects RVV 1.0 wide SEW e128/e256/e512/e1024

**Formal:** ∀ rd ∈ GPR names, uimm ∈ 0..=31, sew ∈ {e128,e256,e512,e1024}, lmul ∈ {m1,m2,m4,m8,mf2,mf4,mf8}, ta ∈ {ta,tu}, ma ∈ {ma,mu}. encode_vsetivli([Reg(rd), Imm(uimm), Symbol(sew), Symbol(lmul), Symbol(ta), Symbol(ma)]) = Word(w) ∧ w = llvm-mc("vsetivli rd, uimm, sew, lmul, ta, ma")
**Contract evidence:** inferred (llvm-mc accepts e128/e256/e512/e1024; assembler/README.md:14 claims the V extension; wrapper encoder/mod.rs:945 passes operands through)
**Documentation conflict:** (none)
**Severity:** medium
**Counterexample:** encode_vsetivli([Reg("x0"), Imm(0), Symbol("e128"), Symbol("m1"), Symbol("tu"), Symbol("mu")])
**Expected / Actual:** Ok(Word(0xc2007057)) / Err("unknown vtypei field: e128")
**Impact:** Valid RVV 1.0 vsetivli with SEW larger than e64 is rejected, so the assembler cannot emit the encodings llvm-mc produces.
**Root cause:** vector.rs:22-25 parse_vtypei only matches e8/e16/e32/e64; wide SEW names fall through to unknown vtypei field.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/vector.rs:22`
```rust
            "e8" => sew = 0b000,
            "e16" => sew = 0b001,
            "e32" => sew = 0b010,
            "e64" => sew = 0b011,
```
**Suggested fix:** Accept the remaining vsew encodings that llvm-mc and RVV 1.0 name.
```rust
            "e8" => sew = 0b000,
            "e16" => sew = 0b001,
            "e32" => sew = 0b010,
            "e64" => sew = 0b011,
            "e128" => sew = 0b100,
            "e256" => sew = 0b101,
            "e512" => sew = 0b110,
            "e1024" => sew = 0b111,
```
**Bug report:** bug_reports/encode_vsetivli_wide_sew.md
**Repro seed:** cc 7b867fe0609d413a8da9502e821573b0e3b3b6ec133c5e5ef096c280a2057c47
**Raw output:**
```text
Test failed: SUT rejected valid vsetivli x0, 0, e128, m1, tu, mu: unknown vtypei field: e128.
minimal failing input: rd = "x0", uimm = 0, sew = "e128", lmul = "m1", ta = "tu", ma = "mu"
```

### B2: encode_vsetivli accepts two operands and defaults vtypei to 0

**Formal:** ∀ ops with |ops| ∈ {0,1,2} (missing vtypei). encode_vsetivli(ops) = Err(_)
**Contract evidence:** inferred (llvm-mc "too few operands for instruction"; wrapper encoder/mod.rs:945 passes operands through)
**Documentation conflict:** (none)
**Severity:** medium
**Counterexample:** encode_vsetivli([Reg("x0"), Imm(0)])
**Expected / Actual:** Err / Ok(Word(0xc0007057))
**Impact:** A truncated `vsetivli rd, uimm` is assembled as `vsetivli rd, uimm, e8, m1, tu, mu` with no error.
**Root cause:** vector.rs:62 parse_vtypei(operands, 2) with len==2 returns default packed vtypei 0.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/vector.rs:62`
```rust
    let vtypei = parse_vtypei(operands, 2)?;
```
**Suggested fix:** Require a vtypei operand before encoding.
```rust
    if operands.len() < 3 {
        return Err("vsetivli: missing vtypei".into());
    }
    let vtypei = parse_vtypei(operands, 2)?;
```
**Bug report:** bug_reports/encode_vsetivli_arity_two.md
**Repro seed:** cc 3dd3bce888d04592b9f405b4c3bfe867614d1f77932ff67bc715e5f0f673ca7d
**Raw output:**
```text
Test failed: arity 2 must Err (llvm-mc too few operands); got Ok(Word(3221254231))
minimal failing input: ops = [Reg("x0"), Imm(0)], fp = "f0"
```

### B3: encode_vsetivli ignores or last-wins extra operands

**Formal:** ∀ rd, uimm ∈ 0..=31, sew, lmul, ta, ma, extra. encode_vsetivli([Reg(rd), Imm(uimm), Symbol(sew), Symbol(lmul), Symbol(ta), Symbol(ma), extra]) = Err(_)
**Contract evidence:** inferred (llvm-mc rejects extra tokens; wrapper encoder/mod.rs:945 passes operands through)
**Documentation conflict:** (none)
**Severity:** medium
**Counterexample:** encode_vsetivli([Reg("x0"), Imm(0), Symbol("e8"), Symbol("m1"), Symbol("tu"), Symbol("mu"), Imm(0)])
**Expected / Actual:** Err / Ok(Word(0xc0007057))
**Impact:** A seventh operand is not rejected. Extra Imm is treated as raw vtypei and discards named fields; repeated names last-win.
**Root cause:** vector.rs:14-20 parse_vtypei walks every remaining operand: Imm returns as raw vtypei, known names overwrite, other kinds continue.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/vector.rs:14`
```rust
    for i in start_idx..operands.len() {
        let name = match &operands[i] {
            Operand::Symbol(s) => s.to_lowercase(),
            Operand::Reg(s) => s.to_lowercase(),
            // Raw immediate: treat as pre-encoded vtypei value
            Operand::Imm(v) => return Ok(*v as u32 & 0x7FF),
            _ => continue,
        };
```
**Suggested fix:** After a complete named vtype or a raw immediate, reject further operands.
```rust
    if operands.len() > 6 {
        return Err(format!("vsetivli: extra operand, got {}", operands.len()));
    }
```
**Bug report:** bug_reports/encode_vsetivli_extra_operand.md
**Repro seed:** (none — shrunk to Imm(0) extra; replay via regression test)
**Raw output:**
```text
Test failed: extra operand Imm(0) must Err for vsetivli (llvm-mc rejects extra); got Ok(Word(3221254231))
minimal failing input: rd = "x0", uimm = 0, sew = "e8", lmul = "m1", ta = "tu", ma = "mu", extra = Imm(0)
```

### B4: encode_vsetivli silently truncates AVL outside 0..=31

**Formal:** ∀ rd ∈ GPR names, uimm ∉ 0..=31, sew, lmul, ta, ma. encode_vsetivli([Reg(rd), Imm(uimm), Symbol(sew), Symbol(lmul), Symbol(ta), Symbol(ma)]) = Err(_)
**Contract evidence:** inferred (llvm-mc "immediate must be an integer in the range [0, 31]"; rustdoc vector.rs:57 names the field uimm[4:0])
**Documentation conflict:** (none) — rustdoc describes the encoding field width, it does not declare out-of-range AVL invalid as a caller precondition, and llvm-mc rejects it
**Severity:** medium
**Counterexample:** encode_vsetivli([Reg("x0"), Imm(-1), Symbol("e8"), Symbol("m1"), Symbol("tu"), Symbol("mu")])
**Expected / Actual:** Err / Ok(Word(0xc00ff057)) (AVL -1 truncated to 31)
**Impact:** Wrong VL is programmed with no assembler error; 32 wraps to 0 and -1 wraps to 31.
**Root cause:** vector.rs:61 `get_imm(operands, 1)? as u32 & 0x1F` masks instead of range-checking.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/vector.rs:61`
```rust
    let uimm = get_imm(operands, 1)? as u32 & 0x1F;
```
**Suggested fix:** Reject AVL outside the documented 5-bit field.
```rust
    let avl = get_imm(operands, 1)?;
    if avl < 0 || avl > 31 {
        return Err(format!("vsetivli: AVL {avl} out of range [0, 31]"));
    }
    let uimm = avl as u32;
```
**Bug report:** bug_reports/encode_vsetivli_uimm_oob.md
**Repro seed:** (none — shrunk to uimm=-1; replay via regression test)
**Raw output:**
```text
Test failed: AVL -1 outside 0..=31 must Err (llvm-mc: immediate must be in [0, 31]); got Ok(Word(3222270039))
minimal failing input: rd = "x0", uimm = -1, sew = "e8", lmul = "m1", ta = "tu", ma = "mu"
```

### B5: encode_vsetivli silently truncates raw vtypei above 10 bits

**Formal:** ∀ rd ∈ GPR names, uimm ∈ 0..=31, v ∈ 1024..=2047. encode_vsetivli([Reg(rd), Imm(uimm), Imm(v)]) = Err(_)
**Contract evidence:** inferred (llvm-mc rejects raw vtypei 1024; rustdoc vector.rs:58 pins vtypei[9:0])
**Documentation conflict:** (none)
**Severity:** medium
**Counterexample:** encode_vsetivli([Reg("x0"), Imm(0), Imm(1024)])
**Expected / Actual:** Err / Ok(Word(0xc0007057)) (1024 masked to 0)
**Impact:** An 11-bit vsetvli-style immediate encodes as vtypei=0 with no error, programming the wrong vector type.
**Root cause:** vector.rs:64 `(vtypei & 0x3FF) << 20` drops bit 10 after parse_vtypei kept 11 bits (`& 0x7FF`).
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/vector.rs:64`
```rust
    let word = (0b11u32 << 30) | ((vtypei & 0x3FF) << 20) | (uimm << 15) | (0b111 << 12) | (rd << 7) | OP_V;
```
**Suggested fix:** Reject a raw vtypei that does not fit in 10 bits.
```rust
    let vtypei = parse_vtypei(operands, 2)?;
    if vtypei > 0x3FF {
        return Err(format!("vsetivli: vtypei {vtypei} out of range [0, 1023]"));
    }
```
**Bug report:** bug_reports/encode_vsetivli_vtypei_imm_oob.md
**Repro seed:** (none — shrunk to v=1024; replay via regression test)
**Raw output:**
```text
Test failed: vtypei imm 1024 outside 0..=1023 must Err (llvm-mc rejects 10-bit overflow); got Ok(Word(3221254231))
minimal failing input: rd = "x0", uimm = 0, v = 1024
```

## Design Caveats

(none)

## Test Files Created

| File | Tests |
|------|-------|
| src/backend/riscv/assembler/encoder/encode_vsetivli_pbt.rs | 11 properties + 6 KAT + 5 regression witnesses |
| src/backend/riscv/assembler/encoder/mod.rs | one-line `mod encode_vsetivli_pbt` registration |

## Reproduction

Whole suite (expects 5 property failures and 5 regression failures):
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_vsetivli -- --test-threads=1
```

B1 wide SEW:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_vsetivli_regression_wide_sew_e128 -- --test-threads=1
```

B2 arity two:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_vsetivli_regression_arity_two -- --test-threads=1
```

B3 extra operand:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_vsetivli_regression_extra_operand -- --test-threads=1
```

B4 AVL OOB:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_vsetivli_regression_uimm_oob -- --test-threads=1
```

B5 vtypei OOB:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_vsetivli_regression_vtypei_imm_oob -- --test-threads=1
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
- pbt-out/run/encode_vsetivli.log
- pbt-out/run/encode_vsetivli_round2.log
- pbt-out/run/encode_vsetivli_neg_fp.log
- pbt-out/bug_reports/encode_vsetivli_wide_sew.md
- pbt-out/bug_reports/encode_vsetivli_wide_sew.html
- pbt-out/bug_reports/encode_vsetivli_arity_two.md
- pbt-out/bug_reports/encode_vsetivli_arity_two.html
- pbt-out/bug_reports/encode_vsetivli_extra_operand.md
- pbt-out/bug_reports/encode_vsetivli_extra_operand.html
- pbt-out/bug_reports/encode_vsetivli_uimm_oob.md
- pbt-out/bug_reports/encode_vsetivli_uimm_oob.html
- pbt-out/bug_reports/encode_vsetivli_vtypei_imm_oob.md
- pbt-out/bug_reports/encode_vsetivli_vtypei_imm_oob.html

## Coverage Report

# PBT Coverage Status

> Last updated: 2026-10-06 23:13 (campaign: coverage)
> Files: 16/16 scanned (100%) | Functions: 222/383 total | PBT candidates: 222 | Tested: 222 (100%) | 1 pass, 222 fail

## Summary

| Metric | Value |
|--------|-------|
| Total source files | 16 |
| Files scanned | 16 / 16 (100%) |
| Total functions (all files) | 383 |
| PBT candidates (from FUNCTION_INDEX) | 222 |
| **Tested (of PBT candidates)** | **222 / 222 (100%)** |
| &nbsp;&nbsp;↳ Pass / Fail / Other | 1 / 222 / -1 |
| **Overall (tested / all functions)** | **222 / 383 (58%)** |
| Untested | 0 |
| Skipped | 0 |

## Module Breakdown

| Module | Scanned | Tested | Skipped | Coverage |
|--------|---------|--------|---------|----------|
|  | 222 | 222 | 0 | 100% |

## Oracle Type Distribution

| Oracle Type | Total | Covered | Skipped | Coverage |
|-------------|-------|---------|---------|----------|
| unknown | 222 | 222 | 0 | 100% |

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
| encode_lui | base.rs |
| encode_auipc | base.rs |
| encode_jal | base.rs |
| encode_jalr | base.rs |
| encode_branch_instr | base.rs |
| encode_load | base.rs |
| encode_store | base.rs |
| encode_alu_imm | base.rs |
| encode_alu_reg | base.rs |
| encode_alu_imm_w | base.rs |
| encode_alu_reg_w | base.rs |
| encode_csr | system.rs |
| encode_fence | system.rs |
| encode_amo | atomics.rs |
| encode_lr | atomics.rs |
| encode_sc | atomics.rs |
| encode_sfence_vma | system.rs |
| encode_csri | system.rs |
| encode_float_load | float.rs |
| encode_float_store | float.rs |
| encode_fp_arith | float.rs |
| encode_fp_arith_d | float.rs |
| encode_fp_unary | float.rs |
| encode_fp_sgnj | float.rs |
| encode_fp_cmp | float.rs |
| encode_fclass | float.rs |
| encode_fcvt_int | float.rs |
| encode_fcvt_from_int | float.rs |
| encode_fcvt_fp | float.rs |
| encode_fmv_x_f | float.rs |
| encode_fmv_f_x | float.rs |
| encode_fma | float.rs |
| encode_c_lui | compressed.rs |
| encode_c_li | compressed.rs |
| encode_c_addi | compressed.rs |
| encode_c_mv | compressed.rs |
| encode_c_add | compressed.rs |
| encode_c_jr | compressed.rs |
| encode_c_jalr | compressed.rs |
| encode_vsetvli | vector.rs |
| encode_vsetivli | vector.rs |
