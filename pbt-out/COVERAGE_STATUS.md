# Coverage status — encode_mov_seg campaign

**Tier:** standard
**Target:** encode_mov_seg (system.rs:274)
**Coverage evidence:** file-level (symbol presence) — `coverage_gaps` reported no native .gcda/.profraw and listed unrelated OH pbt binaries as NOT LINKED for encode_mov_seg. Campaign cargo tests nevertheless executed the real symbol: failing properties returned concrete SUT byte vectors from `InstructionEncoder::encode` → `encode_mov_seg`.

## Sweep round 1 (standard tier allowance)

- Called `coverage_gaps` after first full test run.
- Tool: no line-level data; file-level scan of non-cargo binaries → false NOT LINKED.
- Documented behaviors of encode_mov_seg already each have a property:
  - arity error path (neg_arity)
  - Sreg→GP / GP→Sreg register (diff + invariant + metamorphic)
  - Sreg→r16 0x66 path (diff_sreg_to_r16 — failing bug)
  - r16→Sreg (diff_r16_to_sreg)
  - mem no-seg base/disp/SIB/abs (diff_mem)
  - mem with segment override + SIB (diff_mem_segment, diff_segment_sib — failing bug)
  - r8 rejection (neg_r8 — failing bug)
  - mnemonic alias mov/movl (diff_mnemonic_aliases)
- No additional documented branch without a property. Sweep closed (tier's 1 round done).

## This campaign

| Metric | Value |
|--------|-------|
| Properties | 12 (8 passing, 4 failing) |
| KAT | 4 pass, 2 fail (contract witnesses) |
| Regression witnesses | 3 (all fail — expected until fix) |
| Bugs | 3 root causes |
| Generator runs | 1000 cases/property (proptest) |
