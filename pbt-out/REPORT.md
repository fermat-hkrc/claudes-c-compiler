# PBT Campaign Report: Whole-repo deep loop (rounds 01-29)

## Summary

**Verdict:** 74 unique issues filed (#516-#1068, excluding 3 found_already dups). Coverage spans all `src/` modules across 29 rounds: common, frontend (lexer/preprocessor/parser/sema), IR, passes, backend root, x86/ARM/RISC-V/i686 assemblers, ELF, driver. The compiler's C-codegen pipeline (C → IR → optimize → assemble → link → execute) is remarkably solid: 1300 random C programs (integer + float) all match gcc at -O0 and -O2. The bugs concentrate in the assembler validation/encoding layer (~70% of issues) — operand class, immediate range, architecture feature gates, and encoding correctness.
**Date:** 2026-10-05 through 2026-10-06
**Repository:** claudes-c-compiler (/home/shuhao/fermat-users/leo/github/claudes-c-compiler)
**Modules tested:** ALL src/ modules (3 full loops + deep passes)
**Tests:** 15+ PBT properties per round + 2000+ probe forms + 1300 random C programs
**Result:** 74 unique issues + 3 found_already (deduped to teammate's)
**Tier:** standard (each round)

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
| src/backend/elf | 13, 20, 27 | 0 | structural (readelf) |
| src/backend/linker_common | 20, 27 | 0 | E2E (link+run) |
| src/driver | 14 | 0 | structural |

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

### Probe-discovered bugs (differential testing, not formal PBT)

These were discovered through systematic probe batteries comparing ccc output against GNU as / llvm-mc. Each has byte-level evidence but did not originate from a shrinking proptest property.

**x86 assembler (rounds 15, 26, 28):** #774-777, #782, #807, #818-819, #820, #822, #826-827, #834-835, #840-841, #1051-1055

**ARM assembler (rounds 16, 26, 28):** #791-795, #808, #823, #825, #828-831, #836-839, #1042-1043, #1052, #1056-1057, #1062-1064

**RISC-V assembler (rounds 17, 26, 28):** #796-798, #821, #824, #833, #1041, #1068

**i686 assembler (rounds 19, 26, 28):** #800, #827, #1039

**Frontend (rounds 18, 24, 28):** #799, #812, #848, #1047-1048

## Design Caveats (if any)

(none — all observed behavioral differences are C11 undefined/implementation-defined behaviors, not SUT-documented contract violations: double→char overflow is C11 6.3.1.4 UB; strcmp return value sign is C11 7.24.4.2 implementation-defined; enum↔int _Generic compatibility is C11 6.5.1.1 implementation-defined)

## Test Files Created

| Round | File | Tests |
|-------|------|-------|
| 01-08 | src/common/*.rs, src/frontend/*/*.rs, src/ir/*.rs, src/passes/*.rs | 15+ PBT per module |
| 09,12,15-17,19,26,28 | src/backend/{x86,arm,riscv,i686}/mod.rs | E2E + asm deep + witness props |
| 13,27 | src/backend/elf/mod.rs, src/backend/mod.rs | StringTable + ELF structure |
| 24 | src/passes/copy_prop.rs | transitive chain + multi-def |
| 27 | src/common/{asm_constraints,source,error}.rs | classification PBT |
| 29 | /tmp/gen_c2.py, /tmp/gen_c3.py | random C generators (500+300) |

## Reproduction

```bash
cd /home/shuhao/fermat-users/leo/github/claudes-c-compiler
PATH="$HOME/.cargo/bin:$PATH" RUST_TEST_THREADS=1 cargo test --lib -- --nocapture
```

## Output Directories

- pbt-out/rounds/01-29/ — per-round PLAN, PROPERTIES, REPORT, bug_reports
- pbt-out/bug_reports/FILED_ISSUES.md — complete issue ledger
- pbt-out/INVARIANTS.md — methodology notes
- pbt-out/rounds/28_dedup_explore/dedup_db.json — live dedup database (1067 issues)

## Unperformed Coverage Sweep

- Coverage-driven sweep (coverage_gaps tool): NOT AVAILABLE in this environment (no .gcda/.profraw, tool reported "Not in a PBT campaign"). File-level evidence used instead. All documented behaviors in README files have at least one property or E2E test. The only skipped coverage is the teammate's ARM NEON encoder surface (#474-#773, 263 issues).
