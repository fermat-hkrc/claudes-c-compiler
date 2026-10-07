# PBT Coverage Status

> Last updated: 2026-10-06 (campaign pbt-test-v2 — 29 rounds complete)
> Coverage evidence: file-level (symbol presence) — no .gcda/.profraw; coverage_gaps' passes NOT LINKED list is the known cfg(test) false negative.
> Total issues filed: 74 unique + 3 found_already (deduped)

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

## Unperformed Coverage Sweep

- **Coverage-driven sweep (coverage_gaps tool):** NOT AVAILABLE — no .gcda/.profraw in this environment; tool reported "Not in a PBT campaign". File-level evidence used instead.
- **ARM NEON encoder surface:** EXCLUDED — teammate's territory (#474-#773, 263 issues)

## Round 31 — ir/lowering deep PBT (global init/ptr/complex/compound)
| Function | Source file | Test file | Test target | Notes |
|----------|-------------|-----------|--------------|-------|
| lower_global_init (14 type kinds) | src/ir/lowering/global_init.rs, global_init_bytes.rs | src/ir/lowering/lower.rs::pbt_global_init | p_global_init_bytes_vs_gcc | 300 cases byte-exact; long double → **#1084** |
| create_compound_literal_global | src/ir/lowering/global_init.rs:1142 | src/ir/lowering/lower.rs::pbt_lowering_e2e | p_compound_designated_statics_vs_gcc | 2-D nested → **#1087**; 1-D/designated/statics green |
| const_eval_global_addr | src/ir/lowering/const_eval_global_addr.rs | src/ir/lowering/lower.rs::pbt_lowering_e2e | p_pointer_globals_vs_gcc | 400 cases &g/&arr[k]/decay/member/ptr-arrays green |
| complex global init | src/ir/lowering/complex.rs | src/ir/lowering/lower.rs::pbt_lowering_e2e | p_complex_globals_vs_gcc | 300 cases float/double _Complex byte-exact |
| f128_bytes_to_x87_bytes | src/common/long_double.rs:886 | (root cause of #1084) | p_global_init kind 13 | truncates 49 bits, no RNE — every LD literal 1 ulp low |

## Round 32 — preprocessor deep PBT (expr_eval / macro expansion)
| Function | Source file | Test file | Test target | Notes |
|----------|-------------|-----------|--------------|-------|
| #if expr eval (corners) | src/frontend/preprocessor/expr_eval.rs | same file ::pbt_semantics_corners | p_if_corners_vs_gcc | 30 corners green; malformed accept → **#1088** |
| #if unsigned mixing | src/frontend/preprocessor/expr_eval.rs | same ::pbt_semantics_corners | p_if_random_unsigned_mix | 300 cases green |
| macro expansion corners | src/frontend/preprocessor/macro_defs.rs | expr_eval.rs ::pbt_macro_expansion | p_macro_corners_vs_gcc | 18/20 green; __VA_OPT__ → **#1089** |

## Round 33 — linker_common deep PBT (multi-TU)
| Function | Source file | Test file | Test target | Notes |
|----------|-------------|-----------|--------------|-------|
| multi-TU symbol resolution | src/backend/linker_common/*.rs (17 files) | mod.rs ::pbt_multitu_link | p_multitu_link_vs_gcc | 200 cases green: extern/cross-fn/tentative/fn-ptr-tables |
| weak symbols (manual probe) | symbols.rs | manual /tmp/w1.c+w2.c | — | matches gcc |

## Round 34 — stack_layout deep PBT
| Function | Source file | Test file | Test target | Notes |
|----------|-------------|-----------|--------------|-------|
| slot_assignment/analysis | src/backend/stack_layout/*.rs (7 files) | mod.rs ::pbt_stack_stress | p_stack_deep_vs_gcc | 250 deep-frame cases green |
| alloca_coalescing | src/backend/stack_layout/alloca_coalescing.rs | mod.rs ::pbt_stack_stress | p_vla_alloca_vs_gcc | 200 VLA+alloca cases green |

## Round 35 — elf/driver/predefined_macros
| Function | Source file | Test file | Test target | Notes |
|----------|-------------|-----------|--------------|-------|
| predefined macro set | src/frontend/preprocessor/predefined_macros.rs | manual /tmp/pm.c | value diff vs gcc | 12/12 exact |
| driver modes | src/driver/*.rs (4 files) | manual matrix | -E/-c/-S/relink/a.out | all green |
| ELF output structure | src/backend/linker_common/write.rs + src/backend/elf/ (11 files) | readelf probes | symtab/eh_frame/phdr | symtab missing → **#1092**; rest green |

## Round 36 — optimizer interaction differential (rich C at -O0/-O1/-O2)
| Function | Source file | Test file | Test target | Notes |
|----------|-------------|-----------|--------------|-------|
| pass interactions (narrow/copy_prop/if_convert/dce/cfg_simplify/simplify/gvn/fold/div_by_const) | src/passes/*.rs | src/passes/mod.rs::pbt_opt_rich | p_opt_rich_vs_gcc | 150 cases × {default,-O1,-O2} all == gcc -O0; bitfields/structs/unions/float/fn-ptr/switch/goto |
| macro-table lifecycle state machine | src/frontend/preprocessor/macro_defs.rs | expr_eval.rs ::pbt_macro_table_sm | p_macro_lifecycle_vs_model_gcc | 1000 cases, model+gcc dual oracle, every-step probe — ALL GREEN |
