# PBT Campaign: src/frontend/parser (round 04)

## Scan findings

- **Spec:** `src/frontend/parser/README.md` — module spec documenting: precedence-climbing
  expression parsing with a fixed C11 precedence table (§4), the typedef/identifier
  ambiguity resolved via a live typedef table (§5), declarator "inside-out rule"
  (§6, worked example `int (*fp)(int)` → `[Pointer, FunctionPointer([int])]`), type
  specifier collection with keywords in ANY order ("long unsigned int" ==
  "unsigned long int", §7 + types.rs doc), error recovery strategy (§13), known
  limitations (§14). Spec clauses carried into PROPERTIES.md with citations.
- **Test layout:** inline `#[cfg(test)] mod pbt_tests` modules inside source files, run
  via `cargo test --lib frontend::parser` (repo convention from `src/frontend/lexer/scan.rs`,
  `src/common/*`, and round-03's preprocessor modules; no `tests/` dir; `pub(super)` /
  `pub(crate)` visibility rules out integration tests). proptest 1.11.0 is a declared
  dev-dependency (Cargo.toml).
- **Buildability probe:** (a) `PATH="$HOME/.cargo/bin:$PATH" cargo test --lib
  frontend::parser` (unchanged project test target, from scratch CWD
  `pbt-out/rounds/04_parser/run/`) → compiles clean, `0 passed; 0 failed; 568 filtered`
  — parser has NO existing tests, so rung 1 means adding the repo's own inline-test
  convention to parser files. (b) `PATH="$HOME/.cargo/bin:$PATH" cargo test --lib
  frontend::lexer` → 19 passed, 9 failed — all 9 failures are round-02 lexer bug
  witnesses (intentionally-red regressions, documented in rounds/02_lexer/REPORT.md);
  none touch the parser. Rung 1 confirmed: the inline test harness builds and runs.
- **Harness placement:** rung 1 — extend the repo's own inline test convention. New
  `#[cfg(test)] mod pbt_tests` at the bottom of `src/frontend/parser/parse.rs` (drives
  the whole Parser through its public `new`/`parse` API — expression, declarator,
  type-specifier, statement, and declaration parsing all hang off `parse()`), plus
  change-surface obligations in `src/frontend/preprocessor/text_processing.rs` (existing
  `pbt_tests` module) and `src/frontend/preprocessor/pbt_support.rs`. Runs from scratch
  CWD `pbt-out/rounds/04_parser/run/`.
- **Change surface (commit:HEAD = the round-03 archive commit `cf446c83`):** 5 changed
  functions. Coverage decisions:
  - `split_first_word` (text_processing.rs:286) — REAL production code (directive
    keyword splitting, doc comment on the fn) → property P9.
  - `is_ident_start` (pbt_support.rs:24) — test-support tokenizer helper added by
    round 03; it is the ORACLE for round-03 P8 and P10 differentials, so pinning its
    documented C11 6.4.2.1 classification + maximal-munch behavior has real value →
    property P10.
  - `new_pp` (pipeline.rs:978), `pp_output` (macro_defs.rs:1495), `spacer`
    (conditionals.rs:821) — **not SUT code**: private fixtures inside `#[cfg(test)]
    mod pbt_tests` added by the round-03 commit itself (a 3-line `Preprocessor`
    constructor, a preprocess-to-tokens wrapper, an LCG seeding `FnMut`). A property
    "for" them would be tests-of-tests with no independent oracle (their contract IS
    the round-03 tests that call them). Recorded as skipped below; they execute on
    every `cargo test --lib frontend::preprocessor` run.
- **Candidate modules:** expressions.rs (precedence climbing, C11 table), types.rs
  (specifier order-independence, struct/enum field parsing), declarators.rs (inside-out
  rule, function pointers), statements.rs (statement grammar, no-false-reject),
  parse.rs (entry point, typedef ambiguity via builtin_typedefs/typedefs, error
  recovery, ParsedDeclAttrs flag plumbing).
- **Skipped modules:** `spacer` / `new_pp` / `pp_output` — round-03 `#[cfg(test)]`
  fixtures from the archive commit itself, not SUT (see Change surface above).
  ast.rs — pure data definitions, no parsing logic (README §11). GCC attribute
  plumbing helpers (`parse_gcc_attribute_list`, `dispatch_gcc_attribute`, mode/vector
  attrs) — table-driven attribute collection, exercised only incidentally via P5/P6
  programs; a dedicated attribute property did not fit the standard-tier budget.
  `eval_const_int_expr*` family (declarations.rs) — constant evaluator shared with
  enum/bitfield paths; defer to a later round (noted in COVERAGE_STATUS.md).

## Module: src/frontend/parser
- [x] Scan: identify targets (rounds/04_parser/FUNCTION_INDEX.md: 102 fns, 26 candidates; top-level FUNCTION_INDEX.md already holds the whole-repo union incl. parser)
- [x] Plan: formalize properties (PROPERTIES.md, P1–P10)
- [x] Test: write and run (cargo test --lib frontend::parser + frontend::preprocessor::{text_processing,pbt_support}) — results in PROPERTIES.md
- [x] Review: triage, coverage_gaps sweep round 1, REPORT.md + report.json + COVERAGE.md + COVERAGE_STATUS.md written
