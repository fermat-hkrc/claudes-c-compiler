# PBT Campaign Report: encode_csr (requested encode_system)

## Summary

**Verdict:** 3 high: encode_csr silently accepts extra operands, wraps zimm outside 0..=31 into csrrwi/csrrsi/csrrci, and truncates CSR numbers outside 0..=4095, so mistyped CSR instructions assemble to the wrong SYSTEM word.
**Date:** 2026-10-06
**Repository:** /home/toan/github/claudes-c-compiler
**Modules tested:** encode_csr
**Tests:** 10 properties (plus 3 KAT, 3 regression witnesses)
**Result:** 7 passing, 3 bugs
**Change surface:** (no change source given) — requested encode_system was unresolved in base.rs; mapped to encode_csr
**Coverage evidence:** file-level (symbol presence) — coverage_gaps reported no .gcda/.profraw (Rust cargo test; C++ reporter listed unrelated binaries). Manual arm audit of encode_csr plus sweep property for get_csr_num Reg/decimal paths.
**Tier:** standard

## Modules Tested

| Module | Tests | Bugs | Oracles Used |
|--------|-------|------|-------------|
| encode_csr | 10 | 3 | differential, algebraic.invariant, algebraic.metamorphic, negative_error |

## Bugs Found

### B1: encode_csr ignores extra operands

**Formal:** ∀ mn ∈ {csrrw,csrrs,csrrc}, rd, rs1 ∈ GPRNames, csr ∈ KnownCsrNames, extra ∈ Operands. encode_csr([Reg(rd), Csr(csr), Reg(rs1), extra], funct3(mn)) = Err(_)
**Contract evidence:** inferred (llvm-mc rejects extra operands on csrrw; README.md:6-7 assembler consumes textual assembly)
**Documentation conflict:** (none)
**Severity:** high
**Counterexample:** encode_csr([Reg("x0"), Csr("fflags"), Reg("x0"), Imm(0)], funct3=0b001)
**Expected / Actual:** Err / Ok(Word(1052787))  // 0x00101073 = csrrw x0, fflags, x0
**Impact:** A mistyped extra operand is silently dropped, so the assembler emits a well-formed SYSTEM CSR word instead of diagnosing the line.
**Root cause:** system.rs:40-53 reads only operands[0..2] via get_reg/get_csr_num and never checks operands.len(), so any trailing operands are ignored.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/system.rs:53`
```rust
    Ok(EncodeResult::Word(encode_i(OP_SYSTEM, rd, funct3, rs1, csr as i32)))
```
**Suggested fix:** Reject anything other than exactly three operands before packing.
```rust
    if operands.len() != 3 {
        return Err("csr: expected rd, csr, rs1/zimm".to_string());
    }
```
**Bug report:** bug_reports/encode_csr_extra_operand.md
**Repro seed:** (none — deterministic extra Imm(0); proptest shrunk to csrrw x0, fflags, x0 + Imm(0))
**Raw output:**
```text
Test failed: extra operand must Err for csrrw x0, fflags, x0 (llvm-mc rejects extra operands); got Ok(Word(1052787)) at src/backend/riscv/assembler/encoder/encode_csr_pbt.rs:485.
minimal failing input: (mn, f3) = (
    "csrrw",
    1,
), rd = "x0", (csr_name, _num) = (
    "fflags",
    1,
), rs1 = "x0", extra = Imm(
    0,
)
```

### B2: encode_csr masks out-of-range zimm instead of rejecting

**Formal:** ∀ mn ∈ {csrrw,csrrs,csrrc}, rd ∈ GPRNames, csr ∈ KnownCsrNames, zimm ∉ 0..31. encode_csr([Reg(rd), Csr(csr), Imm(zimm)], funct3(mn)) = Err(_)
**Contract evidence:** inferred (llvm-mc "immediate must be an integer in the range [0, 31]"; RISC-V uimm5)
**Documentation conflict:** system.rs:45 "GNU as allows e.g. `csrrc t0, sstatus, 2` and auto-selects the immediate form." documents auto-select for a valid zimm, not wrapping of out-of-range values. Does not declare zimm=-1/32 invalid or admit a limitation.
**Severity:** high
**Counterexample:** encode_csr([Reg("x0"), Csr("fflags"), Imm(-1)], funct3=0b001)
**Expected / Actual:** Err / Ok(Word(2084979))  // 0x001FD073 = csrrwi x0, fflags, 31
**Impact:** `csrrw rd, csr, 32` is assembled as `csrrwi rd, csr, 0` and `csrrw rd, csr, -1` as `csrrwi rd, csr, 31`, silently writing the wrong immediate into a CSR.
**Root cause:** system.rs:47-49 casts the immediate to u32 and masks with 0x1F, so -1 becomes zimm 31 and 32 becomes zimm 0, then encodes the immediate CSR form.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/system.rs:49`
```rust
        let rs1 = zimm & 0x1F;
```
**Suggested fix:** Reject zimm outside 0..=31 before masking.
```rust
        let zimm = get_imm(operands, 2)?;
        if !(0..=31).contains(&zimm) {
            return Err("csr: zimm out of range 0..=31".to_string());
        }
        let rs1 = (zimm as u32) & 0x1F;
```
**Bug report:** bug_reports/encode_csr_zimm_oob.md
**Repro seed:** (none — deterministic zimm=-1; proptest shrunk to csrrw x0, fflags, -1)
**Raw output:**
```text
Test failed: zimm -1 outside 0..=31 must Err (llvm-mc uimm5); got Ok(Word(2084979)) at src/backend/riscv/assembler/encoder/encode_csr_pbt.rs:502.
minimal failing input: (_mn, f3) = (
    "csrrw",
    1,
), rd = "x0", (csr_name, _num) = (
    "fflags",
    1,
), zimm = -1
```

