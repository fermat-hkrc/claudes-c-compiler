# PBT Coverage Status

> Last updated: 2026-10-06 (campaign: encode_vstore)
> Coverage evidence: file-level (symbol presence) — coverage_gaps reported no .gcda/.profraw; encode_vstore NOT LINKED in C++ reporter binaries. Rust cargo tests executed encode_vstore via cargo test --lib encode_vstore.

## This campaign

| Function | Source | Tested | Result |
|----------|--------|--------|--------|
| encode_vstore | vector.rs:104 | yes | 7 passing / 1 failing (extra operand) |

## Sweep

Manual audit of documented 2-op / format / mem-reg / ABI / isolation / arity / extra / nonzero-offset surface. Closed: standard tier 1 round spent; remaining documented gap is the extra-operand bug.
