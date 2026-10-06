# PBT Coverage Status

> Last updated: 2026-10-06 (campaign: encode_alu_reg / requested encode_op)
> Coverage evidence: file-level (symbol presence) — coverage_gaps reported no .gcda/.profraw; C++ reporter listed unrelated binaries and claimed NOT LINKED. Manual arm audit of encode_alu_reg plus encode_alu_reg_neg_invalid_name sweep.

## Summary

| Metric | Value |
|--------|-------|
| Campaign target | encode_alu_reg (base.rs:260); requested encode_op absent |
| Properties | 8 |
| Passing / Failing | 7 / 1 |
| Bugs | 1 (extra operand ignored) |
| Sweep rounds | 1/1 (invalid integer register names) |
| Effort tier | standard |

## This campaign

| Function | Source File | Tested | Notes |
|----------|-------------|--------|-------|
| encode_alu_reg | base.rs | yes | 7 passing / 1 failing; real SUT via cargo test --lib encode_alu_reg |

## File Coverage (scope)

| Source File | Campaign symbol | Status |
|-------------|-----------------|--------|
| base.rs | encode_alu_reg | tested (this campaign) |

Requested encode_op was not in base.rs. Other base.rs symbols remain out of this campaign's HARD single-symbol scope.
