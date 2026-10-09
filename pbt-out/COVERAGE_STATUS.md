# Coverage Status

**Campaign:** encode_verw (i686)
**Tier:** standard
**Coverage evidence:** file-level (symbol presence) — no .gcda/.profraw for Rust; `coverage_gaps` listed unrelated OH C++ pbt_test binaries and reported encode_verw NOT LINKED there (expected). Execution evidence is cargo lib test `encode_verw_pbt` (11 pass / 6 fail including KAT+regression).

## Sweep round 1

- Tool: `coverage_gaps` → no line-level data; file-level only on non-Rust binaries.
- Documented behaviors of encode_verw already have properties:
  - memory base/disp/SIB/abs (differential, passing)
  - r16 register (differential, passing)
  - segment override (differential, failing → B1)
  - segment+SIB (differential, failing → B1)
  - opcode 0F 00 /5 invariant (passing)
  - arity ≠ 1 (negative, passing)
  - non-r16 / imm / label (negative, failing on non-r16 → B2)
- No additional property written: every match arm and documented contract path already targeted.
- Closed after filing 2 bugs.

## Counts

| Metric | Value |
|--------|-------|
| Properties | 10 |
| Passing | 7 |
| Failing | 3 |
| Bugs | 2 |
| KAT | 5 (4 pass, 1 fail = segment FS) |
| Regression witnesses | 2 (both fail, pin B1/B2) |
