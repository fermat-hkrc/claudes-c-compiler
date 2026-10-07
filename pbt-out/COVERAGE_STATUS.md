# Coverage Status

**Mode:** incremental (standard tier)
**Primary target:** encode_sgtz (pseudo.rs:275)
**Coverage evidence:** file-level (symbol presence) — `coverage_gaps` returned no native .gcda/.profraw for this Rust cargo test run; C++ reporter listed unrelated binaries and marked encode_sgtz NOT LINKED. Execution evidence is the cargo lib test run itself (`cargo test --lib encode_sgtz`, 11 tests, real SUT symbol linked into `ccc` libtest).

## This campaign

| Function | Scanned | Tested | Notes |
|----------|---------|--------|-------|
| encode_sgtz | yes | yes | 8/9 properties passing; 1 bug (extra operand) |

## Contract-surface sweep (1 round, standard)

- llvm-mc differential (`sgtz` and `slt rd, x0, rs`): covered, passing
- R-type field invariant + bounds 0/31: covered, passing
- ABI/xN/fp/zero/Imm alias metamorphic: covered, passing
- Field isolation: covered, passing
- Arity-too-few + invalid operand negative: covered, passing
- Extra operand negative: covered, **failing** (filed bug)

Closed: tier sweep round spent; remaining documented gap is the extra-operand bug.

## Skipped (HARD scope)

All other pseudo.rs functions — outside `--func encode_sgtz`.
