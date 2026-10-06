# PBT Coverage Status

> Last updated: 2026-10-06 (campaign: encode_c_jalr)
> Coverage evidence: file-level (symbol presence) — coverage_gaps reported no .gcda/.profraw; encode_c_jalr NOT LINKED in C++ reporter binaries (Rust cargo tests are not those binaries). Manual audit of the documented C.JALR surface drove every contract.

## Summary

| Metric | Value |
|--------|-------|
| Target | encode_c_jalr |
| Source | compressed.rs:56 |
| Properties | 7 (5 passing, 2 failing) |
| KAT | 6 passing |
| Regression witnesses | 2 failing |
| Bugs | 2 medium |
| Sweep | 1 round (standard); no new properties |

## This campaign

| Function | Source file | Test file | Test target | Notes |
|----------|-------------|-----------|-------------|-------|
| encode_c_jalr | compressed.rs | encode_c_jalr_pbt.rs | cargo test --lib encode_c_jalr | 5 passing / 2 failing; extra and rs1=x0 bugs |

## Documented behaviors vs properties

| Behavior | Property | Status |
|----------|----------|--------|
| Valid 1-op encoding matches llvm-mc | encode_c_jalr_diff_llvm_mc | passing |
| CR-type op/funct4/rs1/rs2 | encode_c_jalr_cr_type_fields | passing |
| ABI vs xN alias | encode_c_jalr_abi_xn_alias | passing |
| rs1 field isolation | encode_c_jalr_field_isolation | passing |
| Empty / FP src Err | encode_c_jalr_neg_arity_fp | passing |
| Extra operand Err | encode_c_jalr_neg_extra | failing (B2) |
| rs1=x0 Err (C.EBREAK) | encode_c_jalr_neg_rs1_x0 | failing (B1) |
