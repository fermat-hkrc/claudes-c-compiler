# PBT Coverage Status

> Last updated: 2026-10-07 (campaign: encode_seqz)
> Coverage evidence: file-level (symbol presence) — coverage_gaps reported no .gcda/.profraw; C++ reporter binaries do not link encode_seqz. Rust execution evidence: `cargo test --lib encode_seqz_pbt`.

## Summary

| Metric | Value |
|--------|-------|
| Target function | encode_seqz |
| Source | src/backend/riscv/assembler/encoder/pseudo.rs:257 |
| Properties | 9 (8 passing, 1 failing) |
| KAT | passing (llvm-mc seqz/sltiu pins) |
| Bugs | 1 (extra operand ignored) |
| Tier sweep | 1 round — manual contract audit (llvm-mc/sltiu/I-type/ABI/isolation/arity-invalid passing; extra filed) |

## Module Breakdown

| Module | Properties | Passing | Failing | Bugs |
|--------|------------|---------|---------|------|
| encode_seqz | 9 | 8 | 1 | 1 |

## Untested documented behaviors

(none remaining within tier budget — extra-operand gap is the filed bug)
