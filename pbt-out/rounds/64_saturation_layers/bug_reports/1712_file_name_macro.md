# Issue #1712 — __FILE_NAME__ predefined macro absent

**Law:** GCC12+/clang define the basename macro
**Witness:** #ifdef __FILE_NAME__ false in ccc, true in clang-21
**Reproduce:**
```bash
ccc p.c -o b && ./b   # 0
clang-21 p.c -o a && ./a   # 1
```
**Tracker:** https://github.com/fermat-hkrc/claudes-c-compiler/issues/1712
