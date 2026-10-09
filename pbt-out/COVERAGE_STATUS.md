# Coverage Status

**Campaign:** encode_movzx (i686 gp_integer)
**Tier:** standard
**Evidence level:** file-level (Rust lib tests exercise the symbol via InstructionEncoder::encode → encode_movzx); native line coverage may appear under pbt-out/code-coverage/ when instrumented.

| Metric | Value |
|--------|-------|
| Target function | encode_movzx |
| Properties | 12 (8 passing, 4 failing) |
| Bugs | 4 reports / 3 root causes |
| Test file | src/backend/i686/assembler/encoder/encode_movzx_pbt.rs |
| Contract-surface sweep | 1 round — coverage_gaps (file-level; no Rust .profraw); added P12 unsupported-shape Err path (passing) |

## Untested in this campaign
HARD scope is encode_movzx only; other gp_integer symbols deferred.
