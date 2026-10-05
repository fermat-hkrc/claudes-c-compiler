# Confirmed Invariants — ccc / src/frontend/lexer (round 02)

Environment quirks:
- Test layout: inline `#[cfg(test)]` modules in source files (`pub(crate)` visibility rules out
  `tests/` integration tests). Run filtered: `cargo test --lib frontend::lexer` from a scratch CWD
  (pbt-out/rounds/02_lexer/run/) so other modules' tests (which mutate TARGET_PTR_SIZE thread-local)
  never share the process. LP64 (ptr size 8) is the thread-local default.
- proptest 1.11.0: `prop_assert_eq!` messages cannot use inline format captures (`{x}`) — the macro
  concat!s the format string; pass values positionally. `return Ok(())` works inside `proptest!{}`
  bodies to skip a case.
- A stack-overflow witness ABORTS the whole test binary (SIGABRT) — keep such repros `#[ignore]`d
  and run them isolated (`--ignored --exact`), or every sibling result is lost.

Confirmed invariants (1024-case proptest runs, this campaign):
- Identifier render→lex round-trip incl. spans/file_id/terminal Eof (P1).
- from_keyword agrees with the C11 §6.4.1 + documented GCC keyword table in both gnu and strict
  modes; bare GNU words (typeof/asm) demote to identifiers in strict mode; Display prints the
  canonical quoted spelling (aliases print canonical) (P2).
- Integer literal round-trip for all u64 values × {none,u,l,ll,ul,ull} × {dec,hex,oct,bin} under
  the documented C11 §6.4.4.1 LP64 promotion table; all 7 promotion boundaries exact (P3/P4).
- Hex float value exact (bitwise) vs README:181 formula for ≤8+4 hex digits and |exp| ≤ 60 (P5);
  KAT: 0x1.8p1=3.0, 0x10p2=64.0, 0x1p-2=0.25.
- Full documented escape table incl. octal/hex truncation-to-byte, \u/\U code points, multichar
  packing with C11 6.4.4.4p10 int-typed (sign-extended) semantics — matches GCC (P8a-d).
- Whitespace/comment/line-marker insertion between tokens is stream-invariant (P9, T1, T4).
- Span sanity on arbitrary UTF-8 ≤64 chars: monotone, in-bounds, terminal Eof at (len,len) (P10).
- Integer/float suffix grammar incl. imaginary i/I/j/J before/after f/F/l/L and random u^a·l^b
  compositions classify per the documented matrix (P11/P11b).
- Surrogate \u escapes fall back to U+FFFD; L"…"/u"…"/u8"…" store code points / bytes as
  documented (P12).

Known-broken invariants (bugs B1–B5 — see REPORT.md; do NOT re-verify as "passing" until fixed):
- B1: hex float with >16-hex-digit integer part → silent 0.0 (scan.rs:242,244).
- B2: hex float exponent i32-truncation / parse-overflow → 1.0 (scan.rs:235,249).
- B3: integer literal >u64::MAX → silent 0 in all bases (scan.rs:199,286,314,371).
- B4: ~4000+ consecutive unknown multibyte chars → stack overflow abort (scan.rs:1173 recursion).
- B5: unterminated block comment leaks its last byte as a token (scan.rs:110 loop bound).

Next-round targets (src/frontend remains):
- parser (token stream → AST) once lexer bugs are fixed; preprocessor pipeline (line-marker
  emission side); sema constant-expression evaluation against the lexer's literal tokens.
- Decimal-L suffix with v > i64::MAX wraps to LongLiteral(v as i64) (negative) — README silent;
  observe against GCC before filing (candidate caveat, not yet a property).
