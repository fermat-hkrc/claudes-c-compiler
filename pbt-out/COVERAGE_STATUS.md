# Coverage Status — round 02 (src/frontend/lexer)

- **Coverage evidence level: file-level (symbol presence)** — the machine has neither gcovr nor lcov
  and no Rust profraw instrumentation was configured, so no line-level data exists.
- `coverage_gaps` (called once, after the first full run) reported every lexer symbol **NOT LINKED**
  in the test binary. That is a false negative for Rust inline tests: `cargo test --lib` compiles
  the inline `mod pbt_tests` into the same binary as the library, and the lexer tests demonstrably
  execute `Lexer::tokenize` & co. (20 passing + 8 failing lexer tests produce/consume `TokenKind`
  values from those symbols). Recorded verbatim per protocol; execution evidence comes from the
  test results themselves, not from symbol scans.
- Sweep round (standard tier, 1 owed): cross-checked every documented behavior in
  src/frontend/lexer/README.md against the ledger. Gaps found and closed with new properties:
  - imaginary integer/float suffix grammar (README:213-217, 245-251) → P11 + P11b (passing)
  - surrogate → U+FFFD fallback (README:283) and wide/u8/u16 string content rules
    (README:248-254) → P12 (passing)
- Scanned: 35 functions in 3 files (FUNCTION_INDEX.md); candidates: 28; excluded: 7 (trivial /
  covered-via-callers / synthetic-pragma lookups), each with a reason.
- Tested: 26 of 28 candidates exercised by at least one property (see COVERAGE.md); the 2
  remaining candidates (Lexer::new constructor, set_gnu_extensions setter) are exercised by
  every test / P2 respectively but carry no dedicated row rationale beyond that.
