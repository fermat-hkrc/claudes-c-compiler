# Bug: Unterminated block comment leaks its last byte as a token

**Issue Synopsis**
`Lexer::skip_whitespace_and_comments`'s block-comment loop
(`src/frontend/lexer/scan.rs:110`, `while self.pos + 1 < self.input.len()`) stops one byte
short of the end of input when no closing `*/` appears. The final byte of an unterminated
block comment is therefore **not consumed** and is lexed as a real token: `int /* gone`
produces `[Int, Identifier("e"), Eof]` — the trailing `e` of the comment text becomes a
phantom identifier. A source file ending in `/* TODO` emits identifier `O`. The comment
consumed-to-EOF intent is documented (README:158-160: block comments are among what the
lexer "Ignor[es]").

**Detection and Validation Methodology**
Deterministic documented-behavior test T2 (whitespace/comment skipping contract,
README:158-160), cross-checked by P9's metamorphic separator invariance (comments are valid
separators). Witness `int /* gone` → `[Int, Identifier("e"), Eof]`, expected `[Int, Eof]`.
Re-confirmed serially (`RUST_TEST_THREADS=1`) and by
`test_lex_comment_regression_unterminated_leaks_last_byte`.

**Reproduction Protocol**
```bash
cd /home/shuhao/fermat-users/leo/github/claudes-c-compiler/pbt-out/rounds/02_lexer/run
PATH="$HOME/.cargo/bin:$PATH" cargo test --lib frontend::lexer::scan::pbt_regression::test_lex_comment_regression_unterminated_leaks_last_byte
# left: [Int, Identifier("e"), Eof]  right: [Int, Eof]
```

**Remediation Strategy**
Fix the loop bound to `while self.pos < self.input.len()` so the scan reaches the true end
of input (the `*/` detection already guards the pair read), and ideally surface a
diagnostic for the unterminated comment as GCC does.

**Law:** an (even unterminated) block comment consumes the rest of the input — no byte of it may become a token.
**Impact:** any file ending in an unterminated comment silently injects a phantom token (its last byte) into the token stream, producing confusing downstream parser errors or silently valid parses.
**Function:** Lexer::skip_whitespace_and_comments
**Source location:** /home/shuhao/fermat-users/leo/github/claudes-c-compiler/src/frontend/lexer/scan.rs:110
**Detected by:** Documented behavior (README:158-160) — deterministic test
**Minimal input:** `int /* gone`
**Expected:** [Int, Eof]
**Actual:** [Int, Identifier("e"), Eof]
**Severity:** medium

**Reproduction:**
```bash
cd /home/shuhao/fermat-users/leo/github/claudes-c-compiler/pbt-out/rounds/02_lexer/run
PATH="$HOME/.cargo/bin:$PATH" cargo test --lib frontend::lexer::scan::pbt_regression::test_lex_comment_regression_unterminated_leaks_last_byte
```
**Regression test:** src/frontend/lexer/scan.rs `pbt_regression::test_lex_comment_regression_unterminated_leaks_last_byte`
