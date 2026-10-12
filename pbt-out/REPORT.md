# Property-Based Testing Campaign — Final Report

**Repo:** fermat-hkrc/claudes-c-compiler (a C compiler in Rust: C front end → IR → x86-64 / i686 / AArch64 / RISC-V backends + own assembler + linker)
**Campaign window:** rounds 01–64 (64 sweep rounds, 57 round directories)
**Issue tracker at close:** 2028 issues, all filed by this campaign. Source files modified in repo: **0**.

## Summary

64 rounds of property-based testing against the compiler, assembler, linker, and runtime interface. Every filed issue was surfaced by a **failing differential property** (byte-exact encode matrices vs gas/gcc/clang/aarch64-as, link-level builtin/intrinsic properties, native execution properties) and shrunk to a minimal counterexample before filing; each issue body carries the witness (`Expected`/`Actual`) and a `## Found by` reproduce block.

Coverage went deep then wide: rounds 01–14 swept front end/IR/passes; 15–29 deep encoder matrices (i686 twins); 30–59 enumerated and exercised **all 416 `encode_*` functions**; 60–64 mined new layers — TLS/ELF/runtime (root-caused the `.tdata` 8-mod-16 law), C23 conformance, inline-asm extensions, `__builtin_*` link-level completion (~73), and the intrinsic headers (75 x86 + 154 NEON).

