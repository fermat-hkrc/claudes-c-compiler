# PBT Coverage Status

> Last updated: 2026-10-07 (campaign pbt-test-v2 — 36 rounds complete)
> Coverage evidence: file-level (symbol presence) — no .gcda/.profraw; coverage_gaps' passes NOT LINKED list is the known cfg(test) false negative.
> Total issues filed: 84 unique + 14 found_already (deduped)

## Round Coverage Map

| Round | Module | Issues | Status |
|-------|--------|--------|--------|
| 01 | src/common | #516-518 (+#2 dup) | ✅ complete |
| 02 | src/frontend/lexer | #526-530 | ✅ complete |
| 03 | src/frontend/preprocessor | #538-540 | ✅ complete |
| 04 | src/frontend/parser | 0 bugs | ✅ complete |
| 05 | src/frontend/sema | #559-563 | ✅ complete |
| 06 | src/ir | #573-576 (+#3 dup) | ✅ complete |
| 07 | src/passes | #759-760 | ✅ complete |
| 08 | src/backend root | #765 | ✅ complete |
| 09 | src/backend/x86 E2E | 0 bugs | ✅ complete |
| 10 | src/backend/arm asm | 0 bugs | ✅ complete |
| 11 | src/backend/riscv asm | 0 bugs | ✅ complete |
| 12 | src/backend/i686 E2E | #770 | ✅ complete |
| 13 | src/backend/elf+linker | 0 bugs | ✅ complete |
| 14 | src/driver | 0 bugs | ✅ complete |
| 15 | x86 assembler deep | #774-777, #782 | ✅ complete |
| 16 | ARM assembler deep | #791-795 | ✅ complete |
| 17 | RISC-V assembler deep | #796-798 | ✅ complete |
| 18 | parser battery | #799 | ✅ complete |
| 19 | i686 asm + C features | #800 | ✅ complete |
| 20 | linker deep | 0 bugs | ✅ complete |
| 21 | exotic C battery | #805 | ✅ complete |
| 22 | ABI/NaN/-S roundtrip | #806-807 | ✅ complete |
| 23-24 | second loop (frontend+passes) | #808, #812 | ✅ complete |
| 25 | second loop (ir+common) | 0 bugs | ✅ complete |
| 26 | asm deep sweep (all 4 arch) | #818-841 | ✅ complete |
| 27 | third loop (common+opt) | 0 bugs | ✅ complete |
| 28 | dedup explore + NEON + FP-imm | #1042-1068 | ✅ complete |
| 29 | random C PBT (500 int+300 float) | 0 bugs | ✅ complete |
| 30 | encode-level adversarial (ARM/RISC-V) + ir/ core | #1073, #1075, #1077, #1078, #1080 (+11 dups) | ✅ complete |
| 31 | ir/lowering deep (globals/ptr/complex/compound) | #1084, #1087 | ✅ complete |
| 32 | preprocessor deep (if-corners/macro/splice) | #1088, #1089 | ✅ complete |
| 33 | linker_common multi-TU | 0 bugs | ✅ complete |
| 34 | stack_layout deep (frames/VLA/alloca) | 0 bugs | ✅ complete |
| 35 | elf/driver/predefined_macros | #1092 (probe) | ✅ complete |
| 36 | optimizer interactions @-O1/-O2 + macro SM (strengthening) | 0 bugs | ✅ complete |

## Unperformed Coverage Sweep

- **Coverage-driven sweep (coverage_gaps tool):** NOT AVAILABLE — no .gcda/.profraw in this environment; tool reported "Not in a PBT campaign". File-level evidence used instead.
- **ARM NEON encoder surface:** EXCLUDED — teammate's territory (#474-#773, 263 issues)
- **Round 36 strengthening (state-machine, 1000 cases):** completed ALL GREEN — macro-table lifecycle model+gcc dual oracle.

Detail tables for rounds 31-36: see pbt-out/COVERAGE.md (appended sections).
