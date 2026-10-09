# Coverage status

> Last updated: 2026-10-09 (campaign: encode_pop)
> Coverage evidence: file-level (symbol presence via cargo lib test) — coverage_gaps: no line-level .gcda/.profraw; tool listed unrelated OH binaries and reported encode_pop NOT LINKED (false negative — cargo test --lib encode_pop_pbt executed the symbol; 13 pass / 11 fail including KATs)
> Tier: standard

## Change-surface function

| Function | Location | Property? | Notes |
|----------|----------|-----------|-------|
| encode_pop | gp_integer.rs:402 | yes | 5 props passing, 3 failing (2 root-cause bugs; b1 also has metamorphic witness b3) |

## Branch / behavior coverage (file-level)

| Behavior | Property | Status |
|----------|----------|--------|
| r32 short form 58+n | encode_pop_diff_r32, encode_pop_invariant_r32_opcode | passing |
| Sreg es/ss/ds/fs/gs | encode_pop_diff_sreg | passing |
| Memory 8F /0 bare | encode_pop_diff_mem | passing |
| Segmented memory | encode_pop_diff_mem_segment, encode_pop_meta_segment_stripped_eq_bare | failing (bug) |
| Arity ≠ 1 → Err | encode_pop_neg_arity | passing |
| cs rejected | encode_pop_neg_cs | passing |
| r8/r16/xmm rejected | encode_pop_neg_r8/r16/xmm | failing (bug) |
| pop == popl alias | encode_pop_meta_pop_eq_popl | passing |

## Contract-surface sweep

- Round 1 (standard tier): coverage_gaps → no line-level data; file-level cargo test executes encode_pop. No additional documented branch without a property. Strengthening covers Sreg invariant, pop≡popl, mixed arity (all pass).
