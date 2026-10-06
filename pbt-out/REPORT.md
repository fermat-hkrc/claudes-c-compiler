# PBT Campaign Report: src/driver + glue (round 14 — FINAL round of the whole-repo loop)

> **Whole-repo loop COMPLETE (rounds 01–14).** 26 PBT bugs filed as issues
> #516–#518, #526–#530, #538–#540, #559–#563, #573–#576 (+#2/#3 dups),
> #759, #760, #765, #770. Final suite: 608 passed / 44 failed (all documented
> red witnesses from rounds 01–08) / 11 ignored. Rounds 09–14 (x86 E2E, arm
> + riscv assembler differentials, i686 E2E, elf helpers, driver plumbing)
> contributed 4 more issues (#765, #770, and the round-09/10/11/13/14
> surfaces are green). Per-round reports: pbt-out/rounds/*/

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
