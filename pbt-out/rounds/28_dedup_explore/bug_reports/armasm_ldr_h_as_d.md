# AArch64 ldr/str h-register (FP16) encoded as d-register (64-bit) — ldr h0,[x0] produces ldr d0,[x0] bytes
> **Filed as [issue #1042](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1042)**

# Bug: AArch64 ldr/str h-register encoded as d-register (wrong size field)
**Law:** `ldr h0, [x0]` is a 16-bit load with size field 10 (halfword); encoding must use opc=10 for FP16 registers. Both GNU as and llvm-mc encode `0000407d` (7d400000).
**Impact:** ccc encodes `000040fd` (fd400000) = `ldr d0, [x0]` (64-bit). Every h-register load/store loads/stores 8 bytes instead of 2 — data corruption. Also affects: `str h0, [x0, #2]` (ccc `002000fc` = str s0; correct `0004007d` = str h0), `ldp h0, h1, [sp, #8]`, `stp h0, h1, [sp], #8`.
**Function:** AArch64 assembler FP register load/store size field
**Detected by:** round-28 FP16 battery (31 forms; 2 byte-diffs on h-register ldr/str, 23 silently accepted arithmetic forms)
**Minimal input:** `.text` / `ldr h0, [x0]` → ccc `000040fd` (d0 load); refs `0000407d` (h0 load)
**Expected:** 16-bit size field for h registers
**Actual:** 64-bit size field (d register encoding)
**Severity:** high (silent data corruption — loads 8 bytes into a 2-byte slot)

## Found by

```bash
cd /home/shuhao/fermat-users/leo/github/claudes-c-compiler
printf '.text\nldr h0, [x0]\n' > /tmp/t.s
./target/release/ccc-arm -mlittle-endian -c -o /tmp/t.o /tmp/t.s
readelf -x .text /tmp/t.o  # 000040fd = ldr d0,[x0] (WRONG: should be 0000407d = ldr h0,[x0])
```

