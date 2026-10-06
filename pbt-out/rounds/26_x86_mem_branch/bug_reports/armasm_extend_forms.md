# AArch64 register-offset loads/stores: bare sxtw/uxtw extension silently accepted (wrong encoding); SIMD ldr b0 [x1,x2] byte mismatch
> **Filed as [issue #836](https://github.com/fermat-hkrc/claudes-c-compiler/issues/836)**

# Bug: AArch64 register-offset load/store extension forms have wrong encodings
**Law:** In register-offset addressing `[xN, xM, ext]`, the extension specifier determines the extend-and-shift option bits. A bare `sxtw` or `uxtw` (without an explicit lsl shift) has a specific option encoding; both refs produce exact bytes. SIMD register loads (`ldr b0`) have their own option encoding.
**Impact:** Two classes found:
1. **Silent accept (7 forms):** `ldr x0, [x1, x2, sxtw]`, `ldr x0, [x1, x2, uxtw]`, `ldr x0, [x1, x2, sxtw #3]`, `ldrb w0, [x1, x2, sxtw]`, `ldrh w0, [x1, x2, sxtw #1]`, `ldrsw x0, [x1, x2, sxtw #2]`, `str x0, [x1, x2, sxtw #3]` — both refs reject; ccc accepts (encodes as `ldr x0,[x1,x2]` with option bits = 0 = lsl — silently dropping the extension).
2. **Byte mismatch (1 form):** `ldr b0, [x1, x2, lsl #0]` — ccc `206862fc`, llvm-mc `2078623c` — wrong option field (0x6 vs 0x7 = b-register load encoding).
**Function:** AArch64 assembler register-offset addressing option encoding
**Detected by:** round-26s 20-form extend battery (13 clean byte-exact ✓; 7 BUG + 1 byte-diff)
**Minimal input:** `.text` / `ldr x0, [x1, x2, sxtw]` → ccc-arm assembles `206862fc` (option=lsl, extension dropped); both refs reject
**Expected:** Err for bare sxtw/uxtw; byte `2078623c` for ldr b0
**Actual:** silently encoded with extension dropped / wrong option field
**Severity:** medium (silent wrong addressing mode — the #776 class)
**Regression test:** (to add)

## Found by

```bash
cd /home/shuhao/fermat-users/leo/github/claudes-c-compiler
printf '.text\nldr x0, [x1, x2, sxtw]\n' > /tmp/t.s
./target/release/ccc-arm -mlittle-endian -c -o /tmp/t.o /tmp/t.s  # accepts → 206862fc (option=lsl)
aarch64-linux-gnu-as /tmp/t.s -o /dev/null  # Error: operand 3 must be a bare register name or an indexed register
```