### B3: encode_csr truncates CSR numbers outside 0..=4095

**Formal:** ∀ mn ∈ {csrrw,csrrs,csrrc}, rd, rs1 ∈ GPRNames, csr_num ∉ 0..4095. encode_csr([Reg(rd), Imm(csr_num), Reg(rs1)], funct3(mn)) = Err(_)
**Contract evidence:** inferred (llvm-mc "immediate must be an integer in the range [0, 4095]"; RISC-V csr[11:0])
**Documentation conflict:** (none)
**Severity:** high
**Counterexample:** encode_csr([Reg("x0"), Imm(-1), Reg("x0")], funct3=0b001)
**Expected / Actual:** Err / Ok(Word(4293922931))  // 0xFFF01073 = csrrw x0, 0xfff, x0
**Impact:** `csrrw x1, 4096, x2` is assembled as CSR 0 (fflags) and `csrrw x0, -1, x0` as CSR 4095, silently targeting the wrong control/status register.
**Root cause:** get_csr_num (system.rs:66) accepts any Imm as u32 with no 12-bit range check; encode_csr then passes `csr as i32` to encode_i, which masks with 0xFFF, so -1 becomes 0xFFF and 4096 becomes 0.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/system.rs:66`
```rust
        Some(Operand::Imm(v)) => Ok(*v as u32),
```
**Suggested fix:** Reject CSR numbers outside 0..=4095 in get_csr_num.
```rust
        Some(Operand::Imm(v)) => {
            if *v < 0 || *v > 4095 {
                Err(format!("CSR number {} out of range 0..=4095", v))
            } else {
                Ok(*v as u32)
            }
        }
```
**Bug report:** bug_reports/encode_csr_csr_oob.md
**Repro seed:** (none — deterministic csr_num=-1; proptest shrunk to csrrw x0, -1, x0)
**Raw output:**
```text
Test failed: csr -1 outside 0..=4095 must Err (llvm-mc csr[11:0]); got Ok(Word(4293922931)) at src/backend/riscv/assembler/encoder/encode_csr_pbt.rs:519.
minimal failing input: (_mn, f3) = (
    "csrrw",
    1,
), rd = "x0", rs1 = "x0", csr_num = -1
```

## Design Caveats

(none)

## Test Files Created

| File | Tests |
|------|-------|
| src/backend/riscv/assembler/encoder/encode_csr_pbt.rs | 10 properties + 3 KAT + 3 regression witnesses |

## Reproduction

Whole suite:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_csr -- --test-threads=1
```

B1 extra operand:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_csr_neg_extra -- --test-threads=1
```

B2 zimm oob:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_csr_neg_zimm_oob -- --test-threads=1
```

B3 csr oob:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_csr_neg_csr_oob -- --test-threads=1
```

Build command (user contract, target swapped):
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_csr -- --test-threads=1
```

## Output Directories

- pbt-out/PLAN.md
- pbt-out/PROPERTIES.md
- pbt-out/REPORT.md
- pbt-out/REPORT.html (rendered from report.json)
- pbt-out/report.json
- pbt-out/COVERAGE.md
- pbt-out/COVERAGE_STATUS.md
- pbt-out/INVARIANTS.md
- pbt-out/FUNCTION_INDEX.md
- pbt-out/bug_reports/encode_csr_extra_operand.md
- pbt-out/bug_reports/encode_csr_extra_operand.html
- pbt-out/bug_reports/encode_csr_zimm_oob.md
- pbt-out/bug_reports/encode_csr_zimm_oob.html
- pbt-out/bug_reports/encode_csr_csr_oob.md
- pbt-out/bug_reports/encode_csr_csr_oob.html
- pbt-out/run/encode_csr_test.log
- pbt-out/run/encode_csr_test2.log
- pbt-out/run/encode_csr_test3.log
- pbt-out/run/encode_csr_test4.log

## Coverage Report

# PBT Coverage Status

> Last updated: 2026-10-06 15:53 (campaign: coverage)
> Files: 12/13 scanned (92%) | Functions: 193/330 total | PBT candidates: 193 | Tested: 193 (100%) | 1 pass, 193 fail

## Summary

| Metric | Value |
|--------|-------|
| Total source files | 13 |
| Files scanned | 12 / 13 (92%) |
| Total functions (all files) | 330 |
| PBT candidates (from FUNCTION_INDEX) | 193 |
| **Tested (of PBT candidates)** | **193 / 193 (100%)** |
| &nbsp;&nbsp;↳ Pass / Fail / Other | 1 / 193 / -1 |
| **Overall (tested / all functions)** | **193 / 330 (58%)** |
| Untested | 0 |
| Skipped | 0 |

## Module Breakdown

| Module | Scanned | Tested | Skipped | Coverage |
|--------|---------|--------|---------|----------|
|  | 193 | 193 | 0 | 100% |

## Oracle Type Distribution

| Oracle Type | Total | Covered | Skipped | Coverage |
|-------------|-------|---------|---------|----------|
| unknown | 193 | 193 | 0 | 100% |

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
