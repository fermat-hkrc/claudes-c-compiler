# Coverage status — encode_out campaign

**Coverage evidence:** file-level (symbol presence) — `coverage_gaps` reported no .gcda/.profraw (Rust target; C++ reporter listed unrelated OH binaries). `encode_out` marked NOT LINKED in those C++ binaries (expected). Execution evidence is the cargo lib test run of `encode_out_pbt` (12 pass / 7 fail including KAT/regressions).

## This campaign
| Metric | Value |
|--------|-------|
| Target | encode_out (system.rs:39) |
| Properties | 8 (5 passing, 3 failing) |
| KAT | 5 passing deterministic + 1 failing (%dx) |
| Regression witnesses | 3 failing (one per bug) |
| Bugs | 3 |
| Generator cases | 1000 (proptest) |
| Tier | standard |

## Documented behaviors covered
- DX-port outb/outw/outl vs llvm-mc
- Imm8-port vs llvm-mc (accepted domain)
- Intel fixed opcode invariant
- outw = 0x66 ‖ outl metamorphic
- Wrong arity → Err
- Wrong registers → must Err (bug: accepts)
- Imm out of imm8 range → must Err (bug: truncates)
- (%dx) memory port form vs llvm-mc (bug: unsupported)

## Sweep
Round 1 complete; no additional documented branch without a property.
