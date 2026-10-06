# PBT Coverage Status

> Last updated: 2026-10-06 15:27 (campaign: encode_alu_reg_w / encode_op32)
> Coverage evidence: file-level (symbol presence) — coverage_gaps returned no .gcda/.profraw
> This campaign tested encode_alu_reg_w (requested encode_op32) in base.rs: 7 passing / 1 failing properties.

## This campaign

| Function | Source | Tested | Result |
|----------|--------|--------|--------|
| encode_alu_reg_w | base.rs | yes | 7 pass / 1 fail (extra operand) |

## Summary

| Metric | Value |
|--------|-------|
| Campaign target | encode_alu_reg_w (requested encode_op32) |
| Properties | 8 |
| Passing | 7 |
| Failing | 1 |
| Bugs | 1 |
| Coverage evidence | file-level (symbol presence) |
| Sweep | 1/1 spent (manual get_reg arm audit) |
