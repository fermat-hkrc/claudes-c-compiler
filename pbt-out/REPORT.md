# PBT Campaign Report: Whole-repo deep loop (rounds 01-59)

## Summary

**Verdict:** 180 unique issues filed (#516-#1204). 59 rounds across all src/ modules, 15 testing levels, 162+ distinct mines. The compiler's core semantics (IR, optimizer, codegen) are remarkably solid — zero bugs found across 40+ consecutive mines on semantic correctness. Bugs concentrate in: lexer validation (~100 gaps), preprocessor constraints (~80 forms), ABI classification (4 backends), driver flags (10 defects), and compiler infrastructure (5 ICE/DoS/non-determinism).

**Date:** 2026-10-05 through 2026-10-07
**Repository:** claudes-c-compiler (/home/shuhao/fermat-users/leo/github/claudes-c-compiler)
**Modules tested:** ALL src/ modules (3 full loops + 30 deep-pass rounds + 27 untested-surface rounds)
**Tests:** 874 test functions · 27 resident property groups · 60,000+ encode cases · 10,000+ randomized constraint/linker/parser fuzz · 4,000 self-differential · 1,000 cross-target qemu · valgrind
**Result:** 180 unique issues + 14 found_already (deduped)
**Tier:** standard

## Modules Tested

| Module | Rounds | Issues | Oracles |
|--------|--------|--------|---------|
| src/common | 01, 08, 24, 27, 31 | #516-518, #765, #806, #1084 | reference + differential |
| src/frontend/lexer | 02, 24, 51, 57 | #526-530, #1175-#1180, #1191-#1194 | gcc token stream |
| src/frontend/preprocessor | 03, 24, 32, 56 | #538-540, #1088-#1089, #1143, #1187-#1197 | gcc -E differential |
| src/frontend/parser | 04, 18, 59 | #799, #848 | gcc -fsyntax-only + fuzz |
| src/frontend/sema | 05, 23, 56 | #559-563, #1089-#1090, #1195-#1197 | gcc differential |
| src/ir | 06, 25, 31 | #573-576, #1087, #1144, #1147 | unit PBT + E2E |
| src/passes | 07, 36, 42 | #759-760, #1115 | self-diff + gcc |
| src/backend root | 08, 38 | #765, #1114-#1119 | interop + qemu |
| src/backend/x86 | 09, 15, 26 | #774-777 | .s differential |
| src/backend/arm | 10, 16, 26 | #791-795, #1118 | .s differential + qemu |
| src/backend/riscv | 11, 17, 26 | #796-798, #1119 | .s differential + qemu |
| src/backend/i686 | 12, 19, 52 | #770, #1183 | E2E + ICE |
| src/backend/elf | 13, 35 | #1092 | readelf structural |
| src/backend/linker_common | 20, 33, 46, 59 | #1166, #1198 | E2E + randomized fuzz |
| src/driver | 14, 35, 45 | #1153-#1165 | flag differential |

## Bugs Found

### Compiler Infrastructure (highest severity)
- **#1183** i686 ICE: float→long double panics (subtract overflow at long_double.rs:1040; trigger: |value| < 1.0)
- **#1185** ALL backends non-deterministic .o files (HashMap iteration in symbol_table.rs:68; .s deterministic but .o varies per run)
- **#1186** Stack overflow ICE from 6 constructs (macro chain ~2500, expression parens ~3000, if/while ~3000, for ~2000, braces ~5000)
- **#1187** 100-file circular include HANGS compiler (DoS; gcc completes immediately)

### ABI Classification (4 backends)
- **#1114** x86-64: SSE ≤8B struct returns in RAX not XMM0
- **#1116** x86-64: {long double} X87-class struct in RAX:RDX not st(0)
- **#1118** ARM64: HFA structs in X0/W0 integer regs (need S/V) — all 8 directions broken
- **#1119** RISC-V: FP struct returns via integer a0 (need fa0/fa1; {f,f} drops member b)

### Semantic / Codegen
- **#1084** f128_bytes_to_x87_bytes truncates 49 bits — every long double literal 1 ulp low
- **#1087** Compound literal nested-array rows drop elements 2..n
- **#1088** #if accepts malformed expressions with no diagnostic (8/13 wrong branch)
- **#1089** __VA_OPT__ emitted as literal tokens
- **#1092** Linker drops .symtab/.strtab — nm reports "no symbols"
- **#1115** volatile LOCALS constant-folded at -O2 (C11 5.1.2.3 violation)
- **#1144** sizeof(VLA) folds to 1024 in any arithmetic
- **#1147** typedef-to-VLA SEGFAULTs
- **#1173** Enum values >int truncated to 0

### Lexer (~100 validation gaps)
- **#1175** Octal 018/09 accepted as decimal 18/9
- **#1176** Duplicate suffixes luL/uu/lll accepted
- **#1178** Empty character constant '' = 0
- **#1179** Float 1e (empty exponent) = 0
- **#1180** 47 empty-prefix literal forms all silently = 0
- **#1191** 28 control characters accepted as identifier chars
- **#1192** Incomplete \U universal char name accepted
- **#1193** Unterminated /* silently consumes rest of file
- **#1194** Mixed narrow/wide string concat rejected (GNU ext)

### Driver/CLI (10 defects)
- **#1153-#1165**: -U not cancel -D, functional -D unsupported, -C ignored, -M no deps, -MF to stdout, -g zero DWARF, CPATH ignored, -Wl,-Map ignored, self-include silent

### Linker
- **#1166** Duplicate strong symbols link silently
- **#1198** Tentative beats strong when tentative file first (order-dependent)

### Inline Asm
- **#1204** "m" constraint produces 0

### GNU Extensions
- **#1150** Block-scope static fn accepted (gcc/clang reject)
- **#1151** Nested fn support inverted vs gcc
- **#1152** asm symbol-rename labels break link
- **#1153** constructor(N) priorities ignored
- **#1190** noinline attribute ignored (function inlined anyway)

### Optimizer
- **#1170** encode_shift u32-underflow panic (#1073 pattern)
- **#1080** encode_ubfiz width=0 u32-underflow

## Design Caveats

All observed behavioral differences not listed above are C11 undefined/implementation-defined or documented design choices. Full details in per-round REPORT.md files.

## Test Files Created

874 test functions across 78 source files, 27 resident property groups. Key files:
- src/passes/mod.rs: self-diff, optimizer, switch, memgraph, cross-target, constraint fuzz
- src/backend/linker_common/mod.rs: multi-TU, randomized symbol fuzz
- src/backend/arm/assembler/encoder/mod.rs: 36-encoder random fuzz
- src/backend/riscv/assembler/encoder/pseudo.rs: li exec-equivalence, RISC-V fuzz
- src/frontend/preprocessor/expr_eval.rs: #if corners, macro SM, pp-operator battery

## Reproduction

All issues have "Found by" sections with exact commands. Resident properties run via:
```bash
PATH="$HOME/.cargo/bin:$PATH" RUST_TEST_THREADS=1 cargo test --lib -- "<property_name>" --nocapture
```

## Output Directories

- pbt-out/rounds/01-59/ — per-round artifacts
- pbt-out/bug_reports/FILED_ISSUES.md — complete ledger
- pbt-out/rounds/28_dedup_explore/dedup_db.json — live dedup database
- pbt-out/COVERAGE.md — coverage detail by round
- pbt-out/COVERAGE_STATUS.md — this status

## Unperformed Coverage Sweeps

| Item | Reason |
|------|--------|
| Line-level coverage (.gcda) | No gcovr/lcov in environment |
| Coverage-guided fuzzing | Needs instrumented build |
| 10k+ TU scale | Performance dimension |
| ARM NEON independent deep test | Teammate territory (291 existing issues) |
