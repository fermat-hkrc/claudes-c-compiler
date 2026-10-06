# PBT Campaign Report: encode_vsetvli

## Summary

**Verdict:** 3 medium: encode_vsetvli rejects RVV 1.0 SEW e128/e256/e512/e1024 (`unknown vtypei field`), encodes two-operand `vsetvli rd, rs1` as vtypei=0 instead of Err, and last-wins or skips extra operands — all disagree with llvm-mc `-mattr=+v`.
**Date:** 2026-10-06
**Repository:** /home/toan/github/claudes-c-compiler
**Modules tested:** encode_vsetvli (src/backend/riscv/assembler/encoder/vector.rs)
**Tests:** 9
**Result:** 6 passing, 3 bugs
**Change surface:** 1 changed function, 1 with a property, 0 error-handling changes
**Coverage evidence:** file-level (symbol presence) — coverage_gaps reported no .gcda/.profraw (Rust cargo tests are not the C++ reporter binaries) and encode_vsetvli NOT LINKED in those binaries. Manual audit of the documented vsetvli surface.
**Tier:** standard

## Modules Tested

| Module | Tests | Bugs | Oracles Used |
|--------|-------|------|-------------|
| encode_vsetvli | 9 | 3 | differential, algebraic.invariant, algebraic.metamorphic, negative_error |

## Bugs Found

### B1: encode_vsetvli rejects RVV 1.0 SEW e128/e256/e512/e1024

**Formal:** ∀ rd,rs1 ∈ GPR, sew ∈ {e128,e256,e512,e1024}, lmul ∈ named LMUL, ta,ma. encode_vsetvli(named) = Word(w) ∧ w = llvm-mc(...)
**Contract evidence:** inferred (RISC-V V 1.0 vsew enumerators; llvm-mc `-mattr=+v` accepts e128/e256/e512/e1024; README.md:14 V standard extension; no comment declares those SEW values invalid)
**Documentation conflict:** (none) — parse_vtypei lists only e8/e16/e32/e64 with no “only these SEW” restriction
**Severity:** medium
**Counterexample:** encode_vsetvli([Reg("x0"), Reg("x0"), Symbol("e128"), Symbol("m1"), Symbol("tu"), Symbol("mu")])
**Expected / Actual:** Ok(Word(0x02007057)) / Err("unknown vtypei field: e128")
**Impact:** Valid RVV 1.0 vsetvli with SEW wider than 64 cannot be assembled.
**Root cause:** vector.rs:22-38 parse_vtypei match omits e128/e256/e512/e1024, so those names hit `unknown vtypei field`.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/vector.rs:22`
```rust
            "e8" => sew = 0b000,
            "e16" => sew = 0b001,
            "e32" => sew = 0b010,
            "e64" => sew = 0b011,
            "m1" => lmul = 0b000,
```
**Suggested fix:** Accept the remaining RVV 1.0 SEW names (vsew=100/101/110/111).
```rust
            "e8" => sew = 0b000,
            "e16" => sew = 0b001,
            "e32" => sew = 0b010,
            "e64" => sew = 0b011,
            "e128" => sew = 0b100,
            "e256" => sew = 0b101,
            "e512" => sew = 0b110,
            "e1024" => sew = 0b111,
            "m1" => lmul = 0b000,
```
**Bug report:** bug_reports/encode_vsetvli_wide_sew.md
**Repro seed:** cc 5441f6f005d09dbc412f22162cefdd2e01c6f953117320761ac0bd0f7464e299
**Raw output:**
```text
Test failed: SUT rejected valid vsetvli x0, x0, e128, m1, tu, mu: unknown vtypei field: e128.
minimal failing input: rd = "x0", rs1 = "x0", sew = "e128", lmul = "m1", ta = "tu", ma = "mu"
```

### B2: encode_vsetvli accepts two-operand form (missing vtypei)

**Formal:** ∀ ops with len≤2 ∨ rd/rs1 ∈ FP ∪ {v0..v31}. encode_vsetvli(ops) = Err(_)
**Contract evidence:** inferred (rustdoc `Encode vsetvli rd, rs1, vtypei`; llvm-mc `too few operands for instruction` on `vsetvli x0, x0`)
**Documentation conflict:** (none)
**Severity:** medium
**Counterexample:** encode_vsetvli([Reg("x0"), Reg("x0")])
**Expected / Actual:** Err / Ok(Word(0x7057))
**Impact:** Omitting vtypei silently encodes e8,m1,tu,mu instead of failing the assemble.
**Root cause:** vector.rs:49-51 parse_vtypei(operands, 2) on a 2-operand list returns Ok(0).
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/vector.rs:49`
```rust
    let rd = get_reg(operands, 0)?;
    let rs1 = get_reg(operands, 1)?;
    let vtypei = parse_vtypei(operands, 2)?;
```
**Suggested fix:** Require a vtypei operand before packing.
```rust
    let rd = get_reg(operands, 0)?;
    let rs1 = get_reg(operands, 1)?;
    if operands.len() < 3 {
        return Err(format!("vsetvli: expected rd, rs1, vtypei, got {} operands", operands.len()));
    }
    let vtypei = parse_vtypei(operands, 2)?;
```
**Bug report:** bug_reports/encode_vsetvli_arity_two.md
**Repro seed:** cc ca42d77b11c2470f607688495b82301ff3d663fb97c224ad04bc0a8308f56462
**Raw output:**
```text
Test failed: arity 2 must Err (llvm-mc too few operands); got Ok(Word(28759))
minimal failing input: ops = [Reg("x0"), Reg("x0")], fp = "f0"
```

