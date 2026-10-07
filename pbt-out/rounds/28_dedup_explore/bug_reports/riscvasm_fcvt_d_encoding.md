# RISC-V fcvt.d.s and fcvt.d.w encode with wrong rs2/rm field bits — 2 conversion instructions produce wrong encodings
> **Filed as [issue #1068](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1068)**

# Bug: RISC-V fcvt.d.s/fcvt.d.w wrong encoding (rs2 field bits)
**Law:** RISC-V FP conversion instructions encode the source format in the rs2 field; both refs produce exact bytes with `-mattr=+f,+d`.
**Impact:** `fcvt.d.s fa0, fa1` → ccc `53f50542` vs mc `53850542` (bits 19-16 differ: ccc 1111 vs mc 1000); `fcvt.d.w fa0, a0` → ccc `537505d2` vs mc `530505d2` (same bit pattern error). These produce DIFFERENT INSTRUCTIONS than intended — the conversion silently targets the wrong source format. 34/36 other RISC-V FP forms byte-exact ✓.
**Function:** RISC-V assembler FP conversion encoder (rs2 field)
**Detected by:** round-28 RISC-V FP battery with +f,+d (36 forms; 34 clean, 2 DIFF)
**Minimal input:** `.text` / `fcvt.d.s fa0, fa1` → ccc `53f50542`; mc(+f,+d) `53850542`
**Severity:** medium (silent wrong conversion — fp32→fp64 becomes something else)
**Contract evidence:** inferred (byte comparison with correct extension flags)

## Found by

```bash
cd /home/shuhao/fermat-users/leo/github/claudes-c-compiler
printf '.text
fcvt.d.s fa0, fa1
' > /tmp/t.s
./target/release/ccc-riscv -c -o /tmp/t.o /tmp/t.s
readelf -x .text /tmp/t.o  # 53f50542 (WRONG: should be 53850542)
llvm-mc -triple=riscv64 -mattr=+f,+d -filetype=obj /tmp/t.s -o /dev/null  # correct
```
