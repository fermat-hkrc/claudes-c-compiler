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
| 26 | src/backend/arm/assembler | writeback imm9 range unvalidated (ldr [x1,#504]! encodes [x1,#248]!) | **new issue filed** (byte-level boundary battery) | [#823](https://github.com/fermat-hkrc/claudes-c-compiler/issues/823) |
| 26 | src/backend/riscv/assembler | imm ranges unvalidated on 4 more paths (auipc/jalr/csrrw/sb) — #796 class systemic | **new issue filed** (boundary battery, byte evidence) | [#824](https://github.com/fermat-hkrc/claudes-c-compiler/issues/824) |
| 26 | src/backend/arm/assembler | parser accepts FP/SIMD names as GPRs across general insts (tst/cmp/mvn/mul/sdiv/and) | **new issue filed** (12-form battery; #814 class at parser level) | [#825](https://github.com/fermat-hkrc/claudes-c-compiler/issues/825) |
| 26 | src/backend/x86/assembler | parser accepts xmm in GPR slots (addq/shlq/pushq/imulq → rax) | **new issue filed** (#825 class on x86 path) | [#826](https://github.com/fermat-hkrc/claudes-c-compiler/issues/826) |
| 26 | src/backend/x86/assembler | parser accepts every non-GPR class in GPR slots (mm/cs/st + i686) | **new issue filed** (#826 class extends to all classes) | [#827](https://github.com/fermat-hkrc/claudes-c-compiler/issues/827) |
| 26 | src/backend/arm/assembler | XZR/WSP/x31 as memory base silently accepted (encodes as SP) | **new issue filed** | [#828](https://github.com/fermat-hkrc/claudes-c-compiler/issues/828) |
| 26 | src/backend/arm/assembler | cset/cneg silently accept AL condition (encodes nv slot = wrong value) | **new issue filed** (85-form cond-code sweep) | [#829](https://github.com/fermat-hkrc/claudes-c-compiler/issues/829) |
| 26 | src/backend/arm/assembler | shift ranges unvalidated on all standalone forms + lslv/lsrv/asrv/rorv missing | **new issue filed** (20-form battery) | [#830](https://github.com/fermat-hkrc/claudes-c-compiler/issues/830) |
| 26 | src/backend/arm/assembler | bfm/extr imm ranges unvalidated (immr/imms: W 0..31, X 0..63) | **new issue filed** (20-form battery) | [#831](https://github.com/fermat-hkrc/claudes-c-compiler/issues/831) |
| 26 | src/backend/arm/assembler | LSE atomics (ldadd/swp/cas) encoded without +lse arch feature gate | **new issue filed** (encodings correct; gate missing) | [#832](https://github.com/fermat-hkrc/claudes-c-compiler/issues/832) |
| 26 | src/backend/riscv/assembler | atomics encoded without +a extension gate (17 forms; encodings correct) | **new issue filed** (#832 class) | [#833](https://github.com/fermat-hkrc/claudes-c-compiler/issues/833) |
| 26 | src/backend/x86/assembler | SIB: %rsp as index silently dropped (160/1280 matrix forms; wrong address) | **new issue filed** (1280-form full SIB matrix) | [#834](https://github.com/fermat-hkrc/claudes-c-compiler/issues/834) |
| 26 | src/backend/x86/assembler | symbol-immediate to symbol-memory rejected (addq $sym, sym2; 5 forms) | **new issue filed** | [#835](https://github.com/fermat-hkrc/claudes-c-compiler/issues/835) |
| 26 | src/backend/arm/assembler | reg-offset extend forms: bare sxtw/uxtw silently dropped (7); ldr b0 byte mismatch (1) | **new issue filed** (20-form battery) | [#836](https://github.com/fermat-hkrc/claudes-c-compiler/issues/836) |
| 26 | src/backend/arm/assembler | smull X-src silently truncated; CRC32 no +crc gate; umsubl/smnegl missing | **new issue filed** (19-form battery) | [#837](https://github.com/fermat-hkrc/claudes-c-compiler/issues/837) |
| 26 | src/backend/arm/assembler | bti/rmif no arch gate; rev32-W silent X-width coercion | **new issue filed** (31-form battery) | [#838](https://github.com/fermat-hkrc/claudes-c-compiler/issues/838) |
| 26 | src/backend/arm/assembler | scalar FP: GPR-as-FP silent coercion + fcmp #0.0→NOP + fnmul/fcmpe/fcsel missing | **new issue filed** (32-form battery) | [#839](https://github.com/fermat-hkrc/claudes-c-compiler/issues/839) |
| 26 | src/backend/x86/assembler | cmpxchg reg-to-reg form rejected (mem form works) | **new issue filed** | [#840](https://github.com/fermat-hkrc/claudes-c-compiler/issues/840) |
| 26 | src/backend/x86/assembler | suffix-less movzx/movsx + cwde/cdqe rejected (8 forms; suffixed forms all byte-exact) | **new issue filed** | [#841](https://github.com/fermat-hkrc/claudes-c-compiler/issues/841) |
| 26 | src/backend/arm/assembler | ldursb/ldursh/ldursw unscaled sign-ext loads missing | **new issue filed** (21-form battery) | [#842](https://github.com/fermat-hkrc/claudes-c-compiler/issues/842) |
| 26 | src/frontend/sema | named zero-width bit-field accepted (C11 constraint violation) | **new issue filed** | [#848](https://github.com/fermat-hkrc/claudes-c-compiler/issues/848) |
| 28 | src/backend/i686/assembler | 12 system/legacy instructions missing (shrdl/xlat/enter/into/bound/arpl/lldt/sldt/str/verr/verw/lar) | **new issue filed** (dedup-clean) | [#1039](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1039) |
| 28 | src/backend/x86/assembler | lzcnt/tzcnt (ABM) + adcx/adox (ADX) missing (4 forms) | **new issue filed** (dedup-clean) | [#1040](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1040) |
| 28 | src/backend/riscv/assembler | Zbb instructions (andn/clz/rol/min etc.) without +zbb gate (15 forms) | **new issue filed** (dedup-clean) | [#1041](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1041) |
| 28 | src/backend/arm/assembler | AArch64 ldr/str h-register (FP16) encoded as d-register (64-bit) — ldr | **new issue filed** | [#1042](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1042) |
| 28 | src/backend/arm/assembler | AArch64 FP16 arithmetic encoded as FP32 — fadd h0,h1,h2 produces fadd  | **new issue filed** | [#1043](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1043) |
| 28 | src/frontend/sema | _Generic conflates signed char with char (C11 distinct types) | **new issue filed** | [#1047](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1047) |
| 28 | src/frontend/sema | _Generic: no-match+no-default accepted (C11 requires exactly one) + non-selected branches not type-checked | **new issue filed** | [#1048](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1048) |
| 28 | src/backend/x86/assembler | SSE4.2 string cmp (4) + MOVBE (2) missing | **new issue filed** | [#1051](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1051) |
| 28 | src/backend/arm/assembler | NEON ld2r/ld4r wrong encoding + contradictory ! accepted | **new issue filed** | [#1052](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1052) |
| 28 | src/backend/x86/assembler | ENTIRE FMA instruction set missing (31 forms: vfmadd/vfmsub/vfnmadd/vfnmsub × 132/213/231 × pd/ps/sd/ss) | **new issue filed** | [#1053](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1053) |
| 28 | src/backend/x86/assembler | BMI1 blsi/blsmsk/blsr + bextri missing (9 forms) | **new issue filed** | [#1054](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1054) |
