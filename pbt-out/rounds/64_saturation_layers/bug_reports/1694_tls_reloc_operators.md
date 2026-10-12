# Issue #1694 — @tlsgd/@tlsld emit R_X86_64_PC32 (wrong reloc type); @indntpoff over-accepted

**Law:** x@tlsgd -> R_X86_64_TLSGD, x@tlsld -> R_X86_64_TLSLD; indntpoff is i386-only
**Witness:** objdump -r: ccc PC32 vs gas TLSGD/TLSLD; movl x@indntpoff,%eax accepted on x86-64 (gas rejects)
**Reproduce:**
```bash
printf '.text\nmovq x@tlsgd(%%rip), %%rax\n' > tg.s
ccc tg.s -c -o b.o && objdump -r b.o   # R_X86_64_PC32
```
**Tracker:** https://github.com/fermat-hkrc/claudes-c-compiler/issues/1694
