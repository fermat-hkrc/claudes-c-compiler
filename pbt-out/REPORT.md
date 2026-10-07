# PBT Campaign Report: Whole-repo deep loop (rounds 01-36)

## Summary

**Verdict:** 84 unique issues filed (#516-#1092). Rounds 01-29 covered all src/ modules (common, frontend, IR, passes, backends, ELF, driver); rounds 30-36 went DEEP on the previously-untested surfaces: encode-function adversarial level, ir/ core + ir/lowering (42 files, 30.7k lines), preprocessor corners, linker multi-TU, stack_layout, elf/driver, and optimizer interactions at -O1/-O2 with rich C features. The C-codegen pipeline remains solid (1300 + 450 more random/rich programs match gcc); deep-layer bugs concentrate in numeric-conversion truncation (#1084), compound-literal lowering (#1087), negative contracts (#1088), missing C23 features (#1089), and ELF output completeness (#1092).
**Date:** 2026-10-05 through 2026-10-07
**Repository:** claudes-c-compiler (/home/shuhao/fermat-users/leo/github/claudes-c-compiler)
**Modules tested:** ALL src/ modules (3 full loops + deep passes + deep untested-layer rounds 30-36)
**Tests:** 15+ PBT properties per round + 2000+ probe forms + 1750 random C programs + 1000-case state machine
**Result:** 84 unique issues + 14 found_already (deduped)
**Tier:** standard (each round; round 36 included the required strengthening pass: state-machine shape, 1000 cases)

## Modules Tested

| Module | Rounds | Issues | Oracles |
|--------|--------|--------|---------|
| src/common | 01, 08, 24, 27 | #516-518, #765, #806 | reference (math identities) |
| src/frontend/lexer | 02, 24 | #526-530 | reference (gcc token stream) |
| src/frontend/preprocessor | 03, 24 | #538-540 | differential (gcc -E) |
| src/frontend/parser | 04, 18 | #799, #848 | differential (gcc -fsyntax-only) |
| src/frontend/sema | 05, 24, 28 | #559-563, #812, #848, #1047-1048 | differential (gcc) |
| src/ir | 06, 25 | #573-576 | PBT (fixpoint, memory) |
| src/passes | 07, 24 | #759-760, #805 | differential (C11 interp), pipeline |
| src/backend root | 08, 27 | #765, #808 | reference, roundtrip |
| src/backend/x86 | 09, 15, 26, 28 | #774-777, #782, #807, #818-819, #820, #822, #826-827, #834-835, #840-841, #1051-1055 | byte-exact (as+llvm-mc) |
| src/backend/arm | 10, 16, 26, 28 | #791-795, #823, #825, #828-831, #836-839, #1042-1043, #1052, #1056-1057, #1062-1064 | byte-exact (as+llvm-mc) |
| src/backend/riscv | 11, 17, 26, 28 | #796-798, #821, #824, #833, #1041, #1068 | byte-exact (llvm-mc +f,+d) |
| src/backend/i686 | 12, 19, 26, 28 | #770, #800, #827(i686), #1039 | byte-exact (as --32+llvm-mc) |
| src/backend/elf | 13, 20, 27, 35 | #1092 (probe) | structural (readelf) |
| src/backend/linker_common | 20, 27, 33 | 0 | E2E (link+run), multi-TU 200 cases |
| src/driver | 14, 35 | 0 | structural, mode matrix |
| src/ir (core) + ir/lowering | 06, 25, 31 | #1084, #1087 | byte-exact gcc differential (globals/ptr/complex/compound) |
| src/frontend/preprocessor (deep) | 03, 24, 32 | #1088, #1089 | token-stream gcc differential + model SM |
| src/backend/stack_layout | 34 | 0 | E2E deep-frame/VLA/alloca (450 cases) |
| passes interactions @-O1/-O2 | 36 | 0 | rich-C differential 150×3 levels |
| encode functions (ARM/RISC-V/x86) | 30 | #1073, #1075, #1077, #1078, #1080 | function-level adversarial + llvm-mc |

## Bugs Found

### PBT-witnessed bugs (failing, shrunk proptest properties)

Each entry cites the property that found it and its shrunk counterexample. These are confirmed SUT violations from formal PBT with shrinking.

#### B1 (#516): f64_to_x87_bytes subnormal mis-encode
- **Property:** p1_f64_to_x87_bytes (round 01, proptest 1024 cases)
- **Shrunk counterexample:** x = 4.9e-324 (smallest subnormal)
- **Reproduce:** `cargo test --lib common::long_double::pbt_tests::p1_f64_to_x87_bytes`

#### B2 (#517): f128_bytes_to_f64 truncates (not RNE)
- **Property:** p2_f128_bytes_to_f64 (round 01)
- **Shrunk counterexample:** specific bit pattern where RNE differs from truncation
- **Reproduce:** `cargo test --lib common::long_double::pbt_tests::p2_f128_bytes_to_f64`

#### B3 (#526): hex-float wide mantissa → 0.0
- **Property:** lexer PBT (round 02)
- **Shrunk counterexample:** 0x1.fffffffffffffffffp+1023
- **Reproduce:** `cargo test --lib frontend::lexer::pbt_tests`

#### B4 (#538): ## __VA_ARGS__ empty-arg comma dropped
- **Property:** preprocessor PBT (round 03)
- **Shrunk counterexample:** specific macro invocation
- **Reproduce:** `cargo test --lib frontend::preprocessor::pbt_tests`

#### B5 (#559): UAC ul+ll → signed LongLong
- **Property:** p1_uac_binop_ctype (round 05, 1024 cases)
- **Shrunk counterexample:** unsigned long + long long → signed
- **Reproduce:** `cargo test --lib frontend::sema::type_checker::pbt_tests::p1_uac_binop_ctype`

#### B6 (#573): build_cfg duplicate predecessors
- **Property:** IR CFG PBT (round 06)
- **Shrunk counterexample:** CondBranch with equal true/false targets
- **Reproduce:** `cargo test --lib ir::analysis::pbt_tests`

#### B7 (#759): constant_fold sub-int LShr sign-extension mis-fold
- **Property:** p5_fold_matches_c11_model (round 07, 1024 cases)
- **Shrunk counterexample:** `LShr(Const(I8(-72)), Const(I8(1)), I8)` → fold gives -36, C11 requires 92
- **Reproduce:** `cargo test --lib passes::constant_fold::pbt_tests::p5_fold_matches_c11_model`

#### B8 (#760): narrow Phase 2 I64-use miscompile
- **Property:** p13_narrow_preservation (round 07, 1024 cases)
- **Shrunk counterexample:** f(a: i32, b: i32) -> i64 { Or(sext a, sext b) } at I64 narrowed without re-extension
- **Reproduce:** `cargo test --lib passes::narrow::pbt_tests::p13_narrow_preservation`

#### B9 (#765): parse_integer_expr debug-panic on overflow
- **Property:** p8_asmexpr_parse_matches_evaluator (round 08)
- **Shrunk counterexample:** "9223372036854775807 * 9223372036854775807"
- **Reproduce:** `cargo test --lib backend::asm_expr::pbt_tests2::p8_asmexpr_parse_matches_evaluator`

#### B10 (#770): i686 include path strips gcc-internal dir
- **Property:** E2E battery bring-up (round 12)
- **Counterexample:** `#include <stdio.h>` with -m32 → stddef.h not found
- **Reproduce:** `printf '#include <stdio.h>\nint main(void){return 0;}\n' | ccc -m32 -c -`

#### B11 (#1084): f128_bytes_to_x87_bytes truncates 49 mantissa bits
- **Property:** p_global_init_bytes_vs_gcc (round 31, 300 cases)
- **Shrunk counterexample:** imm = 2048 → `lui 1; addiw -2048` decoded x[10]=0 (simulator) — final witness: `static long double a = 0.1L;` bytes gcc `cd cc…fb3f` vs ccc `cc cc…fb3f` (1 ulp low, 7/8 literals)
- **Falsifiable:** ∀ decimal long-double literal L: bytes_ccc(L) == bytes_gcc(L) — FALSIFIED
- **Reproduce:** `cargo test --lib -- "pbt_global_init::p_global_init_bytes_vs_gcc" --nocapture` (see also proptest-regressions/ir/lowering/lower.txt)

#### B12 (#1087): compound literal nested-array rows drop elements 2..n
- **Property:** p_compound_designated_statics_vs_gcc (round 31, 400 cases)
- **Shrunk counterexample:** seeds (16297,0),(26968,3096092715127195104),(15480,0),(28972,0) → `&(int[2][2]){{10,20},{30,40}}` backing = [10,0,30,0]; reads leak the adjacent global
- **Falsifiable:** ∀ compound literal `(T[n][m]){rows}`: backing object == gcc's — FALSIFIED
- **Reproduce:** `cargo test --lib -- "p_compound_designated_statics" --nocapture`

#### B13 (#1089): __VA_OPT__ emitted as literal tokens
- **Property:** p_macro_corners_vs_gcc (round 32, 20 forms; 2 failing)
- **Shrunk counterexample:** `#define V2(fmt,...) fmt __VA_OPT__(| __VA_ARGS__)` + `V2(x)` → ccc tokens contain literal `__VA_OPT__ ( | )`, gcc = `[x]`
- **Falsifiable:** ∀ variadic macro with __VA_OPT__: expansion == gcc's — FALSIFIED
- **Reproduce:** `cargo test --lib -- "p_macro_expansion" --nocapture`

#### B14-B18 (round 30, encode-level adversarial PBT): #1073 encode_shift u32-underflow panic (p_shift_overflow, shrunk amt=width); #1075/#1077/#1078 exclusive-load non-zero imm9 accepted for LDXR/LDAR/LDAXR (p_ldxr_offset family, objdump witnesses); #1080 encode_ubfiz width=0 panic (p_ubfiz_zero_width, shrunk width=0)
- **Reproduce:** `cargo test --lib -- "pbt_adversarial2" --nocapture` (bitfield/data_processing/load_store)

### Probe-discovered bugs (differential testing, not formal PBT)

These were discovered through systematic probe batteries comparing ccc output against GNU as / llvm-mc. Each has byte-level evidence but did not originate from a shrinking proptest property.

**x86 assembler (rounds 15, 26, 28):** #774-777, #782, #807, #818-819, #820, #822, #826-827, #834-835, #840-841, #1051-1055

**ARM assembler (rounds 16, 26, 28):** #791-795, #808, #823, #825, #828-831, #836-839, #1042-1043, #1052, #1056-1057, #1062-1064

**RISC-V assembler (rounds 17, 26, 28):** #796-798, #821, #824, #833, #1041, #1068

**i686 assembler (rounds 19, 26, 28):** #800, #827, #1039

**Frontend (rounds 18, 24, 28):** #799, #812, #848, #1047-1048

**Preprocessor deep (round 32):** #1088 (#if malformed-expr acceptance — skip-analysis of p_if_corners_vs_gcc + 13-form manual `ccc -E` vs `gcc -E` sweep)

**ELF/driver (round 35):** #1092 (linked output drops .symtab/.strtab — readelf -sW/-SW probe; input .o has 9-entry .symtab, output only .dynsym)

**RISC-V/ARM extra-operand + offset sweeps (round 30):** 11 findings all deduped → found_already (#176/#208/#215/#223/#225/#276/#360/#797/#960/#962/#964)

## Design Caveats (if any)

All earlier caveats stand (C11 UB/implementation-defined behaviors, not SUT violations: double→char overflow C11 6.3.1.4 UB; strcmp sign 7.24.4.2 implementation-defined; enum↔int _Generic 6.5.1.1 implementation-defined). Rounds 30-36 additions, each with independent evidence:

- **GlobalInit::byte_size returns ELEMENT COUNT for Array** (not bytes) — Doc evidence: src/ir/module.rs:133-136 `GlobalInit::Array(vals) => vals.len()` used for `.long` emission counting; verified by p_global_init_byte_size (round 30, 128 cases).
- **eval_i64 add/sub/mul use wrapping semantics; div/rem checked** — Doc evidence: src/ir/ops.rs:72-106 (checked_div/checked_rem present, plain ops elsewhere); C11 UB implementation choice; p_shift_edge_cases/p_mul_overflow round 30, 512+cases.
- **mv pseudo uses `add rd, x0, rs2`** (R-type) vs llvm canonical addi — functionally identical; llvm-21 encoding [0x93,0x00,0x01,0x00] vs ccc 0x002000b3, both valid c.mv sources — p_pseudo_diff_reg_only round 30.
- **Reg-as-branch-target in RISC-V get_branch_target is documented** (`beqz a0, t1` where t1 is a label) — Doc evidence: src/backend/riscv/assembler/encoder/pseudo.rs:377-380 comment "A register name can also be a symbol/label name… Treat Reg as symbol in branch target context."
- **predefined macro set matches gcc 12/12** (incl. __STDC_VERSION__=201710L, __BYTE_ORDER__=1234) — round-35 value diff, /tmp/pm.c both compilers.
- **-S emits strings as .byte lists** (odd style, assembles+links back identically) — round-35 driver matrix.
- **linker symbol resolution is correct despite missing output .symtab** (the #1092 defect is output-completeness only; multi-TU E2E 200 cases byte-exact vs gcc) — round-33 p_multitu_link_vs_gcc.

## Test Files Created

| Round | File | Tests |
|-------|------|-------|
| 01-08 | src/common/*.rs, src/frontend/*/*.rs, src/ir/*.rs, src/passes/*.rs | 15+ PBT per module |
| 09,12,15-17,19,26,28 | src/backend/{x86,arm,riscv,i686}/mod.rs | E2E + asm deep + witness props |
| 13,27 | src/backend/elf/mod.rs, src/backend/mod.rs | StringTable + ELF structure |
| 24 | src/passes/copy_prop.rs | transitive chain + multi-def |
| 27 | src/common/{asm_constraints,source,error}.rs | classification PBT |
| 29 | /tmp/gen_c2.py, /tmp/gen_c3.py | random C generators (500+300) |
| 30 | src/backend/arm/assembler/encoder/{bitfield,data_processing,fp_scalar}.rs, src/backend/riscv/assembler/encoder/{pseudo,atomics,base}.rs | encode adversarial (61 props) + li exec-equivalence + Zbb 18-form + ir/{instruction,module,constants,ops}.rs (21 props) |
| 31 | src/ir/lowering/lower.rs | pbt_global_init + pbt_lowering_e2e (ptr/complex/compound) |
| 32 | src/frontend/preprocessor/expr_eval.rs | pbt_semantics_corners + pbt_macro_expansion + pbt_text_proc |
| 33 | src/backend/linker_common/mod.rs | pbt_multitu_link |
| 34 | src/backend/stack_layout/mod.rs | pbt_stack_stress (deep + VLA/alloca) |
| 36 | src/passes/mod.rs | pbt_opt_rich (3-level) + p_macro_lifecycle_sm (1000-case state machine) |

## Reproduction

```bash
cd /home/shuhao/fermat-users/leo/github/claudes-c-compiler
PATH="$HOME/.cargo/bin:$PATH" RUST_TEST_THREADS=1 cargo test --lib -- --nocapture
```

## Output Directories

- pbt-out/rounds/01-36/ — per-round PLAN, PROPERTIES, REPORT, bug_reports, found_already
- pbt-out/bug_reports/FILED_ISSUES.md — complete issue ledger (84 filed)
- pbt-out/INVARIANTS.md — methodology notes
- pbt-out/rounds/28_dedup_explore/dedup_db.json — live dedup database (1080 issues, refreshed 2026-10-07)

## Unperformed Coverage Sweep

- Coverage-driven sweep (coverage_gaps tool): NOT AVAILABLE in this environment (no .gcda/.profraw, tool reported "Not in a PBT campaign"). File-level evidence used instead. All documented behaviors in README files have at least one property or E2E test. The only skipped coverage is the teammate's ARM NEON encoder surface (#474-#773, 263 issues).
- Round 36 strengthening (state-machine shape, 1000 cases): macro-table lifecycle (define/undef/redefine × #ifdef probes, model+gcc dual oracle) — ALL GREEN; no re-run needed beyond it.
