# Issue #1693 — -shared .so drops ALL TLS dynamic relocations; dlopen reads 0

**Law:** shared libs keep TLS relocs (TLSGD/GOTTPOFF) for runtime resolution
**Witness:** getlib() via dlopen: gcc 119 (0x77), ccc 0; objdump -R/-r libtlss.so | grep -i tls empty while the .o had GOTTPOFF
**Reproduce:**
```bash
ccc -shared -fPIC tlssolib.c -o libtlss.so && objdump -R libtlss.so | grep -ci tls   # 0
```
**Tracker:** https://github.com/fermat-hkrc/claudes-c-compiler/issues/1693
