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
| 28 | src/backend/x86/assembler | CET/shadow-stack (endbr64/wrssd/etc.) missing | **new issue filed** | [#1055](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1055) |
| 28 | src/backend/arm/assembler | NEON fmaxv/fminv/fmaxnmv/fminnmv + smov missing (5 GAPs) | **new issue filed** | [#1056](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1056) |
| 28 | src/backend/arm/assembler | NEON fcmp-vs-zero: fcmeq/fcmgt → NOP; fcmlt/fcmle wrong; sqshlu/cmls missing | **new issue filed** | [#1057](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1057) |
| 28 | src/backend/arm/assembler | NEON fneg v.4s bit 23 clear (wrong instruction) | **new issue filed** | [#1062](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1062) |
| 28 | src/backend/arm/assembler | prfm register-index encoding wrong (all indexed forms; bare form correct) | **new issue filed** (encoding, not validation #345-347) | [#1063](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1063) |
| 28 | src/backend/arm/assembler | ALL fmov Rd,#imm encode as NOP (10 forms; FP reg never set) | **new issue filed** (HIGH) | [#1064](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1064) |
| 28 | src/backend/riscv/assembler | fcvt.d.s/fcvt.d.w wrong rs2 encoding (2 forms; 34/36 clean) | **new issue filed** | [#1068](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1068) |
| 30 | src/backend/arm/assembler/encoder | encode_shift panic on shift >= width (u32 underflow) | **new issue filed** | [#1073](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1073) |
| 30 | src/backend/arm/assembler/encoder | encode_ldxr_stxr silently encodes imm9≠0 for LDXR with offset | **new issue filed** | [#1075](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1075) |
| 30 | src/backend/arm/assembler/encoder | encode_ldar_stlr silently encodes imm9≠0 for LDAR/STLR with offset | **new issue filed** | [#1077](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1077) |
| 30 | src/backend/arm/assembler/encoder | encode_ldaxr_stlxr LDAXR imm9≠0 with offset (3rd instance of exclusive-load offset pattern) | **new issue filed** | [#1078](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1078) |
| 30 | src/backend/arm/assembler/encoder | encode_ubfiz panic on width=0 (u32 underflow, same pattern as #1073) | **new issue filed** | [#1080](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1080) |
| 31 | src/common/long_double.rs | f128_bytes_to_x87_bytes truncates 49 bits — long double literals 1 ulp low on x86 (0.1L/pi misprint) | **new issue filed** | [#1084](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1084) |
| 31 | src/ir/lowering/global_init.rs | compound literal nested-array rows drop elements 2..n (backing [1,0,4,0,7,0]) | **new issue filed** | [#1087](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1087) |
| 32 | src/frontend/preprocessor/expr_eval.rs | #if accepts malformed exprs silently, 8/13 wrong branch | **new issue filed** | [#1088](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1088) |
| 32 | src/frontend/preprocessor/macro_defs.rs | __VA_OPT__ emitted as literal tokens (not expanded) | **new issue filed** | [#1089](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1089) |
| 35 | src/backend/linker_common | linked output drops .symtab/.strtab (nm "no symbols", no enabling flag) | **new issue filed** | [#1092](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1092) |
| 37 | src/backend/riscv/assembler/encoder/pseudo.rs | encode_bgt silently ignores extra operands | **new issue filed** | [#1095](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1095) |
| 37 | src/backend/riscv/assembler/encoder/pseudo.rs | encode_ble silently ignores extra operands | **new issue filed** | [#1096](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1096) |
| 37 | src/backend/riscv/assembler/encoder/pseudo.rs | encode_bgtu silently ignores extra operands | **new issue filed** | [#1097](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1097) |
| 37 | src/backend/riscv/assembler/encoder/pseudo.rs | encode_bleu silently ignores extra operands | **new issue filed** | [#1098](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1098) |
| 37 | src/backend/riscv/assembler/encoder/pseudo.rs | encode_bltz silently ignores extra operands | **new issue filed** | [#1099](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1099) |
| 37 | src/backend/riscv/assembler/encoder/pseudo.rs | encode_bgtz silently ignores extra operands | **new issue filed** | [#1100](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1100) |
| 37 | src/backend/riscv/assembler/encoder/pseudo.rs | encode_csrr silently ignores extra operands | **new issue filed** | [#1101](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1101) |
| 37 | src/backend/riscv/assembler/encoder/pseudo.rs | encode_csrw silently ignores extra operands | **new issue filed** | [#1102](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1102) |
| 37 | src/backend/riscv/assembler/encoder/pseudo.rs | encode_csrs silently ignores extra operands | **new issue filed** | [#1103](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1103) |
| 37 | src/backend/riscv/assembler/encoder/pseudo.rs | encode_csrc silently ignores extra operands | **new issue filed** | [#1104](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1104) |
| 37 | src/backend/riscv/assembler/encoder/pseudo.rs | encode_fmv_s silently ignores extra operands | **new issue filed** | [#1105](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1105) |
| 37 | src/backend/riscv/assembler/encoder/pseudo.rs | encode_fmv_d silently ignores extra operands | **new issue filed** | [#1106](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1106) |
| 37 | src/backend/riscv/assembler/encoder/pseudo.rs | encode_fabs_s silently ignores extra operands | **new issue filed** | [#1107](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1107) |
| 37 | src/backend/riscv/assembler/encoder/pseudo.rs | encode_fabs_d silently ignores extra operands | **new issue filed** | [#1108](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1108) |
| 37 | src/backend/riscv/assembler/encoder/pseudo.rs | encode_fneg_s silently ignores extra operands | **new issue filed** | [#1109](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1109) |
| 37 | src/backend/riscv/assembler/encoder/pseudo.rs | encode_fneg_d silently ignores extra operands | **new issue filed** | [#1110](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1110) |
| 37 | src/backend/riscv/assembler/encoder/pseudo.rs | encode_la silently ignores extra operands | **new issue filed** | [#1111](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1111) |
| 37 | src/backend/riscv/assembler/encoder/pseudo.rs | encode_lla silently ignores extra operands | **new issue filed** | [#1112](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1112) |
| 38 | src/backend/x86/codegen/returns.rs | SSE-class ≤8B struct returns in RAX not XMM0 — gcc interop garbage | **new issue filed** | [#1114](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1114) |
| 38 | src/passes (volatile storage classification) | volatile locals constant-folded, only final store survives at -O2 | **new issue filed** | [#1115](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1115) |
| 38 | src/backend/call_abi.rs + x86 codegen | X87-class {long double} struct: return RAX:RDX (need st(0)), args RDI:RSI (need memory) | **new issue filed** | [#1116](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1116) |
| 38 | src/backend/call_abi.rs + ARM codegen | ARM64 HFA structs in X0/W0 integer regs (need S/V) — 5/5 interop directions BAD | **new issue filed** | [#1118](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1118) |
| 38 | src/backend/riscv codegen return path | RISC-V FP-struct returns in integer a0 (need fa0/fa1; {f,f} drops b) | **new issue filed** | [#1119](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1119) |
| 39 | src/backend/riscv/assembler/encoder | encode_zbb_unary silently ignores extra operands | **new issue filed** | [#1122](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1122) |
| 39 | src/backend/riscv/assembler/encoder | encode_zbb_unary_f5 silently ignores extra operands | **new issue filed** | [#1123](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1123) |
| 39 | src/backend/riscv/assembler/encoder | encode_zbb_unary_w silently ignores extra operands | **new issue filed** | [#1124](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1124) |
| 39 | src/backend/riscv/assembler/encoder | encode_zbb_zexth silently ignores extra operands | **new issue filed** | [#1125](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1125) |
| 39 | src/backend/riscv/assembler/encoder | encode_shift_imm silently ignores extra operands | **new issue filed** | [#1126](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1126) |
| 39 | src/backend/riscv/assembler/encoder | encode_shift_imm_w silently ignores extra operands | **new issue filed** | [#1127](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1127) |
| 39 | src/backend/riscv/assembler/encoder | encode_amo_suffixed silently ignores extra operands | **new issue filed** | [#1128](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1128) |
| 39 | src/backend/riscv/assembler/encoder | encode_lr_suffixed silently ignores extra operands | **new issue filed** | [#1129](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1129) |
| 39 | src/backend/riscv/assembler/encoder | encode_sc_suffixed silently ignores extra operands | **new issue filed** | [#1130](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1130) |
| 39 | src/backend/arm/assembler/encoder/system.rs | dmb/dsb accept empty+wrong-type operands → silent sy barrier | **new issue filed** | [#1131](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1131) |
| 40 | src/frontend (sema/parser) | same-TU inline+non-inline redefinition accepted; inline body silently wins | **new issue filed** | [#1133](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1133) |
| 40 | src/ir/lowering + sema | & on bit-field accepted (C11 6.5.3.2p1 violation); gcc/clang reject | **new issue filed** | [#1134](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1134) |
| 40 | src/frontend/sema + lowering | assignment/++/-- targets are never validated as modifiable lvalues — 7 | **new issue filed** | [#1135](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1135) |
| 40 | src/frontend/sema + lowering | sizeof on a bit-field accepted (sizeof(s.a)) — C11 6.5.3.4p1 constrain | **new issue filed** | [#1136](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1136) |
| 40 | src/frontend parser+sema | statement-placement constraints absent — break/continue/case/default o | **new issue filed** | [#1137](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1137) |
| 40 | src/frontend parser+sema | declaration constraints absent — same-scope var redefinition, void var | **new issue filed** | [#1138](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1138) |
| 40 | src/frontend/sema | expression operand-type constraints absent — *deref non-pointer, subscript  | **new issue filed** | [#1139](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1139) |
| 40 | src/frontend/sema | declaration/qualifier constraints absent — static init with non-constant SI | **new issue filed** | [#1140](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1140) |
| 40 | src/frontend/sema | bit-field declaration constraints absent — negative width (a:-1), width 99  | **new issue filed** | [#1141](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1141) |
| 40 | src/frontend/sema | linkage/label constraints absent — static-after-nonstatic decl, int-then-do | **new issue filed** | [#1142](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1142) |
| 40 | src/frontend/preprocessor | directive structure constraints absent (6 forms) | **new issue filed** | [#1143](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1143) |
| 40 | src/ir/lowering expr_sizeof/fold | sizeof(VLA) folds to 1024 in arithmetic — sizeof(v)/sizeof(v[0])=256 | **new issue filed** | [#1144](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1144) |
| 41 | compiler runtime (linker_common) | long double fails to LINK on ARM64/RISC-V (__addtf3 family undefined) | **new issue filed** | [#1145](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1145) |
| 41 | src/frontend/sema | wave-6 type constraints (volatile fn, void* deref, incomplete member, void[], wide-init, folded dup case) | **new issue filed** | [#1146](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1146) |
| 41 | src/frontend typedef/array lowering | typedef-to-VLA crashes (SIGSEGV); sizeof 4 vs 20 | **new issue filed** | [#1147](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1147) |
| 41 | src/frontend/sema | VLA placement: struct member + block-static VLA accepted | **new issue filed** | [#1148](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1148) |
| 41 | src/frontend/sema | wave-8: array cast, self-containing struct, enum-fwd object (3 forms) | **new issue filed** | [#1149](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1149) |
| 42 | src/frontend parser/sema | nested-fn support inverted (plain rejected, static accepted) | **new issue filed** | [#1151](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1151) |
| 42 | symbol emitter | asm-label rename splits call/def symbol names → unlink | **new issue filed** | [#1152](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1152) |
| 43 | init_array emission | constructor(N) priorities ignored — declaration order instead | **new issue filed** | [#1153](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1153) |
| 43 | src/frontend/preprocessor/expr_eval.rs | #if operand-type constraints (float/str/cast/pp-number, 5 forms) | **new issue filed** | [#1154](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1154) |
| 45 | src/frontend/lexer | empty hex escape \\x accepted (gcc/clang error) | **new issue filed** | [#1156](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1156) |
| 45 | src/driver | -U does not cancel preceding -D (X stays defined) | **new issue filed** | [#1157](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1157) |
| 45 | src/driver CLI | function-like -D unsupported (implicit-decl warning) | **new issue filed** | [#1158](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1158) |
| 45 | src/frontend/preprocessor/includes.rs | self-include builds silently (no depth diagnostic) | **new issue filed** | [#1159](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1159) |
| 45 | driver -E -C | -C ignored, comments stripped | **new issue filed** | [#1160](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1160) |
| 45 | driver -M | -M omits included headers | **new issue filed** | [#1161](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1161) |
| 45 | driver -MF | dep rule to stdout, no file | **new issue filed** | [#1162](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1162) |
| 45 | driver/backend -g | zero DWARF emitted (flag no-op) | **new issue filed** | [#1163](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1163) |
| 45 | driver include paths | CPATH/C_INCLUDE_PATH ignored | **new issue filed** | [#1164](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1164) |
| 45 | linker args | -Wl,-Map silently ignored (no map file) | **new issue filed** | [#1165](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1165) |
| 46 | linker_common/symbols.rs | duplicate strong symbols link silently (data+function) | **new issue filed** | [#1166](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1166) |
| 46 | riscv encoder pseudo | encode_call silently ignores extra operands | **new issue filed** | [#1167](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1167) |
| 46 | riscv encoder pseudo | encode_tail silently ignores extra operands | **new issue filed** | [#1168](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1168) |
| 46 | riscv encoder pseudo | encode_j_pseudo silently ignores extra operands | **new issue filed** | [#1169](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1169) |
| 46 | riscv encoder pseudo | encode_jr silently ignores extra operands | **new issue filed** | [#1170](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1170) |
| 46 | riscv encoder pseudo | encode_jump silently ignores extra operands | **new issue filed** | [#1171](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1171) |
| 47 | lexer | digraphs unsupported (C95 %: <%% %%> <: :>) | **new issue filed** | [#1172](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1172) |
| 48 | sema enum handling | enum values >int truncated to 0 (4-byte forced) | **new issue filed** | [#1173](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1173) |
| 48 | riscv encoder pseudo | encode_rdcsr ignores extra operands (last unclassified fn) | **new issue filed** | [#1174](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1174) |
| 51 | lexer | octal 8/9 digits accepted as decimal (018→18) | **new issue filed** | [#1175](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1175) |
| 51 | lexer | duplicate suffixes (luL/uu/lll) accepted | **new issue filed** | [#1176](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1176) |
| 51 | parser | contradictory specifiers (5 forms) | **new issue filed** | [#1177](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1177) |
| 51 | lexer | empty char constant accepted (value 0) | **new issue filed** | [#1178](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1178) |
| 51 | lexer | float 1e empty exponent accepted as 0 | **new issue filed** | [#1179](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1179) |
| 51 | lexer | 0x and 0x1p empty hex/exponent accepted (values 0/1) | **new issue filed** | [#1180](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1180) |
| 51 | x86 codegen return path | no TCO — 1M tail recursion SIGSEGV vs gcc returns normally | **new issue filed** | [#1181](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1181) |
| 51 | diagnostics | -Wall missing 5 warning classes (uninit/unused×2/shadow/paren) | **new issue filed** | [#1182](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1182) |
| 52 | src/common/long_double.rs:1040 | i686 ICE: float→long double panics (subtract overflow) | **new issue filed** | [#1183](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1183) |
| 53 | arm encoder bitfield.rs | bfi/bfxil/sbfiz/sbfx width=0 → dup #379/#384/#436/#456 (closed #1184) | **found_already** | [#379](https://github.com/fermat-hkrc/claudes-c-compiler/issues/379) |
| 54 | ALL backends assembler/ELF writer | non-deterministic .o (.s identical, .o differs) | **new issue filed** | [#1185](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1185) |
| 55 | pp macro expansion | 2500-deep chain SIGABRT (stack overflow) | **new issue filed** | [#1186](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1186) |
| 56 | pp includes.rs | 100-file circular include HANGS (DoS) | **new issue filed** | [#1187](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1187) |
| 56 | pp macro_defs.rs | ## at macro end accepted (C11 6.10.3.3p1) | **new issue filed** | [#1188](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1188) |
| 56 | sema type_checker | fn-call arg count + void-return constraints (4 forms) | **new issue filed** | [#1189](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1189) |
| 57 | optimizer inline pass | noinline attribute ignored (inlined anyway) | **new issue filed** | [#1190](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1190) |
| 57 | lexer identifier | 28 control chars accepted as identifiers | **new issue filed** | [#1191](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1191) |
| 57 | lexer escape | incomplete \\U accepted | **new issue filed** | [#1192](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1192) |
| 57 | lexer comment | unterminated /* silently consumes file | **new issue filed** | [#1193](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1193) |
| 57 | lexer string | mixed narrow/wide concat rejected (GNU ext) | **new issue filed** | [#1194](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1194) |
| 57 | pp macro_defs | # not followed by parameter | **new issue filed** | [#1195](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1195) |
| 57 | pp #line | #line -1 accepted | **new issue filed** | [#1196](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1196) |
| 57 | pp expr/directives | 5 constraints: div-zero in #if, digit-name, undef-empty, double-else | **new issue filed** | [#1197](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1197) |
| 59 | linker symbol resolution | tentative beats strong when tentative file first (order-dependent) | **new issue filed** | [#1203](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1203) |
| 59 | asm_expr | "m" constraint produces 0 | **new issue filed** | [#1204](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1204) |
| 59 | traits.rs:347 | int & float bitAND panics (ICE) | **new issue filed** | [#1205](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1205) |
| 59 | traits.rs | 6 type-mismatch operator violations (struct+int/ptr&int/float%int/~float/struct==) | **new issue filed** | [#1206](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1206) |
| 60 | sema case/init | case float/string + desig out-of-bounds | **new** | [#1207](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1207) |
| 60 | sema lvalue/call | 4 more: const-arr write, enum-assign, call-var, fn-assign | **new** | [#1209](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1209) |
| 60 | sema unary & | &5 and &enum_const accepted → SIGSEGV | **new** | [#1210](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1210) |
| 60 | sema cast | (void)5 as value accepted | **new** | [#1211](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1211) |
| 61 | sema incomplete | FAM not last + sizeof incomplete array | **new** | [#1212](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1212) |
| 61 | sema align/decl | _Alignas(3) + array of functions | **new** | [#1213](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1213) |
| 61 | preprocessor | extra macro args + bare defined + #if 1/0 | **new** | [#1214](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1214) |
| 61 | sema register | & of register variable | **new** | [#1215](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1215) |
| 61 | sema prototype | int f(void) then int f(int) | **new** | [#1216](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1216) |
| 61 | lexer scan.rs:837 | long hex escape ICE (u32 mul overflow) | **new** | [#1217](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1217) |
| 61 | lexer UCN/ident | invalid UCN + raw UTF-8 ident accepted | **new** | [#1218](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1218) |
| 61 | lexer | empty character constant '' accepted | **new** | [#1219](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1219) |
| 61 | lexer | incomplete float exponent 1e+ accepted | **new** | [#1220](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1220) |
| 61 | vector compare | GNU vector == != < > <= >= SIGSEGV | **new** | [#1221](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1221) |
| 61 | vector | GNU vector unary - / ~ SIGSEGV at runtime | **new** | [#1222](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1222) |
| 61 | vector | GNU vector && ?: ! accepted (gcc rejects) | **new** | [#1223](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1223) |
| 61 | vector ++ | GNU vector ++/-- first lane +=8 | **new** | [#1224](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1224) |
| 61 | constraint | _Static_assert with floating constant accepted | **new** | [#1225](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1225) |
| 61 | constraint | compound literal with VLA type accepted | **new** | [#1226](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1226) |
| 62 | | _Alignas(-1) on a global ICE at common.rs:1116 | **new** | [#1227](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1227) |
| 62 | | empty enum and incomplete enum object accepted | **new** | [#1228](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1228) |
| 62 | | array-of-incomplete-struct as function parameter accept | **new** | [#1229](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1229) |
| 62 | | _Generic duplicate compatible associations and void sel | **new** | [#1230](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1230) |
| 62 | lexer | invalid integer 0x/0b/08/09 | **new** | [#1231](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1231) |
| 62 | | address of bit-field accepted | **new** | [#1232](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1232) |
| 62 | | designated initializer for nonexistent member accepted | **new** | [#1233](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1233) |
| 62 | | address of bit-field — DUP #1134, closed | dup | [#1232](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1232) |
| 62 | | duplicate struct member accepted | **new** | [#1234](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1234) |
| 62 | | function initialized like a variable accepted | **new** | [#1235](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1235) |
| 62 | | invalid file-scope storage class (auto/register/__threa | **new** | [#1236](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1236) |
| 62 | | _Alignof applied to incomplete type accepted | **new** | [#1237](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1237) |
| 62 | | conditional-expression type mismatch (struct vs int) ac | **new** | [#1238](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1238) |
| 62 | | [*] VLA star accepted outside function prototype scope | **new** | [#1239](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1239) |
| 62 | | aggregate used as scalar — (int)struct and !struct acce | **new** | [#1240](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1240) |
| 62 | | negative designated initializer index ICE (add/mul over | **new** | [#1241](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1241) |
| 62 | | '#' in replacement list not followed by a macro paramet | **new** | [#1242](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1242) |
| 62 | | __attribute__((mode(XX))) unknown machine mode accepted | **new** | [#1243](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1243) |
| 62 | | '##' cannot appear at either end of a macro replacement | **new** | [#1244](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1244) |
| 62 | | vector_size not a multiple of element size / negative a | **new** | [#1245](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1245) |
| 62 | | static variable in for-loop clause accepted | **new** | [#1246](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1246) |
| 62 | | incomplete UCN and stray backslash-at-EOF accepted | **new** | [#1247](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1247) |
| 62 | | pointer initialized from floating constant accepted | **new** | [#1248](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1248) |
| 62 | | _Alignas specified for a bit-field accepted | **new** | [#1249](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1249) |
| 62 | | 'void' must be the only parameter — extra params accept | **new** | [#1250](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1250) |
| 62 | | ellipsis-only parameter list int f(...) accepted | **new** | [#1251](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1251) |
| 62 | | function returning a function accepted | **new** | [#1252](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1252) |
| 62 | | parameter name omitted in a function definition accepte | **new** | [#1253](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1253) |
| 62 | | _Atomic-qualified array type accepted | **new** | [#1254](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1254) |
| 62 | | _Alignas specified for a function parameter accepted | **new** | [#1255](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1255) |
| 62 | | negative size in [static N] array parameter accepted | **new** | [#1256](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1256) |
| 62 | | function definition with incomplete return type accepte | **new** | [#1257](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1257) |
| 62 | | unknown asm clobber register accepted | **new** | [#1258](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1258) |
| 62 | | statement-expression at file scope accepted | **new** | [#1259](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1259) |
| 62 | | #if wide/utf character constants evaluate as 0 (L'A'/u' | **new** | [#1260](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1260) |
| 62 | | GNU vector assigned from scalar accepted | **new** | [#1261](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1261) |
| 62 | | returns_nonnull on a non-pointer-returning function acc | **new** | [#1262](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1262) |
| 62 | | _Generic with empty association list ICE (associations[ | **new** | [#1263](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1263) |
| 62 | | empty scalar initializer int x={} accepted | **new** | [#1264](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1264) |
| 62 | | VLA parameter size refers to a later parameter accepted | **new** | [#1265](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1265) |
| 62 | | offsetof applied to a non-struct type accepted | **new** | [#1266](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1266) |
| 62 | | va_arg with incomplete type void accepted | **new** | [#1267](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1267) |
| 62 | | tentative definition of incomplete struct/union later c | **new** | [#1268](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1268) |
| 62 | | block-scope extern with initializer accepted | **new** | [#1269](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1269) |
| 62 | | GNU builtin arity not checked — too few/too many args a | **new** | [#1270](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1270) |
| 62 | | declaration accepted where a statement is required (lab | **new** | [#1271](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1271) |
| 62 | | trailing/empty comma in declarator lists accepted | **new** | [#1272](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1272) |
| 62 | | GNU designated range initializer empty/OOB ranges accep | **new** | [#1273](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1273) |
| 62 | | duplicate macro parameter names accepted | **new** | [#1274](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1274) |
| 62 | | 'defined' used as a macro name accepted | **new** | [#1275](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1275) |
| 62 | | _Atomic applied to an already _Atomic-qualified type ac | **new** | [#1276](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1276) |
| 62 | | _Thread_local combined with auto/register accepted | **new** | [#1277](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1277) |
| 62 | | pointer arithmetic on incomplete struct type accepted | **new** | [#1278](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1278) |
| 62 | | static/type qualifiers in a non-parameter array declara | **new** | [#1279](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1279) |
| 62 | | enumerators declared in a type-name (cast/sizeof) not in scope | **new** | [#1280](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1280) |
| 62 | | _Pragma operand is not required to be a parenthesized s | **new** | [#1281](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1281) |
| 62 | | unknown preprocessing directives are silently ignored | **new** | [#1282](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1282) |
| 62 | | stray backtick token accepted | **new** | [#1283](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1283) |
| 62 | | alias between a function and a variable accepted | **new** | [#1284](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1284) |
| 62 | | cleanup attribute accepts a callback with the wrong pro | **new** | [#1285](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1285) |
| 62 | | asm goto with an empty label list accepted | **new** | [#1286](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1286) |
| 62 | | cast to an array type accepted | **new** | [#1287](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1287) |
| 62 | | unparenthesized comma inside array brackets parsed as a | **new** | [#1288](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1288) |
| 62 | | empty token-paste of both operands accepted as an empty | **new** | [#1289](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1289) |
| 62 | | call argument count is not checked against a prototype | **new** | [#1290](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1290) |
| 62 | | call of a function with incomplete return type accepted | **new** | [#1291](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1291) |
| 62 | | block-scope _Thread_local without static/extern accepte | **new** | [#1292](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1292) |
| 62 | | _Alignof applied to a bit-field accepted | **new** | [#1293](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1293) |
| 62 | | _Static_assert trailing comma after the message accepte | **new** | [#1294](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1294) |
| 62 | | tag reused as a different kind (struct vs union vs enum | **new** | [#1295](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1295) |
| 62 | | old-style parameter declarations on a prototyped functi | **new** | [#1296](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1296) |
| 62 | | variably modified type at file scope accepted | **new** | [#1297](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1297) |
| 62 | | _Alignas argument is not required to be an integer cons | **new** | [#1298](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1298) |
| 62 | | va_start used in a non-variadic function accepted | **new** | [#1299](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1299) |
| 62 | | __builtin_choose_expr first argument is not required to | **new** | [#1300](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1300) |
| 62 | | file-scope identifier starting with $ is emitted as an  | **new** | [#1301](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1301) |
| 62 | | incompatible types accepted in return and argument pass | **new** | [#1302](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1302) |
| 62 | | file-scope initializer is not required to be a constant | **new** | [#1303](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1303) |
| 62 | | #include of a macro that does not expand to a header na | **new** | [#1304](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1304) |
| 62 | | division by zero in a #if expression accepted | **new** | [#1305](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1305) |
| 62 | | huge array dimension panics in types.rs:815 (multiply o | **new** | [#1306](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1306) |
| 62 | | enumerator at i64::MAX panics in types.rs:828 (add over | **new** | [#1307](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1307) |
| 62 | | defined() with a non-identifier operand accepted | **new** | [#1308](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1308) |
| 62 | | K&R declaration list names a parameter that is not in t | **new** | [#1309](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1309) |
| 62 | | int array initialized from a string literal accepted | **new** | [#1310](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1310) |
| 62 | | constructor/destructor priority outside 0..65535 accept | **new** | [#1311](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1311) |
| 62 | | section attribute with no argument accepted | **new** | [#1312](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1312) |
| 62 | | bit-field declared with no type specifier accepted | **new** | [#1313](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1313) |
| 62 | | function-like macro invoked with too many arguments acc | **new** | [#1314](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1314) |
| 62 | | unterminated argument list invoking a function-like mac | **new** | [#1315](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1315) |
| 62 | | invalid storage class on a function accepted (_Thread_l | **new** | [#1316](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1316) |
| 62 | | _Thread_local combined with typedef accepted | **new** | [#1317](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1317) |
| 62 | | GNU error attribute is ignored when the function is cal | **new** | [#1318](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1318) |
| 62 | | negative vector_size hangs the compiler | **new** | [#1319](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1319) |
| 62 | | aligned(1<<32)/aligned(1<<40) panics in elf_writer_comm | **new** | [#1320](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1320) |
| 62 | | conflicting type specifiers accepted (int float, unsign | **new** | [#1321](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1321) |
| 62 | | compound literal of incomplete or void type accepted | **new** | [#1322](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1322) |
| 62 | | duplicate GNU __label__ declaration accepted | **new** | [#1323](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1323) |
| 62 | | storage class other than register accepted on a paramet | **new** | [#1324](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1324) |
| 62 | | static/type qualifier on a non-outermost parameter arra | **new** | [#1325](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1325) |
| 62 | | C++-style const/volatile/restrict qualifier after a fun | **new** | [#1326](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1326) |
| 62 | | __auto_type accepted without an initializer or with a v | **new** | [#1327](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1327) |
| 62 | | duplicate GNU asm operand names accepted | **new** | [#1328](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1328) |
| 62 | | #pragma GCC error is ignored | **new** | [#1329](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1329) |
| 62 | | GNU interrupt attribute accepted on a function with no  | **new** | [#1330](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1330) |
| 62 | | GNU attributes that require an argument accepted with n | **new** | [#1331](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1331) |
| 62 | | #line with a string instead of a line number accepted | **new** | [#1332](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1332) |
| 62 | | #define/#undef with no identifier accepted | **new** | [#1333](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1333) |
| 62 | | #if/#elif with no expression accepted | **new** | [#1334](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1334) |
| 62 | | incomplete enum object at file scope never completed ac | **new** | [#1335](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1335) |
| 62 | | duplicate labels in the same function accepted | **new** | [#1336](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1336) |
| 62 | | goto to an undefined label accepted | **new** | [#1337](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1337) |
| 62 | | case/default label outside a switch accepted | **new** | [#1338](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1338) |
| 62 | | multiple storage-class specifiers in one declaration ac | **new** | [#1339](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1339) |
| 62 | | goto into a GNU statement-expression accepted | **new** | [#1340](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1340) |
| 62 | | file-scope asm volatile accepted | **new** | [#1341](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1341) |
| 62 | | pointer-to-VLA: sizeof(*p) and p[i][j] indexing use poi | **new** | [#1345](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1345) |
| 62 | | variable-length array with an initializer accepted | **new** | [#1346](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1346) |
| 62 | | sizeof(__func__) is the pointer size, not strlen+1 | **new** | [#1347](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1347) |
| 62 | | declaration in place of a statement after if/else/while | **new** | [#1348](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1348) |
| 62 | | pure `inline` function gets an emitted external definit | **new** | [#1351](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1351) |
| 62 | | unknown CLI options silently accepted (-std=bogus -Wbogus -fwhatever -mnope --bogus) | **new** | [#1352](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1352) |
| 62 | | -nostdinc is a no-op (system headers still found) | **new** | [#1353](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1353) |
| 62 | | cross targets search host x86-64 /usr/include; builtin stddef/stdarg missing | **new** | [#1354](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1354) |
| 62 | | plain char signed on ARM/RISC-V (ABI: unsigned) | **new** | [#1355](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1355) |
| 62 | | RISC-V struct {float;double} by-value arg: double corrupt | **new** | [#1359](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1359) |
| 62 | | #pragma pack(non-power-of-2) applied, no diagnostic (gcc/clang ignore) | **new** | [#1364](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1364) |
| 62 | | array designator index exceeding bounds accepted | **new** | [#1365](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1365) |
| 62 | | offsetof on bit-field accepted | **new** | [#1368](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1368) |
| 62 | | typedef + extern/static/auto accepted (3 forms) | **new** | [#1369](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1369) |
| 62 | | const-fold (_Bool)A op (_Bool)B: no normalization, byte-truncated | **new** | [#1370](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1370) |
| 62 | | parser stack overflow at ~4000 nested parens (SIGABRT) | **new** | [#1371](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1371) |
| 62 | | struct member of later-completed incomplete type accepted | **new** | [#1372](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1372) |
| 62 | | unparenthesized assignment/comma in ?: operands accepted | **new** | [#1373](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1373) |
| 62 | | stray mid-file backslash (\\+NL) accepted | **new** | [#1377](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1377) |
| 62 | | __auto_type from array keeps array type, reads garbage | **new** | [#1378](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1378) |
| 62 | | __auto_type with bit-field initializer accepted | **new** | [#1379](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1379) |
| 62 | | #endif trailing token in skipped group swallows rest of file | **new** | [#1380](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1380) |
| 62 | | -E -P/-dD no-ops (linemarkers kept, defines not dumped) | **new** | [#1384](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1384) |
| 62 | | __has_attribute expression position = implicit call (no output) | **new** | [#1385](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1385) |
| 62 | | non-constant exprs in case/array-size/bitfield/enum contexts | **new** | [#1386](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1386) |
| 62 | | -E linemarkers structurally invalid (gcc/clang cannot re-consume) | **new** | [#1387](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1387) |
| 62 | | unknown __attribute__ names silently ignored (no warning) | **new** | [#1388](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1388) |
| 62 | | static-init pointer comparisons fold to 0 (gcc folds right) | **new** | [#1391](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1391) |
| 62 | | pack(1)+anon :0 bitfield: unit-break lost (sizeof 2 vs 5) | **new** | [#1392](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1392) |
| 62 | | -std= no semantic effect (c89 accepts C99 constructs) | **new** | [#1393](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1393) |
| 62 | | _Pragma via macro expansion silent no-op (pack lost) | **new** | [#1394](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1394) |
| 62 | | string prefix concat u8+u / u+U accepted (gcc/clang err) | **new** | [#1399](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1399) |
| 62 | | extended-asm single-% register names accepted (gcc rejects) | **new** | [#1400](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1400) |
| 62 | | static-init fold treats __builtin_object_size as signed | **new** | [#1401](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1401) |
| 62 | | predefined macros missing: __TIMESTAMP__/__STDC_IEC_559__/__NO_INLINE__ | **new** | [#1402](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1402) |
| 62 | | _Static_assert inside enum body accepted | **new** | [#1403](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1403) |
| 62 | | __typeof__ drops const qualifier | **new** | [#1407](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1407) |
| 62 | | [2][static 2] / [const *] param qualifier placement accepted | **new** | [#1408](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1408) |
| 62 | | assembler .globl comma-list silently LOCAL | **new** | [#1409](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1409) |
| 62 | | assembler silently drops .space/.float/.double/.single/.octa/unknown dirs | **new** | [#1410](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1410) |
| 62 | | assembler .equ/.equiv symbols undefined at link (.set works) | **new** | [#1413](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1413) |
| 62 | | assembler location counter `.` undefined (.-label fails link) | **new** | [#1414](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1414) |
| 62 | | .cfi_* dropped, no .eh_frame ever (own -S contract broken) | **new** | [#1415](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1415) |
| 62 | | assembler .warning prints nothing (gas warns) | **new** | [#1416](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1416) |
| 62 | | -g accepted, zero DWARF; .file/.loc dropped | **new** | [#1417](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1417) |
| 62 | | asm data-expr comparisons fold 0; ?: accepted→0 | **new** | [#1418](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1418) |
| 62 | | .align N,FILL fill byte ignored (zeros) | **new** | [#1419](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1419) |
| 62 | | assembler .fail/.abort silently ignored (gas fatal) | **new** | [#1420](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1420) |
| 62 | | .long symbol → 0, no reloc (32-bit ptr tables null) | **new** | [#1421](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1421) |
| 62 | | cross targets: arm entry=image base SIGILL; riscv entry=main SEGV; riscv -nostdlib still dynamic | **new** | [#1422](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1422) |
| 62 | | -Wl, args silently ignored (-Map never written) | **new** | [#1423](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1423) |
| 62 | | -pie accepted but ET_EXEC emitted (no-op) | **new** | [#1426](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1426) |
| 62 | | visibility("hidden") attr and -fvisibility=hidden ignored (hid exported) | **new** | [#1427](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1427) |
| 62 | | ifunc compiles but link fails undefined symbols | **new** | [#1437](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1437) |
| 62 | | i686 asm: $sym@GOTOFF rejected; PLT32/GOT32 type quirks | **new** | [#1440](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1440) |
| 62 | | ARM ngc/ngcs REJECTED unsupported (corrected from constant-bytes claim) | **new (corrected)** | [#1445](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1445) |
| 62 | | ARM uxth/uxtb X-dest sf=1 wrong form | **closed DUP of #868/#873** | [#1446](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1446) |
| 62 | | ARM sturb/sturh missing (store mirror of #842) | **new** | [#1447](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1447) |
| 62 | | ARM fmov imm forms -> NOP silently (FP consts vanish) | **new** | [#1448](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1448) |
| 62 | | ARM fcmpe/fcsel/hlt missing mnemonics | **new** | [#1449](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1449) |
| 62 | | RV FCVT default rm=111 vs reference 000 (S→D,W→D) | **new** | [#1450](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1450) |
| 62 | | ARM ldr xN,=imm literal pool stores ZEROS | **new** | [#1451](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1451) |
| 62 | | function aligned(N) silently ignored (.text stays 16) | **new** | [#1458](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1458) |
| 62 | | extern weakref accepted (gcc/clang: static linkage required) | **new** | [#1459](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1459) |
| 62 | | mixed-type bitfield layout sizeof 12 vs gcc 8 (ABI div) | **new** | [#1460](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1460) |
| 62 | | ARM smov/frecpe/frsqrte missing (3 SIMD mnemonics) | **new** | [#1467](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1467) |
| 62 | | ARM sqneg silently encodes cmgt #0 (WRONG instr) | **new** | [#1468](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1468) |
| 62 | | ARM fabd/fmulx/suqadd/usqadd missing (4 SIMD) | **new** | [#1469](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1469) |
| 62 | | ARM saddw/uaddw/ssubw/usubw ALL encode wrong size+Q (8 forms) | **new** | [#1470](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1470) |
| 62 | | RV csrwi/csrsi/csrci missing (CSR immediate pseudos) | **new** | [#1475](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1475) |
| 62 | | ARM clrex CRm=0 vs conventional CRm=15 | **new** | [#1476](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1476) |
| 62 | | ARM by-element MLA/MLS/FMLA/FMLS invalid encoding (H/L bits) | **new** | [#1477](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1477) |
| 62 | | ARM scalar uqshrn/sqrshrn/uqrshrn/sqshrun/sqrshrun missing | **new** | [#1478](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1478) |
| 62 | | ARM scalar FP: fnmul/frecpe/frecps/frsqrte/frsqrts/frecpx missing | **new** | [#1479](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1479) |
| 62 | | x86 cvtsi2ss/sd + cvttss/sd2si ALL missing (8 variants) | **new** | [#1480](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1480) |
| 62 | | x86 10 SSE3/4 missing: phaddsw/psubsw/pmaddubsw/psign*/pmuldq/pcmpeqq/mpsadbw/pcmpestrm | **new** | [#1481](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1481) |
| 62 | | x86 13 x87 FPU missing: fst/fcom*/fucom*/fcmov*/fsave/frstor | **new** | [#1482](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1482) |
| 62 | | x86 AVX: 10 missing (vcvt*/vucomi*/vsqrt*/vfmadd*) + verr | **new** | [#1483](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1483) |
| 62 | | x86 BT/BTR/BTS/BTC always REX.W (32-bit becomes 64-bit) | **new** | [#1487](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1487) |
| 62 | | x86 xadd reg-reg rejected (memory form works) | **new** | [#1488](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1488) |
| 62 | | x86 enter/iret/lodsl missing | **new** | [#1489](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1489) |
| 62 | | x86 mulx/pdep/pext operand swap (VEX.vvvv wrong) | **new** | [#1490](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1490) |
| 62 | | x86 MMX always gets 0x66 prefix (mm0 becomes xmm0) | **new** | [#1491](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1491) |
| 62 | | x86 BMI1 blsi/blsmsk/blsr + tzcnt/lzcnt missing | **new** | [#1492](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1492) |
| 62 | | x86 imulw drops 0x66 (16-bit becomes 32-bit) | **new** | [#1493](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1493) |
| 62 | | x86 push/pop fs/gs -> push/pop rsp/rbp (WRONG REG) | **new** | [#1494](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1494) |
| 62 | | x86 leal drops 0x67 address-size prefix | **new** | [#1495](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1495) |
| 62 | | x86 extractps/insertps missing (SSE4.1) | **new** | [#1496](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1496) |
| 62 | | ARM SHA1/SHA256 crypto all 10 missing (AES works) | **new** | [#1497](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1497) |
| 62 | | i686 lds/les/lss/lfs/lgs + pushal/popal missing (7) | **new** | [#1503](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1503) |
| 62 | | ARM sqshlu + sqdmlal/sqdmlsl/sqdmull scalar + sqdmulh/uqdmulh missing (6) | **new** | [#1504](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1504) |
| 62 | | x86 sysret always REX.W (sysret vs sysretq conflated) | **new** | [#1505](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1505) |
| 62 | | x86 clflushopt/clwb missing | **new** | [#1506](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1506) |
| 62 | | ARM msr/mrs rejects timer tval registers (3) | **new** | [#1507](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1507) |
| 62 | | ARM fmaxp/fminp/fmaxnmp/fminnmp pairwise missing | **new** | [#1508](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1508) |
| 62 | | ARM vector fcvt rounding 16 forms missing (scalar works) | **new** | [#1509](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1509) |
| 62 | | ARM fcvtxn/movi d0/orr-bic-imm alias missing (5) | **new** | [#1510](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1510) |
| 62 | | lexer [0...3] maximal-munch violation (range designator) | **new** | [#1516](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1516) |
| 62 | | #if comma operator evaluates FIRST operand (wrong branch) | **new** | [#1517](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1517) |
| 62 | | macro arg count not validated for F(,) | **new** | [#1518](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1518) |
| 62 | | pragma pack leaks __ccc_pack_set_1 marker into -E | **new** | [#1519](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1519) |
| 62 | | const-fold int cmp uchar/ushort as unsigned (fold≠runtime) | **new** | [#1520](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1520) |
| 62 | | duplicate case labels accepted silently | **new** | [#1521](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1521) |
| 62 | | ALL -W flags silently ignored (zero diagnostics) | **new** | [#1532](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1532) |
| 62 | | -f/-m flags no-op (funsigned-char semantics, stack-protector) | **new** | [#1533](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1533) |
| 62 | | -M/-MM no include deps (incremental builds break) + --help/-save-temps no-op | **new** | [#1534](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1534) |
| 62 | | ALL -O levels produce identical code (optimizer never engages) | **new** | [#1535](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1535) |
| 62 | | ELF interposition broken (exe .dynsym never exports; lib self-calls win) | **new** | [#1539](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1539) |
| 62 | | RISC-V/ARM branch+jal overflow silently wraps (fwd→back) | **new** | [#1540](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1540) |
| 62 | | ARM lacks ldpsw + ldtrsb/h/sw + prfum (7 missing loads) | **new** | [#1541](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1541) |
| 62 | | ARM rejects shll/shll2 aliases (sshll/ushll work) | **new** | [#1542](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1542) |
| 62 | | x86 lacks entire SHA-NI set (7 sha1/sha256 forms) | **new** | [#1543](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1543) |
| 62 | | Bug: __builtin_constant_p(string-literal) is 0 — gcc is | **new** | [#1544](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1544) |
| 62 | | Bug: sizeof (int){1} rejected — parser eats sizeof(type | **new** | [#1545](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1545) |
| 62 | | movbe all 3 widths missing | **new** | [#1546](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1546) |
| 62 | | AVX/AVX2 gather+varshift+maskmov+vpermpd+vpblendw+f128 | **new** | [#1547](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1547) |
| 62 | | XSAVE family 6 memory forms missing (xgetbv/xsetbv ok) | **new** | [#1548](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1548) |
| 62 | | SSE4a extrq/insertq/movntss/movntsd missing | **new** | [#1549](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1549) |
| 62 | | RISC-V Zba+Zbs 15 forms missing (Zbb works) | **new** | [#1550](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1550) |
| 62 | | VMX vmxon/vmread/vmwrite/invept/invvpid missing | **new** | [#1555](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1555) |
| 62 | | monitor/mwait missing | **new** | [#1556](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1556) |
| 62 | | AMD SVM vmrun/vmload/vmsave/stgi/clgi/skinit/invlpga missing | **new** | [#1557](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1557) |
| 62 | | RISC-V Zicond+Zicbo* 9 forms missing | **new** | [#1563](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1563) |
| 62 | | RISC-V Zfh flh/fsh/fmv.x.h/fmv.h.x missing | **new** | [#1564](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1564) |
| 62 | | i686 ltr/clts + ud2a both widths missing | **new** | [#1565](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1565) |
| 62 | | x86-64 ltr %eax accepted (gas: size mismatch) | **new** | [#1566](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1566) |
| 62 | | AArch64 MTE whole extension missing (7+ forms) | **new** | [#1567](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1567) |
| 62 | | AArch64 PAC instructions missing (5+ forms, bti works) | **new** | [#1568](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1568) |
| 62 | | AArch64 SVE whole extension missing (parse+table) | **new** | [#1569](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1569) |
| 62 | | x86 AVX-512 k-register ops 7 forms missing | **new** | [#1570](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1570) |
| 62 | | RISC-V V partial: mul/FP/reduce/cmp classes missing | **new** | [#1571](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1571) |
| 62 | | x86 misc/system 17 forms in 8 clusters missing | **new** | [#1572](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1572) |
| 62 | | AArch64 RCPC2 ldapur family 5 forms missing | **new** | [#1573](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1573) |
| 62 | | RISC-V scalar crypto Zkn/Zknh 12 forms missing | **new** | [#1574](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1574) |
| 62 | | ARM .skip/.lcomm; RV .skip/.lcomm/.float/.double rejected | **new** | [#1575](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1575) |
| 62 | | x86-64 .org .+4 dot symbol rejected | **new** | [#1576](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1576) |
| 62 | | x86-64 .intel_syntax unsupported (Label operands) | **new** | [#1577](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1577) |
| 62 | | leading .byte with no .text: no active section | **new** | [#1578](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1578) |
| 62 | | x86 IN/OUT DX-register forms missing (imm forms work) | **new** | [#1579](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1579) |
| 62 | | x86-64 xlatb missing | **new** | [#1580](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1580) |
| 62 | | AArch64 CASP (LSE128) missing | **new** | [#1581](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1581) |
| 62 | | RISC-V H ext: hfence + 7 H/VS CSR names missing | **new** | [#1582](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1582) |
| 62 | | x86-64 rsm missing | **new** | [#1583](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1583) |
| 63 | | inline-asm impossible constraint s/P accepted | **new** | [#1584](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1584) |
| 63 | | __builtin_shuffle/shufflevector/convertvector missing | **new** | [#1585](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1585) |
| 63 | | .type %gnu_indirect_function rejected | **new** | [#1586](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1586) |
| 63 | | x87 fclex/finit/fsincos missing (siblings work) | **new** | [#1587](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1587) |
| 63 | | AVX vcvtsd2ss/vroundps/vldmxcsr/vstmxcsr missing | **new** | [#1588](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1588) |
| 64 | | unnamed bitfield inflates struct sizeof (8v5, 4v3) | **new** | [#1589](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1589) |
| 64 | | _Static_assert sizeof(struct/union/enum) vacuous | **new** | [#1590](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1590) |
| 64 | | driver -u misparsed as input file | **new** | [#1591](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1591) |
| 64 | | UCN mid-identifier rejected (leading works) | **new** | [#1592](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1592) |
| 64 | | $ identifier → bogus assembler error | **new** | [#1593](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1593) |
| 64 | | asm-label rename dropped on definitions; vars "bad register" | **new** | [#1594](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1594) |
| 64 | | __builtin_object_size always (size_t)-1 | **new** | [#1595](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1595) |
| 64 | | __builtin_apply_args/__builtin_return missing | **new** | [#1596](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1596) |
| 64 | | __builtin_fma/fmaf missing | **new** | [#1597](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1597) |
| 64 | | builtins wave2: powi/bcopy/index/rindex missing | **new** | [#1598](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1598) |
| 64 | | builtins wave3: memchr/stpcpy/sincos/cpu_is/eh 5 more | **new** | [#1599](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1599) |
| 64 | | types_compatible_p ignores pointee qualifiers | **new** | [#1600](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1600) |
| 64 | | types_compatible_p void(void) parse error | **new** | [#1601](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1601) |
| 64 | | ARM+RV lack .equiv/.sleb128/.uleb128 (x86 has) | **new** | [#1602](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1602) |
| 64 | | __PIC__ never defined (gcc: always) | **new** | [#1603](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1603) |
| 64 | | __builtin_speculation_safe_* missing | **new** | [#1604](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1604) |
| 64 | | __STDC_NO_ATOMICS__ defined as 0 (gcc: undefined) | **new** | [#1605](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1605) |
| 64 | | long-double literals 1 ULP low (truncation) — 6/12 sampled | **new** | [#1606](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1606) |
| 64 | | object/typedef redeclaration compat absent (3 forms) | **new** | [#1607](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1607) |
| 64 | | initializer type constraints absent (4 forms) | **new** | [#1608](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1608) |
| 64 | | bit-field base-type constraints absent (7 forms) | **new** | [#1609](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1609) |
| 64 | | ## in object-like macro not expanded | **new** | [#1610](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1610) |
| 64 | | __typeof__(VLA) decays to pointer (4 vs 12) | **new** | [#1611](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1611) |
| 64 | | hex-float subnormal literals → +0.0 (decimal path OK) | **new** | [#1612](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1612) |
| 64 | | wide-prefix string concatenation L+u accepted | **new** | [#1613](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1613) |
| 64 | | 17 FP math builtins absent (sqrtl-family + truncf/fminf/fmaxf) | **new** | [#1614](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1614) |
| 64 | | 5 driver query flags rejected (-print-prog-name/--help/...) | **new** | [#1615](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1615) |
| 64 | | __DATE__/__TIME__ hardcoded (Jan 1 2025 / 00:00:00) | **new** | [#1616](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1616) |
| 64 | | enum value not required to be ICE (incl. 1/0) | **new** | [#1617](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1617) |
| 64 | | duplicate function parameter names accepted | **new** | [#1618](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1618) |
| 64 | | x86 multi-byte NOP mem forms (0F 1F) all rejected | **new** | [#1619](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1619) |
| 64 | | undefined numeric local label (1f) → silent disp-0 jump | **new** | [#1620](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1620) |
| 64 | | cvtss2si/cvtsd2si to %eax spurious REX.W (wrong width) | **new** | [#1621](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1621) |
| 64 | | AT&T cbtw/cwtl/cwtd sign-extension mnemonics missing | **new** | [#1622](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1622) |
| 64 | | %cs:/%ds:/%ss: segment prefixes rejected (fs/gs only) | **new** | [#1623](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1623) |
| 64 | | MMX reg-count shifts (8) + movdq2q/movq2dq rejected | **new** | [#1624](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1624) |
| 64 | | 16 AVX2 vpmovzx/sx + vcvt float-family forms rejected | **new** | [#1625](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1625) |
| 64 | | btsw drops 0x66 (16-bit BTS silently 32-bit) + lock bts reg,reg accepted | **new** | [#1626](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1626) |
| 64 | | verr %ax missing (verw facet = #1356) | **new** | [#1627](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1627) |
| 64 | | fsub/fdiv %st,%st(1) encode REVERSED opcode (dce9/dcf9) — HIGH | **new** | [#1628](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1628) |
| 64 | | movw seg-load drops 0x66 — DUP of #1396, closed | **dup** | [#1629](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1629) → [#1396](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1396) |
| 64 | | .eqv/.equiv→0 + .irpc single-0x00 (wrong-value directives) | **new** | [#1630](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1630) |
| 64 | | .macro expansion before explicit section → no active section | **new** | [#1631](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1631) |
| 64 | | AArch64 scalar fcmp/fcmpe #0.0 explicit → NOP (stale flags) | **new** | [#1632](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1632) |
| 64 | | AArch64 16 LSE min/max atomics (ldsmax/ldsmin/ldumax/ldumin × 4) all missing | **new** | [#1633](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1633) |
| 64 | | AArch64 ldsmax (LDSMAX — signed atomic MAX, relaxed (no ordering)) rejected | **new** | [#1634](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1634) |
| 64 | | AArch64 ldsmaxa (LDSMAXA — signed atomic MAX, acquire) rejected | **new** | [#1635](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1635) |
| 64 | | AArch64 ldsmaxl (LDSMAXL — signed atomic MAX, release) rejected | **new** | [#1636](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1636) |
| 64 | | AArch64 ldsmaxal (LDSMAXAL — signed atomic MAX, acquire+release) rejected | **new** | [#1637](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1637) |
| 64 | | AArch64 ldsmin (LDSMIN — signed atomic MIN, relaxed) rejected | **new** | [#1638](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1638) |
| 64 | | AArch64 ldsmina (LDSMINA — signed atomic MIN, acquire) rejected | **new** | [#1639](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1639) |
| 64 | | AArch64 ldsminl (LDSMINL — signed atomic MIN, release) rejected | **new** | [#1640](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1640) |
| 64 | | AArch64 ldsminal (LDSMINAL — signed atomic MIN, acquire+release) rejected | **new** | [#1641](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1641) |
| 64 | | AArch64 ldumax (LDUMAX — unsigned atomic MAX, relaxed) rejected | **new** | [#1642](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1642) |
| 64 | | AArch64 ldumaxa (LDUMAXA — unsigned atomic MAX, acquire) rejected | **new** | [#1643](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1643) |
| 64 | | AArch64 ldumaxl (LDUMAXL — unsigned atomic MAX, release) rejected | **new** | [#1644](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1644) |
| 64 | | AArch64 ldumaxal (LDUMAXAL — unsigned atomic MAX, acquire+release) rejected | **new** | [#1645](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1645) |
| 64 | | AArch64 ldumin (LDUMIN — unsigned atomic MIN, relaxed) rejected | **new** | [#1646](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1646) |
| 64 | | AArch64 ldumina (LDUMINA — unsigned atomic MIN, acquire) rejected | **new** | [#1647](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1647) |
| 64 | | AArch64 lduminl (LDUMINL — unsigned atomic MIN, release) rejected | **new** | [#1648](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1648) |
| 64 | | AArch64 lduminal (LDUMINAL — unsigned atomic MIN, acquire+release) rejected | **new** | [#1649](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1649) |
| 64 | | NEON sdot/udot/usdot dot-product family missing | **new** | [#1650](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1650) |
| 64 | | FCMLA/FCADD complex arithmetic missing | **new** | [#1651](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1651) |
| 64 | | FP16 vector arithmetic (fadd/fmul/fmla v.8h) missing | **new** | [#1652](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1652) |
| 64 | | fcvtn2 .4h reserved arrangement accepted (SIGILL bytes) | **new** | [#1653](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1653) |
| 64 | | AArch64 setf8 (SETF8 — set NZCV from byte-8 value in Wn) rejected | **new** | [#1654](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1654) |
| 64 | | AArch64 setf16 (SETF16 — set NZCV from halfword value in Wn) rejected | **new** | [#1655](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1655) |
| 64 | | AArch64 cfinv (CFINV — invert PSTATE.C (carry flag)) rejected | **new** | [#1656](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1656) |
| 64 | | AArch64 axflag (AXFLAG — convert NZCV to alternative flag order) rejected | **new** | [#1657](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1657) |
| 64 | | AArch64 xaflag (XAFLAG — restore NZCV from alternative order) rejected | **new** | [#1658](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1658) |
| 64 | | AArch64 esb (ESB — Error Synchronization Barrier (RAS)) rejected | **new** | [#1659](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1659) |
| 64 | | i686 lcall $sel,$off (far call) rejected | **new** | [#1660](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1660) |
| 64 | | i686 DR0-DR7 debug-register MOVs rejected | **new** | [#1661](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1661) |
| 64 | | x86-64 gf2p8affineqb rejected (GFNI/VAES gap) | **new** | [#1662](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1662) |
| 64 | | x86-64 gf2p8affineinvqb rejected (GFNI/VAES gap) | **new** | [#1663](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1663) |
| 64 | | x86-64 gf2p8mulb rejected (GFNI/VAES gap) | **new** | [#1664](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1664) |
| 64 | | x86-64 vgf2p8affineqb rejected (GFNI/VAES gap) | **new** | [#1665](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1665) |
| 64 | | x86-64 vaeskeygenassist rejected (GFNI/VAES gap) | **new** | [#1666](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1666) |
| 64 | | x86-64 vaesimc rejected (GFNI/VAES gap) | **new** | [#1667](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1667) |
| 64 | | AArch64 sshll source-arrangement mismatch accepted | **new** | [#1668](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1668) |
| 64 | | x86 XOP: XOP vfrcz family (fraction-extract) missing | **new** | [#1669](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1669) |
| 64 | | x86 XOP: XOP vpcmov (per-variable bit select) missing | **new** | [#1670](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1670) |
| 64 | | x86 XOP: XOP integer multiply-accumulate vpmacs*/vpmacss* missing | **new** | [#1671](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1671) |
| 64 | | x86 XOP: XOP byte-permute vpperm/vpermil2ps/vpermil2pd missing | **new** | [#1672](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1672) |
| 64 | | x86 XOP: XOP horizontal add/sub vphaddw/vphaddubq/vphsubw missing | **new** | [#1673](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1673) |
| 64 | | x86 XOP: XOP rotate/shift-by-register vprotb/vprotd/vpshlb missing | **new** | [#1674](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1674) |
| 64 | | i686 aam (AAM) rejected | **new** | [#1675](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1675) |
| 64 | | i686 aad (AAD) rejected | **new** | [#1676](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1676) |
| 64 | | i686 daa (DAA) rejected | **new** | [#1677](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1677) |
| 64 | | i686 das (DAS) rejected | **new** | [#1678](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1678) |
| 64 | | i686 salc (SALC) rejected | **new** | [#1679](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1679) |
| 64 | | x86 monitorx/mwaitx (AMD) rejected | **new** | [#1680](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1680) |
| 64 | | RISC-V Zacas amocas rejected | **new** | [#1681](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1681) |
| 64 | | x86 LOCK/REP prefix validation absent (7 illegal combos accepted) | **new** | [#1682](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1682) |
| 64 | | x86 loop family half-missing + i686 jcxz drops 0x67 | **new** | [#1683](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1683) |
| 64 | | x86 bnd jmp (F2 branch prefix) rejected | **new** | [#1684](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1684) |
| 64 | | AArch64 sysl (system read form) rejected | **new** | [#1685](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1685) |
| 64 | | RISC-V sinval.vma (Svinval) rejected | **new** | [#1686](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1686) |
| 64 | | RISC-V CSR medeleg rejected | **new** | [#1687](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1687) |
| 64 | | RISC-V CSR mideleg rejected | **new** | [#1688](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1688) |
| 64 | | RISC-V CSR menvcfg rejected | **new** | [#1689](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1689) |
| 64 | | RISC-V CSR senvcfg rejected | **new** | [#1690](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1690) |
| 64 | | _Float16/__fp16/f16 suffix unrecognized (both backends, ARM-native type) | **new** | [#1691](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1691) |
| 64 | | TLS init zero + cross-module corruption (.tdata %16=8) | **new** | [#1692](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1692) |
| 64 | | shared-lib TLS relocs dropped (dlopen 0) | **new** | [#1693](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1693) |
| 64 | | @tlsgd/@tlsld -> PC32; indntpoff accepted | **new** | [#1694](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1694) |
| 64 | | PT_GNU_RELRO missing (GOT writable) | **new** | [#1695](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1695) |
| 64 | | backtrace 1-frame: FDEs have no CFA rules | **new** | [#1696](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1696) |
| 64 | | -Wl,-z,now silent no-op | **new** | [#1697](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1697) |
| 64 | | asm goto fallthrough jumps to last label (skips next stmt) | **new** | [#1698](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1698) |
| 64 | | inline-asm %= / Nf/Nb labels not expanded | **new** | [#1699](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1699) |
| 64 | | inline-asm "x" constraint input not materialized | **new** | [#1700](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1700) |
| 64 | | movd/movq GPR<->MMX rejected (0F 6E/7E) | **new** | [#1701](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1701) |
| 64 | | C23 auto silently = int (miscompile) | **new** | [#1702](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1702) |
| 64 | | C23 _BitInt absent | **new** | [#1703](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1703) |
| 64 | | C23 constexpr absent | **new** | [#1704](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1704) |
| 64 | | C23 [[attributes]] absent | **new** | [#1705](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1705) |
| 64 | | C23 typeof_unqual absent | **new** | [#1706](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1706) |
| 64 | | C23 nullptr/nullptr_t absent | **new** | [#1707](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1707) |
| 64 | | C23 unreachable() absent | **new** | [#1708](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1708) |
| 64 | | C23 digit separator ' absent | **new** | [#1709](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1709) |
| 64 | | #elifdef silent wrong branch + __has_c_attribute false | **new** | [#1710](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1710) |
| 64 | | #embed silently zero bytes | **new** | [#1711](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1711) |
| 64 | | __FILE_NAME__ macro absent | **new** | [#1712](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1712) |
| 64 | | __builtin_exp2 missing (link-level) | **new** | [#1713](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1713) |
| 64 | | __builtin_expm1 missing (link-level) | **new** | [#1714](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1714) |
| 64 | | __builtin_log1p missing (link-level) | **new** | [#1715](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1715) |
| 64 | | __builtin_cbrt missing (link-level) | **new** | [#1716](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1716) |
| 64 | | __builtin_sinh missing (link-level) | **new** | [#1717](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1717) |
| 64 | | __builtin_cosh missing (link-level) | **new** | [#1718](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1718) |
| 64 | | __builtin_tanh missing (link-level) | **new** | [#1719](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1719) |
| 64 | | __builtin_asinh missing (link-level) | **new** | [#1720](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1720) |
| 64 | | __builtin_acosh missing (link-level) | **new** | [#1721](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1721) |
| 64 | | __builtin_atanh missing (link-level) | **new** | [#1722](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1722) |
| 64 | | __builtin_asin missing (link-level) | **new** | [#1723](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1723) |
| 64 | | __builtin_acos missing (link-level) | **new** | [#1724](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1724) |
| 64 | | __builtin_atan missing (link-level) | **new** | [#1725](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1725) |
| 64 | | __builtin_tgamma missing (link-level) | **new** | [#1726](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1726) |
| 64 | | __builtin_lgamma missing (link-level) | **new** | [#1727](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1727) |
| 64 | | __builtin_erf missing (link-level) | **new** | [#1728](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1728) |
| 64 | | __builtin_erfc missing (link-level) | **new** | [#1729](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1729) |
| 64 | | __builtin_logb missing (link-level) | **new** | [#1730](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1730) |
| 64 | | __builtin_trunc missing (link-level) | **new** | [#1731](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1731) |
| 64 | | __builtin_nearbyint missing (link-level) | **new** | [#1732](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1732) |
| 64 | | __builtin_rint missing (link-level) | **new** | [#1733](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1733) |
| 64 | | __builtin_fmod missing (link-level) | **new** | [#1734](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1734) |
| 64 | | __builtin_fdim missing (link-level) | **new** | [#1735](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1735) |
| 64 | | __builtin_atan2 missing (link-level) | **new** | [#1736](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1736) |
| 64 | | __builtin_hypot missing (link-level) | **new** | [#1737](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1737) |
| 64 | | __builtin_nexttoward missing (link-level) | **new** | [#1738](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1738) |
| 64 | | __builtin_remainder missing (link-level) | **new** | [#1739](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1739) |
| 64 | | __builtin_ldexp missing (link-level) | **new** | [#1740](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1740) |
| 64 | | __builtin_exp2f missing (link-level) | **new** | [#1741](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1741) |
| 64 | | __builtin_expm1f missing (link-level) | **new** | [#1742](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1742) |
| 64 | | __builtin_log1pf missing (link-level) | **new** | [#1743](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1743) |
| 64 | | __builtin_cbrtf missing (link-level) | **new** | [#1744](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1744) |
| 64 | | __builtin_sinhf missing (link-level) | **new** | [#1745](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1745) |
| 64 | | __builtin_tanf missing (link-level) | **new** | [#1746](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1746) |
| 64 | | __builtin_asinf missing (link-level) | **new** | [#1747](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1747) |
| 64 | | __builtin_atanf missing (link-level) | **new** | [#1748](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1748) |
| 64 | | __builtin_tgammaf missing (link-level) | **new** | [#1749](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1749) |
| 64 | | __builtin_erff missing (link-level) | **new** | [#1750](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1750) |
| 64 | | __builtin_logbf missing (link-level) | **new** | [#1751](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1751) |
| 64 | | __builtin_nearbyintf missing (link-level) | **new** | [#1752](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1752) |
| 64 | | __builtin_rintf missing (link-level) | **new** | [#1753](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1753) |
| 64 | | __builtin_log2f missing (link-level) | **new** | [#1754](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1754) |
| 64 | | __builtin_log10f missing (link-level) | **new** | [#1755](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1755) |
| 64 | | __builtin_log2l missing (link-level) | **new** | [#1756](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1756) |
| 64 | | __builtin_strdup missing (link-level) | **new** | [#1757](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1757) |
| 64 | | __builtin_roundeven missing (link-level) | **new** | [#1758](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1758) |
| 64 | | __builtin_roundevenf missing (link-level) | **new** | [#1759](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1759) |
| 64 | | __builtin_roundevenl missing (link-level) | **new** | [#1760](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1760) |
| 64 | | __builtin_issignaling missing (link-level) | **new** | [#1761](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1761) |
| 64 | | __builtin_clzg missing (link-level) | **new** | [#1762](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1762) |
| 64 | | __builtin_ctzg missing (link-level) | **new** | [#1763](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1763) |
| 64 | | __builtin_popcountg missing (link-level) | **new** | [#1764](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1764) |
| 64 | | __builtin_modf missing (link-level) | **new** | [#1765](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1765) |
| 64 | | __builtin_frexp missing (link-level) | **new** | [#1766](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1766) |
| 64 | | tls_model attr ignored (GOTTPOFF vs TPOFF32) | **new** | [#1767](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1767) |
| 64 | | deprecated/warn_unused_result/format/nonnull no warnings | **new** | [#1768](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1768) |
| 64 | | __builtin_strpbrk missing (straggler) | **new** | [#1769](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1769) |
| 64 | | __builtin_stpncpy missing (straggler) | **new** | [#1770](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1770) |
| 64 | | __builtin_isnanf missing (b3/b4) | **new** | [#1771](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1771) |
| 64 | | __builtin_isinff missing (b3/b4) | **new** | [#1772](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1772) |
| 64 | | __builtin_isnanl missing (b3/b4) | **new** | [#1773](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1773) |
| 64 | | __builtin_isinfl missing (b3/b4) | **new** | [#1774](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1774) |
| 64 | | __builtin_cabs missing (b3/b4) | **new** | [#1775](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1775) |
| 64 | | __builtin_cproj missing (b3/b4) | **new** | [#1776](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1776) |
| 64 | | __builtin_carg missing (b3/b4) | **new** | [#1777](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1777) |
| 64 | | __builtin_bcmp missing (b3/b4) | **new** | [#1778](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1778) |
| 64 | | __builtin_bzero missing (b3/b4) | **new** | [#1779](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1779) |
| 64 | | __builtin__exit missing (b3/b4) | **new** | [#1780](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1780) |
| 64 | | __builtin_imaxabs missing (b3/b4) | **new** | [#1781](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1781) |
| 64 | | __builtin_setjmp missing (b3/b4) | **new** | [#1782](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1782) |
| 64 | | __builtin_longjmp missing (b3/b4) | **new** | [#1783](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1783) |
| 64 | | __builtin_apply_args missing (b3/b4) | **DUP→#1596 closed** | [#1784](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1784) |
| 64 | | __builtin_apply missing (b3/b4) | **DUP→#1596 closed** | [#1785](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1785) |
| 64 | | __builtin_return missing (b3/b4) | **DUP→#1596 closed** | [#1786](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1786) |
| 64 | | C95 digraphs unsupported | **new** | [#1787](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1787) |
| 64 | | u'' literal type int (4 not 2) | **new** | [#1788](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1788) |
| 64 | | bitfield width exceeds type accepted | **new** | [#1789](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1789) |
| 64 | | C23 enum fixed underlying type rejected | **new** | [#1790](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1790) |
| 64 | | default warnings absent: implicit-int/missing-return/unknown-escape | **new** | [#1791](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1791) |
| 64 | | .intel_syntax accepted but not implemented | **new** | [#1792](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1792) |
| 64 | | SSE3 intrinsics missing (_mm_addsub_ps) | **new** | [#1793](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1793) |
| 64 | | SSE4.1 blend intrinsics missing (_mm_blend_epi16) | **new** | [#1794](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1794) |
| 64 | | SSE4.1 dot-product intrinsics missing (_mm_dp_ps) | **new** | [#1795](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1795) |
| 64 | | POPCNT intrinsics missing (_mm_popcnt_u64) | **new** | [#1796](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1796) |
| 64 | | AVX intrinsics missing (_mm256_add_ps / _mm256_cvtss_f32) | **new** | [#1797](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1797) |
| 64 | | AVX2 gather intrinsics missing (_mm_i32gather_epi32) | **new** | [#1798](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1798) |
| 64 | | BMI tzcnt intrinsics missing (_tzcnt_u32) | **new** | [#1799](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1799) |
| 64 | | LZCNT intrinsics missing (_lzcnt_u32) | **new** | [#1800](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1800) |
| 64 | | F16C intrinsics missing (_mm_cvtps_ph) | **new** | [#1801](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1801) |
| 64 | | AVX-512F intrinsics missing (_mm512_add_epi32 / _mm512_mask_add_epi32 / _mm512_castsi512_si128) | **new** | [#1802](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1802) |
| 64 | | XOP intrinsics missing (_mm_roti_epi32) | **new** | [#1803](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1803) |
| 64 | | FMA4 intrinsics missing (_mm_macc_ps) | **new** | [#1804](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1804) |
| 64 | | intrinsic _mm_movedup_pd missing | **new** | [#1805](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1805) |
| 64 | | intrinsic _mm_minpos_epu16 missing | **new** | [#1806](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1806) |
| 64 | | intrinsic _mm_ceil_pd / _mm_ceil_ps missing | **new** | [#1807](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1807) |
| 64 | | intrinsic _mm_round_ps / _mm_round_pd missing | **new** | [#1808](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1808) |
| 64 | | intrinsic _mm256_sub_pd missing | **new** | [#1809](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1809) |
| 64 | | intrinsic _mm256_mul_ps missing | **new** | [#1810](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1810) |
| 64 | | intrinsic _mm256_zeroupper missing | **new** | [#1811](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1811) |
| 64 | | intrinsic _mm256_broadcastb_epi8 missing | **new** | [#1812](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1812) |
| 64 | | intrinsic _mm256_avg_epu8 missing | **new** | [#1813](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1813) |
| 64 | | intrinsic _blsr_u32 missing | **new** | [#1814](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1814) |
| 64 | | intrinsic _bextr_u32 missing | **new** | [#1815](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1815) |
| 64 | | intrinsic _tzcnt_u64 missing | **new** | [#1816](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1816) |
| 64 | | intrinsic _mm_lddqu_si128 missing | **new** | [#1817](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1817) |
| 64 | | intrinsic _mm256_cmp_ps + _mm256_movemask_ps missing | **new** | [#1818](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1818) |
| 64 | | intrinsic _mm256_blend_ps missing | **new** | [#1819](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1819) |
| 64 | | intrinsic _mm_dp_pd missing | **new** | [#1820](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1820) |
| 64 | | intrinsic _mm256_permute2f128_pd missing | **new** | [#1821](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1821) |
| 64 | | intrinsic _mm256_testz_si256 missing | **new** | [#1822](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1822) |
| 64 | | intrinsic _mm_fmsub_ps missing | **new** | [#1823](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1823) |
| 64 | | intrinsic _mm_fmaddsub_ps / _mm_fmsubadd_ps missing | **new** | [#1824](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1824) |
| 64 | | intrinsic _mm_fnmadd_ps / _mm_fnmsub_ps missing | **new** | [#1825](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1825) |
| 64 | | intrinsic _blsi_u64 missing | **new** | [#1826](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1826) |
| 64 | | intrinsic _blsmsk_u64 missing | **new** | [#1827](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1827) |
| 64 | | intrinsic _bextr_u64 missing | **new** | [#1828](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1828) |
| 64 | | intrinsic _mulx_u64 missing | **new** | [#1829](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1829) |
| 64 | | intrinsic _mm256_sad_epu8 missing | **new** | [#1830](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1830) |
| 64 | | intrinsic _mm256_sign_epi16 missing | **new** | [#1831](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1831) |
| 64 | | intrinsic _mm256_packus_epi32 missing | **new** | [#1832](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1832) |
| 64 | | intrinsic _andn_u64 missing | **new** | [#1833](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1833) |
| 64 | | intrinsic _mm_move_epi64 missing | **new** | [#1834](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1834) |
| 64 | | intrinsic _mm_mul_epi32 missing | **new** | [#1835](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1835) |
| 64 | | intrinsic _mm256_sqrt_ps missing | **new** | [#1836](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1836) |
| 64 | | intrinsic _mm256_max_ps / _mm256_min_ps missing | **new** | [#1837](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1837) |
| 64 | | intrinsic _mm256_and_ps (and _or_ps/_xor_ps) missing | **new** | [#1838](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1838) |
| 64 | | intrinsic _mm256_sllv_epi32 missing | **new** | [#1839](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1839) |
| 64 | | intrinsic _mm256_srlv_epi32 missing | **new** | [#1840](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1840) |
| 64 | | intrinsic _mm256_srav_epi32 missing | **new** | [#1841](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1841) |
| 64 | | intrinsic _mm256_movemask_ps missing | **new** | [#1842](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1842) |
| 64 | | intrinsic _mm256_cvtepi8_epi16 (and the 7 other sign-extend family members) missing | **new** | [#1843](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1843) |
| 64 | | intrinsic _mm_fmadd_ss (and the _ss scalar family) missing | **new** | [#1844](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1844) |
| 64 | | intrinsic _mm_fmadd_sd (and the _sd scalar family) missing | **new** | [#1845](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1845) |
| 64 | | intrinsic __readeflags / __writeeflags missing | **new** | [#1846](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1846) |
| 64 | | intrinsic _mm512_sub_epi32 missing | **new** | [#1847](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1847) |
| 64 | | intrinsic _mm512_mullo_epi32 missing | **new** | [#1848](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1848) |
| 64 | | intrinsic _mm512_mul_epu32 missing | **new** | [#1849](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1849) |
| 64 | | intrinsic _mm512_ternarylogic_epi32 missing | **new** | [#1850](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1850) |
| 64 | | intrinsic _mm512_maskz_add_epi32 missing | **new** | [#1851](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1851) |
| 64 | | intrinsic _mm512_min_epi32 / _mm512_max_epi32 missing | **new** | [#1852](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1852) |
| 64 | | intrinsic _mm512_srli_epi32 (and the shift family) missing | **new** | [#1853](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1853) |
| 64 | | intrinsic _mm512_cmpgt_epi32_mask missing | **new** | [#1854](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1854) |
| 64 | | intrinsic _mm512_add_ps missing | **new** | [#1855](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1855) |
| 64 | | intrinsic _mm512_mul_ps missing | **new** | [#1856](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1856) |
| 64 | | intrinsic _mm512_fmadd_ps missing | **new** | [#1857](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1857) |
| 64 | | intrinsic _mm512_cvtepi32_ps missing | **new** | [#1858](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1858) |
| 64 | | intrinsic _kand_mask16 / _kortestz_mask16_u8 (k-op intrinsics) missing | **new** | [#1859](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1859) |
| 64 | | intrinsic _mm512_add_epi8 missing | **new** | [#1860](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1860) |
| 64 | | intrinsic _mm512_add_epi16 missing | **new** | [#1861](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1861) |
| 64 | | intrinsic _mm512_madd_epi16 missing | **new** | [#1862](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1862) |
| 64 | | intrinsic _mm512_cvtepi8_epi16 (512-bit sign-extend family) missing | **new** | [#1863](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1863) |
| 64 | | intrinsic _mm256_mask_add_epi32 missing | **new** | [#1864](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1864) |
| 64 | | intrinsic _mm_gf2p8mul_epi8 missing | **new** | [#1865](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1865) |
| 64 | | intrinsic _mm_gf2p8affine_epi64_epi8 missing | **new** | [#1866](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1866) |
| 64 | | intrinsic _mm512_andnot_si512 missing | **new** | [#1867](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1867) |
| 64 | | intrinsic _mm512_madd52hi_epu64 missing | **new** | [#1868](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1868) |
| 64 | | intrinsic _mm512_madd52lo_epu64 missing | **new** | [#1869](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1869) |
| 64 | | intrinsic _mm512_permutexvar_epi8 missing | **new** | [#1870](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1870) |
| 64 | | intrinsic _mm512_popcnt_epi32 / _mm512_popcnt_epi64 missing | **new** | [#1871](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1871) |
| 64 | | intrinsic _mm512_dpbusd_epi32 / _mm512_dpbssd_epi32 missing | **new** | [#1872](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1872) |
| 64 | | intrinsic _rdpid_u32 missing | **new** | [#1873](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1873) |
| 64 | | intrinsic _mm_clwb / _mm_clflushopt intrinsic form missing | **new** | [#1874](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1874) |
| 64 | | NEON intrinsic vmulq_f32 missing | **new** | [#1875](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1875) |
| 64 | | NEON intrinsic vmulq_u8 missing | **new** | [#1876](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1876) |
| 64 | | NEON intrinsic vld1q_u32 missing | **new** | [#1877](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1877) |
| 64 | | NEON intrinsic vst1q_u32 missing | **new** | [#1878](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1878) |
| 64 | | NEON intrinsic vorrq_s32 missing | **new** | [#1879](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1879) |
| 64 | | NEON intrinsic vmaxq_s32 missing | **new** | [#1880](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1880) |
| 64 | | NEON intrinsic vminq_f32 missing | **new** | [#1881](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1881) |
| 64 | | NEON intrinsic vabsq_s16 missing | **new** | [#1882](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1882) |
| 64 | | NEON intrinsic vnegq_s32 missing | **new** | [#1883](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1883) |
| 64 | | NEON intrinsic vmlaq_f32 missing | **new** | [#1884](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1884) |
| 64 | | NEON intrinsic vfmaq_f32 missing | **new** | [#1885](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1885) |
| 64 | | NEON intrinsic vpaddq_u32 missing | **new** | [#1886](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1886) |
| 64 | | NEON intrinsic vaddvq_u32 missing | **new** | [#1887](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1887) |
| 64 | | NEON intrinsic vaddvq_s8 missing | **new** | [#1888](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1888) |
| 64 | | NEON intrinsic vmaxvq_f32 missing | **new** | [#1889](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1889) |
| 64 | | NEON intrinsic vcgtq_s32 missing | **new** | [#1890](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1890) |
| 64 | | NEON intrinsic vreinterpretq_f32_u32 missing | **new** | [#1891](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1891) |
| 64 | | NEON intrinsic vcvtq_f32_s32 missing | **new** | [#1892](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1892) |
| 64 | | NEON intrinsic vrndq_f32 missing | **new** | [#1893](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1893) |
| 64 | | NEON intrinsic vsqrtq_f32 missing | **new** | [#1894](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1894) |
| 64 | | NEON intrinsic vqaddq_s8 missing | **new** | [#1895](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1895) |
| 64 | | NEON intrinsic vqdmulhq_s16 missing | **new** | [#1896](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1896) |
| 64 | | NEON intrinsic vcvtq_u32_f32 missing | **new** | [#1897](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1897) |
| 64 | | NEON intrinsic vrecpeq_f32 missing | **new** | [#1898](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1898) |
| 64 | | NEON intrinsic vshrq_n_s64 missing | **new** | [#1899](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1899) |
| 64 | | NEON intrinsic vld2q_u32 missing (b2) | **new** | [#1900](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1900) |
| 64 | | NEON intrinsic vld3_s8 missing (b2) | **new** | [#1901](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1901) |
| 64 | | NEON intrinsic vst2_u16 missing (b2) | **new** | [#1902](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1902) |
| 64 | | NEON intrinsic vuzp1q_s16 missing (b2) | **new** | [#1903](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1903) |
| 64 | | NEON intrinsic vtbl1_s8 missing (b2) | **new** | [#1904](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1904) |
| 64 | | NEON intrinsic vtbx1_s8 missing (b2) | **new** | [#1905](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1905) |
| 64 | | NEON intrinsic vaddl_s16 missing (b2) | **new** | [#1906](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1906) |
| 64 | | NEON intrinsic vaddw_s16 missing (b2) | **new** | [#1907](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1907) |
| 64 | | NEON intrinsic vsubl_u16 missing (b2) | **new** | [#1908](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1908) |
| 64 | | NEON intrinsic vmull_s16 missing (b2) | **new** | [#1909](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1909) |
| 64 | | NEON intrinsic vmull_u8 missing (b2) | **new** | [#1910](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1910) |
| 64 | | NEON intrinsic vshll_n_s16 missing (b2) | **new** | [#1911](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1911) |
| 64 | | NEON intrinsic vrecpsq_f32 missing (b2) | **new** | [#1912](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1912) |
| 64 | | NEON intrinsic vclzq_s32 missing (b2) | **new** | [#1913](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1913) |
| 64 | | NEON intrinsic vclsq_s8 missing (b2) | **new** | [#1914](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1914) |
| 64 | | NEON intrinsic vpadalq_s16 missing (b2) | **new** | [#1915](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1915) |
| 64 | | NEON intrinsic vcvtq_n_s32_f32 missing (b2) | **new** | [#1916](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1916) |
| 64 | | NEON intrinsic vcvtq_n_f32_s32 missing (b2) | **new** | [#1917](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1917) |
| 64 | | NEON intrinsic vshlq_s64 missing (b2) | **new** | [#1918](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1918) |
| 64 | | NEON intrinsic vqshlq_s32 missing (b2) | **new** | [#1919](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1919) |
| 64 | | NEON intrinsic vpmaxq_f32 missing (b2) | **new** | [#1920](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1920) |
| 64 | | NEON intrinsic vcreate_s8 missing (b2) | **new** | [#1921](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1921) |
| 64 | | NEON intrinsic vld1q_f64 missing (b2) | **new** | [#1922](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1922) |
| 64 | | NEON intrinsic vcageq_f32 missing (b2) | **new** | [#1923](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1923) |
| 64 | | NEON intrinsic vaddq_f64 missing (b3) | **new** | [#1924](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1924) |
| 64 | | NEON intrinsic vsubq_f64 missing (b3) | **new** | [#1925](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1925) |
| 64 | | NEON intrinsic vmulq_f64 missing (b3) | **new** | [#1926](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1926) |
| 64 | | NEON intrinsic vmaxq_f64 missing (b3) | **new** | [#1927](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1927) |
| 64 | | NEON intrinsic vsqrtq_f64 missing (b3) | **new** | [#1928](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1928) |
| 64 | | NEON intrinsic vbicq_u32 missing (b3) | **new** | [#1929](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1929) |
| 64 | | NEON intrinsic vmvnq_u32 missing (b3) | **new** | [#1930](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1930) |
| 64 | | NEON intrinsic vornq_u32 missing (b3) | **new** | [#1931](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1931) |
| 64 | | NEON intrinsic vrev64q_u32 missing (b3) | **new** | [#1932](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1932) |
| 64 | | NEON intrinsic vqsubq_s8 missing (b3) | **new** | [#1933](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1933) |
| 64 | | NEON intrinsic vrshr_n_s32 missing (b3) | **new** | [#1934](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1934) |
| 64 | | NEON intrinsic vshlq_n_s64 missing (b3) | **new** | [#1935](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1935) |
| 64 | | NEON intrinsic vaddlvq_u32 missing (b3) | **new** | [#1936](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1936) |
| 64 | | NEON intrinsic vld1q_lane_u64 missing (b3) | **new** | [#1937](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1937) |
| 64 | | NEON intrinsic vqmovun_s32 missing (b3) | **new** | [#1938](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1938) |
| 64 | | NEON intrinsic vaddhn_s32 missing (b3) | **new** | [#1939](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1939) |
| 64 | | NEON intrinsic vrshrn_n_s32 missing (b3) | **new** | [#1940](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1940) |
| 64 | | NEON intrinsic vsraq_n_s32 missing (b3) | **new** | [#1941](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1941) |
| 64 | | NEON intrinsic vaddq_f16 missing (b3) | **new** | [#1942](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1942) |
| 64 | | NEON intrinsic vmulq_f16 missing (b3) | **new** | [#1943](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1943) |
| 64 | | NEON intrinsic vcvt_f64_f32 missing (b3) | **new** | [#1944](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1944) |
| 64 | | NEON intrinsic vcvt_f32_f64 missing (b3) | **new** | [#1945](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1945) |
| 64 | | NEON intrinsic vld1_u32 missing (b4) | **new** | [#1946](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1946) |
| 64 | | NEON intrinsic vld1_s8 missing (b4) | **new** | [#1947](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1947) |
| 64 | | NEON intrinsic vst1_u32 missing (b4) | **new** | [#1948](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1948) |
| 64 | | NEON intrinsic vst1q_lane_u32 missing (b4) | **new** | [#1949](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1949) |
| 64 | | NEON intrinsic vsubhn_s32 missing (b4) | **new** | [#1950](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1950) |
| 64 | | NEON intrinsic vabdq_s32 missing (b4) | **new** | [#1951](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1951) |
| 64 | | NEON intrinsic vabdq_f32 missing (b4) | **new** | [#1952](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1952) |
| 64 | | NEON intrinsic vceqzq_s32 missing (b4) | **new** | [#1953](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1953) |
| 64 | | NEON intrinsic vcgtzq_f32 missing (b4) | **new** | [#1954](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1954) |
| 64 | | NEON intrinsic vcaleq_f32 missing (b4) | **new** | [#1955](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1955) |
| 64 | | NEON intrinsic vpadd_f32 missing (b4) | **new** | [#1956](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1956) |
| 64 | | NEON intrinsic vpmax_f32 missing (b4) | **new** | [#1957](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1957) |
| 64 | | NEON intrinsic vpmin_s32 missing (b4) | **new** | [#1958](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1958) |
| 64 | | NEON intrinsic vqshlq_n_s32 missing (b4) | **new** | [#1959](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1959) |
| 64 | | NEON intrinsic vqshrun_n_s32 missing (b4) | **new** | [#1960](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1960) |
| 64 | | NEON intrinsic vqrshrn_n_s32 missing (b4) | **new** | [#1961](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1961) |
| 64 | | NEON intrinsic vshr_n_s8 missing (b4) | **new** | [#1962](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1962) |
| 64 | | NEON intrinsic vshl_s8 missing (b4) | **new** | [#1963](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1963) |
| 64 | | NEON intrinsic vnegq_f64 missing (b4) | **new** | [#1964](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1964) |
| 64 | | NEON intrinsic vabsq_f64 missing (b4) | **new** | [#1965](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1965) |
| 64 | | NEON intrinsic vrecpe_f32 missing (b4) | **new** | [#1966](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1966) |
| 64 | | NEON intrinsic vrsqrte_f32 missing (b4) | **new** | [#1967](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1967) |
| 64 | | NEON intrinsic vrsqrtsq_f32 missing (b4) | **new** | [#1968](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1968) |
| 64 | | NEON intrinsic vdupq_n_s64 missing (b4) | **new** | [#1969](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1969) |
| 64 | | NEON intrinsic vdup_n_f64 missing (b4) | **new** | [#1970](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1970) |
| 64 | | NEON intrinsic vsetq_lane_f64 missing (b4) | **new** | [#1971](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1971) |
| 64 | | NEON intrinsic vcopyq_laneq_s32 missing (b4) | **new** | [#1972](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1972) |
| 64 | | NEON intrinsic vcvtmq_s32_f32 missing (b4) | **new** | [#1973](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1973) |
| 64 | | NEON intrinsic vrndmq_f32 missing (b4) | **new** | [#1974](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1974) |
| 64 | | NEON intrinsic vrndpq_f32 missing (b4) | **new** | [#1975](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1975) |
| 64 | | NEON intrinsic vrndaq_f32 missing (b4) | **new** | [#1976](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1976) |
| 64 | | NEON intrinsic vmaxnmq_f32 missing (b4) | **new** | [#1977](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1977) |
| 64 | | NEON intrinsic vminnmq_f64 missing (b4) | **new** | [#1978](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1978) |
| 64 | | NEON intrinsic vreinterpretq_u32_s32 missing (b4) | **new** | [#1979](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1979) |
| 64 | | NEON intrinsic vaddl_high_s16 missing (b5) | **new** | [#1980](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1980) |
| 64 | | NEON intrinsic vmull_high_s16 missing (b5) | **new** | [#1981](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1981) |
| 64 | | NEON intrinsic vaddw_high_s16 missing (b5) | **new** | [#1982](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1982) |
| 64 | | NEON intrinsic vsubl_high_u16 missing (b5) | **new** | [#1983](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1983) |
| 64 | | NEON intrinsic vshll_high_n_s16 missing (b5) | **new** | [#1984](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1984) |
| 64 | | NEON intrinsic vmlal_s16 missing (b5) | **new** | [#1985](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1985) |
| 64 | | NEON intrinsic vmlsl_s16 missing (b5) | **new** | [#1986](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1986) |
| 64 | | NEON intrinsic vmlsq_f32 missing (b5) | **new** | [#1987](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1987) |
| 64 | | NEON intrinsic vfmaq_lane_f32 missing (b5) | **new** | [#1988](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1988) |
| 64 | | NEON intrinsic vmlaq_lane_f32 missing (b5) | **new** | [#1989](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1989) |
| 64 | | NEON intrinsic vld1_dup_u32 missing (b5) | **new** | [#1990](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1990) |
| 64 | | NEON intrinsic vqabsq_s8 missing (b5) | **new** | [#1991](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1991) |
| 64 | | NEON intrinsic vqnegq_s8 missing (b5) | **new** | [#1992](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1992) |
| 64 | | NEON intrinsic vrshlq_s32 missing (b5) | **new** | [#1993](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1993) |
| 64 | | NEON intrinsic vrshrq_n_s32 missing (b5) | **new** | [#1994](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1994) |
| 64 | | NEON intrinsic vshrn_n_s16 missing (b5) | **new** | [#1995](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1995) |
| 64 | | NEON intrinsic vext_u16 missing (b5) | **new** | [#1996](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1996) |
| 64 | | NEON intrinsic vzip1_u32 missing (b5) | **new** | [#1997](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1997) |
| 64 | | NEON intrinsic vuzp1_u32 missing (b5) | **new** | [#1998](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1998) |
| 64 | | NEON intrinsic vtrn1_s16 missing (b5) | **new** | [#1999](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1999) |
| 64 | | NEON intrinsic vrev64q_u8 missing (b5) | **new** | [#2000](https://github.com/fermat-hkrc/claudes-c-compiler/issues/2000) |
| 64 | | NEON intrinsic vabal_s16 missing (b5) | **new** | [#2001](https://github.com/fermat-hkrc/claudes-c-compiler/issues/2001) |
| 64 | | NEON intrinsic vcagtq_f32 missing (b5) | **new** | [#2002](https://github.com/fermat-hkrc/claudes-c-compiler/issues/2002) |
| 64 | | NEON intrinsic vcaltq_f32 missing (b5) | **new** | [#2003](https://github.com/fermat-hkrc/claudes-c-compiler/issues/2003) |
| 64 | | NEON intrinsic vmulq_s8 missing (b6) | **new** | [#2004](https://github.com/fermat-hkrc/claudes-c-compiler/issues/2004) |
| 64 | | NEON intrinsic vmulq_s32 missing (b6) | **new** | [#2005](https://github.com/fermat-hkrc/claudes-c-compiler/issues/2005) |
| 64 | | NEON intrinsic vaddq_s64 missing (b6) | **new** | [#2006](https://github.com/fermat-hkrc/claudes-c-compiler/issues/2006) |
| 64 | | NEON intrinsic vsubq_s64 missing (b6) | **new** | [#2007](https://github.com/fermat-hkrc/claudes-c-compiler/issues/2007) |
| 64 | | NEON intrinsic vandq_s64 missing (b6) | **new** | [#2008](https://github.com/fermat-hkrc/claudes-c-compiler/issues/2008) |
| 64 | | NEON intrinsic vmulq_n_f32 missing (b6) | **new** | [#2009](https://github.com/fermat-hkrc/claudes-c-compiler/issues/2009) |
| 64 | | NEON intrinsic vmlaq_n_s32 missing (b6) | **new** | [#2010](https://github.com/fermat-hkrc/claudes-c-compiler/issues/2010) |
| 64 | | NEON intrinsic vqdmulhq_lane_s16 missing (b6) | **new** | [#2011](https://github.com/fermat-hkrc/claudes-c-compiler/issues/2011) |
| 64 | | NEON intrinsic vqrdmulhq_s16 missing (b6) | **new** | [#2012](https://github.com/fermat-hkrc/claudes-c-compiler/issues/2012) |
| 64 | | NEON intrinsic vqdmlal_s16 missing (b6) | **new** | [#2013](https://github.com/fermat-hkrc/claudes-c-compiler/issues/2013) |
| 64 | | NEON intrinsic vqdmlsl_s16 missing (b6) | **new** | [#2014](https://github.com/fermat-hkrc/claudes-c-compiler/issues/2014) |
| 64 | | NEON intrinsic vdupq_lane_u32 missing (b6) | **new** | [#2015](https://github.com/fermat-hkrc/claudes-c-compiler/issues/2015) |
| 64 | | NEON intrinsic vtst_s32 missing (b6) | **new** | [#2016](https://github.com/fermat-hkrc/claudes-c-compiler/issues/2016) |
| 64 | | NEON intrinsic vceq_s8 missing (b6) | **new** | [#2017](https://github.com/fermat-hkrc/claudes-c-compiler/issues/2017) |
| 64 | | NEON intrinsic vcgeq_f32 missing (b6) | **new** | [#2018](https://github.com/fermat-hkrc/claudes-c-compiler/issues/2018) |
| 64 | | NEON intrinsic vcleq_f32 missing (b6) | **new** | [#2019](https://github.com/fermat-hkrc/claudes-c-compiler/issues/2019) |
| 64 | | NEON intrinsic vcltq_f32 missing (b6) | **new** | [#2020](https://github.com/fermat-hkrc/claudes-c-compiler/issues/2020) |
| 64 | | NEON intrinsic vneg_f64 missing (b6) | **new** | [#2021](https://github.com/fermat-hkrc/claudes-c-compiler/issues/2021) |
| 64 | | NEON intrinsic vabs_f64 missing (b6) | **new** | [#2022](https://github.com/fermat-hkrc/claudes-c-compiler/issues/2022) |
| 64 | | NEON intrinsic vshlq_u8 missing (b6) | **new** | [#2023](https://github.com/fermat-hkrc/claudes-c-compiler/issues/2023) |
| 64 | | NEON intrinsic vabal_u16 missing (b6) | **new** | [#2024](https://github.com/fermat-hkrc/claudes-c-compiler/issues/2024) |
| 64 | | NEON intrinsic vsubw_s16 missing (b6) | **new** | [#2025](https://github.com/fermat-hkrc/claudes-c-compiler/issues/2025) |
| 64 | | NEON intrinsic vshll_n_u16 missing (b6) | **new** | [#2026](https://github.com/fermat-hkrc/claudes-c-compiler/issues/2026) |
| 64 | | NEON intrinsic vrsra_n_s32 missing (b6) | **new** | [#2027](https://github.com/fermat-hkrc/claudes-c-compiler/issues/2027) |
| 64 | | NEON intrinsic vpaddl_u8 missing (b6) | **new** | [#2028](https://github.com/fermat-hkrc/claudes-c-compiler/issues/2028) |
| 64 | | NEON intrinsic vpadal_s16 missing (b6) | **new** | [#2029](https://github.com/fermat-hkrc/claudes-c-compiler/issues/2029) |
| 64 | | __builtin_tanl missing (l-backfill) | **new** | [#2030](https://github.com/fermat-hkrc/claudes-c-compiler/issues/2030) |
| 64 | | __builtin_asinl missing (l-backfill) | **new** | [#2031](https://github.com/fermat-hkrc/claudes-c-compiler/issues/2031) |
| 64 | | __builtin_acosl missing (l-backfill) | **new** | [#2032](https://github.com/fermat-hkrc/claudes-c-compiler/issues/2032) |
| 64 | | __builtin_atanl missing (l-backfill) | **new** | [#2033](https://github.com/fermat-hkrc/claudes-c-compiler/issues/2033) |
| 64 | | __builtin_sinhl missing (l-backfill) | **new** | [#2034](https://github.com/fermat-hkrc/claudes-c-compiler/issues/2034) |
| 64 | | __builtin_coshl missing (l-backfill) | **new** | [#2035](https://github.com/fermat-hkrc/claudes-c-compiler/issues/2035) |
| 64 | | __builtin_tanhl missing (l-backfill) | **new** | [#2036](https://github.com/fermat-hkrc/claudes-c-compiler/issues/2036) |
| 64 | | __builtin_expm1l missing (l-backfill) | **new** | [#2037](https://github.com/fermat-hkrc/claudes-c-compiler/issues/2037) |
| 64 | | __builtin_log1pl missing (l-backfill) | **new** | [#2038](https://github.com/fermat-hkrc/claudes-c-compiler/issues/2038) |
| 64 | | __builtin_log10l missing (l-backfill) | **new** | [#2039](https://github.com/fermat-hkrc/claudes-c-compiler/issues/2039) |
| 64 | | __builtin_cbrtl missing (l-backfill) | **new** | [#2040](https://github.com/fermat-hkrc/claudes-c-compiler/issues/2040) |
| 64 | | __builtin_tgammal missing (l-backfill) | **new** | [#2041](https://github.com/fermat-hkrc/claudes-c-compiler/issues/2041) |
| 64 | | __builtin_erfl missing (l-backfill) | **new** | [#2042](https://github.com/fermat-hkrc/claudes-c-compiler/issues/2042) |
| 64 | | __builtin_erfcl missing (l-backfill) | **new** | [#2043](https://github.com/fermat-hkrc/claudes-c-compiler/issues/2043) |
| 64 | | __builtin_fmal missing (l-backfill) | **new** | [#2044](https://github.com/fermat-hkrc/claudes-c-compiler/issues/2044) |
| 64 | | intrinsic _mm512_maskz_loadu_epi8 missing (backfill) | **new** | [#2045](https://github.com/fermat-hkrc/claudes-c-compiler/issues/2045) |
| 64 | | intrinsic _mm512_cvtepi16_epi32 missing (backfill) | **new** | [#2046](https://github.com/fermat-hkrc/claudes-c-compiler/issues/2046) |
| 64 | | intrinsic _mm512_shuffle_i32x4 missing (backfill) | **new** | [#2047](https://github.com/fermat-hkrc/claudes-c-compiler/issues/2047) |
| 64 | | intrinsic _mm256_cvtph_ps missing (backfill) | **new** | [#2048](https://github.com/fermat-hkrc/claudes-c-compiler/issues/2048) |
| 64 | | intrinsic _mm256_cvtps_ph missing (backfill) | **new** | [#2049](https://github.com/fermat-hkrc/claudes-c-compiler/issues/2049) |
| 64 | | NEON intrinsic vdotq_s32 missing (backfill) | **new** | [#2050](https://github.com/fermat-hkrc/claudes-c-compiler/issues/2050) |
| 64 | | NEON intrinsic vdotq_u32 missing (backfill) | **new** | [#2051](https://github.com/fermat-hkrc/claudes-c-compiler/issues/2051) |
| 64 | | NEON intrinsic vmmlaq_s32 missing (backfill) | **new** | [#2052](https://github.com/fermat-hkrc/claudes-c-compiler/issues/2052) |
| 64 | | NEON intrinsic vusdotq_s32 missing (backfill) | **new** | [#2053](https://github.com/fermat-hkrc/claudes-c-compiler/issues/2053) |
| 64 | | NEON intrinsic vsm4eq_u32 missing (backfill) | **new** | [#2054](https://github.com/fermat-hkrc/claudes-c-compiler/issues/2054) |
| 64 | | NEON intrinsic vsm4ekeyq_u32 missing (backfill) | **new** | [#2055](https://github.com/fermat-hkrc/claudes-c-compiler/issues/2055) |
| 64 | | NEON intrinsic vbfdotq_f32 missing (backfill) | **new** | [#2056](https://github.com/fermat-hkrc/claudes-c-compiler/issues/2056) |
