# Issue #1768 — diagnostic attributes deprecated/warn_unused_result/format/nonnull produce NO warning

**Law:** each emits its gcc warning class at the use site
**Witness:** all four snippets: gcc warns each, ccc compiles silently
**Reproduce:**
```bash
gcc -c d3.c -o /dev/null  # -Wformat warning
ccc -c d3.c -o /dev/null   # silence
```
**Tracker:** https://github.com/fermat-hkrc/claudes-c-compiler/issues/1768
