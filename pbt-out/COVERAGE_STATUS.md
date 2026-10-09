# PBT Coverage Status

> Last updated: 2026-10-09 (campaign: encode_mov_mem_reg)
> Coverage evidence: file-level (symbol presence) — `coverage_gaps` reported no .gcda/.profraw (Rust cargo lib test not producing gcov/llvm-cov artifacts the tool reads). Unrelated OH C++ pbt binaries listed; encode_mov_mem_reg marked NOT LINKED there (expected — wrong language/binary set).
> Campaign execution evidence: `cargo test --lib encode_mov_mem_reg -- --test-threads=1` linked and ran production `InstructionEncoder::encode` → `encode_mov_mem_reg` (10 passed / 6 failed including KAT/regression witnesses).

## Summary

| Metric | Value |
|--------|-------|
| This campaign target | encode_mov_mem_reg |
| Properties | 8 (5 passing, 3 failing) |
| Bugs filed | 3 |
| Sweep rounds (standard) | 1 (done) |
| Line-level coverage | none (no .gcda/.profraw) |
| File-level evidence | cargo test executes symbol |

## Module Breakdown

| Module | Scanned | Tested | Skipped | Coverage |
|--------|---------|--------|---------|----------|
| encode_mov_mem_reg (gp_integer.rs) | 1 | 1 | 0 | executed via cargo test |

## Untested (this campaign HARD scope)

(none — sole target encode_mov_mem_reg has properties)
