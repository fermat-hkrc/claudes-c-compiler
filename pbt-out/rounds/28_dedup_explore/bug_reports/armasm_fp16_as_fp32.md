# AArch64 FP16 arithmetic encoded as FP32 — fadd h0,h1,h2 produces fadd s0,s1,s2 bytes (wrong type field)
> **Filed as [issue #1043](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1043)**

# Bug: AArch64 FP16 arithmetic encoded as FP32 (wrong type field)
**Law:** FP16 (h-register) arithmetic uses type=11 (half) in the FP data-processing encoding; FP32 (s-register) uses type=00. These are different instruction encodings.
**Impact:** ccc encodes `fadd h0, h1, h2` as `2028221e` = fadd **s**0,s1,s2 (type=00); correct is `2028e21e` = fadd **h**0,h1,h2 (type=11). Every FP16 arithmetic instruction silently becomes the FP32 version — completely wrong semantics. Affects 19+ instructions: fadd, fsub, fmul, fdiv, fneg, fabs, fsqrt, fmax, fmin, fmov h,w / w,h, fcvtzs/fcvtns/fcvtps/fcvtas/fcvtms h,w, scvtf/ucvtf h,w. Also fcmp/fcmpe silently accepted.
**Function:** AArch64 assembler FP arithmetic type field selection
**Detected by:** round-28 FP16 battery (31 forms; 19 BUG + 2 byte-diff on ldr/str filed separately)
**Minimal input:** `.text` / `fadd h0, h1, h2` → ccc `2028221e` (fadd s0,s1,s2); llvm-mc(+fullfp16) `2028e21e` (fadd h0,h1,h2)
**Expected:** type=11 for h registers
**Actual:** type=00 (s register encoding)
**Severity:** high (silent wrong instruction — half-precision computation silently becomes single-precision)

## Found by

```bash
cd /home/shuhao/fermat-users/leo/github/claudes-c-compiler
printf '.text\nfadd h0, h1, h2\n' > /tmp/t.s
./target/release/ccc-arm -mlittle-endian -c -o /tmp/t.o /tmp/t.s
readelf -x .text /tmp/t.o  # 2028221e = fadd s0,s1,s2 (WRONG: should be 2028e21e = fadd h0,h1,h2)
```

