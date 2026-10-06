# PBT Coverage Status

> Last updated: 2026-10-06 (campaign: encode_csr / requested encode_system)
> Coverage evidence: file-level (symbol presence) — coverage_gaps reported no .gcda/.profraw (Rust cargo test, C++ reporter listed unrelated binaries).

## This campaign

| Metric | Value |
|--------|-------|
| Target | encode_csr (requested encode_system) |
| Source | src/backend/riscv/assembler/encoder/system.rs:40 |
| Properties | 10 (7 passing, 3 failing) |
| KAT | 3 passing |
| Regression witnesses | 3 failing (as expected) |
| Bugs | 3 |
| Sweep | 1/1 spent (Reg-as-CSR + decimal CSR name) |

## Summary

Requested `--func encode_system` is absent from base.rs. Mapped to encode_csr, the SYSTEM I-type CSR encoder (csrrw/csrrs/csrrc). encode_csr is exercised by cargo test --lib encode_csr. Remaining documented gaps are the three filed bugs (extra operand, zimm oob, csr oob).
