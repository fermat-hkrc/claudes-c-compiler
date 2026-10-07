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

## Round 37 — RISC-V unclaimed extra-operand sweep
| Function | Source file | Test file | Test target | Notes |
|----------|-------------|-----------|--------------|-------|
| encode_bgt/ble/bgtu/bleu/bltz/bgtz/csrr/csrw/csrs/csrc/fmv_s/fmv_d/fabs_s/fabs_d/fneg_s/fneg_d/la/lla (18 fns) | src/backend/riscv/assembler/encoder/pseudo.rs | pseudo.rs ::pbt_extra_unclaimed | p_extra_unclaimed_sweep | 19/19 silently ignore extras; encode_bgez → teammate #1094; 18 filed **#1095-#1112** (canonical #1094 format, concrete Actual encodings) |

## Round 38 — high-value silent-wrong-code battery
| Function | Source file | Test file | Test target | Notes |
|----------|-------------|-----------|--------------|-------|
| struct-return epilogue (SSE single-eightbyte) | src/backend/x86/codegen/returns.rs:74 | src/passes/mod.rs::pbt_high_value | p_struct_interop_gcc_calls_ccc | 250-case gcc↔ccc interop differential; ≤8B SSE returns in RAX → **#1114**; {double,double}/mixed/args green |
| punning/wrapping/variadic | — | src/passes/mod.rs::pbt_high_value | p_punning_wrap_variadic_vs_gcc | 300×2 levels green (union punning, char* roundtrip, u32/u64 wrap, va_arg mixed) |
| X87-class struct ABI ({long double}) | src/backend/call_abi.rs + x86 codegen | round-38 manual probes | interop | RAX:RDX/RDI:RSI instead of st(0)/memory → **#1116** |
| ARM64 HFA struct ABI | src/backend/call_abi.rs + ARM codegen | round-38 clang/qemu interop | 5-direction exit-code witness | X0/W0 integer regs instead of S/V — all 5 BAD → **#1118** |
| RISC-V FP-struct return | src/backend/riscv codegen | round-38 disasm + qemu witness | mkf/mkd/mk2 | integer a0, fa0 never loaded, {f,f} drops b → **#1119** (args fa0/fa1 correct) |

## Round 39 — encode sweep 2 (base/zbb/atomics-suffixed)
| Function | Source file | Test file | Test target | Notes |
|----------|-------------|-----------|--------------|-------|
| encode_zbb_unary/_f5/_w, encode_zbb_zexth, encode_shift_imm/_w, encode_amo/lr/sc_suffixed (9 fns) | src/backend/riscv/assembler/encoder/{base,atomics}.rs | base.rs ::pbt_extra_base_zbb | p_extra_base_zbb_sweep | 10/10 ignore extras; zbb_binary→encode_alu_reg #946 skipped; 9 filed **#1122-#1130** |
| i686 struct ABI (5 directions) | src/backend/i686 | round-38 gcc -m32 interop | — | ALL GREEN (matches gcc) |
| fn-ptr FP-struct calls | x86 codegen | round-38 probe | — | breakage = #1114 propagation (mk return garbage), not new |

