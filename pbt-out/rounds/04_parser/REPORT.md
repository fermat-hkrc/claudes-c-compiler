# PBT Campaign Report: src/frontend/parser (round 04)

## Summary

**Verdict:** No bugs — 10 properties (16 test functions) all passing, serially reconfirmed; the parser's precedence climbing, specifier order-independence, declarator inside-out rule, statement grammar, typedef context-sensitivity, and crash-freedom all held under adversarial generation, and the commit's 5-function change surface is fully dispositioned (2 real properties, 3 recorded as test-fixture skips).
**Date:** 2026-10-05
**Repository:** /home/shuhao/fermat-users/leo/github/claudes-c-compiler
**Modules tested:** src/frontend/parser (parse.rs, expressions.rs, types.rs, declarators.rs, statements.rs, declarations.rs via the public `Parser::new`/`parse` driver); change-surface obligations in src/frontend/preprocessor (text_processing.rs, pbt_support.rs)
**Tests:** 17 (P1–P10 + P10b failure-branch property, KAT/witness variants)
**Result:** 17 passing, 0 bugs
**Change surface:** 5 changed functions (commit cf446c83 = the round-03 archive commit), 5 dispositioned — split_first_word → P9 (passing), is_ident_start → P10 + P10b failure-branch property (passing), spacer/new_pp/pp_output → Skipped modules (#[cfg(test)] fixtures from the archive commit itself, not SUT code; see PLAN.md); 2 marked error-handling changes covered by failure-path properties: P9's KAT + malformed-input branches, and P10b (separator bytes REJECTED by is_ident_start/is_ident_cont actively terminate identifiers in the oracle tokenizer)
**Coverage evidence:** file-level (symbol presence) — no line-level data: this machine has no gcovr/lcov and no Rust coverage instrumentation was injected; `coverage_gaps` inspected the lib-test binary and returned the file-level fallback, flagging the 5 change-surface names as NOT LINKED (they are `#[cfg(test)]`-scoped or generic local names; P9/P10 call split_first_word/is_ident_start directly and pass with real assertions, so their execution is certain). Sweep round 1 executed: no documented parser surface left undriven beyond the recorded skips.
**Tier:** standard (10 properties, run counts set explicitly: 1024 for P1–P5/P9, 512 for P4b/P6b/P7/P8/P10, 256 for P6c gcc-free cases; 2 differential/reference oracles — P1 vs an independent C11 6.5 precedence-climb reference, P3 vs C11 6.7.2p5; metamorphic round-trip P2 + context-flip P7; crash-only P6 with documented rejection chain; strengthening round executed: P6c deep-nesting + P5b local-decl/for-init added and re-run).

## Modules Tested

| Module | Tests | Bugs | Oracles Used |
|--------|-------|------|--------------|
| parse.rs / expressions.rs (driver: Parser::parse) | P1, P2, P2-sub, P5, P5b, P6, P6b, P6c, P7 | 0 | differential (independent C11 6.5 precedence-climb reference), algebraic round-trip (independent full-parenthesization renderer), invariant (well-formedness by construction), metamorphic (typedef context flip), crash-only (panic-freedom under README §13) |
| types.rs / declarations.rs | P3, P3-KAT, P8, P5b | 0 | reference (C11 6.7.2p5 specifier order-independence), invariant (struct field preservation, gcc 9.4 ground-truth probe for the array model) |
| declarators.rs | P4, P4b | 0 | reference (C inside-out reading rule, README §6 + declarators.rs:154-199 documented layout, gcc sizeof ground truth) |
| statements.rs | P5, P5b, P6* | 0 | invariant, crash-only |
| preprocessor text_processing.rs (change surface) | P9 | 0 | reference (the function's own documented contract + KATs) |
| preprocessor pbt_support.rs (change surface) | P10 ×2, P10b | 0 | reference (C11 6.4.2.1 ASCII nondigit classification, tokenizer ws-invariance/idempotence), negative_error (rejected bytes terminate identifiers) |

## Bugs Found

(none)

## Design Caveats (if any)

(none)

## Test-Run Notes (development-time test-bug fixes — not SUT findings)

Four properties failed during development; all four were TEST bugs, fixed and
re-verified (details in PROPERTIES.md "Post-run notes"): P2 Member-on-literal
rendering hazard (`876.g5` maximal-munches as a float), P5 regex emitting `+ =`
instead of the `+=` token, P4 reversed fold direction versus the documented
derived-chain convention (the SUT is C-correct — gcc-verified), P4b asserting a
pre-documentation layout for extra pointer indirections (documented layout:
declarators.rs:196-199). No property was weakened to pass: each fix corrected the
generator's rendering or aligned the assertion with the documented contract.

## Test Files Created

| File | Tests |
|------|-------|
| src/frontend/parser/parse.rs (`mod pbt_tests`, appended) | 13 test fns (P1–P8 + strengthened P5b/P6b/P6c + P3 KAT) |
| src/frontend/preprocessor/text_processing.rs (`mod pbt_round04`, appended) | 1 (P9) |
| src/frontend/preprocessor/pbt_support.rs (`mod round04_tests` + `mod round04_failure_branch`, appended) | 3 (P10 ×2, P10b) |

## Reproduction

Whole suite (from scratch CWD, as run):

```bash
cd /home/shuhao/fermat-users/leo/github/claudes-c-compiler/pbt-out/rounds/04_parser/run
PATH="$HOME/.cargo/bin:$PATH" cargo test --lib frontend::parser
PATH="$HOME/.cargo/bin:$PATH" cargo test --lib pbt_round04
PATH="$HOME/.cargo/bin:$PATH" cargo test --lib round04_tests
```

Build command (user contract, reused verbatim; target swapped for tests above):

```bash
cd /home/shuhao/fermat-users/leo/github/claudes-c-compiler
PATH="$HOME/.cargo/bin:$PATH" cargo check --lib
```

Serial reconfirmation (no failures to replay; proves suite stability):

```bash
cd /home/shuhao/fermat-users/leo/github/claudes-c-compiler/pbt-out/rounds/04_parser/run
RUST_TEST_THREADS=1 PATH="$HOME/.cargo/bin:$PATH" cargo test --lib frontend::parser
RUST_TEST_THREADS=1 PATH="$HOME/.cargo/bin:$PATH" cargo test --lib pbt_round04
RUST_TEST_THREADS=1 PATH="$HOME/.cargo/bin:$PATH" cargo test --lib round04_tests
```

gcc ground-truth probe for the P4 model:

```bash
cd /tmp && printf '#include <stdio.h>\nint main(void){ int (*a)[5]; int *b[5]; printf("%%zu %%zu\\n", sizeof(*a), sizeof(*b)); return 0; }\n' > p4g.c && gcc -o p4g p4g.c && ./p4g
# 20 8
```

## Output Directories

- pbt-out/REPORT.md — this report (round 04)
- pbt-out/REPORT.html — customer-facing overview (auto-rendered from report.json)
- pbt-out/PROPERTIES.md — property ledger P1–P10 with IR blocks and final statuses
- pbt-out/PLAN.md — campaign plan, scan findings, harness placement, skips
- pbt-out/COVERAGE.md — per-function coverage rows (round 04)
- pbt-out/COVERAGE_STATUS.md — coverage statistics + sweep evidence
- pbt-out/report.json — machine-readable report (source of truth for REPORT.html)
- pbt-out/INVARIANTS.md — confirmed invariants + environment quirks (appended round 04)
- pbt-out/FUNCTION_INDEX.md — whole-repo function index (union; parser entries predate this round)
- pbt-out/rounds/04_parser/ — round archive: FUNCTION_INDEX.md (module pipe table, 102 fns), CHANGE_SURFACE.md, change-surface.json, build.log, dependencies.json, run/ (scratch CWD incl. proptest-regressions)
- pbt-out/bug_reports/ — round-03 bugs (B1–B3) remain; round 04 added none

## Coverage Report

# PBT Coverage Status

> Last updated: 2026-10-05 09:42 (campaign: coverage)
> Files: 6/6 scanned (100%) | Functions: 12/154 total | PBT candidates: 12 | Tested: 12 (100%) | 12 tested

## Summary

| Metric | Value |
|--------|-------|
| Total source files | 6 |
| Files scanned | 6 / 6 (100%) |
| Total functions (all files) | 154 |
| PBT candidates (from FUNCTION_INDEX) | 12 |
| **Tested (of PBT candidates)** | **12 / 12 (100%)** |
| **Overall (tested / all functions)** | **12 / 154 (8%)** |
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
| parse.rs | 88 | 1 | 3 | 300% | covered |
| declarations.rs | 17 | 6 | 1 | 17% | partial |
| expressions.rs | 15 | 10 | 1 | 10% | partial |
| types.rs | 15 | 9 | 1 | 11% | partial |
| declarators.rs | 10 | 7 | 0 | 0% | untested |
| statements.rs | 9 | 0 | 1 | - | excluded |

## Files Not Yet Scanned (1)

| Source File | Module |
|-------------|--------|
| declarators.rs | declarators.rs |

## Recommended Focus

> **Priority 3 — Scan uncovered files**
> 1 file(s) not yet scanned: declarators.rs (1 files)
> Run `pi-pbt scan <dir>` to add them to FUNCTION_INDEX.md.
