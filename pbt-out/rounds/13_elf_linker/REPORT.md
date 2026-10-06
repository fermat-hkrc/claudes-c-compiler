# PBT Campaign Report: elf + linker_common + stack_layout (round 13)

## Summary

**Verdict:** No bugs; 4 properties pass. StringTable interning invariants, the canonical escape parser (vs an independent renderer over the documented escape forms), the GNU numeric-label helper laws, and the multi-target ELF structure KAT (ET_REL + correct e_machine for all four targets) all hold.
**Date:** 2026-10-06
**Tests:** 4 properties
**Result:** 4 passing, 0 bugs
**Tier:** standard

## Modules Tested
| Module | Tests | Bugs | Oracles |
|---------|-------|------|---------|
| elf::string_table | 1 (P1) | 0 | algebraic.invariant (interning) |
| elf::parse_string | 1 (P2) | 0 | differential (independent renderer) |
| elf::numeric_labels | 1 (P3) | 0 | reference (GNU 1f/1b table laws) |
| four backends' object writers | 1 (P4) | 0 | reference (readelf e_machine/e_type) |

## Bugs Found
(none)

## Test Files Created
| File | Tests |
|------|-------|
| src/backend/elf/mod.rs (pbt_tests) | P1-P3 |
| src/backend/mod.rs (pbt_struct) | P4 |

## Reproduction
cd /home/shuhao/fermat-users/leo/github/claudes-c-compiler
PATH="$HOME/.cargo/bin:$PATH" RUST_TEST_THREADS=1 cargo test --lib backend::elf::pbt_tests backend::pbt_struct --nocapture

## Output Directories
- pbt-out/rounds/13_elf_linker/
