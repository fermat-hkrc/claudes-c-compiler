# Issue #1790 — C23 fixed underlying enum type enum F : type rejected

**Law:** C23 enum trailing type (clang: sizeof==1)
**Witness:** enum F : unsigned char {A=200,B=255} -> ccc parse error at :
**Reproduce:**
```bash
clang-21 ef.c -o a && ./a   # 1 255
ccc ef.c -o b   # parse error
```
**Tracker:** https://github.com/fermat-hkrc/claudes-c-compiler/issues/1790
