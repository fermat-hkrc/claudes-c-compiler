# PBT Campaign Report: encode_fence

## Summary

**Verdict:** 1 high: encode_fence maps Imm(0) to a full iorw barrier, so `fence 0, 0` assembles as `fence iorw, iorw` (0x0ff0000f instead of 0x0000000f); plus 4 medium bugs (extra operand ignored, arity-1 defaults to full fence, out-of-order letters accepted, invalid operand kinds default to iorw).
**Date:** 2026-10-06
**Repository:** /home/toan/github/claudes-c-compiler
**Modules tested:** encode_fence
**Tests:** 8
**Result:** 3 passing, 5 bugs
**Change surface:** requested encode_fence not in base.rs; mapped to system.rs:5 (1 function, 8 properties, 4 error-path properties)
**Coverage evidence:** file-level (symbol presence) — coverage_gaps returned no .gcda/.profraw (C++ reporter listed unrelated binaries). Manual arm audit of encode_fence: empty, FenceArg pair, non-FenceArg wildcard, and len==1 else-arm all executed.
**Tier:** standard

## Modules Tested

| Module | Tests | Bugs | Oracles Used |
|--------|-------|------|-------------|
| encode_fence | 8 | 5 | differential, algebraic.metamorphic, algebraic.invariant, negative_error |

## Bugs Found

### B1: encode_fence treats Imm(0) as a full iorw barrier

**Formal:** ∀ a, b ∈ in-order-subsequences(iorw) ∪ {0}. encode_fence(op(a), op(b)) = llvm-mc("fence a, b") where op(0)=Imm(0) and op(letters)=FenceArg(letters)
**Contract evidence:** inferred (llvm-mc accepts `fence 0, 0` as 0x0000000f; parser.rs emits Imm(0) for numeric 0; encode_instruction passes operands through)
**Documentation conflict:** (none). system.rs:7 documents empty operands as fence iorw, iorw, not Imm(0).
**Severity:** high
**Counterexample:** encode_fence([Imm(0), Imm(0)])
**Expected / Actual:** 0x0000000f / 0x0ff0000f
**Impact:** Valid GNU/LLVM `fence 0, 0` and mixed `fence 0, rw` assemble as a stronger barrier than requested.
**Root cause:** system.rs:11 and system.rs:15 — any non-FenceArg, including Imm(0), is replaced with 0xF.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/system.rs:11`
```rust
            _ => 0xF,
```
**Suggested fix:** Decode Imm(0) as 0; reject other non-FenceArg kinds.
```rust
            Operand::Imm(0) => 0,
            Operand::FenceArg(s) => parse_fence_bits(s)?,
            other => return Err(format!("invalid fence operand: {:?}", other)),
```
**Bug report:** bug_reports/encode_fence_imm0_as_full_barrier.md
**Repro seed:** cc cbebc82bb67bed29b7cfeaa6f5a50554524258cf63e6ab94585cd6b2924c7fa2
**Raw output:**
```text
SUT 0ff0000f != llvm-mc 0000000f for fence 0, 0
minimal failing input: a = "0", b = "0"
```

### B2: encode_fence ignores extra operands

**Formal:** ∀ pred, succ ∈ in-order-subsequences(iorw), extra ∈ Operand. encode_fence([FenceArg(pred), FenceArg(succ), extra]) is Err
**Contract evidence:** inferred (llvm-mc `fence iorw, iorw, x0` → invalid operand; wrapper passes operands through)
**Documentation conflict:** (none)
**Severity:** medium
**Counterexample:** encode_fence([FenceArg("i"), FenceArg("i"), Imm(0)])
**Expected / Actual:** Err / Ok(Word)
**Impact:** Extra tokens after a valid fence are dropped; typos assemble.
**Root cause:** system.rs:8 `operands.len() >= 2` uses only the first two operands.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/system.rs:8`
```rust
    } else if operands.len() >= 2 {
```
**Suggested fix:** Require exactly two operands on the non-empty path.
```rust
    } else if operands.len() == 2 {
```
**Bug report:** bug_reports/encode_fence_extra_operand.md
**Repro seed:** (first example; successes: 0)
**Raw output:**
```text
extra operand must Err for fence i, i; got Ok(Word(142606351))
minimal failing input: pred = "i", succ = "i", extra = Imm(0)
```

### B3: encode_fence with one operand encodes fence iorw, iorw

**Formal:** ∀ op ∈ Operand. encode_fence([op]) is Err
**Contract evidence:** inferred (llvm-mc `fence iorw` / `fence 0` → too few operands)
**Documentation conflict:** (none). system.rs:7 documents the empty default only.
**Severity:** medium
**Counterexample:** encode_fence([FenceArg("iorw")])
**Expected / Actual:** Err / Ok(Word(0x0ff0000f))
**Impact:** A truncated `fence rw` becomes a full barrier instead of an error.
**Root cause:** system.rs:18-19 else branch (len == 1) hard-codes (0xF, 0xF) and always returns Ok.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/system.rs:19`
```rust
        (0xF, 0xF)
```
**Suggested fix:** Reject arity 1.
```rust
        return Err("fence requires 0 or 2 operands".into());
