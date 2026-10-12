# Issue #1767 — tls_model attribute accepted but ignored (local-exec emits initial-exec relocs)

**Law:** tls_model("local-exec") => TPOFF32 at -fPIC (gcc)
**Witness:** objdump -r: ccc GOTTPOFF for BOTH ie and le vars; gcc TPOFF32 for le
**Reproduce:**
```bash
ccc -fPIC -c d5.c -o a.o && objdump -r a.o   # both GOTTPOFF
```
**Tracker:** https://github.com/fermat-hkrc/claudes-c-compiler/issues/1767
