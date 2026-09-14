# PBT Campaign Report: encode_branch

## Summary

**Date:** 2026-09-14
**Repository:** claudes-c-compiler
**Modules tested:** encode_branch
**Tests:** 9 properties (plus 3 KAT + 3 regression witnesses)
**Result:** 6 passing, 3 bugs
**Effort tier:** standard (5–8 properties/target, ≥1000 cases, 1 coverage-driven contract-surface sweep)

## Modules Tested

| Module | Tests | Bugs | Oracles Used |
|--------|-------|------|-------------|
| encode_branch | 9 properties (6 pass, 3 fail) | 3 | differential, algebraic.invariant, algebraic.metamorphic, negative_error |

## Bugs Found

### 1. encode_branch rejects immediate PC-offset form `b #imm`
- **Law:** ∀ aligned imm in [-2^27, 2^27-4]. encode_branch([Imm(imm)]) = Word(llvm-mc("b #imm"))
- **Shrunk counterexample:** Imm(-134217728); also Imm(0), Imm(4)
- **Expected:** Word matching llvm-mc (`b #0` → 0x14000000, `b #4` → 0x14000001, `b #-134217728` → 0x16000000)
- **Actual:** Err("expected symbol at operand 0, got Some(Imm(...))") — encode_branch only calls get_symbol and never fills imm26
- **Root cause:** missing Imm encoding path; comment says imm26 is filled by linker/assembler for reloc form only
- **Impact:** gas-compat hole for hand-written `b #imm`; codegen currently emits labels so compiler output is unaffected
- **Severity:** medium
- **Fix:** match Imm, range-check alignment and ±128 MiB, return Word(0b000101<<26 | ((imm/4) as u32 & 0x03ffffff)); Err otherwise
- **Bug report:** pbt-out/bug_reports/encode_branch_imm_offset.md
- **Serial reconfirmation:** PBT_TEST_JOBS=1 reproduced

### 2. encode_branch ignores extra operands
- **Law:** B takes a single target; encode_branch([Symbol(s), extra]) must be Err
- **Shrunk counterexample:** [Symbol("labl0"), Reg("x0")]
- **Expected:** Err (llvm-mc: invalid operand)
- **Actual:** Ok(WordWithReloc Jump26) — get_symbol only inspects operand 0
- **Root cause:** no arity check
- **Impact:** typos such as `b foo, x0` silently assemble
- **Severity:** medium
- **Fix:** return Err when operands.len() != 1
- **Bug report:** pbt-out/bug_reports/encode_branch_extra_operand.md
- **Serial reconfirmation:** PBT_TEST_JOBS=1 reproduced

### 3. encode_branch accepts :lo12: / modifier operands as Jump26 symbols
- **Law:** B operand is a label or encodable integer PC offset; Modifier/ModifierOffset must be Err
- **Shrunk counterexample:** Modifier { kind: "lo12", symbol: "foo" }
- **Expected:** Err (llvm-mc does not treat `:lo12:` as B)
- **Actual:** Ok(WordWithReloc Jump26 to "foo") — get_symbol discards kind
- **Root cause:** get_symbol treats Modifier as a plain symbol
- **Impact:** wrong relocation class relative to source text
- **Severity:** medium
- **Fix:** reject Modifier/ModifierOffset in encode_branch (or in get_symbol when used for B/BL)
- **Bug report:** pbt-out/bug_reports/encode_branch_modifier.md
- **Serial reconfirmation:** PBT_TEST_JOBS=1 reproduced

## Design Caveats

Parser-misclassified Reg/Cond/Barrier names at the B target slot are treated as symbols and emit Jump26. encode_branch_symbol_misclassified confirms this.
Doc evidence: `src/backend/arm/assembler/encoder/mod.rs:982-986` — "The parser misclassifies symbol names that collide with register names, condition codes, or barrier names. These are valid symbols in context."

## Test Files Created

| File | Tests |
|------|-------|
| src/backend/arm/assembler/encoder/compare_branch.rs (mod encode_branch_pbt) | 9 properties + 3 KAT + 3 regression witnesses |

## Output Directories

- pbt-out/PLAN.md — campaign checklist
- pbt-out/PROPERTIES.md — property ledger
- pbt-out/REPORT.md — this report
- pbt-out/COVERAGE.md — per-function coverage table
- pbt-out/COVERAGE_STATUS.md — coverage statistics
- pbt-out/FUNCTION_INDEX.md — merged function index (encode_branch marked yes)
- pbt-out/INVARIANTS.md — confirmed encode_branch invariants
- pbt-out/bug_reports/encode_branch_imm_offset.md
- pbt-out/bug_reports/encode_branch_extra_operand.md
- pbt-out/bug_reports/encode_branch_modifier.md

## Contract-surface sweep

STANDARD owes 1 round. `coverage_gaps` had no LLVM profraw. Manual arm audit of get_symbol (the only branching helper encode_branch calls): Symbol/Label/SymbolOffset already covered; Modifier/ModifierOffset hit by the failing negative property; Imm/empty/Mem/Shift covered; undocumented catch-all kinds not targeted. Added encode_branch_symbol_misclassified for the documented Reg/Cond/Barrier workaround (passing). Sweep closed: tier round spent.

## Coverage Report

# PBT Coverage Status

> Last updated: 2026-09-14 01:54 (campaign: coverage)
> Files: 6/6 scanned (100%) | Functions: 12/184 total | PBT candidates: 12 | Tested: 12 (100%) | 0 pass, 12 fail

## Summary

| Metric | Value |
|--------|-------|
| Total source files | 6 |
| Files scanned | 6 / 6 (100%) |
| Total functions (all files) | 184 |
| PBT candidates (from FUNCTION_INDEX) | 12 |
| **Tested (of PBT candidates)** | **12 / 12 (100%)** |
| &nbsp;&nbsp;↳ Pass / Fail / Other | 0 / 12 / 0 |
| **Overall (tested / all functions)** | **12 / 184 (7%)** |
| Untested | 0 |
| Skipped | 0 |

## Module Breakdown

| Module | Scanned | Tested | Skipped | Coverage |
|--------|---------|--------|---------|----------|
|  | 12 | 12 | 0 | 100% |

## Oracle Type Distribution

| Oracle Type | Total | Covered | Skipped | Coverage |
|-------------|-------|---------|---------|----------|
| unknown | 12 | 12 | 0 | 100% |

## File Coverage

| Source File | Funcs | Candidates | Tested | Coverage | Status |
|-------------|-------|------------|--------|----------|--------|
| cast.rs | 6 | 1 | 1 | 100% | covered |
| compare_branch.rs | 21 | 4 | 4 | 100% | covered |
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
