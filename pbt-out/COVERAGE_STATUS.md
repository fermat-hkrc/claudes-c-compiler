# PBT Coverage Status

> Last updated: 2026-10-07 (campaign: encode_sltz)
> Scope: encode_sltz only (HARD --func). Coverage evidence: file-level (symbol presence) + cargo test execution of encode_sltz_pbt.

## Summary

| Metric | Value |
|--------|-------|
| Target function | encode_sltz |
| Source | pseudo.rs:269 |
| Properties | 9 (8 passing, 1 failing) |
| Bugs | 1 (extra operand) |
| KAT gate | pass (llvm-mc + SUT) |
| Generator runs | 1000 per property (proptest) |
| Coverage evidence | file-level — no .profraw for Rust cargo; symbol exercised by cargo test --lib encode_sltz |

## Documented behaviors vs properties

| Behavior | Property | Status |
|----------|----------|--------|
| Matches llvm-mc `sltz rd, rs` | encode_sltz_diff_llvm_mc | passing |
| Matches llvm-mc `slt rd, rs, x0` | encode_sltz_diff_llvm_mc_slt | passing |
| Equals encode_alu_reg SLT x0 | encode_sltz_eq_slt_x0 | passing |
| R-type SLT field layout | encode_sltz_isa_fields | passing |
| ABI / xN / Imm alias | encode_sltz_abi_xn_alias | passing |
| Field isolation | encode_sltz_field_isolation | passing |
| Arity < 2 → Err | encode_sltz_neg_arity | passing |
| Invalid operand → Err | encode_sltz_neg_invalid | passing |
| Extra operand → Err | encode_sltz_neg_extra | failing (bug) |

## Contract-surface sweep

- Tool: coverage_gaps → no line-level data; encode_sltz NOT LINKED in C++ OH binaries (expected for Rust SUT).
- Manual audit closed the standard tier's one round: all documented behaviors have a property; remaining gap is the filed extra-operand bug.
