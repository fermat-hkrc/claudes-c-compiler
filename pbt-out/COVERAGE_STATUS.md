# Coverage Status

**Campaign:** encode_invlpg (standard tier)
**Coverage evidence:** file-level (symbol presence) — `coverage_gaps` reported no .gcda/.profraw for this Rust run (harness looks at unrelated OH C++ binaries and says encode_invlpg NOT LINKED there). Execution evidence is cargo test output: 17 tests in `encode_invlpg_pbt` exercised `InstructionEncoder::encode` → `encode_invlpg` (12 pass / 5 fail).

## This campaign

| Metric | Value |
|--------|-------|
| Target function | encode_invlpg |
| Properties | 10 (8 passing, 2 failing) |
| KAT / regression | 4 KAT pass, 1 KAT fail, 2 regression fail |
| Bugs | 1 (missing segment prefix) |
| Documented behaviors covered | opcode 0F 01 /7, base+disp, SIB, abs disp32, ESP/EBP edges, segment prefix (failing), segment+SIB (failing), metamorphic vs lidt, arity error, non-memory error |

## Sweep (standard, 1 round)

- `coverage_gaps` → no line-level data; file-level OH binaries irrelevant for Rust SUT.
- All documented behaviors of encode_invlpg already have properties. Closed after filing 1 bug.
