# x86-64 assembler lacks the ENTIRE FMA instruction set (vfmadd/vfmsub/vfnmadd/vfnmsub × 132/213/231 × pd/ps/sd/ss) — 31 forms all rejected
> **Filed as [issue #1053](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1053)**

# Bug: x86-64 assembler lacks the entire FMA instruction set
**Law:** FMA (Fused Multiply-Add) is a fundamental x86 extension (FMA3/FMA4); GNU as and llvm-mc assemble all forms. The 132/213/231 suffixes encode different operand orderings of a*b+c.
**Impact:** NO FMA instruction assembles: vfmadd, vfmsub, vfnmadd, vfnmsub in all precision variants (pd/ps for packed double/single, sd/ss for scalar) and all operand orders (132/213/231). Also memory-source forms. Any external .s or inline asm using FMA fails. ccc's internal C pipeline also never emits FMA (verified: no vfmadd in any -O2 output).
**Function:** x86 assembler mnemonic table (FMA entries absent)
**Detected by:** round-28 FMA battery (31 forms; ALL 31 rejected by ccc, ALL 31 accepted by as)
**Minimal input:**  /  → ccc rejects; as accepts
**Severity:** medium (major functionality gap for FP-heavy code; the compiler also can't use FMA even when the CPU supports it)
**Contract evidence:** inferred (double-reference acceptance; FMA is in x86/README's claimed coverage: "SSE through SSE4.1, AVX/AVX2" — FMA is adjacent to AVX)
**Regression test:** (deterministic: any FMA form must assemble)

## Found by

```bash
cd /home/shuhao/fermat-users/leo/github/claudes-c-compiler
printf '.text
vfmadd132pd %%xmm0, %%xmm1, %%xmm2
' > /tmp/t.s
./target/release/ccc -c -o /tmp/t.o /tmp/t.s  # error
as /tmp/t.s -o /dev/null  # OK
```
