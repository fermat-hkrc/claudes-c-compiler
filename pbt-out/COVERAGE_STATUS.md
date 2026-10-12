# Coverage Status

> Last updated: 2026-10-12 (campaign: encode_cmovcc)
> Coverage evidence: file-level (symbol presence) — no .gcda/.profraw from this run (build tree not reconfigured for instrumentation). `coverage_gaps` reported encode_cmovcc as NOT LINKED due to Rust name mangling in the binary scan; the live cargo lib test `encode_cmovcc_pbt` exercises the production symbol via `InstructionEncoder::encode` → `encode_cmovcc` (confirmed by KAT/PBT byte-level oracles).

## This campaign

| Function | Linked into test binary | Properties | Result |
|----------|-------------------------|------------|--------|
| encode_cmovcc | yes (via encode_cmovcc_pbt / InstructionEncoder::encode) | 8 (+1 strengthen mem_w16) | 5 pass / 3 fail (3 bugs) |

## Sweep round 1 (standard)

- Tool: coverage_gaps → no line-level data; file-level note recorded.
- Documented behaviors already covered: RR32/RR16-w/mem base-SIB-abs differential, segment differential (failing B1), unsuffixed r16 (failing B2), opcode invariant, alias metamorphic, neg arity/bad-op/r8/non-GP/mismatch (failing B3).
- Strengthen: added `encode_cmovcc_diff_mem_w16_llvm_mc` (explicit w mem→r16) — PASS (1000 cases).
- Close: every documented contract surface of encode_cmovcc has a property; remaining gaps are the three filed bugs, not missing properties.
