# Issue #1788 — u'x' literal keeps type int — sizeof(u'x')==4 (gcc/clang: 2)

**Law:** u'' has type char16_t (2 bytes)
**Witness:** sizeof table: ccc 4/4, gcc+clang 4/2; value/encoding correct (u'界'==30028 everywhere)
**Reproduce:**
```bash
ccc wl2.c -o a && ./a   # ... 4 4
gcc wl2.c -o b && ./b   # ... 4 2
```
**Tracker:** https://github.com/fermat-hkrc/claudes-c-compiler/issues/1788
