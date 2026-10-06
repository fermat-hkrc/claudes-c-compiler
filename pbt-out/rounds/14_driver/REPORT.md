# PBT Campaign Report: src/driver + glue (round 14 — final)

## Summary

**Verdict:** No bugs; mode plumbing holds for both selectable targets across all four CompileModes (-E/-S/-c/Full): each mode produces its well-formed artifact (text containing the function, ET_REL object with the right machine, runnable zero-exit executable).
**Date:** 2026-10-06
**Tests:** 1 property (64 cases: 2 targets × 4 modes × 16 sources)
**Result:** 1 passing, 0 bugs
**Tier:** standard

## Bugs Found
(none)

## Test Files Created
| File | Tests |
|------|-------|
| src/driver/mod.rs (pbt_tests) | p1_driver_mode_plumbing |

## Reproduction
cd /home/shuhao/fermat-users/leo/github/claudes-c-compiler
PATH="$HOME/.cargo/bin:$PATH" RUST_TEST_THREADS=1 cargo test --lib driver::pbt_tests --nocapture

## Output Directories
- pbt-out/rounds/14_driver/
