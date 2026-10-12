# Issue #1698 — asm goto fallthrough jumps to the LAST label, skipping the next statement

**Law:** non-taken asm goto falls through to the statement after the asm
**Witness:** probe(5): ccc returns -1 (r=100 run never executes), gcc returns 100; je-taken case correct
**Reproduce:**
```bash
ccc ag.c -o agx && ./agx   # -1 200
gcc ag.c -o agg && ./agg   # 100 200
```
**Tracker:** https://github.com/fermat-hkrc/claudes-c-compiler/issues/1698
