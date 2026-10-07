# PBT Coverage Status

> Last updated: 2026-10-07 (campaign: encode_v_arith_vx, English)
> Coverage evidence: file-level (symbol presence) — coverage_gaps reported no .gcda/.profraw and encode_v_arith_vx NOT LINKED in C++ reporter binaries; Rust cargo tests are not those binaries.

## Summary

| Metric | Value |
|--------|-------|
| Target | encode_v_arith_vx |
| Source | src/backend/riscv/assembler/encoder/vector.rs:134 |
| Properties | 8 (6 passing, 2 failing) |
| KAT | 9 passing |
| Regression witnesses | 2 failing |
| Bugs | 2 |
| Effort tier | standard |
| Sweep | 1 round (manual; coverage_gaps file-level NOT LINKED) |

## This campaign

| Function | Source file | Tested | Notes |
|----------|-------------|--------|-------|
| encode_v_arith_vx | vector.rs | yes | 6 pass / 2 fail; extra-operand and mask-v0.t bugs |

## Oracle Type Distribution

| Oracle Type | Count | Status |
|-------------|-------|--------|
| differential | 2 | 1 passing (llvm-mc 3-op), 1 failing (mask v0.t) |
| algebraic.invariant | 2 | passing |
| algebraic.metamorphic | 2 | passing |
| negative_error | 2 | 1 passing (arity/bad regs), 1 failing (extra) |
