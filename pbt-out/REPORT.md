# PBT Campaign Report: encode_cbz

## Summary

**Date:** 2026-09-14
**Repository:** claudes-c-compiler
**Modules tested:** encode_cbz
**Tests:** 10 properties (plus 4 KAT + 4 regression witnesses)
**Result:** 7 passing, 4 bugs
**Effort tier:** standard (5–8 properties/target, ≥1000 cases, 1 coverage-driven contract-surface sweep)

## Modules Tested

| Module | Tests | Bugs | Oracles Used |
|--------|-------|------|-------------|
| encode_cbz | 10 properties (7 pass, 3 fail) | 4 | differential, algebraic.invariant, algebraic.metamorphic, negative_error |

## Bugs Found

### 1. encode_cbz rejects immediate PC-offset form `cbz/cbnz Rt, #imm`
- **Law:** ∀ GPR rt, is_nz, aligned imm in [-2^20, 2^20-4]. encode_cbz([Reg(rt), Imm(imm)], is_nz) = Word(llvm-mc("cbz/cbnz rt, #imm"))
- **Shrunk counterexample:** [Reg("x0"), Imm(-1048576)], is_nz=false; also Imm(0), Imm(4)
- **Expected:** Word matching llvm-mc (`cbz x0, #0` → 0xb4000000, `cbz w0, #0` → 0x34000000, `cbnz x0, #4` → 0xb5000020, `cbz x0, #-1048576` → 0xb4800000)
- **Actual:** Err("expected symbol at operand 1, got Some(Imm(...))") — encode_cbz only calls get_symbol for operand 1 and never fills imm19
- **Root cause:** missing Imm encoding path; comment says imm19 is filled by linker/assembler for reloc form only
- **Impact:** gas-compat hole for hand-written `cbz x0, #imm`; codegen currently emits labels so compiler output is unaffected
- **Severity:** medium
- **Fix:** match Imm at operand 1, range-check alignment and ±1 MiB, return Word((sf<<31)|(0b011010<<25)|(op<<24)|(((imm/4) as u32 & 0x7ffff)<<5)|rt); Err otherwise
- **Bug report:** pbt-out/bug_reports/encode_cbz_imm_offset.md
- **Serial reconfirmation:** PBT_TEST_JOBS=1 reproduced

### 2. encode_cbz ignores extra operands
- **Law:** CBZ/CBNZ take a register and a target; encode_cbz([Reg(rt), Symbol(s), extra], is_nz) must be Err
- **Shrunk counterexample:** [Reg("x0"), Symbol("labl0"), Reg("x1")], is_nz=false
- **Expected:** Err (llvm-mc: invalid operand)
- **Actual:** Ok(WordWithReloc CondBr19) — get_reg/get_symbol only inspect operands 0 and 1
- **Root cause:** no arity check
- **Impact:** typos such as `cbz x0, foo, x1` silently assemble
- **Severity:** medium
- **Fix:** return Err when operands.len() != 2
- **Bug report:** pbt-out/bug_reports/encode_cbz_extra_operand.md
- **Serial reconfirmation:** PBT_TEST_JOBS=1 reproduced

### 3. encode_cbz treats SP as XZR
- **Law:** ARM CBZ Rt is Wt/Xt; register 31 is XZR/WZR, not SP/WSP. llvm-mc rejects `cbz sp, L`
- **Shrunk counterexample:** [Reg("sp"), Symbol("L")], is_nz=false
- **Expected:** Err
- **Actual:** Ok(WordWithReloc { word: 0xb400001f, CondBr19, "L", 0 }) — same as `cbz xzr, L`. parse_reg_num maps sp to 31
- **Root cause:** parse_reg_num aliases sp/wsp to 31; encode_cbz does not reject SP
- **Impact:** `cbz sp, L` becomes a compare-and-branch on XZR
- **Severity:** high
- **Fix:** reject sp/wsp (and require GPR Wt/Xt/WZR/XZR/LR)
- **Bug report:** pbt-out/bug_reports/encode_cbz_sp_as_zr.md
- **Serial reconfirmation:** PBT_TEST_JOBS=1 reproduced

### 4. encode_cbz accepts FP/SIMD register names as Rt
- **Law:** ARM CBZ Rt is a GPR. llvm-mc rejects `cbz d0, L` and s/q/v/h/b prefixes
- **Minimal input:** [Reg("d0"), Symbol("L")], is_nz=false
- **Expected:** Err
- **Actual:** Ok(WordWithReloc) with sf=0 and Rt=0 — parse_reg_num accepts d/s/q/v/h/b; is_64bit_reg is false for them
- **Root cause:** no FP-register rejection in encode_cbz / get_reg
- **Impact:** `cbz d0, L` is assembled as `cbz w0, L`
- **Severity:** high
- **Fix:** reject FP/SIMD names (is_fp_reg) before encoding
- **Bug report:** pbt-out/bug_reports/encode_cbz_fp_reg.md
- **Serial reconfirmation:** PBT_TEST_JOBS=1 reproduced (property shrunk to sp; d0 confirmed by regression test)

