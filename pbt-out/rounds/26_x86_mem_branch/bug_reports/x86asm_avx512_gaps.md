# x86 assembler's 'initial AVX-512' covers only unmasked VL forms — zmm (512-bit) entirely and all {​%k} mask-predication forms are rejected
> **Filed as [issue #820](https://github.com/fermat-hkrc/claudes-c-compiler/issues/820)**

**Law:** the x86 README documents "initial AVX-512" support; GNU as and llvm-mc (with the matching -mattr) assemble the full EVEX surface.

**Impact:** external .s files using AVX-512 fail to assemble on: (a) ALL zmm (512-bit) register forms — vpxord/vpxorq/vpaddd/vpaddq/vmovdqu64/vaddpd/vmulps/vpermd/vpord/vpcmpeqd/vptestmd/vrndscalepd/vreduceps %zmm... (15/17 probe forms); (b) ALL mask-predication forms — `vpaddd %ymm0, %ymm1, %ymm2{%k1}`, `vaddpd %xmm..., %xmm..., %xmm2{%k1}{z}` (GNU as accepts; ccc rejects). What works: unmasked VL forms (`vpxord %xmm0, %xmm1, %xmm2`).

**Function:** x86 assembler EVEX encoder
**Source location:** src/backend/x86/assembler/encoder/avx.rs (EVEX path: zmm register-class + {%%k} mask operand syntax unhandled)
**Detected by:** round-26 AVX-512 battery (17 forms, double-reference: as ∧ llvm-mc -mattr=+avx512* accept, ccc rejects 15)
**Minimal input:** `.text\nvpaddd %zmm6, %zmm7, %zmm8` → ccc: rejected
**Expected:** assembles (README: "initial AVX-512")
**Actual:** rejected
**Severity:** low (completeness — the internal C pipeline never emits avx512; external-asm compat)
**Contract evidence:** documented — src/backend/x86/README.md:61 "covers ... initial AVX-512" (the shipped subset is narrower than any reading of "initial" that includes 512-bit registers, the feature's defining width)
**Fix:** extend the EVEX encoder to the zmm register class (LL'RC fields, 512-bit vector length) and parse the {%%k1}[{z}] mask suffix on any EVEX form.
**Regression test:** deterministic: assemble one zmm form and one masked VL form.
