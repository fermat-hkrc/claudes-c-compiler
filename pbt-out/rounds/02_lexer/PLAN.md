# PBT Campaign: src/frontend/lexer (ccc C compiler — lexer)

Spec: src/frontend/lexer/README.md (module design doc, 387 lines) — documents the token model,
number-literal parsing rules (hex float formula, octal backtrack, ellipsis special case),
integer suffix grammar, C11 §6.4.4.1 promotion table, escape-sequence table, string/char
handling, line markers, and span tracking. Cited per-property as `README:<topic>`.

## Scan findings
- **Test layout:** inline Rust `#[cfg(test)] mod tests` blocks inside source files (convention in `src/common/*.rs` etc., extended by round 01 with `mod pbt_tests` + proptest). No `tests/` integration dir; lexer items are `pub(crate)` so integration tests cannot reach them. Historical `pbt-out/`, `pbt-native/` excluded from layout search.
- **Buildability probe:** user build contract `PATH="$HOME/.cargo/bin:$PATH" cargo check --lib` → green (pbt-out/rounds/02_lexer/build.log; 1 pre-existing warning in i686 peephole, unrelated). Project test runner probe: `PATH="$HOME/.cargo/bin:$PATH" cargo test --lib frontend::lexer` → runner works (0 pre-existing lexer tests). Full-suite regression check after adding tests: `cargo test --lib` → 523 passed, 14 failed, 11 ignored — the 14 failures are this campaign's 8 witnesses + round 01's 6 kept-red witnesses (common/long_double, const_eval); nothing pre-existing broke.
- **PBT framework:** proptest 1.11.0 already in `[dev-dependencies]` (project-owned since round 01). No acquisition needed.
- **Harness placement:** rung 1 — appended `#[cfg(test)] mod pbt_tests` and `mod pbt_regression` to `src/frontend/lexer/scan.rs` (same-file inline convention; `use crate::frontend::lexer::token::…` reaches token.rs items). No build-file changes; cargo discovers inline tests. Run filtered: `cargo test --lib frontend::lexer` (avoids in-process interference with other modules' tests that mutate the `TARGET_PTR_SIZE` thread-local; each test thread has its own thread-local anyway).
- **Candidate modules:** `lexer::scan` (tokenize + all lex_*/make_*/parse_* functions — the campaign target), `lexer::token` (from_keyword, Display).
- **Skipped modules:** `lexer::mod.rs` (2 re-export lines), `try_pragma_pack_token` / `try_pragma_visibility_token` (synthetic preprocessor-only identifiers, plain prefix+parse — covered by deterministic T3 anyway), `hex_digit_val` (trivial table), `Token::new/is_eof` (trivial), `peek_next`/`parse_simple_float_suffix` (covered via the paths that call them).
- **Oracle classification summary** (pbt-oracles): no state machine (Lexer is single-pass; one `tokenize` call per input; no interleaved public mutation). Strongest oracles: **reference** (README:181 hex-float formula, README:218 C11 promotion table, README:275 escape table, C11 §6.4.1 keyword list, IEEE-754 overflow semantics) and **algebraic round-trip** (identifier, int-literal render→lex), **metamorphic** whitespace/comment insertion, **crash-only** arbitrary-UTF-8 termination (justified: API has no error channel).

## Module: lexer (scan.rs + token.rs)
- [x] Scan: identify targets (FUNCTION_INDEX.md: 35 functions, 28 candidates)
- [x] Plan: formalize properties (PROPERTIES.md — P1..P13 + T-series, all with IR blocks)
- [x] Test: write and run (inline mods in scan.rs; 1024-case proptest; final: 20 passed / 8 failed / 1 ignored)
- [x] Review: triage results (5 SUT bugs B1–B5 confirmed serially, bug reports + regression tests written; 3 test bugs fixed; coverage_gaps sweep round done → P11/P12 added and passing)
