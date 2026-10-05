# Bug: Hex float literals wider than 64 bits silently evaluate to 0.0

**Issue Synopsis**
`Lexer::lex_hex_float` parses the hex-float integer and fraction parts with
`u64::from_str_radix(...).unwrap_or(0)` (`src/frontend/lexer/scan.rs:242,244`). Any C99 hex
floating constant whose integer part needs more than 16 hex digits (true value ≥ 2^64) fails
the `u64` parse and silently becomes **0.0** — no diagnostic, no approximation, a completely
wrong constant. `0x10000000000000000p0` is exactly 2^64, perfectly representable as an f64
and accepted by GCC with no warning; ccc evaluates it to `0.0`.

**Detection and Validation Methodology**
Property P6 (reference oracle, README:181 formula "value = (int_part + frac_part) * 2^exp"):
proptest 1024 cases, generator over 1..20 hex integer digits × exponent −1000..1000, asserting
a literal with a nonzero mantissa never evaluates to ±0.0. Shrunk counterexample:
`0x10000000000000000p0` → `FloatLiteral(0.0)`. Re-confirmed serially (`RUST_TEST_THREADS=1`)
and by the deterministic regression test `test_lex_hex_float_regression_wide_mantissa_zero`.

**Reproduction Protocol**
```bash
cd /home/shuhao/fermat-users/leo/github/claudes-c-compiler/pbt-out/rounds/02_lexer/run
PATH="$HOME/.cargo/bin:$PATH" cargo test --lib frontend::lexer::scan::pbt_regression::test_lex_hex_float_regression_wide_mantissa_zero
# left: FloatLiteral(0.0)  right: FloatLiteral(1.8446744073709552e19)
```

**Remediation Strategy**
Parse the hex digits into a `u128` (or accumulate digit-by-digit into an `f64` with running
scale, or reuse the existing full-precision `parse_long_double_to_f128_bytes` path used for
the `l` suffix) instead of `u64::from_str_radix(...).unwrap_or(0)`. Never substitute a silent
0 for an out-of-range parse.

**Law:** a nonzero hex floating constant must not evaluate to 0.0 (value must follow README:181).
**Impact:** every program using a large hex-float constant (e.g. exact 2^64 scale factors,
`0x1p64`, `0x10000000000000000p0`) silently compiles with 0.0 in its place — silent wrong-code.
**Function:** Lexer::lex_hex_float
**Source location:** /home/shuhao/fermat-users/leo/github/claudes-c-compiler/src/frontend/lexer/scan.rs:242
**Detected by:** Reference (README:181 hex-float formula), 1024-case proptest
**Minimal input:** `0x10000000000000000p0`
**Expected:** FloatLiteral(18446744073709551616.0) (2^64)
**Actual:** FloatLiteral(0.0)
**Severity:** high

**Reproduction:**
```bash
cd /home/shuhao/fermat-users/leo/github/claudes-c-compiler/pbt-out/rounds/02_lexer/run
PATH="$HOME/.cargo/bin:$PATH" cargo test --lib frontend::lexer::scan::pbt_regression::test_lex_hex_float_regression_wide_mantissa_zero
```
**Regression test:** src/frontend/lexer/scan.rs `pbt_regression::test_lex_hex_float_regression_wide_mantissa_zero`
