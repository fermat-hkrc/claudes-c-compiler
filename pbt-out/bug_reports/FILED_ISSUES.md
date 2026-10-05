# Filed / Deduped Issues — campaign pbt-test-v2 (whole-repo loop)

| Round | Module | Bug | Disposition | Issue |
|---|---|---|---|---|
| 01 | src/common | f64_to_f128_bytes_lossless \|x\|<1.0 underflow | **found_already** — same root cause (long_double.rs:1040) as #2; scope widened to all \|x\|<1.0, comment posted | #2 (+comment) |
| 01 | src/common | f64_to_x87_bytes_simple subnormal mis-encode | **new issue filed** | [#516](https://github.com/fermat-hkrc/claudes-c-compiler/issues/516) |
| 01 | src/common | f128_bytes_to_f64 truncates (not RNE) | **new issue filed** | [#517](https://github.com/fermat-hkrc/claudes-c-compiler/issues/517) |
| 01 | src/common | __builtin_bswap32 sign-extended I32 | **new issue filed** | [#518](https://github.com/fermat-hkrc/claudes-c-compiler/issues/518) |
| 02 | src/frontend/lexer | hex-float wide mantissa → 0.0 | **new** | [#526](https://github.com/fermat-hkrc/claudes-c-compiler/issues/526) |
| 02 | src/frontend/lexer | hex-float exponent i32 truncation | **new** | [#527](https://github.com/fermat-hkrc/claudes-c-compiler/issues/527) |
| 02 | src/frontend/lexer | 17-hex-digit int literal → 0 | **new** | [#528](https://github.com/fermat-hkrc/claudes-c-compiler/issues/528) |
| 02 | src/frontend/lexer | unknown-char recursion stack overflow (SIGABRT) | **new** | [#529](https://github.com/fermat-hkrc/claudes-c-compiler/issues/529) |
| 02 | src/frontend/lexer | unterminated comment leaks last byte token | **new** | [#530](https://github.com/fermat-hkrc/claudes-c-compiler/issues/530) |
| 03 | src/frontend/preprocessor | ## __VA_ARGS__ empty-arg comma dropped | **new** | [#538](https://github.com/fermat-hkrc/claudes-c-compiler/issues/538) |
| 03 | src/frontend/preprocessor | empty macro glues adjacent tokens (-EMPTY- → --) | **new** | [#539](https://github.com/fermat-hkrc/claudes-c-compiler/issues/539) |
| 03 | src/frontend/preprocessor | #error diagnostic file absolutized | **new** | [#540](https://github.com/fermat-hkrc/claudes-c-compiler/issues/540) |
