# Coverage Status

> Last updated: 2026-10-07 (campaign: encode_snez)
> Coverage evidence: file-level (symbol presence) — `coverage_gaps` found no .gcda/.profraw (Rust cargo tests are not the C++ reporter binaries); it listed encode_snez as NOT LINKED against unrelated host PBT binaries. Execution evidence is the cargo test run of `encode_snez_pbt` (11 tests: 9 pass, 2 fail on the same extra-operand bug).

| Metric | Value |
|--------|-------|
| Target function | encode_snez |
| Source | pseudo.rs:263 |
| Test file | encode_snez_pbt.rs |
| Properties | 9 (8 passing, 1 failing) + KAT + regression |
| Documented behaviors exercised | llvm-mc snez, llvm-mc sltu x0, encode_alu_reg SLTU metamorphic, R-type fields, ABI/xN/Imm alias, field isolation, arity-too-few, invalid operand, extra operand |
| Remaining gap | extra-operand rejection (filed bug) |

Sweep closed: tier standard owes 1 coverage_gaps round; spent. Remaining documented gap is the extra-operand bug.
