# AArch64 NEON fneg v.4s encodes with bit 23 clear (wrong S field) — produces wrong instruction
> **Filed as [issue #1062](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1062)**

# Bug: AArch64 NEON fneg vector encodes with bit 23 clear
**Law:** FNEG (vector) requires the S field (bit 23) = 1 for the FP two-register-miscellaneous encoding. Both refs produce bit 23 = 1.
**Impact:** `fneg v0.4s, v1.4s` → ccc `20f8206e` (bit 23 = 0, WRONG) vs mc `20f8a06e` (bit 23 = 1, CORRECT). The wrong encoding produces a different instruction. 49/50 NEON forms byte-exact (all other NEON arithmetic, FP, load/store, bitwise clean ✓).
**Function:** AArch64 assembler NEON fneg encoder
**Detected by:** round-28 NEON full battery (50 forms; 49 OK, 1 byte-diff)
**Minimal input:** `.text` / `fneg v0.4s, v1.4s` → ccc `20f8206e`; mc `20f8a06e`
**Severity:** medium (silent wrong instruction)
**Contract evidence:** inferred (double-reference byte comparison)

## Found by

```bash
cd /home/shuhao/fermat-users/leo/github/claudes-c-compiler
printf '.text
fneg v0.4s, v1.4s
' > /tmp/t.s
./target/release/ccc-arm -mlittle-endian -c -o /tmp/t.o /tmp/t.s
readelf -x .text /tmp/t.o  # 20f8206e (bit 23=0, WRONG)
llvm-mc -triple=aarch64 -filetype=obj /tmp/t.s -o /dev/null  # 20f8a06e (bit 23=1)
```
