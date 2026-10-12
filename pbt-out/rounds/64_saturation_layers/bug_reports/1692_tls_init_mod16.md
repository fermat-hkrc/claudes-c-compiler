# Issue #1692 — TLS initializers silently zero + cross-module TLS writes (.tdata 8-mod-16 law)

**Law:** PT_TLS initial image must land per ELF TLS rules; GNU ld aligns .tdata start to 16
**Witness:** tlsmn.c: __thread int tvar=7; __thread char buf[64]; main(){return tvar;} -> gcc exit 7, ccc exit 0 (4/4). PT_TLS vaddr 0x4031c8 (8 mod 16) fails; ...d0 (0 mod 16) passes; &tvar-FS == -80 identical to gcc => copy lost, not addressing. i686 twin (0x0804b0a8); -static exempt (prints 7).
**Reproduce:**
```bash
printf '#include <stdio.h>\n__thread int tvar=7;\n__thread char buf[64];\nint main(){printf("tvar=%d\\n",tvar);}\n' > t.c
ccc t.c -o c && ./c   # tvar=0
gcc t.c -o g && ./g   # tvar=7
```
**Tracker:** https://github.com/fermat-hkrc/claudes-c-compiler/issues/1692
