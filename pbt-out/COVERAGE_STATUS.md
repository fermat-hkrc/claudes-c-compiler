# Coverage Status

**Campaign:** encode_prefetch_0f0d (i686)
**Tier:** standard
**Coverage evidence:** file-level (symbol presence) — `coverage_gaps` reported no .gcda/.profraw for this Rust target; C++ reporter listed unrelated OH binaries and said `encode_prefetch_0f0d` NOT LINKED there (expected: SUT is Rust, exercised via `cargo test --lib encode_prefetch_0f0d`).

## This campaign

| Metric | Value |
|--------|-------|
| Target function | encode_prefetch_0f0d |
| Source | src/backend/i686/assembler/encoder/system.rs:25 |
| Properties | 9 (8 passing, 1 failing) |
| KAT / deterministic | 5 (4 pass, 1 fail segment FS) + 1 regression witness (fail) |
| Bugs | 1 high (missing segment prefix) |
| Documented behaviors covered | opcode 0F 0D /1, base+disp, SIB, ESP/EBP edges, abs32, arity error, non-mem error, segment prefix, metamorphic opcode stability |

## Sweep (round 1)

- `coverage_gaps`: no line-level data; file-level NOT LINKED in C++ binaries (N/A for Rust).
- All documented contract surfaces of `encode_prefetch_0f0d` have properties.
- Closed after filing segment-prefix bug.
