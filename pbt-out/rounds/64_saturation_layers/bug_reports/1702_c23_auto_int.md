# Issue #1702 — C23 auto silently parsed as the storage class — auto v=1.5 becomes int

**Law:** C23 auto infers the initializer type (clang: sizeof==8, double)
**Witness:** sizeof(v)=4, (int)v==1, zero diagnostics (ccc); clang: 8 and 1. gcc-9 predates but warns
**Reproduce:**
```bash
ccc au2.c -o b && ./b   # 4 1
clang-21 -std=c23 au2.c -o a && ./a   # 8 1
```
**Tracker:** https://github.com/fermat-hkrc/claudes-c-compiler/issues/1702
