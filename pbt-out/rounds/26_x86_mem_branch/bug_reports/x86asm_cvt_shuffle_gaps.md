# x86 assembler lacks the SSE conversion/shuffle subset: cvtt*2si* entirely (reg and mem), cvtsi2*/pshuf* memory-source forms, maskmovq/maskmovdqu
> **Filed as [issue #818](https://github.com/fermat-hkrc/claudes-c-compiler/issues/818)**

**Law:** the assembler's documented SSE coverage ("SSE through SSE4.1") includes the scalar conversion and shuffle instructions; both GNU as and llvm-mc assemble every form below.

**Impact:** external .s files using these forms fail to assemble. Scope (all double-reference confirmed: as AND llvm-mc accept, ccc rejects):
- `cvttss2si %xmm0, %eax` — the whole cvtt* truncate family is missing, REGISTER source included (cvttss2si/cvttss2siq/cvttsd2si/cvttsd2siq)
- `cvtsi2sdl 8(%rax), %xmm0`, `cvtsi2sdq`, `cvtsi2ssl` — memory-source forms (register forms work)
- `pshuflw/pshufhw/pshufd $imm, mem, %xmm` — memory-source forms (register forms work)
- `maskmovq %mm1, %mm2`, `maskmovdqu %xmm1, %xmm2` — missing entirely

**Function:** x86 assembler SSE encoder (conversion/shuffle subset)
**Source location:** src/backend/x86/assembler/encoder/sse.rs (cvtt family absent; cvtsi2*/pshuf* accept register sources only; no maskmov arm)
**Detected by:** round-26 memory-operand + conversion probe matrix (24 forms; the register-only round-15 families never generated memory sources)
**Minimal input:** `.text\ncvttss2si %xmm0, %eax` → ccc: unhandled; as+llvm-mc: assemble
**Expected:** assembles (SSE scalar conversion)
**Actual:** rejected
**Severity:** low (completeness; same class as #777/#782 — no wrong code)
**Contract evidence:** inferred (double-reference acceptance; x86/README "SSE through SSE4.1")
**Fix:** add the cvtt*2si{,q} arms (0F 2C/2D with 66/F2/F3 prefixes), memory-source parsing in the cvtsi2* and pshuf* arms, and the maskmovq/dqu arm (0F F7).
**Regression test:** deterministic: assemble the four minimal forms.
