# Issue #1700 — inline-asm "x" (SSE) constraint input never materialized into xmm

**Law:** the compiler must load the input into the allocated xmm before the template
**Witness:** movss %1,%0 :"=x"(g):"x"(f) with f=1.5 prints 0.00 (ccc) vs 1.50 (gcc); disasm: xorpd %xmm1,%xmm1; movss %xmm1,%xmm0 — constant never loaded
**Reproduce:**
```bash
ccc xm.c -o xmx && ./xmx   # 0.00
gcc xm.c -o xmg && ./xmg   # 1.50
```
**Tracker:** https://github.com/fermat-hkrc/claudes-c-compiler/issues/1700
