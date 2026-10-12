# Issue #1791 — default-on gcc warnings absent: implicit-int, unknown-escape (ccc silent)

**Law:** both are default-on in gcc
**Witness:** f(x){return x;} -> gcc -Wimplicit-int, ccc silent; "a\qb" -> gcc unknown-escape, ccc silent. (EDIT: missing-return claim retracted — ccc DOES warn there)
**Reproduce:**
```bash
gcc -c w1.c -o /dev/null  # warns
ccc -c w1.c -o /dev/null   # silent
```
**Tracker:** https://github.com/fermat-hkrc/claudes-c-compiler/issues/1791