```
**Bug report:** bug_reports/encode_fence_arity_one.md
**Repro seed:** (first example; successes: 0)
**Raw output:**
```text
single operand must Err; got Ok(Word(267386895))
minimal failing input: op = FenceArg("iorw")
```

### B4: encode_fence accepts out-of-order and duplicate iorw letters

**Formal:** ∀ pred ∈ {wroi, irow, ri, wi, oi, wr, ro, wo, ii, rr, ww, IORW, I, Rw}, succ ∈ in-order-subsequences(iorw). encode_fence([FenceArg(pred), FenceArg(succ)]) is Err
**Contract evidence:** inferred (llvm-mc: letters selected in-order from iorw or be 0). Parser is_fence_arg accepts any i/o/r/w combo of length 1..=4, so wroi is caller-reachable.
**Documentation conflict:** encoder/mod.rs:442 "Parse a fence ordering string (e.g., \"iorw\") into a 4-bit mask." — asserted, not a domain restriction excluding order.
**Severity:** medium
**Counterexample:** encode_fence([FenceArg("wroi"), FenceArg("i")])
**Expected / Actual:** Err / Ok(Word) with pred=0xF
**Impact:** Mistyped fence arguments still assemble as if the letters were in order.
**Root cause:** encoder/mod.rs:446-449 parse_fence_bits uses contains() per letter.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/mod.rs:446`
```rust
    if s.contains('i') { bits |= 8; }
    if s.contains('o') { bits |= 4; }
    if s.contains('r') { bits |= 2; }
    if s.contains('w') { bits |= 1; }
```
**Suggested fix:** Accept only in-order subsequences of iorw.
```rust
fn parse_fence_bits(s: &str) -> Result<u32, String> { /* in-order scan of iorw */ }
```
**Bug report:** bug_reports/encode_fence_out_of_order_letters.md
**Repro seed:** (first example; successes: 0)
**Raw output:**
```text
out-of-order/duplicate/uppercase wroi must Err; got Ok(Word(260046863))
minimal failing input: pred = "wroi", succ = "i"
```

### B5: encode_fence maps registers and non-zero immediates to iorw

**Formal:** ∀ kind ∈ {Reg, Symbol, Csr, RoundingMode, Mem, Imm(n) where n≠0}. encode_fence([kind, FenceArg("rw")]) is Err ∧ encode_fence([FenceArg("rw"), kind]) is Err
**Contract evidence:** inferred (llvm-mc rejects `fence x0, x0` and `fence 1, 2`)
**Documentation conflict:** (none)
**Severity:** medium
**Counterexample:** encode_fence([Reg("x0"), FenceArg("rw")])
**Expected / Actual:** Err / Ok(Word) with pred=0xF
**Impact:** Invalid assembly is emitted as a real FENCE full (or mixed) barrier.
**Root cause:** system.rs:11 `_ => 0xF` wildcard (same arm as B1, different invalid kinds).
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/system.rs:11`
```rust
            _ => 0xF,
```
**Suggested fix:** Accept only FenceArg and Imm(0); Err otherwise.
```rust
            Operand::FenceArg(s) => parse_fence_bits(s)?,
            Operand::Imm(0) => 0,
            other => return Err(format!("invalid fence operand: {:?}", other)),
```
**Bug report:** bug_reports/encode_fence_invalid_operand.md
**Repro seed:** (first example; successes: 0)
**Raw output:**
```text
invalid pred Reg("x0") must Err; got Ok(Word(254803983))
minimal failing input: bad = Reg("x0")
```

## Design Caveats

(none)

## Test Files Created

| File | Tests |
|------|-------|
| src/backend/riscv/assembler/encoder/encode_fence_pbt.rs | 8 properties + 3 KAT + 5 regression witnesses |

## Reproduction

Whole suite (from the campaign run directory):
```bash
cd /home/toan/github/claudes-c-compiler/pbt-out/run
cargo test --manifest-path /home/toan/github/claudes-c-compiler/Cargo.toml --lib encode_fence -- --test-threads=1
```

B1:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_fence_imm0_diff_llvm_mc -- --test-threads=1
```

B2:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_fence_neg_extra -- --test-threads=1
```

B3:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_fence_neg_arity -- --test-threads=1
```

B4:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_fence_neg_out_of_order -- --test-threads=1
```

B5:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_fence_neg_invalid_operand -- --test-threads=1
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
- pbt-out/bug_reports/encode_fence_imm0_as_full_barrier.md
- pbt-out/bug_reports/encode_fence_imm0_as_full_barrier.html
- pbt-out/bug_reports/encode_fence_extra_operand.md
- pbt-out/bug_reports/encode_fence_extra_operand.html
- pbt-out/bug_reports/encode_fence_arity_one.md
- pbt-out/bug_reports/encode_fence_arity_one.html
- pbt-out/bug_reports/encode_fence_out_of_order_letters.md
- pbt-out/bug_reports/encode_fence_out_of_order_letters.html
- pbt-out/bug_reports/encode_fence_invalid_operand.md
- pbt-out/bug_reports/encode_fence_invalid_operand.html
- pbt-out/run/ (cargo test CWD)

## Coverage Report

# PBT Coverage Status

> Last updated: 2026-10-06 16:12 (campaign: coverage)
> Files: 12/13 scanned (92%) | Functions: 194/330 total | PBT candidates: 194 | Tested: 194 (100%) | 1 pass, 194 fail

## Summary

| Metric | Value |
|--------|-------|
| Total source files | 13 |
| Files scanned | 12 / 13 (92%) |
| Total functions (all files) | 330 |
| PBT candidates (from FUNCTION_INDEX) | 194 |
| **Tested (of PBT candidates)** | **194 / 194 (100%)** |
| &nbsp;&nbsp;↳ Pass / Fail / Other | 1 / 194 / -1 |
| **Overall (tested / all functions)** | **194 / 330 (59%)** |
| Untested | 0 |
| Skipped | 0 |

## Module Breakdown

| Module | Scanned | Tested | Skipped | Coverage |
|--------|---------|--------|---------|----------|
|  | 194 | 194 | 0 | 100% |

## Oracle Type Distribution

| Oracle Type | Total | Covered | Skipped | Coverage |
|-------------|-------|---------|---------|----------|
| unknown | 194 | 194 | 0 | 100% |

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
