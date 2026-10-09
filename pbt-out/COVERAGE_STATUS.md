# PBT Coverage Status

> Last updated: 2026-10-09 (campaign: encode_mov_infer_size)
> Scope: HARD `encode_mov_infer_size` only | Tier: standard
> Coverage evidence: file-level (cargo test execution of production symbol; no .gcda/.profraw)

## Summary

| Metric | Value |
|--------|-------|
| Change-surface functions | 1 (encode_mov_infer_size) |
| With properties | 1 |
| Properties | 9 (7 passing, 2 failing) |
| Bugs | 2 |
| Contract-surface sweep | 1 round (coverage_gaps → file-level) |

## This campaign target

| Function | Source | Linked/Executed | Notes |
|----------|--------|-----------------|-------|
| encode_mov_infer_size | gp_integer.rs | yes (cargo lib-test via encode("mov")) | 7/9 props pass; 2 bugs filed |

## coverage_gaps note

Tool reported no native line coverage and "NOT LINKED" against unrelated OH pbt binaries. Rust campaign evidence is the `ccc` lib-test binary running `InstructionEncoder::encode` with mnemonic `mov`, which dispatches to `encode_mov_infer_size` (mod.rs:163). KATs and 1000-case proptest properties exercise RR/ImmReg/MemReg/CR/Sreg/negative paths.
