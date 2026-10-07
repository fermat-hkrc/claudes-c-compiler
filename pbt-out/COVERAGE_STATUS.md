# COVERAGE_STATUS.md

> Last updated: 2026-10-06
> Campaign: pbt-test-v2 (whole-repo loop, 29 rounds)
> Coverage evidence level: file-level (symbol presence)
> Total: 74 unique issues filed (#516-#1068) + 3 found_already

## Module Status

| Module | Files | PBT Coverage | Issues |
|--------|-------|-------------|--------|
| src/common | 15 | 8/15 files with PBT (7 glue skipped w/ reason) | #516-518, #765, #806 |
| src/frontend/lexer | ~10 | full | #526-530 |
| src/frontend/preprocessor | ~15 | full | #538-540 |
| src/frontend/parser | ~8 | full (battery + PBT) | #799, #848 |
| src/frontend/sema | ~12 | full | #559-563, #812, #1047-1048 |
| src/ir | 53 | full (13 properties) | #573-576 |
| src/passes | 18 | full (14 properties + LICM/IVSR) | #759-760, #805 |
| src/ir/lowering | 42 | E2E differential (9 deep passes) | 0 (all match gcc) |
| src/backend root | 17 | full (8 properties) | #765, #808 |
| src/backend/x86 | 53 | E2E + asm deep (58 families) | #774-777, #782, #807, #818-820, #822, #826-827, #834-835, #840-841, #1051-1055 |
| src/backend/arm | 41 | asm deep (100+ families) | #791-795, #808, #823, #825, #828-831, #836-839, #1042-1043, #1052, #1056-1057, #1062-1064 |
| src/backend/riscv | 42 | asm deep (100+ families) | #796-798, #821, #824, #833, #1041, #1068 |
| src/backend/i686 | 40 | E2E + asm validation | #770, #800, #827, #1039 |
| src/backend/elf | 12 | 1/12 with PBT (11 structural) | 0 |
| src/backend/linker_common | 17 | E2E only | 0 |
| src/backend/stack_layout | 7 | E2E only | 0 |
| src/driver | 5 | 1/5 with PBT | 0 |

## Skipped Surfaces (with reason)

- **src/common/temp_files.rs** — I/O glue, integration-bound
- **src/common/type_builder.rs** — API deep-bound to parser AST
- **ARM NEON encoders** — teammate's territory (#474-#773)
- **Coverage-driven sweep** — no .gcda/.profraw available
