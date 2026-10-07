# Coverage status — encode_blez campaign

**Tier:** standard
**Coverage evidence:** file-level (symbol presence) — no Rust .profraw/.gcda consumed by `coverage_gaps`; C++ reporter listed unrelated host PBT binaries and marked `encode_blez` NOT LINKED.
**Execution evidence:** `cargo test --lib encode_blez -- --test-threads=1` ran 14 tests against production `encode_blez` in `pseudo.rs` (12 property passes, 1 property fail, 1 KAT pass, 1 regression fail).

| Metric | Value |
|--------|-------|
| Target function | encode_blez |
| Properties | 13 |
| Passing | 12 |
| Failing | 1 (extra operand) |
| Bugs filed | 1 |
| Sweep rounds used | 1 / 1 |

Documented behaviors with a property:
- llvm-mc `blez rs, 0` differential
- llvm-mc `bge x0, rs, 0` expansion differential
- in-tree `encode_branch_instr` BGE metamorphic
- B-type ISA field layout (rs1=x0, rs2=rs, funct3=BGE)
- ABI / xN / fp alias
- Symbol / Label / Reg / Imm targets
- Imm-as-rs bare register number
- arity < 2, invalid rs, invalid target negatives
- extra operand negative (failing → bug)
