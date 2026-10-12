# Issue #1792 — .intel_syntax noprefix accepted but not implemented

**Law:** directive switches assembler dialect (gas emits 89d8)
**Witness:** .intel_syntax noprefix + mov eax,ebx -> ccc: unsupported mov operand combination [Label("eax"),...] (parsed as AT&T)
**Reproduce:**
```bash
gcc -c is.s -o a.o && objdump -s -j .text a.o   # 89d8 83c105
ccc is.s -c -o b.o   # error
```
**Tracker:** https://github.com/fermat-hkrc/claudes-c-compiler/issues/1792
