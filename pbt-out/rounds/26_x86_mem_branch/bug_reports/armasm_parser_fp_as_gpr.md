# ARM parser accepts FP/SIMD register names as GPRs across the general instruction set (tst/cmp/mvn/mul/sdiv/and) — #814's class at the parser level
> **Filed as [issue #825](https://github.com/fermat-hkrc/claudes-c-compiler/issues/825)**

# Bug: ARM parser accepts FP/SIMD register names as GPRs across the general instruction set
**Law:** General-purpose (integer) instructions take X/W registers only; FP/SIMD names (d/s/q/v/h/b) in GPR operand slots must be rejected.
**Impact:** `tst d0, d0` encodes `1f0000ea` = tst xzr, x0 (d0's numeric suffix parsed as reg 0); `cmp s0, s0` → `1f0000eb` = cmp wzr, w0; `mul s0, s0, s0` → `007c001b` = mul w0, w0, w0; `sdiv h0, h0, h0` → `000cc01a` = sdiv w0, w0, w0; `and x0, q0, x1` → `0000018a` = and x0, x0, x1; `mvn x0, s0` → `e00320aa` = mvn x0, w0. Every invalid FP assembly silently becomes the corresponding GPR instruction. Both as and llvm-mc reject all six.
**Function:** ARM assembler parser (register-name → register-class binding; NOT the encode_* unit level of #814 — this is the .s text path, a separate code location)
**Detected by:** round-26h FP-name-in-GPR-slot battery (12 forms, 7 BUGs, all double-reference confirmed)
**Minimal input:** `.text` / `mul s0, s0, s0` → ccc-arm assembles `007c001b` (mul w0,w0,w0)
**Expected:** Err (register class mismatch)
**Actual:** silently encoded as the numeric-suffix GPR
**Severity:** medium (silent wrong instruction on 6+ mnemonics; #814 class confirmed systemic at the parser level)
**Regression test:** src/backend/arm/mod.rs pbt_asm_deep (to be added: mul s0,s0,s0 must be rejected)

## Found by

```bash
cd /home/shuhao/fermat-users/leo/github/claudes-c-compiler
printf '.text\nmul s0, s0, s0\n' > /tmp/t.s
./target/release/ccc-arm -mlittle-endian -c -o /tmp/t.o /tmp/t.s && readelf -x .text /tmp/t.o  # 007c001b
aarch64-linux-gnu-as -o /dev/null /tmp/t.s  # Error: operand mismatch
```

