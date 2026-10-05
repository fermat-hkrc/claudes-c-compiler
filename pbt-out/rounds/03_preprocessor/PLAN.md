# PBT Campaign: src/frontend/preprocessor (round 03)

## Scan findings

- **Spec:** `src/frontend/preprocessor/README.md` — a detailed module spec documenting every
  behavior: C11 phases 2–4, macro expansion algorithm (6.10.3–6.10.3.4), stringification
  (6.10.3.2), token pasting, variadic macros, conditional stack semantics, intmax_t/uintmax_t
  `#if` arithmetic (C99 6.10.1), line-marker output contract, anti-paste guards, blue paint.
  Spec clauses extracted into PROPERTIES.md with `README.md:<section>` citations.
- **Test layout:** inline `#[cfg(test)] mod pbt_tests` modules inside source files, run via
  `cargo test --lib frontend::preprocessor` (repo convention established by
  `src/frontend/lexer/scan.rs` + `src/common/*`; no `tests/` dir exists; `pub(crate)` module
  visibility rules out integration tests). proptest 1.11.0 is a declared dev-dependency.
- **Buildability probe:** `PATH="$HOME/.cargo/bin:$PATH" cargo test --lib frontend::lexer`
  (project-owned target, unchanged) → compiles and runs: 19 passed, 9 failed — all 9 failures
  are round-02 lexer bug witnesses (B1–B5, intentionally-red regression tests), none touch the
  preprocessor. Rung 1 confirmed: extend the inline-test harness, run filtered by module path.
- **Harness placement:** rung 1 — extend the repo's own inline test convention. New
  `#[cfg(test)] mod pbt_tests` at the bottom of `src/frontend/preprocessor/{pipeline,conditionals,
  text_processing,macro_defs}.rs`. Runs from scratch CWD `pbt-out/rounds/03_preprocessor/run/`
  (round-02 invariant: keeps other modules' tests out of the process).
- **GCC differential oracle:** host `gcc 9.4.0` at /usr/bin/gcc verified with KAT probes
  (`gcc -E -P`): `-EMPTY-` → `- -`, `S(a  b)` → `"a b"`, `S("q\"t")` → `"\"q\\\"t\""`,
  `#if -1u < 0` → false. Used as reference for P8 (token-stream differential).
- **Candidate modules:** conditionals (eval_const_expr + ConditionalStack state machine),
  text_processing (join_continued_lines, strip_block_comments), macro_defs (expansion engine:
  stringify, paste, blue paint, args), pipeline (public preprocess: #if chain, line
  preservation, error/warning contracts), expr_eval (via the #if pipeline).
- **Skipped modules:** includes.rs fs-bound resolvers (`handle_include`, `resolve_include_path`,
  `read_c_source_file`, `inject_*`) — live-filesystem integration, no hermetic oracle; their
  pure helpers (`detect_include_guard`, `clean_path`, `normalize_include_path`) are exercised
  via pipeline behaviors where reachable. predefined_macros.rs / builtin_macros.rs loaders —
  static table population, fixtures only. pragmas.rs — covered via pipeline #pragma properties
  in round-03 stretch (see P10 note); core pragma dispatch exercised through P9/P10 programs.

## Module: src/frontend/preprocessor
- [x] Scan: identify targets (FUNCTION_INDEX.md: 171 fns, 68 candidates)
- [x] Plan: formalize properties (PROPERTIES.md)
- [x] Test: write and run (cargo test --lib frontend::preprocessor) — 12 pass / 3 fail (B1–B3), serially reconfirmed
- [x] Review: triage results, coverage sweep round 1 (coverage_gaps → P11 pragmas + P9b #line), REPORT.md + report.json + COVERAGE.md written; 3 bugs filed