### B3: encode_vsetvli ignores or last-wins extra operands

**Formal:** ∀ rd,rs1,sew,lmul,ta,ma, extra. encode_vsetvli([rd,rs1,sew,lmul,ta,ma,extra]) = Err(_)
**Contract evidence:** inferred (llvm-mc rejects extra tokens; encode_instruction passes extras through at mod.rs:941)
**Documentation conflict:** (none)
**Severity:** medium
**Counterexample:** encode_vsetvli([Reg("x0"), Reg("x0"), Symbol("e8"), Symbol("m1"), Symbol("tu"), Symbol("mu"), Symbol("e8")])
**Expected / Actual:** Err / Ok(Word(0x7057))
**Impact:** A seventh operand is not rejected; repeated named fields last-win, Imm hijacks vtypei, Mem/Csr/Label are skipped.
**Root cause:** encode_vsetvli does not check arity; parse_vtypei walks remaining operands, overwriting known names, returning on Imm, and skipping other kinds.
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
**Suggested fix:** Reject operand lists longer than a complete vsetvli.
```rust
    if operands.len() > 6 {
        return Err(format!("vsetvli: extra operand, got {}", operands.len()));
    }
```
**Bug report:** bug_reports/encode_vsetvli_extra_operand.md
**Repro seed:** (proptest shrunk extra = Symbol("e8"); no cc line emitted for this case in the first run beyond the extra property fail)
**Raw output:**
```text
Test failed: extra operand Symbol("e8") must Err for vsetvli (llvm-mc rejects extra); got Ok(Word(28759))
minimal failing input: rd = "x0", rs1 = "x0", sew = "e8", lmul = "m1", ta = "tu", ma = "mu", extra = Symbol("e8")
```

## Design Caveats

(none)

## Test Files Created

| File | Tests |
|------|-------|
| src/backend/riscv/assembler/encoder/encode_vsetvli_pbt.rs | 9 properties + 6 KAT + 3 regression witnesses |
| src/backend/riscv/assembler/encoder/mod.rs | #[cfg(test)] mod encode_vsetvli_pbt |

## Reproduction

Named-vtype differential (passing):
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_vsetvli_diff_llvm_mc_named -- --test-threads=1
```

Whole suite:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_vsetvli -- --test-threads=1
```

B1 wide SEW:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_vsetvli_regression_wide_sew_e128 -- --test-threads=1
```

B2 arity:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_vsetvli_regression_arity_two -- --test-threads=1
```

B3 extra operand:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_vsetvli_regression_extra_operand -- --test-threads=1
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
- pbt-out/bug_reports/encode_vsetvli_wide_sew.md
- pbt-out/bug_reports/encode_vsetvli_wide_sew.html
- pbt-out/bug_reports/encode_vsetvli_arity_two.md
- pbt-out/bug_reports/encode_vsetvli_arity_two.html
- pbt-out/bug_reports/encode_vsetvli_extra_operand.md
- pbt-out/bug_reports/encode_vsetvli_extra_operand.html
- src/backend/riscv/assembler/encoder/encode_vsetvli_pbt.rs

## Coverage Report

# PBT Coverage Status

> Last updated: 2026-10-06 22:52 (campaign: coverage)
> Files: 16/16 scanned (100%) | Functions: 221/383 total | PBT candidates: 221 | Tested: 221 (100%) | 1 pass, 221 fail

## Summary

| Metric | Value |
|--------|-------|
| Total source files | 16 |
| Files scanned | 16 / 16 (100%) |
| Total functions (all files) | 383 |
| PBT candidates (from FUNCTION_INDEX) | 221 |
| **Tested (of PBT candidates)** | **221 / 221 (100%)** |
| &nbsp;&nbsp;↳ Pass / Fail / Other | 1 / 221 / -1 |
| **Overall (tested / all functions)** | **221 / 383 (58%)** |
| Untested | 0 |
| Skipped | 0 |

## Module Breakdown

| Module | Scanned | Tested | Skipped | Coverage |
|--------|---------|--------|---------|----------|
|  | 221 | 221 | 0 | 100% |

## Oracle Type Distribution

| Oracle Type | Total | Covered | Skipped | Coverage |
|-------------|-------|---------|---------|----------|
| unknown | 221 | 221 | 0 | 100% |

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
