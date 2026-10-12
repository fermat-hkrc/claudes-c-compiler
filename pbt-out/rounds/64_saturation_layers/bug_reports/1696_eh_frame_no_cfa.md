# Issue #1696 — Compiler .eh_frame FDEs contain no CFA rules — backtrace() returns 1 frame

**Law:** every compiler FDE must encode CFA rules matching the prologue
**Witness:** 4-deep call: backtrace() n=1 (ccc) vs 6 (gcc); objdump --dwarf=frames shows FDE pc=main = DW_CFA_nop x3
**Reproduce:**
```bash
ccc bt.c -o bt && ./bt   # n=1
gcc bt.c -o btg && ./btg   # n=6
```
**Tracker:** https://github.com/fermat-hkrc/claudes-c-compiler/issues/1696
