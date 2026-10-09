# Coverage Status

**Campaign target:** encode_in (src/backend/i686/assembler/encoder/system.rs:75)
**Effort tier:** standard
**Coverage evidence:** file-level (symbol presence) — no .gcda/.profraw for the Rust cargo lib target; `coverage_gaps` inspected unrelated OH C++ pbt binaries and reported encode_in NOT LINKED there (expected). Execution evidence is the cargo test run of `encode_in_pbt` (12 passing / 7 failing cases including KAT/regression).

## Sweep round 1 (standard allowance)

- Tool: `coverage_gaps` → no line-level data; file-level only on OH C++ binaries.
- Documented behaviors of encode_in already have properties:
  - DX-port differential + opcode invariant
  - imm8-port differential + opcode invariant
  - metamorphic inw = 66|inl
  - metamorphic DX vs imm opcode families
  - negative arity
  - negative wrong registers (FAIL → bug B1)
  - negative imm OOR (FAIL → bug B2)
  - differential (%dx) memory form (FAIL → bug B3)
- No additional documented branch without a property; sweep closed.

## Counts

| Metric | Value |
|--------|-------|
| Properties | 10 |
| Passing | 7 |
| Failing | 3 |
| Bugs filed | 3 |
| KAT | 6 (5 pass, 1 fail on (%dx)) |
| Regression witnesses | 3 (all fail, confirming bugs) |
