# PBT Campaign: src/frontend/sema (round 05)

## Scan findings

- **Spec:** `src/frontend/sema/README.md` — module spec documenting: information-gathering
  (not strict checking) design, `SemanticAnalyzer` lifecycle (new → set_diagnostics →
  analyze → take_diagnostics → into_result), `SemaResult` contents (functions /
  type_context / expr_types / const_values), `FunctionInfo` semantics (is_noreturn
  sticky, implicit decls default `Int`/`variadic:true`/`is_defined:false`), TypeContext
  undo-log scoping ("undo changes to enum_constants, struct_layouts, ctype_cache,
  typedefs, typedef_alignments" + the documented empty-layout carve-out),
  ExprTypeChecker coverage (UAC rules, shifts→promoted-left, comparisons→int,
  sizeof→ULong, enum promotion "int → unsigned int → long long → unsigned long long"),
  SemaConstEval coverage table, builtins database (`LibcAlias`/`Identity`/
  `ConstantF64`/`Intrinsic`, is_builtin atomic/sync families), -Wreturn-type rules
  (diverging stmts, infinite loops, if/else both-diverge, switch default+segments,
  noreturn calls, main exempt), and Known Limitations. Spec clauses carried into
  PROPERTIES.md with citations.
- **Test layout:** inline `#[cfg(test)] mod pbt_tests` / `mod pbt_regression` modules
  inside source files, run via `cargo test --lib` (repo convention from
  `src/common/*`, `src/frontend/lexer/scan.rs`, rounds 03/04 preprocessor+parser
  modules; no `tests/` dir; `pub(crate)` visibility rules out integration tests).
  proptest 1.11.0 is a declared dev-dependency (Cargo.toml). proptest-regressions/
  holds persisted seeds per module path.
- **Buildability probe:** `PATH="$HOME/.cargo/bin:$PATH" cargo test --lib` (unchanged
  project test target, from scratch CWD `pbt-out/rounds/05_sema/run/`) → compiles
  clean; **554 passed, 20 failed, 11 ignored**. All 20 failures are prior rounds'
  documented intentionally-red bug witnesses (round-01 `common::long_double` /
  `common::const_eval`, round-02 `frontend::lexer`, round-03
  `frontend::preprocessor`) — see rounds/02_lexer/REPORT.md and
  rounds/03_preprocessor/REPORT.md. Zero failures touch sema; sema currently has NO
  tests. Rung 1 confirmed: the inline test harness builds and runs.
- **Harness placement:** rung 1 — extend the repo's own inline test convention. New
  `#[cfg(test)] mod pbt_tests` at the bottom of `src/frontend/sema/type_checker.rs`,
  `type_context.rs`, `analysis.rs`, `const_eval.rs`, `builtins.rs`. Runs from scratch
  CWD `pbt-out/rounds/05_sema/run/`. Build via the user contract command with the
  target swapped for the test target (`cargo test --lib frontend::sema`).
- **Change surface (commit:HEAD = the round-04 archive commit `aa13cf0d`):** 3 changed
  functions, all round-04 TEST artifacts added by that commit itself. Coverage
  decisions:
  - `parse_src` (src/frontend/parser/parse.rs:1306) — test helper added by round 04
    INSIDE `#[cfg(test)] mod pbt_tests`; it wraps real production
    `Parser::new(Lexer::tokenize(src)).parse()` and carries the round's
    error-handling change (parse.rs error recovery). Covered by a NEW failure-path
    property P14 in the same module (malformed input → error_count > 0, no panic).
  - `sut_tokens` (src/frontend/preprocessor/pbt_support.rs:438) — test-support
    wrapper (`pp.preprocess` + marker strip + tokens) with its own documented
    contract ("Run the SUT preprocessor and return the token stream (markers
    stripped)"). Covered by NEW property P15 pinning that contract.
  - `p9_split_first_word_contract` (src/frontend/preprocessor/text_processing.rs:480)
    — is itself a round-04 `proptest!` property (a test, not SUT code). A property
    "for" it would be tests-of-tests with no independent oracle; its SUT
    (`split_first_word`) already carries round-04 P9. Re-executed by the probe run
    above (554-passing set). Recorded as skipped below.
- **Candidate modules:** analysis.rs (SemanticAnalyzer.analyze entry, enum variant
  processing, -Wreturn-type fall-through model, functions map, implicit decls),
  type_checker.rs (enum_constant_type GCC promotion bands, infer_binop_ctype /
  usual arithmetic conversions via ExprTypeChecker), type_context.rs (TypeContext
  undo-log scope state machine, seed_builtin_typedefs), const_eval.rs (SemaConstEval
  integer/ternary/elvis constant semantics, sizeof/_Alignof ABI table), builtins.rs
  (is_builtin atomic/sync family recognition, LibcAlias mapping,
  strip_sync_size_suffix contract).
- **Skipped modules:** `p9_split_first_word_contract` — a round-04 proptest function
  (test code, not SUT; its SUT split_first_word carries round-04 P9; re-executed in
  the probe run above). README.md — prose spec, not code. `mod.rs` — 8-line module
  re-export, no logic. builtins.rs `BUILTIN_MAP` construction (629-line static table
  literal) — data, exercised through resolve_builtin/is_builtin properties.

## Module: sema
- [x] Scan: identify targets
- [x] Plan: formalize properties (15 entries approved with IR blocks; see PROPERTIES.md)
- [x] Test: write and run (15 properties @1024 cases + 3 KAT gates + 5 regression witnesses; 10 passing, 5 failing)
- [x] Review: triage results (5 failing → SUT bugs b1-b5, serial-reconfirmed with RUST_TEST_THREADS=1; 5 bug reports + deterministic red regressions)

## Contract-surface sweep (standard tier: 1 round — done)
- `coverage_gaps` called after the first full run. No line-level coverage (no
  gcovr/lcov on this box; instrumentation inactive) — file-level symbol-presence
  evidence only. It reported the 3 change-surface symbols as NOT LINKED; that is
  a false negative of the symbol probe for `#[cfg(test)]`-nested Rust fns: P14
  calls parse_src directly, P15 calls sut_tokens directly, and round-04's
  p9_split_first_word_contract executed green in the probe run (554-passing
  baseline).
- No additional documented-behavior gap surfaced beyond the change surface (each
  linked sema function's documented behaviors carry properties in PROPERTIES.md).
  Sweep round consumed; campaign closed on "tier's rounds done".
