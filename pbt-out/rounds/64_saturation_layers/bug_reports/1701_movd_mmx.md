# Issue #1701 — movd/movq GPR<->MMX register forms rejected (0F 6E / 0F 7E)

**Law:** movd %eax,%mm0 assembles to 0f6ec0 (gas)
**Witness:** ccc: "unsupported movd operands"; gas: 0f6ec0/0f7ec0; also breaks inline-asm "y" constraint
**Reproduce:**
```bash
printf '.text\nmovd %%eax, %%mm0\n' > mm.s && ccc mm.s -c -o o   # error
```
**Tracker:** https://github.com/fermat-hkrc/claudes-c-compiler/issues/1701
