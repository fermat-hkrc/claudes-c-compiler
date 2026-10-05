# Bug: Integer literals larger than u64::MAX silently evaluate to 0

**Issue Synopsis**
Every integer literal path parses digits with `u64::from_str_radix(...).unwrap_or(0)` /
`text.parse::<u64>().unwrap_or(0)` (`src/frontend/lexer/scan.rs:199` hex, `:286` binary,
`:314` octal, `:371` decimal). A literal whose mathematical value exceeds 2^64−1 fails the
parse and silently becomes **0**. GCC/Clang emit "integer constant is too large for its
type" and keep the value wrapped mod 2^64 (their accumulate loop is modular); ccc has no
diagnostic channel in the lexer at all, so the collapse to 0 is silent. Note `0x10000000000000001`
(true value 2^64+1 ≡ 1 mod 2^64) lexes to `IntLiteral(0)` — wrong under every candidate
convention (mod-2^64 gives 1, saturation gives u64::MAX).

**Detection and Validation Methodology**
Property P7 (reference oracle — wraparound-mod-2^64 accumulation, the GCC convention):
proptest 1024 cases over hex/binary/octal digit strings of 1..20 digits, comparing the token
payload against `u128::from_str_radix(digits) as u64`. Shrunk counterexample:
`0x10000000000000001` → payload 0, expected 1. Decimal arm confirmed deterministically:
`18446744073709551616` (2^64) → IntLiteral(0), `99999999999999999999` (10^20) → IntLiteral(0),
`0xFFFFFFFFFFFFFFFFF` → IntLiteral(0). Re-confirmed serially (`RUST_TEST_THREADS=1`) and by
`test_lex_int_regression_u64_overflow_zero`.

**Reproduction Protocol**
```bash
cd /home/shuhao/fermat-users/leo/github/claudes-c-compiler/pbt-out/rounds/02_lexer/run
PATH="$HOME/.cargo/bin:$PATH" cargo test --lib frontend::lexer::scan::pbt_regression::test_lex_int_regression_u64_overflow_zero
# left: IntLiteral(0)  right: IntLiteral(1)
```

**Remediation Strategy**
Parse digit strings into `u128` and truncate to `u64` (equivalent to GCC's modular
accumulation), emitting a diagnostic through the compiler's error path (as GCC does) when
the value exceeds ULLONG_MAX.

**Law:** u64 payload of an integer literal ≡ its digits interpreted in the literal's base (mod 2^64).
**Impact:** oversized constants (seen in hash/mask tables, e.g. `0xFFFFFFFFFFFFFFFFF`) silently become 0.
**Function:** Lexer::lex_hex_number / lex_binary_number / lex_octal_number / lex_decimal_number
**Source location:** /home/shuhao/fermat-users/leo/github/claudes-c-compiler/src/frontend/lexer/scan.rs:199
**Detected by:** Reference (mod-2^64 wraparound convention) + determinstic decimal/hex probes
**Minimal input:** `0x10000000000000001`
**Expected:** IntLiteral(1) (value mod 2^64; GCC warns and keeps 1)
**Actual:** IntLiteral(0)
**Severity:** medium

**Reproduction:**
```bash
cd /home/shuhao/fermat-users/leo/github/claudes-c-compiler/pbt-out/rounds/02_lexer/run
PATH="$HOME/.cargo/bin:$PATH" cargo test --lib frontend::lexer::scan::pbt_regression::test_lex_int_regression_u64_overflow_zero
```
**Regression test:** src/frontend/lexer/scan.rs `pbt_regression::test_lex_int_regression_u64_overflow_zero`
