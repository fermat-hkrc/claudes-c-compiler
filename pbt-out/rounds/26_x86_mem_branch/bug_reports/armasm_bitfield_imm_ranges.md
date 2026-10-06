# AArch64 bitfield (bfm/extr) immediates silently accept out-of-range values — immr/imms range checks missing (W: 0..31, X: 0..63)
> **Filed as [issue #831](https://github.com/fermat-hkrc/claudes-c-compiler/issues/831)**

# Bug: AArch64 bitfield (bfm/extr) immediates silently accept out-of-range values
**Law:** bfm's immr and imms are 6-bit for X (0..63) and 5-bit for W (0..31); extr's shift is 6-bit for X and 5-bit for W. Values outside are unencodable and both refs reject.
**Impact:** `bfm w0, w1, #0, #32` (W-form imms=32 > 31 max), `bfm x0, x1, #0, #64` (imms=64 > 63), `bfm x0, x1, #64, #0` (immr=64), `bfm x0, x1, #-1, #0` (negative), `extr w0, w1, w2, #32` (W-shift=32), `extr x0, x1, x2, #64` — all 6 silently accepted; every in-range form (14) byte-exact to llvm-mc. The out-of-range values wrap into the immediate fields, producing wrong bit-field extracts.
**Function:** AArch64 assembler bitfield instruction encoding
**Detected by:** round-26m 20-form bitfield battery (all 14 valid byte-exact; 6 boundary BUGs)
**Minimal input:** `.text` / `bfm w0, w1, #0, #32` → ccc-arm assembles; both refs reject
**Expected:** Err (imms > 31 in W form)
**Actual:** silently encoded (imms masks to 0)
**Severity:** medium (silent wrong bitfield — #830 class on the bitfield path)
**Regression test:** (to add)

## Found by

```bash
cd /home/shuhao/fermat-users/leo/github/claudes-c-compiler
printf '.text\nbfm w0, w1, #0, #32\n' > /tmp/t.s
./target/release/ccc-arm -mlittle-endian -c -o /tmp/t.o /tmp/t.s  # accepts!
aarch64-linux-gnu-as /tmp/t.s -o /dev/null  # Error
```

