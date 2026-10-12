# Issue #1789 — bit-field width exceeding its type accepted (int a:33, char a:9)

**Law:** C11 6.7.2.1p4: width must not exceed type width (gcc errors both)
**Witness:** struct S{int a:33;} compiles in ccc; gcc: error: width of a exceeds its type
**Reproduce:**
```bash
gcc -c t.c -> error; ccc t.c -o t -> builds
```
**Tracker:** https://github.com/fermat-hkrc/claudes-c-compiler/issues/1789
