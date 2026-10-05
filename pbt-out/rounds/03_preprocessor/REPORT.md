# PBT Campaign Report: src/frontend/preprocessor (round 03)

## Summary

**Verdict:** 3 bugs need fixing — 2 medium (`-EMPTY-` glues to `--`, changing tokens fed to the parser; `, ## __VA_ARGS__` drops a comma for a supplied-but-empty variadic argument, changing call arity) and 1 low (`#error` diagnostics carry an absolutized path instead of the input filename).
**Date:** 2026-10-05
**Repository:** /home/shuhao/fermat-users/leo/github/claudes-c-compiler
**Modules tested:** src/frontend/preprocessor (pipeline, conditionals, text_processing, macro_defs, expr_eval via pipeline, pragmas via sweep)
**Tests:** 15 (13 properties + deterministic regression/KAT witnesses)
**Result:** 12 passing, 3 failing (all 3 failures = confirmed SUT bugs B1–B3, serially reconfirmed with RUST_TEST_THREADS=1)
**Change surface:** (no change source given — whole-module campaign on src/frontend/preprocessor)
**Coverage evidence:** file-level (symbol presence) — no line-level data: this machine has neither gcovr nor lcov, and the coverage_gaps fallback probe found no preprocessor symbols in the inspected test binary (execution is nevertheless certain: the properties call these functions directly and earlier iterations panicked inside them). Sweep round 1 executed: added P11 (pragma table) and P9b (#line override) for the two documented surfaces the ledger had not yet driven.
**Tier:** standard (properties per target 13, generator runs set explicitly to ≥1024 for P1/P3/P4/P5/P6/P7/P2 and 256–512 for the gcc-spawn/derivative-heavy ones, 2 differential oracles — P8 vs gcc 9.4 and P1 vs an independent C99 6.10.1 evaluator, 1 coverage-sweep round).

## Modules Tested

| Module | Tests | Bugs | Oracles Used |
|--------|-------|------|-------------|
| conditionals.rs | P1, P1b, P3 | 0 | differential (independent C99 6.10.1 evaluator), state machine (independent stack model) |
| text_processing.rs | P4, P5, P5b | 0 | algebraic round-trip/idempotence + independent phase-2/phase-3 reference |
| macro_defs.rs | P6, P7 | 0 | reference (C11 6.10.3.2), algebraic metamorphic (## paste) |
| pipeline.rs | P2, P8, P8b, P9, P9b, P10, P11, 2 regression | 3 (B1, B2, B3) | reference, differential (gcc 9.4), invariant, negative_error |

## Bugs Found

### B1: `, ## __VA_ARGS__` deletes the comma for a supplied-but-empty variadic argument
**Formal:** ∀ program `#define VAC(fmt, ...) g(fmt, ## __VA_ARGS__)`: tokens(preprocess(`VAC(a, )`)) == tokens(gcc -E -P(`VAC(a, )`)) — falsified.
**Contract evidence:** inferred (module README's "GCC Compatibility Posture"; the GNU extension's documented example `DBG("hello")` → `fprintf(stderr, "hello")` covers only the ABSENT case; gcc 9.4 keeps the comma for the SUPPLIED-empty case, verified empirically)
**Documentation conflict:** README: "when `__VA_ARGS__` is empty and appears to the right of `##` preceded by a comma, the comma is removed" — the text does not distinguish absent from supplied-empty; gcc does. Does not declare the input invalid, so the finding stands; severity unaffected.
**Severity:** medium
**Counterexample:** `VAC(a, )` — fmt argument `a` plus one explicitly supplied empty variadic argument.
**Expected / Actual:** tokens `[g ( a , )]` / `[g ( a )]`
**Impact:** silently changes expansion arity for wrapper/generated code calling variadic macros with an explicit empty last argument.
**Root cause:** macro_defs.rs:856 — `get_va_args` (macro_defs.rs:1121) collapses "argument supplied but empty" into the same `""` as "absent", and the `##` handler deletes the comma on `va_args.is_empty()`.
**Bug report:** bug_reports/preprocessor_va_args_empty_argument_comma.md
**Repro seed:** c7181e76815e7dc40cd26a6bb5266921f7b0666afb675d8c1aa910811cb95e93
**Raw output:** `left: ["g", "(", "a", ")"], right: ["g", "(", "a", ",", ")"]` (SUT vs gcc -E -P)

### B2: `#error`/`#warning` diagnostics report an absolutized file path
**Formal:** ∀ msg: after `set_filename("t.c")`, `preprocess` with `#error msg` on active line L yields errors() == [(file="t.c", line=L, col=2)] — falsified at the file field.
**Contract evidence:** inferred (signature: `set_filename(name)` names the file; `__FILE__` in the same run expands to `"t.c"`; driver pipeline.rs:388 formats `err.file:line:col: error:` gcc-style)
**Documentation conflict:** README side-channel table promises diagnostics with "file/line/col" for "GCC-compatible `file:line:col: error:` output formatting"; the code returns the absolutized include-stack top instead. (not independently verified)
**Severity:** low
**Counterexample:** `Preprocessor::new(); set_filename("t.c"); preprocess("#error boom\n")`
**Expected / Actual:** `errors()[0].file == "t.c"` / `== "/home/shuhao/fermat-users/leo/github/claudes-c-compiler/t.c"`
**Impact:** every user-visible diagnostic prints a host-absolute path and contradicts `__FILE__` for the same line.
**Root cause:** pipeline.rs:649 `current_file()` returns `include_stack.last()`, which `set_filename` (pipeline.rs:632) pushes as `make_absolute(path)`.
**Bug report:** bug_reports/preprocessor_error_file_absolutized.md
**Repro seed:** 91fb0a610d8dda313385f440f9d9cc9053400d990e8cb6a6e687579b1df43b6
**Raw output:** `left: "/home/shuhao/fermat-users/leo/github/claudes-c-compiler/t.c", right: "t.c"`

### B3: empty object-like macro expansion glues adjacent operator tokens
**Formal:** ∀ op ∈ {-, +, /, <, =}: tokens(preprocess(`#define EMPTY` + `<op>EMPTY<op>`)) == tokens(gcc -E -P(same)) — falsified for `-EMPTY-`.
**Contract evidence:** inferred (the anti-paste guard's own documented purpose, README "would_paste_tokens inserts protective spaces between adjacent tokens that would otherwise form unintended multi-character tokens"; gcc emits `- -`)
**Documentation conflict:** (none — the guard documents the intent; macro_defs.rs:253's empty-expansion early return bypasses it)
**Severity:** medium
**Counterexample:** line `-EMPTY-` after `#define EMPTY`.
**Expected / Actual:** tokens `[-, -]` / `[--]`
**Impact:** `-` `-` becomes `--`, `+`+`+` becomes `++`, `/`+`/` becomes a `//` comment — silent token mutation in lexer input.
**Root cause:** macro_defs.rs:253 `append_with_paste_guard` returns before the trailing-edge guard when `expanded.is_empty()`.
**Bug report:** bug_reports/preprocessor_empty_macro_token_glue.md
**Repro seed:** (deterministic KAT)
**Raw output:** `assertion 'left == right' failed: line: -EMPTY-  left: ["--"] right: ["-", "-"]`

## Design Caveats (if any)

(none)

## Test Files Created

| File | Tests |
|------|-------|
| src/frontend/preprocessor/pbt_support.rs (new, #[cfg(test)]) | shared oracle support (C tokenizer, independent C99 6.10.1 evaluator, gcc -E runner) |
| src/frontend/preprocessor/conditionals.rs (inline pbt_tests) | P1, P1b, P3 |
| src/frontend/preprocessor/text_processing.rs (inline pbt_tests) | P4, P5, P5b |
| src/frontend/preprocessor/macro_defs.rs (inline pbt_tests) | P6, P7 |
| src/frontend/preprocessor/pipeline.rs (inline pbt_tests / pbt_kat / pbt_sweep / pbt_regression) | P2, P8, P8b, P9, P9b, P10, P11 + 2 regression witnesses |
| src/frontend/preprocessor/mod.rs | +2 lines (`#[cfg(test)] mod pbt_support`) |

## Reproduction

```bash
cd /home/shuhao/fermat-users/leo/github/claudes-c-compiler/pbt-out/rounds/03_preprocessor/run
PATH="$HOME/.cargo/bin:$PATH" cargo test --lib frontend::preprocessor                      # 12 pass, 3 fail
PATH="$HOME/.cargo/bin:$PATH" RUST_TEST_THREADS=1 cargo test --lib frontend::preprocessor   # serial reconfirmation
PATH="$HOME/.cargo/bin:$PATH" cargo test --lib frontend::preprocessor::pipeline::pbt_regression::test_preprocess_regression_va_args_empty_arg_keeps_comma   # B1
PATH="$HOME/.cargo/bin:$PATH" cargo test --lib frontend::preprocessor::pipeline::pbt_regression::test_preprocess_regression_error_file_field                 # B2
PATH="$HOME/.cargo/bin:$PATH" cargo test --lib frontend::preprocessor::pipeline::pbt_kat::p8b_empty_macro_paste_guard_kat                                        # B3
```
Build command (user contract, unchanged): `PATH="$HOME/.cargo/bin:$PATH" cargo check --lib` (run in /home/shuhao/fermat-users/leo/github/claudes-c-compiler) — succeeded before the campaign (rounds/03_preprocessor/build.log); the test target compiles under `cargo test --lib` with the same toolchain.

## Output Directories

- pbt-out/rounds/03_preprocessor/: PLAN.md, PROPERTIES.md, REPORT.md (round copy of this report), report.json, COVERAGE.md, COVERAGE_STATUS.md, FUNCTION_INDEX.md, INVARIANTS.md, build.log, probe.log, run/ (scratch: gcc differential cases, witnesses w1.c/l1.c/vac.c), bug_reports/ (3 .md)
- pbt-out/ (canonical, refreshed for round 03): REPORT.md (this file), REPORT.html, PROPERTIES.md, PLAN.md, COVERAGE.md, COVERAGE_STATUS.md, report.json, bug_reports/*.md + *.html

## Coverage Report

# PBT Coverage Status

> Last updated: 2026-10-05 09:17 (campaign: coverage)
> Files: 0/11 scanned (0%) | Functions: 0/171 total | PBT candidates: 0 | Tested: 0 (0%) | 0 tested

## Summary

| Metric | Value |
|--------|-------|
| Total source files | 11 |
| Files scanned | 0 / 11 (0%) |
| Total functions (all files) | 171 |
| PBT candidates (from FUNCTION_INDEX) | 0 |
| **Tested (of PBT candidates)** | **0 / 0 (0%)** |
| **Overall (tested / all functions)** | **0 / 171 (0%)** |
| Untested | 0 |
| Skipped | 0 |

## File Coverage

| Source File | Funcs | Candidates | Tested | Coverage | Status |
|-------------|-------|------------|--------|----------|--------|
| conditionals.rs | 28 | 24 | 0 | 0% | untested |
| expr_eval.rs | 8 | 2 | 0 | 0% | untested |
| includes.rs | 15 | 4 | 0 | 0% | untested |
| macro_defs.rs | 41 | 20 | 0 | 0% | untested |
| pipeline.rs | 29 | 12 | 0 | 0% | untested |
| pragmas.rs | 8 | 1 | 0 | 0% | untested |
| text_processing.rs | 8 | 5 | 0 | 0% | untested |
| builtin_macros.rs | 11 | 0 | 0 | - | excluded |
| predefined_macros.rs | 15 | 0 | 0 | - | excluded |
| utils.rs | 8 | 0 | 0 | - | excluded |

## Files Not Yet Scanned (10)

| Source File | Module |
|-------------|--------|
| builtin_macros.rs | builtin_macros.rs |
| conditionals.rs | conditionals.rs |
| expr_eval.rs | expr_eval.rs |
| includes.rs | includes.rs |
| macro_defs.rs | macro_defs.rs |
| pipeline.rs | pipeline.rs |
| pragmas.rs | pragmas.rs |
| predefined_macros.rs | predefined_macros.rs |
| text_processing.rs | text_processing.rs |
| utils.rs | utils.rs |

## Recommended Focus

> **Priority 3 — Scan uncovered files**
> 10 file(s) not yet scanned: builtin_macros.rs (1 files), conditionals.rs (1 files), expr_eval.rs (1 files), includes.rs (1 files), macro_defs.rs (1 files), pipeline.rs (1 files), pragmas.rs (1 files), predefined_macros.rs (1 files), text_processing.rs (1 files), utils.rs (1 files)
> Run `pi-pbt scan <dir>` to add them to FUNCTION_INDEX.md.
