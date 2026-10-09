# Coverage Status

**Campaign:** encode_pop16 (i686)
**Tier:** standard
**Date:** 2026-10-09

## Summary

| Metric | Value |
|--------|-------|
| Target function | encode_pop16 |
| Source | src/backend/i686/assembler/encoder/system.rs:325 |
| Properties | 8 (4 passing, 4 failing) |
| Bugs filed | 3 |
| Coverage evidence | file-level / execution via cargo test (coverage_gaps: no .profraw; reported unrelated OH binaries NOT LINKED — Rust lib tests still executed the symbol; SUT byte outputs prove linkage) |

## Module breakdown

| Module | Scanned | Tested | Notes |
|--------|---------|--------|-------|
| encode_pop16 | yes | yes | HARD scope sole target |
| other system.rs fns | indexed | skipped | HARD scope |

## Untested in this campaign

All other functions in system.rs (HARD scope: encode_pop16 only).

## Contract-surface sweep

1 round via `coverage_gaps`: no line-level data; file-level tool mis-attributed (OH binaries). Documented behaviors all have properties: r16 differential, Sreg differential, memory (+segment), r16 invariant, popw↔popl GP metamorphic, Sreg metamorphic, arity/cs negative, wrong-width negative. Closed because tier's one sweep round is done and every documented behavior has a property.
