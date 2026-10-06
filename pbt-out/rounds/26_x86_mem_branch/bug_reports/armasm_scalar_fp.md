# AArch64 scalar FP: X/W register names silently accepted as FP operands (fadd/fsqrt/fmax); fcmp #0.0 encodes NOP; fnmul/fcmpe/fcsel missing
> **Filed as [issue #839](https://github.com/fermat-hkrc/claudes-c-compiler/issues/839)**

# Bug: AArch64 scalar FP instruction validation and encoding defects
**Law:** FP instructions require FP register names (s/d/h); `#0.0` comparison has a dedicated encoding; fnmul/fcmpe/fcsel are standard mnemonics.
**Impact:** Four classes (21/32 byte-exact ✓ including all valid s/d arithmetic):
1. **Silent wrong-instruction (5 BUGs):** `fadd x0, x1, x2` → `2028221e` = fadd **s**0,s1,s2; `fmul w0,w1,w2` → s-form; `fdiv x0` → s; `fsqrt x0` → `20c0211e` = fsqrt s0,s1; `fmax x0` → s-form. Every GPR name in an FP slot silently encodes as the s-register with the same number.
2. **Wrong encoding (2 BYTE-DIFFs):** `fcmp d0, #0.0` → ccc encodes `1f2003d5` = **NOP** (not the fcmp-vs-zero `0820601e`); `fcmp s1, #0.0` → also NOP (`2820201e` expected). The comparison against literal zero is silently skipped.
3. **Missing mnemonics (4 GAPs):** `fnmul d0,d1,d2` / `fnmul s0,s1,s2`; `fcmpe d0,d1` / `fcmpe s0,s1` (raise-exception compare); `fcsel d0,d1,d2,eq` / `fcsel s0,s1,s2,mi` (FP conditional select) — all standard, both refs accept.
**Function:** AArch64 assembler scalar FP encoder (register-class check + #0.0 comparison path + mnemonic table)
**Detected by:** round-26v 32-form scalar FP battery (21 clean byte-exact; 5+2+4 = 11 defects)
**Minimal input:** `.text` / `fcmp d0, #0.0` → ccc-arm encodes a NOP; the comparison never happens
**Expected:** Err for GPR names; `0820601e` for fcmp d0,#0.0; Ok for fnmul/fcmpe/fcsel
**Actual:** silent s-register coercion / NOP encoding / rejection of valid mnemonics
**Severity:** high (fcmp #0.0 → NOP is a silent semantic skip; GPR-as-FP is silent wrong instruction)
**Regression test:** (to add)

## Found by

```bash
cd /home/shuhao/fermat-users/leo/github/claudes-c-compiler
printf '.text\nfcmp d0, #0.0\n' > /tmp/t.s
./target/release/ccc-arm -mlittle-endian -c -o /tmp/t.o /tmp/t.s
readelf -x .text /tmp/t.o  # 1f2003d5 = NOP (should be 0820601e = fcmp d0, #0)
llvm-mc -triple=aarch64 -filetype=obj /tmp/t.s -o /dev/null  # correct encoding
```

