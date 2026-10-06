# PBT Campaign Report: src/backend/x86 (round 09)

## Summary

**Verdict:** No bugs; 3 end-to-end properties pass. 480 randomly generated C programs (u64 wrap-arithmetic/bitwise/compare/cast expression functions with 1..10 arguments, a loop kernel, and double-precision SSE functions) compiled by ccc at -O0 AND -O2 through the full pipeline (codegen + peephole + builtin assembler + builtin linker) execute natively with stdout byte-identical to gcc on every program. One generator defect was fixed en route (K&R implicit-int parameters made `a1 << 32` UB — a test bug, not a SUT bug).
**Date:** 2026-10-05
**Repository:** claudes-c-compiler (/home/shuhao/fermat-users/leo/github/claudes-c-compiler)
**Modules tested:** src/backend/x86 (full pipeline: codegen/, codegen/peephole/, assembler/, linker/ — composition)
**Tests:** 3 E2E properties (p1+p2 joint harness, p3 doubles)
**Result:** 3 passing, 0 bugs
**Change surface:** (none — whole-scope round; baseline commit 20533174)
**Coverage evidence:** none (native) — `coverage_gaps` answered "Not in a PBT campaign" (manual session). Behavioral evidence instead: every compiled program exercised IR→asm→assembler→linker→native execution at both opt levels; per-file unit properties for the emitters were skipped with reasons (PLAN.md) — their composition is what the E2E oracle tests.
**Tier:** standard

## Modules Tested

| Module | Tests | Bugs | Oracles Used |
|--------|-------|------|--------------|
| x86 codegen + peephole + assembler + linker (composition) | 3 (P1, P2 joint; P3) | 0 | differential (native execution vs gcc), algebraic.invariant (O2 vs O0) |

Final serial run: p1p2 (160 programs × 3 compilations each) 39.2s ok; p3 (160 double programs) 36.2s ok. Backend suite baseline unchanged (201/1/6 — the 1 is round-08's #765 red witness).

## Bugs Found

(none)

## Design Caveats (if any)

- K&R-style parameter lists (undeclared params) default to implicit int — a generator foot-gun recorded for future rounds: always declare parameter types in generated C.

## Test Files Created

| File | Tests |
|------|-------|
| src/backend/x86/mod.rs (`pbt_e2e`) | p1p2_e2e_codegen_and_opt, p3_e2e_double_codegen |

## Reproduction

```bash
cd /home/shuhao/fermat-users/leo/github/claudes-c-compiler
PATH="$HOME/.cargo/bin:$PATH" RUST_TEST_THREADS=1 cargo test --lib backend::x86::pbt_e2e --nocapture
```

## Output Directories

- pbt-out/rounds/09_x86/ — PLAN.md, PROPERTIES.md, REPORT.md (this file), report.json, COVERAGE.md, run/ (generated programs + binaries; successful cases cleaned up)