## Design Caveats

Parser-misclassified Reg/Cond/Barrier names at the CBZ target slot are treated as symbols and emit CondBr19. encode_cbz_symbol_misclassified confirms this.
Doc evidence: `src/backend/arm/assembler/encoder/mod.rs:982-986` — "The parser misclassifies symbol names that collide with register names, condition codes, or barrier names. These are valid symbols in context."

Unaligned/out-of-range Imm currently Err because *all* Imm is rejected (see bug 1). encode_cbz_neg_imm_unaligned_oor therefore passes for the wrong reason until Imm encoding is added; the valid-Imm contract is the failing differential, not this caveat.

llvm-mc accepted `cbz x0, :lo12:foo` as a branch19 fixup; no Modifier negative-error was filed (unlike encode_branch, where llvm-mc rejected `:lo12:`).

## Test Files Created

| File | Tests |
|------|-------|
| src/backend/arm/assembler/encoder/compare_branch.rs (mod encode_cbz_pbt) | 10 properties + 4 KAT + 4 regression witnesses |

## Output Directories

- pbt-out/PLAN.md — campaign checklist
- pbt-out/PROPERTIES.md — property ledger
- pbt-out/REPORT.md — this report
- pbt-out/COVERAGE.md — per-function coverage table
- pbt-out/COVERAGE_STATUS.md — coverage statistics
- pbt-out/FUNCTION_INDEX.md — merged function index (encode_cbz marked yes)
- pbt-out/INVARIANTS.md — confirmed encode_cbz invariants
- pbt-out/bug_reports/encode_cbz_imm_offset.md
- pbt-out/bug_reports/encode_cbz_extra_operand.md
- pbt-out/bug_reports/encode_cbz_sp_as_zr.md
- pbt-out/bug_reports/encode_cbz_fp_reg.md

## Contract-surface sweep

Tier `standard` owes 1 coverage-driven round. `coverage_gaps` had no LLVM profraw in this session (same environment quirk as prior campaigns). Sweep was a manual arm audit of get_symbol:

- Reg/Cond/Barrier parser-misclassification arms → encode_cbz_symbol_misclassified (passing)
- other-kind arm (Mem/Shift/Extend/RegArrangement/Expr/RegList) → encode_cbz_neg_bad_label_kind (passing)

Close reason: the tier's 1 sweep round is done. Documented get_symbol behaviors now have properties. Remaining documented Imm/arity/Rt contracts are the failing properties above.

Skipped target: (none). Built and tested the real `encode_cbz` via `cargo test --lib`.

## Coverage Report

# PBT Coverage Status

> Last updated: 2026-09-14 02:06 (campaign: coverage)
> Files: 6/6 scanned (100%) | Functions: 13/184 total | PBT candidates: 13 | Tested: 13 (100%) | 0 pass, 13 fail

## Summary

| Metric | Value |
|--------|-------|
| Total source files | 6 |
| Files scanned | 6 / 6 (100%) |
| Total functions (all files) | 184 |
| PBT candidates (from FUNCTION_INDEX) | 13 |
| **Tested (of PBT candidates)** | **13 / 13 (100%)** |
| &nbsp;&nbsp;↳ Pass / Fail / Other | 0 / 13 / 0 |
| **Overall (tested / all functions)** | **13 / 184 (7%)** |
| Untested | 0 |
| Skipped | 0 |

## Module Breakdown

| Module | Scanned | Tested | Skipped | Coverage |
|--------|---------|--------|---------|----------|
|  | 13 | 13 | 0 | 100% |

## Oracle Type Distribution

| Oracle Type | Total | Covered | Skipped | Coverage |
|-------------|-------|---------|---------|----------|
| unknown | 13 | 13 | 0 | 100% |

## File Coverage

| Source File | Funcs | Candidates | Tested | Coverage | Status |
|-------------|-------|------------|--------|----------|--------|
| cast.rs | 6 | 1 | 1 | 100% | covered |
| compare_branch.rs | 21 | 5 | 5 | 100% | covered |
| constants.rs | 34 | 1 | 1 | 100% | covered |
| data_processing.rs | 36 | 4 | 4 | 100% | covered |
| load_store.rs | 20 | 1 | 1 | 100% | covered |
| neon.rs | 68 | 1 | 1 | 100% | covered |

## Recommended Focus

> **Priority 1 — Fix failing tests**
> These functions have failing PBT properties — fix before adding new tests.

| Function | Source |
|----------|--------|
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
