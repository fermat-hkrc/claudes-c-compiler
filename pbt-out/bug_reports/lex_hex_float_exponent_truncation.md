# Bug: Hex float exponent truncated to i32 / parse-overflow → wrong value (0x1p4294967296 = 1.0)

**Issue Synopsis**
In `Lexer::lex_hex_float` the binary exponent is parsed as `i64`
(`src/frontend/lexer/scan.rs:235`, `exp_str.parse().unwrap_or(0)`) and then used as
`2.0_f64.powi(exp as i32)` (`scan.rs:249`). Exponents ≥ 2^31 wrap when cast to `i32`
(4294967296 → 0, 2147483648 → −2147483648) and exponents longer than 19 digits fail the
`i64` parse and become 0. `0x1p4294967296` therefore evaluates to **1.0** where the true
value overflows the double range (**inf**); `0x1p-4294967296` evaluates to **1.0** where the
true value underflows to **0.0**. GCC accepts these literals (warning: floating constant
exceeds range) with values inf / 0.

**Detection and Validation Methodology**
Property P6b (reference oracle — IEEE-754 overflow/underflow semantics of the README:181
formula): fixed witness set over exponent spellings {2^31, 2^32, 2^64, 10^20, −…}. Shrunk
counterexample `0x1p4294967296` → 1.0. Re-confirmed serially (`RUST_TEST_THREADS=1`) and by
deterministic regression test `test_lex_hex_float_regression_exponent_truncation`.

**Reproduction Protocol**
```bash
cd /home/shuhao/fermat-users/leo/github/claudes-c-compiler/pbt-out/rounds/02_lexer/run
PATH="$HOME/.cargo/bin:$PATH" cargo test --lib frontend::lexer::scan::pbt_regression::test_lex_hex_float_regression_exponent_truncation
# asserts is_infinite() — got 1.0
```

**Remediation Strategy**
Keep the exponent in `i64` and scale in `f64` with overflow-aware logic (e.g. clamp the
exponent to ±1100 before `powi`, or multiply by `2f64.powi(k)` in chunks with early
inf/0 short-circuit); never let an unrepresentable exponent alias to exponent 0.

**Law:** value("0x1p" + e) must be infinite for huge positive e and 0.0 for huge negative e.
**Impact:** pathological-but-valid constants silently mis-evaluate (1.0 instead of inf/0).
**Function:** Lexer::lex_hex_float
**Source location:** /home/shuhao/fermat-users/leo/github/claudes-c-compiler/src/frontend/lexer/scan.rs:249
**Detected by:** Reference (IEEE-754 exponent semantics + README:181 formula)
**Minimal input:** `0x1p4294967296`
**Expected:** FloatLiteral(inf) — true value 2^4294967296 overflows double
**Actual:** FloatLiteral(1.0)
**Severity:** medium

**Reproduction:**
```bash
cd /home/shuhao/fermat-users/leo/github/claudes-c-compiler/pbt-out/rounds/02_lexer/run
PATH="$HOME/.cargo/bin:$PATH" cargo test --lib frontend::lexer::scan::pbt_regression::test_lex_hex_float_regression_exponent_truncation
```
**Regression test:** src/frontend/lexer/scan.rs `pbt_regression::test_lex_hex_float_regression_exponent_truncation`