## Round 39b — ARM malformed-input matrix (empty/missing/wrong-type)
| Function | Source file | Test file | Test target | Notes |
|----------|-------------|-----------|--------------|-------|
| ARM 6-file malformed matrix | data_processing/load_store/system/compare_branch/bitfield/fp_scalar | ::pbt_malformed_matrix per file | catch_unwind silent-Ok/panic sweep | 6/6 green after extraction; dmb/dsb empty+wrong-type silent sy → **#1131** |
| hint imm range / msr field validation | system.rs | round-39 probes | llvm-mc diff | both GREEN (hint #100 valid ≤127; msr xyzzy errors) |
| volatile local folding | src/passes (storage classification) | round-38 /tmp/vol2.c f6 | access-count vs gcc | constants folded, 1/6 accesses survive → **#1115** |
| long double runtime arith / >16B MEMORY structs | x86 backend | round-38 probes | interop | both GREEN (exact mul/div/cmp; 5×double hidden-pointer ✓) |


## Round 41 — cross-target runtime differential
| Function | Source file | Test file | Test target | Notes |
|----------|-------------|-----------|--------------|-------|
| ARM64/RISC-V/i686 codegen at runtime | all backends | src/passes/mod.rs::pbt_cross_target | p_cross_target_vs_gcc_x64 | 150 × 3 targets green (qemu exit-code diff) |
| fp128 runtime linkage | linker_common/resolve_lib | round-41 probe | long double link | __addtf3 family undefined → **#1145** |

## Round 41b — constraint waves 6-9 + runtime corners
| Function | Source file | Test file | Test target | Notes |
|----------|-------------|-----------|--------------|-------|
| type-system constraints | sema declarators/init | /tmp/neg6-9.sh batteries | 60+ forms | waves 6-9: #1146 (6), #1148 (2), #1149 (3); extensions → #1088/#1143 |
| typedef VM types | parser/sema typedef | /tmp/tdv*.c | crash witness | typedef-VLA SEGFAULT → #1147 |
| mixed bitfield layout | lowering struct layout | /tmp/rt11.c | byte diff vs gcc+clang | == clang exactly; gcc slack-only diff (unspecified) — no bug |
| i686 FP-struct interop | i686 backend | /tmp/i32f*.c | gcc -m32 diff | all green (st(0)) |
| goto/VLA stack balance | stack_layout | /tmp/rt12.c | 1000-round stress | green both levels |

## Round 42 — adversarial optimizer + GNU extensions
| Function | Source file | Test file | Test target | Notes |
|----------|-------------|-----------|--------------|-------|
| optimizer adversarial patterns | src/passes | mod.rs::pbt_opt_adversarial | 400×3 levels | green (INT_MIN abs, ovf-guards, rotates, strength-reduction) |
| memory-graph fuzz | codegen | mod.rs::pbt_memgraph | 300×3 levels | green (aliasing, fn-ptr effects) |
| nested functions | parser/sema | /tmp/nest*.c | gcc diff | inversion → #1150 + #1151 |
| asm labels | symbol emitter | /tmp/asmname.c | link diff | split rename → #1152 |
| GNU ext surface | — | /tmp/gnux.c /tmp/attr.c | byte/value diff | stmt-expr/typeof/computed-goto/packed/aligned all green |

## Round 43 — attribute/builtin semantics
| Function | Source file | Test file | Test target | Notes |
|----------|-------------|-----------|--------------|-------|
| constructor priorities | init_array emission | /tmp/cd.c | order diff | priorities ignored → #1153 |
| #if operand types | pp expr_eval | /tmp/pp43.sh | 5-form diff | float/str/cast/ppnum accepted → #1154 |
| builtin bits + complex arith | builtins/codegen | /tmp/b43.c | value diff -O2 | green (Smith div exact) |
| asm volatile barrier | optimizer | /tmp/asmv.c | disasm witness | both loads+stores kept = gcc |
| _Alignas member / cleanup | frontend | /tmp/al43.c /tmp/cu.c | layout+order | green |

## Round 44 — pragma/builtin/layout battery
| Function | Source file | Test file | Test target | Notes |
|----------|-------------|-----------|--------------|-------|
| pragma pack stack | preprocessor pragmas | /tmp/r44.c | 5-struct size chain | green (push/1/2/pop/pop) |
| pack×bitfields + builtin mem/ffs + atomics + 512-switch | lexer/builtins/codegen | /tmp/r44b.c /tmp/r44c.c /tmp/bigsw.c | byte/value diff | all green |
| string-literal newline check | src/frontend/lexer | /tmp/nl.c | gcc/clang diff | accepted → **#1155** |

## Round 45 — escape corners + driver/CLI flags
| Function | Source file | Test file | Test target | Notes |
|----------|-------------|-----------|--------------|-------|
| hex-escape validation | lexer | python-form isolation | \x empty forms | accepted → #1156 |
| -U/-D ordering, functional -D, -C, -M | src/driver | /tmp/uu.c /tmp/fm.c /tmp/ec.c /tmp/mm.c | gcc diff | 4 breaks → #1157 #1158 #1160 #1161 |
| self-include depth | includes.rs | /tmp/deep.h | gcc diff | silent object → #1159 |
| UTF-8 strings/comments, \8, \7777, -isystem, include_next, search order | — | /tmp/e45.c etc | value diff | all green |

## Round 46 — robustness + linker semantics
| Function | Source file | Test file | Test target | Notes |
|----------|-------------|-----------|--------------|-------|
| parser robustness | frontend | mod.rs::pbt_mutation_fuzz | p_mutation_no_ice | 600 mutants, 0 ICE/hang |
| -E round-trip | preprocessor | /tmp/rt1.c | recompile identity | green (ccc→ccc, gcc→ccc) |
| strong/strong resolution | linker symbols | /tmp/lk | gcc diff | silent link → #1166 |
| archive lazy/whole/cross-addr | linker archive | /tmp/lk lazy.a | order matrix | lazy+deps+addr green; --whole-archive → #1165 ext |

## Rounds 47-48 — deepening + final mines
| Mine | Result | Issues |
|------|--------|--------|
| family matrices (7 filed bugs) | complete form matrices posted | comments on #1144/#1118/#1135/#1140/#1088/#1147/#1114 |
| C: enum wide values | ccc truncates >int to 0 | #1173 |
| D: variadic default promotions | green | — |
| E: error-recovery cascade | benign (≤3.5x gcc) | — |
| F: multi-TU static interplay | green | — |
| G: wide/unicode char values | exact match | — |
| H: deep recursive types | valid green; member-on-int → #1139 ext | comment |

## Round 49 — new testing levels
| Level | Test | Cases | Status |
|-------|------|-------|--------|
| self-differential (-O0 ≡ -O2, oracle-free) | mod.rs::pbt_selfdiff | 2000 | green |
| valgrind memory validation | /tmp/vgbat.c battery | full-feature × 2 levels | zero errors |

## Round 49b — thousand-case deepening v2 (all green, zero new SUT bugs)
| Function | Source file | Test file | Test target | Notes |
|----------|-------------|-----------|--------------|-------|
| li encode +7 edge strategies | riscv encoder pseudo.rs | ::pbt_li_exec | 6000 cases | green |
| macro lifecycle SM (6 names × ≤30 ops) | pp expr_eval.rs | ::pbt_macro_table_sm | 4000 cases | green (harness PROBE-substring fixed → line-exact) |
| optimizer self-diff +rec/fnptr/str | passes mod.rs | ::pbt_selfdiff | 4000 × 2 levels | green |
| switch tables ≤200 cases | passes mod.rs | ::pbt_switch_tables | 800 × 2 | green |
| adversarial optimizer | passes mod.rs | ::pbt_opt_adversarial | 1000 × 3 | green |
| cross-target + levels regression | passes mod.rs | ::pbt_cross_target | 270 | green after call-site repair |

## Round 50 — encode deep fuzz (36+19 encoders × random operand vectors)
| Function | Source file | Test file | Test target | Notes |
|----------|-------------|-----------|--------------|-------|
| 36 ARM encoders × random ops (edge reg names + extreme imms, 0..6 operands) | arm encoder mod.rs | ::pbt_encode_fuzz | p_encode_fuzz_no_panic | 20000 cases green after gating the KNOWN #74/#80 path (encode_add_sub i64::MIN negation overflow via cmp/cmn/ccmp) — no NEW panic |
| 19 RISC-V encoders × random ops | riscv pseudo.rs | ::pbt_encode_fuzz_rv | p_rv_encode_fuzz_no_panic | 15000 cases — zero panic |
| memgraph regression (call-site collateral repair) | passes mod.rs | ::pbt_memgraph | 300×3 levels | green after cmain(0) fix — zero SUT bugs |
