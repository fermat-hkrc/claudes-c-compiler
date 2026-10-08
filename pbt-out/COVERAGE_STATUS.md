# PBT Coverage Status

> Last updated: 2026-10-07 (campaign pbt-test-v2 — 59 rounds complete)
> Coverage evidence: file-level (symbol presence) — no .gcda/.profraw; coverage_gaps' passes NOT LINKED list is the known cfg(test) false negative.
> Total issues filed: 180 unique + 14 found_already (deduped)
> Unperformed coverage sweep: line-level coverage (.gcda) NOT available in this environment — recorded as incomplete.

## Round Coverage Map

| Round | Module | Issues | Status |
|-------|--------|--------|--------|
| 01 | src/common | #516-518 | ✅ complete |
| 02 | src/frontend/lexer | #526-530 | ✅ complete |
| 03 | src/frontend/preprocessor | #538-540 | ✅ complete |
| 04 | src/frontend/parser | 0 bugs | ✅ complete |
| 05 | src/frontend/sema | #559-563 | ✅ complete |
| 06 | src/ir | #573-576 | ✅ complete |
| 07 | src/passes | #759-760 | ✅ complete |
| 08 | src/backend root | #765 | ✅ complete |
| 09 | src/backend/x86 E2E | 0 bugs | ✅ complete |
| 10 | src/backend/arm asm | 0 bugs | ✅ complete |
| 11 | src/backend/riscv asm | 0 bugs | ✅ complete |
| 12 | src/backend/i686 E2E | #770 | ✅ complete |
| 13 | src/backend/elf+linker | 0 bugs | ✅ complete |
| 14 | src/driver | 0 bugs | ✅ complete |
| 15-17 | asm deep (x86/ARM/RISC-V) | #774-798 | ✅ complete |
| 18 | parser battery | #799 | ✅ complete |
| 19-20 | i686 + linker deep | #800 | ✅ complete |
| 21-22 | exotic C / ABI/NaN | #805-807 | ✅ complete |
| 23-25 | second loop | #808, #812 | ✅ complete |
| 26 | asm deep all 4 arch | #818-841 | ✅ complete |
| 27 | third loop | 0 bugs | ✅ complete |
| 28 | dedup explore + NEON | #1042-1068 | ✅ complete |
| 29 | random C PBT | 0 bugs | ✅ complete |
| 30 | encode-level adversarial | #1073-#1080 | ✅ complete |
| 31 | ir/lowering deep | #1084, #1087 | ✅ complete |
| 32 | preprocessor deep | #1088, #1089 | ✅ complete |
| 33 | linker_common multi-TU | 0 bugs | ✅ complete |
| 34 | stack_layout deep | 0 bugs | ✅ complete |
| 35 | elf/driver/predefined | #1092 | ✅ complete |
| 36 | optimizer interaction + macro SM | 0 bugs | ✅ complete |
| 37-39 | RISC-V encode sweep | #1095-#1131 | ✅ complete |
| 40-42 | family deepening + GNU ext | #1133-#1153 | ✅ complete |
| 43-45 | escapes/driver mine | #1156-#1165 | ✅ complete |
| 46 | robustness fuzz + linker | #1166 | ✅ complete |
| 47-48 | final sweep | #1072-#1074 | ✅ complete |
| 49 | self-diff + valgrind | 0 bugs | ✅ complete |
| 50 | encode deep fuzz 35k | 0 new | ✅ complete |
| 51 | frontend/IR deep dig | #1175-#1179 | ✅ complete |
| 52 | i686 deep | #1183 ICE | ✅ complete |
| 53 | encode systematic | 0 new (all DUP) | ✅ complete |
| 54 | determinism | #1185 non-det | ✅ complete |
| 55 | macro depth + stack | #1186 SIGABRT | ✅ complete |
| 56 | pp constraints + DoS | #1187-#1197 | ✅ complete |
| 57 | sema + attributes | #1190 noinline | ✅ complete |
| 58 | constraint fuzz 2000 | 0 new (gated) | ✅ complete |
| 59 | linker fuzz 1000 + asm | #1198, #1204 | ✅ complete |

## Unperformed / Incomplete Coverage

| Item | Reason | Status |
|------|--------|--------|
| Line-level coverage (.gcda) | No gcovr/lcov in environment | ⚠️ INCOMPLETE |
| Coverage-guided fuzzing | Needs instrumented build | ⚠️ NOT PERFORMED |
| ARM NEON 65 fns independent deep test | Teammate territory (291 issues) | ⚠️ COVERAGE-VERIFIED ONLY |
| 10k+ TU scale | Performance dimension | ⚠️ NOT PERFORMED |
| -g DWARF emission | Known broken (#1163) | ⚠️ KNOWN GAP |

## Test Infrastructure

| Metric | Value |
|--------|-------|
| Total test functions | 874 |
| Test files | 78 |
| Resident property groups | 27 |
| Proptest regression seeds | 5 files |
| Per-round reports | 57 directories |
EOF
