# AArch64: bti/rmif/rev32-W encoded without arch feature gates or with wrong encoding — 3 classes in data-processing instructions
> **Filed as [issue #838](https://github.com/fermat-hkrc/claudes-c-compiler/issues/838)**

# Bug: AArch64 data-processing instructions encoded without arch gates or with wrong width handling
**Law:** BTI requires the +bti extension; RMIF requires +mte; rev32 on W registers is a documented alias that GNU as and llvm-mc both accept. Each must be gated and encoded per its encoding rules.
**Impact:** Three classes (25/31 clean byte-exact ✓ including rbit/rev/clz/cls/rev16/pac/nop/yield/wfe/wfi/sev/sevl):
1. `bti` (all 5 forms: bare, c, j, jc, and default) — silently encoded without the +bti gate (encodings correct: `5f2403d5` = bti c, `1f2403d5` = bti). Same class as #832/#833.
2. `rmif x0, #3, #nzcv` — silently encoded (`1f2003d5`) without +mte gate. llvm-mc needs `-mattr=+mte`.
3. `rev32 w0, w1` — llvm-mc and GNU as accept this alias (rev32 W→W is valid: `rev32 w0, w1` is an alias form). ccc silently encodes `2008c0da` = rev32 **x**0, w1 — the W destination silently becomes X. Both refs reject rev32 on W (the alias form is rev32 w/rev16 x only).
**Function:** AArch64 assembler BTI/RMIF/REV32 paths
**Detected by:** round-26u 31-form data-processing battery (25 clean; 6 BUG = 5 BTI + 1 RMIF + 1 rev32-W with overlap)
**Minimal input:** `.text` / `bti` → ccc-arm assembles `1f2403d5`; llvm-mc default rejects, needs `-mattr=+bti`
**Expected:** Err without +bti / +mte; Err for rev32 w0,w1 (W-dest alias not defined)
**Actual:** silently encoded (correct bytes for BTI/RMIF; wrong width for rev32-W)
**Severity:** low (BTI/RMIF gate) + medium (rev32-W silent width change)
**Regression test:** (to add)

## Found by

```bash
cd /home/shuhao/fermat-users/leo/github/claudes-c-compiler
printf '.text\nbti\n' > /tmp/t.s
./target/release/ccc-arm -mlittle-endian -c -o /tmp/t.o /tmp/t.s  # accepts
llvm-mc -triple=aarch64 -filetype=obj /tmp/t.s -o /dev/null  # error: instruction requires: bti
```

