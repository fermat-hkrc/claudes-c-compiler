# x86 assembler silently accepts out-of-range SSE imm8 operands (pshufd $256 / $-129, pinsrw $256)
> **Filed as [issue #822](https://github.com/fermat-hkrc/claudes-c-compiler/issues/822)**

**Law:** SSE immediate operands are imm8 (0..255 unsigned or -128..127 signed per form); out-of-range immediates are unencodable and both GNU as and llvm-mc reject them.

**Impact:** silent wrong encoding of external .s — the #776 imm-truncation class on the SSE path: `pshufd $256, %xmm0, %xmm1` (imm8 field takes the low byte 0) and `pshufd $-129` are silently assembled by ccc while both references reject.

**Detected by:** round-26 imm8-boundary probe (as AND llvm-mc reject all three; ccc accepts all three)
**Minimal input:** `.text\npshufd $256, %xmm0, %xmm1`
**Severity:** medium (silent wrong value class, like #776/#792)
**Contract evidence:** inferred (double-reference rejection; imm8 field width)
**Fix:** range-check SSE imm8 operands ([-128,255] band per form) before packing.
**Regression test:** deterministic: the three forms must be rejected.
