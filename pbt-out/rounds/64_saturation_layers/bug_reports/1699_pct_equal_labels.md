# Issue #1699 — inline-asm %= unique-id and Nf/Nb local labels not expanded

**Law:** %= expands to a per-asm unique id; 1f/1b local labels documented with it
**Witness:** jmp 2%=f; 1%=: ... -> ccc: error: undefined symbols: 2%=f; gcc runs and prints 2
**Reproduce:**
```bash
ccc uq.c -o uqx   # error
gcc uq.c -o uqg && ./uqg   # 2
```
**Tracker:** https://github.com/fermat-hkrc/claudes-c-compiler/issues/1699
