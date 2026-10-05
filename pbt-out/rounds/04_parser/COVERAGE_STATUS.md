# PBT Coverage Status — cumulative through round 04

**Coverage evidence level: file-level (symbol presence)** — no line-level data.
This machine has no gcovr/lcov (C/C++) and no Rust coverage instrumentation was
injected into the cargo build for this campaign (build contract:
`cargo check --lib`, target-swapped for tests). The `coverage_gaps` tool inspected
the lib-test binary and returned the file-level fallback; it flagged the five
change-surface names (spacer, pp_output, is_ident_start, new_pp,
split_first_word) as NOT LINKED — they are `#[cfg(test)]`-scoped or generic local
symbols, so presence-detection misses them; P9/P10 call split_first_word and
is_ident_start directly and pass with real assertions, so execution is certain.

## Round 04 (src/frontend/parser) — standard tier

- **Scanned:** 102 functions across 8 files (rounds/04_parser/FUNCTION_INDEX.md,
  pipe table with PBT-candidate classification; the whole-repo union index stays
  at pbt-out/FUNCTION_INDEX.md).
- **Tested (property-driven):** the full expression path (parse_expr →
  assignment → conditional → binary levels → cast → unary → postfix → primary),
  type specifier collection/resolution (all 21 valid C11 6.7.2 multisets ×
  permutations), declarator inside-out folding (bare/paren/fptr forms), the
  statement grammar (no-false-reject incl. local declarations and for-init),
  the parse entry loop under malformed/truncated/deeply-nested garbage, typedef
  context sensitivity, struct field preservation, and the two real change-surface
  functions (split_first_word, is_ident_start/tokenizer).
- **Sweep round 1 (standard tier):** executed — `coverage_gaps` returned
  file-level evidence only and flagged nothing beyond the change-surface names
  already dispositioned by P9/P10 or recorded as fixture skips. No additional
  documented parser surface left without a property; campaign closed with the
  tier's sweep budget spent (rounds done: strengthening round 1 + sweep round 1).
- **Not driven this round (recorded, not silently skipped):** GCC attribute
  plumbing (parse.rs:645-898) — incidental via P5/P5b/P6 only;
  `eval_const_int_expr*` (declarations.rs:909-1083) — deferred.
- **Bugs found in covered code:** none (round 04). Round 03's B1–B3
  (preprocessor) remain open in pbt-out/bug_reports/.

## Prior rounds (archived under pbt-out/rounds/)

- Round 01 `src/common`: 4 bugs (issues #516-518 + dup).
- Round 02 `src/frontend/lexer`: 5 bugs (issues #526-530), 9 red regression
  witnesses still intentionally failing in `cargo test --lib frontend::lexer`.
- Round 03 `src/frontend/preprocessor`: 3 bugs (issues #538-540), 3 red
  regression witnesses in `cargo test --lib frontend::preprocessor`.
