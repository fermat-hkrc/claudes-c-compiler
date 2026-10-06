# PBT Campaign Report: src/backend/riscv (round 11)

## Summary

**Verdict:** No bugs; the builtin RISC-V assembler's encodings are byte-identical to llvm-mc over 200 generated label-free RV64 instruction sequences (20-40 instructions each) covering register-register ALU (incl. w-forms), immediates, lui, loads/stores and shift amounts.
**Date:** 2026-10-05
**Modules tested:** src/backend/riscv (builtin assembler general surface)
**Tests:** 1 property (p1)
**Result:** 1 passing, 0 bugs
**Coverage evidence:** none (native; manual session). Behavioral: every generated sequence assembled by the real pipeline (Driver argv0=ccc-riscv -c) and byte-compared with llvm-mc -triple=riscv64.
**Tier:** standard

## Modules Tested
| Module | Tests | Bugs | Oracles |
|---------|-------|------|---------|
| riscv assembler (builtin, general surface) | 1 (P1) | 0 | differential (llvm-mc byte-exact) |

## Bugs Found
(none)

## Test Files Created
| File | Tests |
|------|-------|
| src/backend/riscv/mod.rs (pbt_asm) | p1_riscv_asm_matches_llvm_mc |

## Reproduction
cd /home/shuhao/fermat-users/leo/github/claudes-c-compiler
PATH="$HOME/.cargo/bin:$PATH" RUST_TEST_THREADS=1 cargo test --lib backend::riscv::pbt_asm --nocapture

## Output Directories
- pbt-out/rounds/11_riscv/ (PLAN, PROPERTIES, REPORT, report.json, run/)
