# Coverage Status — encode_alu campaign

**Tier:** standard
**Coverage evidence:** file-level (symbol presence) — `coverage_gaps` found no Rust `.profraw`/`.gcda` from this run (tool reported unrelated host C++ binaries; Rust cargo lib test executed `encode_alu` via `InstructionEncoder::encode` in `encode_alu_pbt.rs`). Execution evidence: cargo test log `pbt-out/run/encode_alu_i686.log` (11 passed / 8 failed including properties that call the SUT).

## Summary
| Metric | Value |
|--------|-------|
| Target function | encode_alu |
| Source | gp_integer.rs:433 |
| Properties | 9 (4 passing, 5 failing) |
| Bugs filed | 4 |
| Sweep rounds owed/done | 1 / 1 (coverage_gaps called; no new documented branch reachable with stronger oracle beyond residual Symbol/Label/GOTPC reloc forms) |

## Residual documented surfaces without dedicated property
- `_GLOBAL_OFFSET_TABLE_` / R_386_GOTPC imm→reg path (gp_integer.rs:484-494) — needs reloc-aware oracle; not differential-byte-comparable to bare llvm-mc without object emission.
- SymbolDiff / Label-as-memory arms (gp_integer.rs:573-640) — same reloc limitation.
These are recorded, not invented dead-code targets.

## Module breakdown
| Module | Tested | Bugs |
|--------|--------|------|
| encode_alu | yes | 4 |
