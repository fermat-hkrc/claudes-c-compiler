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
| 07 | src/passes | constant_fold sub-int LShr/UDiv/URem sign-extension mis-fold | **new issue filed** (dedup: 15+ keyword combos + identifier + full-title sweep #555–#755, no hits) | [#759](https://github.com/fermat-hkrc/claudes-c-compiler/issues/759) |
| 07 | src/passes | narrow Phase 2 I64-use miscompile (README "restricted to I32/U32 output" unenforced) | **new issue filed** (same dedup protocol) | [#760](https://github.com/fermat-hkrc/claudes-c-compiler/issues/760) |
| 08 | src/backend root | parse_integer_expr debug-panic on arith overflow / shift >= 64 (Err contract) | **new issue filed** (dedup: 5 keyword combos, no hits) | [#765](https://github.com/fermat-hkrc/claudes-c-compiler/issues/765) |
| 12 | src/backend/i686 | i686 target strips arch-neutral gcc-internal include path (stddef unfindable) | **new issue filed** (dedup: 5 combos, no hits) | [#770](https://github.com/fermat-hkrc/claudes-c-compiler/issues/770) |
| 15 | src/backend/x86/assembler | packed min/max family missing (minps/maxps/minpd/maxpd) | **new issue filed** (sweep round; dedup clean) | [#782](https://github.com/fermat-hkrc/claudes-c-compiler/issues/782) |
| 16 | src/backend/arm/assembler | W/X width mixing silently accepted (incl. W mem base) | **new issue filed** (PBT witness p2b; dedup clean) | [#791](https://github.com/fermat-hkrc/claudes-c-compiler/issues/791) |
| 16 | src/backend/arm/assembler | imm ranges unvalidated (movz/movk imm16, ldur, ldp imm7×8) | **new issue filed** (p2 shrunk witness) | [#792](https://github.com/fermat-hkrc/claudes-c-compiler/issues/792) |
| 16 | src/backend/arm/assembler | shift/hw ranges unvalidated (lsl #64, ror w #32, movk w hw 32, lsl #13) | **new issue filed** (p1 shrunk witness) | [#793](https://github.com/fermat-hkrc/claudes-c-compiler/issues/793) |
| 16 | src/backend/arm/assembler | addressing forms unvalidated (ldp alignment, index scale) | **new issue filed** (double-confirmed probes) | [#794](https://github.com/fermat-hkrc/claudes-c-compiler/issues/794) |
| 16 | src/backend/arm/assembler | SP/WSP in register-file-only slots (encodes as XZR!) | **new issue filed** (p1 shrunk witness) | [#795](https://github.com/fermat-hkrc/claudes-c-compiler/issues/795) |
| 17 | src/backend/riscv/assembler | imm12 range unvalidated — addi 2048 encodes as -2048 | **new issue filed** (p_imm12 witness, bytes verified) | [#796](https://github.com/fermat-hkrc/claudes-c-compiler/issues/796) |
| 17 | src/backend/riscv/assembler | shamt range unvalidated (slli 64, slliw 32, negative) | **new issue filed** (p_shamt witness) | [#797](https://github.com/fermat-hkrc/claudes-c-compiler/issues/797) |
| 17 | src/backend/riscv/assembler | malformed forms accepted (imm in reg slot, beq raw offset, fence.i operands) | **new issue filed** (p_malformed witness) | [#798](https://github.com/fermat-hkrc/claudes-c-compiler/issues/798) |
| 19 | src/backend/i686/assembler | GPR width mismatch silently accepted (i686 path, sibling of #774) | **new issue filed** (PBT witness, double-reference) | [800](https://github.com/fermat-hkrc/claudes-c-compiler/issues/800) |
| 21 | x86 peephole | tail-call opt missed when result joins shared epilogue (1M-deep recursion segfaults at -O2 vs gcc loop) | **new issue filed** (asm-confirmed; README documents TCO) | [805](https://github.com/fermat-hkrc/claudes-c-compiler/issues/805) |
| 22 | src/backend/x86/assembler | irconst_to_bits does a NUMERIC cast, not a bitcast — signbit(-0.0) folds to | **new issue filed** (round-22 battery; dedup clean) | [806](https://github.com/fermat-hkrc/claudes-c-compiler/issues/806) |
| 22 | src/backend/x86/assembler | builtin x86 assembler encodes `movb $imm, %r8` in the 3-byte C6 form — neve | **new issue filed** (round-22 battery; dedup clean) | [807](https://github.com/fermat-hkrc/claudes-c-compiler/issues/807) |
| 23 | riscv codegen | -S output lacks .attribute arch — external assemblers reject | **new issue filed** (roundtrip oracle) | [808](https://github.com/fermat-hkrc/claudes-c-compiler/issues/808) |
| 24 | src/frontend/sema | same-scope enum redefinition accepted (C11 6.7.2.3) | **new issue filed** (gcc-differential; distinct from #560) | [812](https://github.com/fermat-hkrc/claudes-c-compiler/issues/812) |
| 26 | src/backend/x86/assembler | SSE cvt/shuffle subset gaps (cvtt family entirely, mem-source cvtsi2*/pshuf*, maskmovq/dqu) | **new issue filed** (probe matrix, double-reference) | [818](https://github.com/fermat-hkrc/claudes-c-compiler/issues/818) |
| 26 | src/backend/x86/assembler | x87 subset gaps (fst st, fcomi, fcmovcc family, fstsw/fsave aliases, ffree/fnop/fincstp/fdecstp) | **new issue filed** (51-form battery) | [819](https://github.com/fermat-hkrc/claudes-c-compiler/issues/819) |
| 26 | src/backend/x86/assembler | AVX-512: zmm entirely + all {%k} mask forms rejected (README: "initial AVX-512") | **new issue filed** (17-form battery) | [820](https://github.com/fermat-hkrc/claudes-c-compiler/issues/820) |
| 26 | src/backend/riscv/assembler | RISC-V assembler lacks part of the compressed (RVC) subset: c.lwsp/c.sw/c.be | **new issue filed** | [#821](https://github.com/fermat-hkrc/claudes-c-compiler/issues/821) |
| 26 | src/backend/x86/assembler | x86 assembler silently accepts out-of-range SSE imm8 operands (pshufd $256 / | **new issue filed** | [#822](https://github.com/fermat-hkrc/claudes-c-compiler/issues/822) |
