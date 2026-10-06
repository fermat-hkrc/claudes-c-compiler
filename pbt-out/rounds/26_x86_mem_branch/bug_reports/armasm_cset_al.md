# AArch64 cset/cneg silently accept the AL (always) condition — llvm-mc rejects (should be written as mov/neg)
> **Filed as [issue #829](https://github.com/fermat-hkrc/claudes-c-compiler/issues/829)**

# Bug: AArch64 cset/cneg silently accept the AL (always) condition
**Law:** cset/cneg encode as csinc/csneg with the INVERSE condition; the AL (always, cond=1110) condition makes the conditional-select degenerate (always-taken = just a mov/neg) and is rejected by both GNU as and llvm-mc.
**Impact:** `cset x0, al` encodes `e0f79f9a` (csinc x0, xzr, xzr, nv — the AL encoding maps to the NEVER slot in the csinc alias table, producing a constant-0 result instead of the "always set to 1" the source text implies). `cneg x0, x1, al` → `20f481da` (csneg x0, x1, x1, nv — produces x1 not -x1). Silent wrong semantics.
**Function:** AArch64 assembler condition-code validation in cset/cneg paths
**Detected by:** round-26k 85-form condition-code sweep (all 14 codes × 5 instruction families; 83 clean, AL in cset/cneg silently accepted)
**Minimal input:** `.text` / `cset x0, al` → ccc-arm assembles `e0f79f9a`; llvm-mc rejects
**Expected:** Err (cond=al in cset/cneg alias)
**Actual:** silently encoded into the nv slot — wrong runtime value
**Severity:** medium (silent wrong semantics — the encoded instruction produces the INVERSE of what the text implies)
**Regression test:** (to add)

## Found by

```bash
cd /home/shuhao/fermat-users/leo/github/claudes-c-compiler
printf '.text\ncset x0, al\n' > /tmp/t.s
./target/release/ccc-arm -mlittle-endian -c -o /tmp/t.o /tmp/t.s && readelf -x .text /tmp/t.o  # e0f79f9a
llvm-mc -triple=aarch64 -filetype=obj /tmp/t.s -o /dev/null  # error: condition 'al' is not valid
```