Analysis (non-bug observation): the SIMD/FP surface has **three independent layers** — assembler tables, builtin dispatch, intrinsic headers — with non-overlapping gap sets: 12/12 sampled NEON intrinsics absent from `include/arm_neon.h` had working assembler encodings. Volume: 920 ledger rows (734 **new**); round 62 = 259 (encoder wave), round 64 = 441 rows → 442 filed, 439 surviving (3 closed as dups of #1596 by self-audit).

## Modules Tested

| Module (source area) | Property battery | Oracle | Outcome |
|---|---|---|---|
| `src/frontend/lexer` | token-coverage properties (digraphs, C23 keywords, separators, u/U/u8 literals, escapes) | gcc / clang-21 | failing counterexamples filed |
| `src/frontend/parser` | grammar-acceptance properties (K&R, nested fn, VLA forms, bit-field width, enum `:` type, attributes) | gcc / clang-21 | width>type accepted, enum-`:` rejected — filed |
| `src/frontend/sema` | C23 type properties, default-warning properties, inline models | gcc / clang-21 | gaps filed; C99 inline/restrict/always_inline pass |
| `src/ir` + `src/passes` | const-fold / narrow / div-by-const properties (rounds 01–14) | gcc -O0/-O2 | saturated |
| `src/backend/x86` (118 encode fns) | byte-exact matrices: legacy/SSE/AVX/AVX-512/BMI/GFNI/XOP/FMA4/system/x87; LOCK/REP legality | gcc-as / clang-21 | hundreds of byte counterexamples filed |
| `src/backend/arm` (189 fns) | NEON/sys/ext matrices with `.arch` gating; immediate-range properties | aarch64-linux-gnu-as 2.34 | filed; oracle rendering caveat in ## Reproduction |
| `src/backend/riscv` (117 fns) | IMAFD/CSR/Zbb/Zacas/H properties; li-equivalence; `.insn` | clang-21 --target=riscv64 | only missing-extension gaps remain |
| `src/backend/elf_writer*` + linker | PT_TLS layout, PT_GNU_RELRO, .eh_frame CFI, PIE types, -z,now, -M deps, weak/tentative symbols, -S/-E round-trips | gcc / readelf / native run | root-caused failures filed |
| Runtime interface (TLS, dlopen, backtrace, ucontext, fork, ctors) | native execution differential properties | gcc-built binaries | #1692/#1693/#1696 vein |
| GNU builtins dispatch | 300+ link-level properties (compile-only insufficient: implicit-decl is a warning) | gcc -lm / clang-21 -lm | ~73 failing probes filed |
| Intrinsic headers (system immintrin.h + repo `include/arm_neon.h`) | 6 x86 batches (~230) + 6 NEON batches (~190) link/run properties, header-grep cross-check | gcc flags / clang-21 cross | 75 + 154 failing probes filed |
| Drivers (`ccc`, `ccc-arm`, `ccc-riscv`, `ccc-i686`) | flag-effect properties (-O output identity, -march validation, -pie type, syntax-switch, @file, stdin) | gcc / readelf | no-op failures filed / annotated |

## Bugs Found

All 2028 tracker issues originate from failing, shrunk properties. Witness = shrunk counterexample; reproduce = minimal command. Full corpus: `pbt-out/bug_reports/FILED_ISSUES.md` → issue links (each carries Expected/Actual + `## Found by`).

**Silent miscompile / wrong data (HIGH):**
- **#1692** TLS initializers read as 0; writes land in adjacent module TLS. witness: `__thread int tvar=7; __thread char buf[64]; main(){return tvar;}` → oracle exit 7, ccc exit 0 (4/4 runs); law pinned: PT_TLS p_vaddr % 16 == 8 fails, % 16 == 0 passes; `&tvar-FS == -80` identical to gcc (copy lost, not addressing). reproduce: `ccc tlsmn.c -o c && ./c  # tvar=0` vs `gcc … # tvar=7`.
- **#1693** `-shared -fPIC` `.so` drops all TLS dynamic relocations. witness: dlopen + `getlib()` → gcc 119, ccc 0; `objdump -R/-r libtlss.so | grep -i tls` empty. reproduce: build both `.so`s, run dlopen main.
- **#1698** `asm goto` fallthrough skips the next statement. witness: `probe(5)` → −1 (ccc) vs 100 (gcc). reproduce: 6-line probe in issue.
- **#1700** inline-asm `"x"` constraint input never materialized. witness: `movss %1,%0 :"=x"(g):"x"(f)`, f=1.5 → 0.00 vs 1.50; disassembly shows the constant never reaching any xmm.
- **#1702** C23 `auto v = 1.5` silently int. witness: `sizeof(v)` → 4 vs 8 (clang-21 -std=c23), truncated value, zero diagnostics.
- **#1628** x87 `fsub %st,%st(1)` → `dce9` vs gas `dce1` (one-instruction witness).
- **#1632** AArch64 `fcmp d0, #0.0` → `1f2003d5` (NOP) — single-instruction byte diff.
- **#1490** mulx/pdep/pext swap sources: `c4e263f6c8` vs `c4e27bf6cb`; reconfirmed this round.

**Encoder-table byte counterexamples (rounds 15–62, ~450 issues):** one-instruction witnesses each (`gas=<bytes> ccc=<bytes|rejected>`), e.g. #1660 lcall, #1661 DR MOVs, #1683 LOOP/jcxz (i686 drops 0x67), #1634–#1649 LSE 16 singles, #1650–#1659 dotprod/complex/FP16/flag-ops, #1662–#1674 GFNI/VAES/BCD/XOP, #1687–#1690 RV CSR names, #1682 LOCK/REP illegal combos accepted.

**Intrinsic-header layer (witness per issue = oracle links/runs, ccc `undefined symbol`, and `grep -w <name> include/arm_neon.h` → no match):** 75 x86 (e.g. #1796 `_mm_popcnt_u64`; #1847–#1866 AVX-512; #1817–#1833 AVX cmp/blend/permute + BMI64; #1865–#1866 GFNI) and 154 NEON (e.g. #1877 vld1q_u32, #1878 vst1q_u32, #1909 vmull_s16, #1924 vaddq_f64, #1942 vaddq_f16, #1980 vaddl_high_s16).

**Builtin dispatch (~90):** witness = gcc/clang `-lm` link success vs ccc `undefined symbol __builtin_X`; e.g. #1713 exp2, #1731 trunc, #1775 cabs, #1782 setjmp, #1769/#1770 strpbrk/stpncpy.

**C23 / conformance:** witnesses are accept/run diffs vs clang-21 -std=c23 (or gcc): #1691 _Float16/__fp16, #1703 _BitInt, #1704 constexpr, #1705 [[attrs]], #1707 nullptr, #1709 digit-sep, #1710 #elifdef wrong branch (x=3 vs 2), #1711 #embed zero bytes (0 0 vs 65 66), #1787 digraphs rejected, #1788 sizeof(u'x') 4 vs 2, #1789 `int a:33` accepted, #1790 enum `:` rejected.

**Toolchain/hardening:** #1695 no PT_GNU_RELRO (readelf witness), #1696 .eh_frame FDEs pure DW_CFA_nop → backtrace() 1 vs 6, #1697 `-Wl,-z,now` no-op (dynamic section empty vs BIND_NOW), #1792 `.intel_syntax` accepted-then-rejected, #1767 tls_model ignored under -fPIC (GOTTPOFF vs TPOFF32).

**Author-documented limitations, reclassified per audit rule** (Contract evidence + author quote; Severity one step down, "documented by the author"):
- **#1535** — all `-O` levels byte-identical. Contract evidence: documented limitation (`README.md:143-145`): "**Optimization levels**: All levels (`-O0` through `-O3`, `-Os`, `-Oz`) run the same optimization pipeline. Separate tiers will be added as the compiler matures." Severity: medium — documented by the author. (Campaign witness: 4-way `cmp` of binaries identical; gcc differs.)
- **#1691** — `_Float16`/`__fp16`/`f16` unrecognized. Contract evidence: documented limitation (`src/frontend/preprocessor/predefined_macros.rs:137-140`): "TODO: Implement _Float* as proper builtin types with correct semantics … The macro approach works for glibc header compatibility but loses precision." Severity: low — documented by the author. (Campaign witness: parse error vs clang accept.)
- **#1534/#1161** — `-M` lists only the source file. Contract evidence: documented limitation (`src/driver/pipeline.rs:346-348`): "TODO: Currently only lists the source file as a dependency. A full implementation should preprocess and list all #included headers." Severity: medium (from HIGH) — documented by the author. (Campaign witness: output `dep.o: dep.c`, dep.h missing.)
- **#1875–#2029 NEON-intrinsics corpus** — Contract evidence: documented limitation (`README.md:149-150`): "GNU extensions: Partial `__attribute__` support. NEON intrinsics are partially implemented (core 128-bit operations work)." Severity: low (from medium), documented by the author. **The code contradicts the quote's second half**: the campaign's failing witnesses include core 128-bit operations themselves (`vaddq_f64` #1924, `vmulq_f32` #1875, `vmaxq_s32` #1880, `vst1q_u32` #1878), so "core 128-bit operations work" understates the gap.
- **_Atomic semantics** — Contract evidence: documented limitation (`README.md:151-152`): "**Atomics**: `_Atomic` is parsed but treated as the underlying type (the qualifier is not tracked through the type system)." Severity: low — documented by the author. Campaign probes: parse and single-threaded `x++`/`+=`/`__atomic_*` builtins all pass; single-threaded properties cannot refute the documented non-atomic lowering, so this stays as the author states (no separate issue; honest note).

**Self-audit outcomes:** #1784–#1786 closed as dups of #1596; #1598 body corrected (issignaling actually fails → #1761); #1791's missing-return claim retracted by editing title+body after a clean-directory retest inverted it.

## Design Caveats

None.

## Test Files Created

- `pbt-out/scratch/` — repo-local scratch root (quarantine after multi-user /tmp collisions: `rg`, `p1`, `bt`, `sa`, `rv` name crashes logged).
- Byte-matrix harnesses: `gen.py` (x86), `genarm.py` (AArch64, `.arch` gating), `genrv.py` (riscv64, `-march=rv64ima` no-c rule), `gen32.py` (i686); helpers `bytes_of` / `diff_batch`.
- Link-level matrices: `btm/iso*.py` — 300+ `__builtin_*` names (gcc/clang `-lm` oracles; compile-only probing established as false green because ccc demotes unknown builtins to implicit declarations — itself filed behavior).
- Intrinsic matrices: `ivv*.py` (x86, 6 batches), `nv*.py` (NEON, 6 batches; clang cross-oracle + `grep include/arm_neon.h` validation).
- 25-issue random re-validation sweep script (clean-directory retests; produced the #1791 retraction).
- 57 round directories under `pbt-out/rounds/<NN>_<name>/` (PLAN.md, PROPERTIES.md, REPORT.md, bug_reports/).
- Ledger: `pbt-out/bug_reports/FILED_ISSUES.md` (920 round-tagged rows, dup-marked).

## Reproduction

Every issue carries a `## Found by` block; canonical property forms:

```bash
# Byte-exact encode property (rounds 15+)
printf '.text\n<insn>\n' > t.s
gcc -c t.s -o a.o && objdump -s -j .text a.o        # oracle bytes
./target/debug/ccc t.s -c -o b.o && objdump -s -j .text b.o

# Builtin / intrinsic link property
printf '#include <immintrin.h>\nint probe(void){ <intrinsic>; }\nint main(void){return probe();}\n' > t.c
gcc -m<flag> t.c -o a -w          # witness: links+runs
./target/debug/ccc -m<flag> t.c -o b   # counterexample: undefined symbol

# NEON intrinsic property (cross-oracle + header grep)
clang-21 --target=aarch64-linux-gnu -c t.c -o a -w
./target/debug/ccc-arm t.c -o b
grep -w <name> include/arm_neon.h

# TLS runtime property (#1692)
printf '#include <stdio.h>\n__thread int tvar=7;\n__thread char buf[64];\nint main(){printf("tvar=%d\n",tvar);}\n' > t.c
gcc t.c -o g && ./g               # tvar=7
./target/debug/ccc t.c -o c && ./c # tvar=0
```

Environment/oracle constraints (test-rig facts, quoted verbatim):
- `aarch64-linux-gnu-as --version` → `GNU assembler (GNU Binutils for Ubuntu) 2.34` — rejects pauth/MTE/bf16/rmif mnemonics; those surfaces were skipped (no oracle), never reported clean.
- `gcc --version` → `gcc (Ubuntu 9.4.0-1ubuntu1~20.04.2) 9.4.0`; `gcc -std=c23` → `error: unrecognized command line option '-std=c23'` — clang-21 (`Ubuntu clang version 21.1.5 …`) is the C23 oracle.
- `command -v qemu-aarch64` → absent; `command -v aarch64-linux-gnu-gcc` → absent — ARM findings are compile/link-level only.
- binutils `objdump -r` does not render AArch64 TLSLE relocations on this build; `llvm-readobj -r` does — all ARM relocation verdicts use llvm-readobj (a false "zero relocations" alarm was caught this way before filing).

Skip-rules (equivalent-encoding classes, verified oracle-dual-accept with identical semantics before skipping; methodology rules, not SUT claims): REX.W on segment/BT ops, imm8-vs-imm32 sizing, RV `li` expansion variants (both sequences decode to the identical value), NOP-fill variants, branch bytes pending R_RISCV_BRANCH/JAL relocation with zero immediate (legal ELF convention; `readelf -r` check mandatory before filing any branch byte-diff).

## Output Directories

- `pbt-out/` — campaign root
- `pbt-out/bug_reports/` — `FILED_ISSUES.md`, per-bug markdown/html, `issue_bodies.jsonl`, `issue_db.tsv`
- `pbt-out/rounds/` — 57 round directories (`01_*` … `64_*`)
- `pbt-out/scratch/` — repo-local harness scratch
- `pbt-out/COVERAGE.md`, `pbt-out/COVERAGE_STATUS.md`, `pbt-out/FUNCTION_INDEX.md`, `pbt-out/INVARIANTS.md`, `pbt-out/dependencies.json`
- `pbt-out/REPORT.md` — this report
