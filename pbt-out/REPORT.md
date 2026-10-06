# PBT Campaign Report: x86 assembler deep encoder sweep (round 15)

## Summary

**Verdict:** 5 new bugs filed (#774-#777, #782), all in the x86 builtin assembler — a surface round 09's C-level E2E never exercised. All are double-reference confirmed (GNU as AND llvm-mc agree; ccc disagrees): #774 GPR width mismatch silently coerced, #775 SIMD width silently promoted, #776 immediate overflow silently truncated (add $2^32 → add $0), #777 suffix-less popcnt/lzcnt/tzcnt rejected, #782 packed min/max family (minps/maxps/minpd/maxpd) missing entirely (scalar forms work). Across 58 template families (18 original + 20 sweep + mutations), every form both references accept encodes byte-identically to llvm-mc. Dedup: zero tracker hits for all five classes (titles+bodies, all states; the teammate's 263 encoder issues #474-#773 are all ARM NEON).
**Date:** 2026-10-06
**Repository:** claudes-c-compiler (/home/shuhao/fermat-users/leo/github/claudes-c-compiler)
**Modules tested:** src/backend/x86/assembler (parser + encoders, via -c on generated .s)
**Tests:** 3 PBT properties (collection-style) + 5 deterministic red regressions
**Result:** p1 failing (→ #774/#777), p2 failing (→ #774/#775/#776), p3 failing (→ #782); 5 red witnesses
**Change surface:** (none — deep pass over round-09's 0-bug surface)
**Coverage evidence:** file-level (symbol presence) — coverage_gaps reported no line-level data (no .gcda/.profraw) and a NOT LINKED list for src/passes functions that is the known false negative for #[cfg(test)]-compiled Rust lib symbols (round-07's properties call those functions directly — their panics originate inside them). The sweep round therefore enumerated the assembler's documented surfaces against the generated families (58 total) and found #782.
**Tier:** standard (1 coverage-driven sweep round consumed)

## Modules Tested

| Module | Tests | Bugs | Oracles Used |
|--------|-------|------|--------------|
| x86/assembler parser + encoders | p1 (400 forms), p2 (400 mutants), p3 (400 sweep forms), 5 red regressions | 5 (#774-#777, #782) | differential (llvm-mc byte-exact) + negative_error (double-reference: GNU as ∧ llvm-mc) |

## Bugs Found

### B1 (#774): GPR operand width mismatch silently coerced
**Formal:** ∀ form with size suffix S and register of width W≠S: both references reject ⇒ ccc must reject. Falsified: `movl %eax, %r8w`, `addq %r10d, %r10`, `movl 596(%rsp), %r13`.
**Contract evidence:** inferred (double-reference: GNU as and llvm-mc both reject)
**Documentation conflict:** (none)
**Severity:** medium
**Counterexample:** `movl %eax, %r8w` → assembles `41 89 c0` (mov %eax,%r8d)
**Expected / Actual:** error / silently reinterpreted at the opcode's width
**Impact:** external .s files assemble to different instructions than written; internal C pipeline unaffected (round-09 E2E green).
**Bug report:** bug_reports/x86asm_gpr_width_silent_coercion.md
**Repro seed:** test_x86asm_regression_gpr_width_rejected
**Raw output:** (in the bug report)

### B2 (#775): SIMD register width mismatch silently promoted
**Formal:** ∀ AVX form with mixed xmm/ymm: both references reject ⇒ ccc must reject. Falsified: `vaddps %ymm0, %ymm1, %xmm2`.
**Contract evidence:** inferred (double-reference)
**Severity:** medium
**Counterexample:** `vaddps %ymm0, %ymm1, %xmm2` → `c5 f4 58 d0` (xmm2 as ymm2)
**Bug report:** bug_reports/x86asm_simd_width_silent_promotion.md
**Repro seed:** test_x86asm_regression_simd_width_rejected

### B3 (#776): immediate overflow silently truncated
**Formal:** ∀ imm outside the opcode's imm width: both references reject ⇒ ccc must reject. Falsified: `addq $4294967296, %rax` → `48 05 00 00 00 00`.
**Contract evidence:** inferred (double-reference)
**Severity:** medium (silent wrong constant — the worst class)
**Bug report:** bug_reports/x86asm_imm32_overflow_silent_truncation.md
**Repro seed:** test_x86asm_regression_imm32_overflow_rejected

### B4 (#777): suffix-less popcnt/lzcnt/tzcnt rejected
**Formal:** ∀ bare width-inferable form: both references accept ⇒ ccc must accept. Falsified: `popcnt %rax, %rbx`.
**Contract evidence:** inferred (double-reference acceptance)
**Severity:** low (completeness gap, no wrong code)
**Bug report:** bug_reports/x86asm_bare_popcnt_lzcnt_tzcnt_rejected.md
**Repro seed:** test_x86asm_regression_bare_popcnt_assembles

### B5 (#782): packed min/max family missing (sweep round)
**Formal:** ∀ documented SSE form: both references accept ⇒ ccc must accept. Falsified: `minps/maxps/minpd/maxpd %xmm, %xmm` (scalar minss/maxss/minsd/maxsd assemble fine).
**Contract evidence:** inferred (double-reference acceptance; x86/README documents "SSE through SSE4.1" for the builtin assembler)
**Severity:** low
**Bug report:** bug_reports/x86asm_packed_minmax_missing.md
**Repro seed:** test_x86asm_regression_packed_minmax_assembles

## Design Caveats (if any)

(none)

## Test Files Created

| File | Tests |
|------|-------|
| src/backend/x86/mod.rs (pbt_asm_deep) | p1, p2, p3 + 5 red regressions (#774-#777, #782) |

## Reproduction

```bash
cd /home/shuhao/fermat-users/leo/github/claudes-c-compiler
PATH="$HOME/.cargo/bin:$PATH" RUST_TEST_THREADS=1 cargo test --lib backend::x86::pbt_asm_deep --nocapture
```

## Output Directories

- pbt-out/rounds/15_x86_asm_deep/ — PLAN.md, PROPERTIES.md, report.json, COVERAGE.md, bug_reports/x86asm_*.{md} (×5), run/ (this report also lives at pbt-out/REPORT.md as the current-round summary)
- Methodology notes (not caveats): llvm-mc-only rejections are GNU-permissive convention and are NOT filed (the double-reference protocol exists to avoid that false-positive class); AVX-512 EVEX forms need matched -mattr on both sides (follow-up); ARM NEON is the teammate's surface (#474-#773) and was never generated.
