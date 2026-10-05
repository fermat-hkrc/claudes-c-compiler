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
| 05 | src/frontend/sema | UAC ul+ll → signed LongLong | **new** | [#559](https://github.com/fermat-hkrc/claudes-c-compiler/issues/559) |
| 05 | src/frontend/sema | enum scope shadow leak | **new** | [#560](https://github.com/fermat-hkrc/claudes-c-compiler/issues/560) |
| 05 | src/frontend/sema | enum counter i64::MAX panic | **new** | [#561](https://github.com/fermat-hkrc/claudes-c-compiler/issues/561) |
| 05 | src/frontend/sema | TypeContext undo-log resurrection | **new** | [#562](https://github.com/fermat-hkrc/claudes-c-compiler/issues/562) |
| 05 | src/frontend/sema | unsigned negation no wrap | **new** | [#563](https://github.com/fermat-hkrc/claudes-c-compiler/issues/563) |
| 06 | src/ir | build_cfg duplicate preds | **new** | [#573](https://github.com/fermat-hkrc/claudes-c-compiler/issues/573) |
| 06 | src/ir | DF(entry) cycle miss | **new** | [#574](https://github.com/fermat-hkrc/claudes-c-compiler/issues/574) |
| 06 | src/ir | coerce_to U8/I8 early-return | **new** (related #3) | [#575](https://github.com/fermat-hkrc/claudes-c-compiler/issues/575) |
| 06 | src/ir | ir-local subnormal encoders | **new** (related #2/#516/#517) | [#576](https://github.com/fermat-hkrc/claudes-c-compiler/issues/576) |
| 06 | src/ir | cast_float→U8 I8(-56) repr | **found_already** — same function/law/failure as #3 | #3 |
