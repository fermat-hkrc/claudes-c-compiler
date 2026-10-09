# Coverage status — encode_test campaign

**Tier:** standard
**Target:** encode_test @ src/backend/i686/assembler/encoder/gp_integer.rs:645
**Coverage evidence:** file-level (symbol presence / execution via cargo test) — native line coverage tool reported no .gcda/.profraw for this Rust host build; `coverage_gaps` also listed unrelated C++ binaries. Evidence of execution: 11 passing + 9 failing test outcomes from `cargo test --lib encode_test_` exercising InstructionEncoder::encode → encode_test.

## This campaign

| Function | Linked/Executed | Properties | Notes |
|----------|-----------------|------------|-------|
| encode_test | yes (cargo lib test) | 10 (5 pass / 5 fail) | RR/ImmReg/ImmMem bare pass; RegMem, segment, width, non-GP fail |

## Sweep (standard, 1 round)

- `coverage_gaps` called once after first full run.
- Tool: no line-level data; file-level said NOT LINKED (stale OH C++ binary set — not applicable to this Rust crate).
- Contract surface already covered by properties: RR, Imm→Reg, Imm→Mem bare, Reg→Mem, segment Imm→Mem, metamorphic seg, arity, mismatched width, non-GP.
- Residual untested (no stronger oracle without reloc harness): Label/symbol memory forms for TEST; out-of-range imm truncation characterization.
- Close reason: tier sweep round complete; documented behaviors have properties.
