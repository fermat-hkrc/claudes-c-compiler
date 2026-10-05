# Confirmed Invariants — ccc / src/frontend/preprocessor (round 03)

Environment quirks:
- Test layout: inline `#[cfg(test)]` modules in source files, run filtered
  `cargo test --lib frontend::preprocessor` from a scratch CWD
  (pbt-out/rounds/03_preprocessor/run/). `pub(super)` internals are reachable from
  inside the module tree; a shared `pbt_support.rs` (`#[cfg(test)]` mod in mod.rs)
  carries the oracles.
- gcc 9.4.0 (`gcc -E -P`) works as a differential oracle from inside cargo tests:
  scratch files under pbt-out/rounds/03_preprocessor/run/p8scratch (anchored via
  CARGO_MANIFEST_DIR). ~5–10 ms/case, 256 cases ≈ 2 s. Ident-chunk vocabulary must
  avoid gcc's no-underscore predefined macros (`linux`, `unix`) — restrict letters to
  a–t, or gcc expands them to 1 while the SUT leaves them verbatim.
- proptest 1.11: string-literal generators are REGEXES — `"\\"` is an invalid regex
  (use Just), `"*"` invalid (use `"[*]"`), and escape-pair classes need 8 backslashes
  in Rust source per escaped pair. prop_assert_eq! rejects inline format captures.
- Round-02 quirk still applies: failing tests in other modules (lexer witnesses) make
  `cargo test --lib` (unfiltered) exit nonzero; always filter by module path.

Confirmed invariants (1024-case runs unless noted):
- eval_const_expr agrees with an independent C99 6.10.1 intmax/uintmax evaluator over
  1024 random ASTs × random renderings (bases dec/hex/oct, u/l suffix mixes, redundant
  parens, random spacing); boundary matrix exact incl. -1u < 0 false, hex > i64::MAX
  unsigned, precedence and ternary right-associativity (P1/P1b).
- The #if multi-stage pipeline (resolve_defined → expand → resolve → idents→0 →
  evaluate) picks the reference branch for exprs over defined macros, undefined
  idents, defined()/defined X (P2, 1024 cases).
- ConditionalStack matches an independent model over 1024 random well-formed
  #if/#elif/#else/#endif trees (depth ≤ 4): per-line presence AND blank-line
  preservation both exact (P3).
- join_continued_lines == documented GCC phase-2 splice (incl. backslash+ws+newline)
  and is idempotent (P4, 1024); strip_block_comments == independent C11 phase-3
  stripper (block comment → one space, // to newline, literals verbatim, unterminated
  scans to end) and is idempotent (P5, 1024; literals verbatim P5b).
- #x stringification matches an independent C11 6.10.3.2 stringizer over idents,
  numbers, ws runs, string/char literals with escapes (P6, 1024). a##b concatenates
  RAW operands (direct, indirect XCAT, empty right operand) (P7, 1024).
- Output line count == source line count + 1 marker, __LINE__ == source line, #line N
  shifts subsequent lines by the C11 6.10.4 rule (gcc-verified: next line is N) (P9,
  P9b). #pragma pack/push/pop/weak/redefine_extname/push_macro/pop_macro all match the
  README table incl. side channels (P11, 512).

SUT observations (domain notes, not bugs; inputs are ill-formed C):
- Hex/octal literals exceeding u64::MAX parse to 0 (conditionals.rs unwrap_or(0));
  gcc warns "integer constant is too large" and differs. Unsigned-suffix letter
  combinations are accepted liberally (Uu) where gcc rejects.
- #if signed-overflow WRAPS with gcc 9.4 parity (gcc warns "integer overflow in
  preprocessor expression" and produces the same wrapped value — verified on witness
  rounds/03_preprocessor/run/w1.c); C leaves it undefined and the SUT emits no warning.
