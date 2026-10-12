# Issue #1697 — -Wl,-z,now accepted but no DT_FLAGS/FLAGS_1 BIND_NOW emitted

**Law:** -z now => DT_FLAGS BIND_NOW + DT_FLAGS_1 NOW
**Witness:** readelf -dW: ccc empty of FLAGS; gcc shows (FLAGS) BIND_NOW + (FLAGS_1) NOW PIE
**Reproduce:**
```bash
ccc -Wl,-z,now x.c -o x && readelf -dW x | grep FLAGS   # empty
```
**Tracker:** https://github.com/fermat-hkrc/claudes-c-compiler/issues/1697
