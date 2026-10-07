# AArch64 assembler lacks fmaxv/fminv/fmaxnmv/fminnmv (FP horizontal max/min) and smov (signed move vector to GPR) — 5 forms
> **Filed as [issue #1056](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1056)**

# Bug: AArch64 assembler lacks FP horizontal max/min and smov
**Law:** fmaxv/fminv (find max/min across all lanes) and fmaxnmv/fminnmv (NaN-suppressing variants) are standard NEON FP instructions. smov extracts a signed element from a vector to a GPR.
**Impact:** 5 GAPs: fmaxv s0,v1.4s, fminv s0,v1.4s, fmaxnmv s0,v1.4s, fminnmv s0,v1.4s, smov w0,v1.b[0]. All are fundamental for vectorized reduction code. 31/36 other NEON dataproc forms work (addp/smaxv/sminv/umaxv/uminv/addv/saddlv/uaddlv/dup/ins/umov/mov/rev64/rev32/ext/zip1/zip2/uzp1/uzp2/trn1/trn2 all correct except saddlp/uaddlp/sadalp/uadalp/addv encoding bugs already on #642/#601).
**Function:** AArch64 assembler NEON reduction path
**Detected by:** round-28 NEON dataproc battery (36 forms; 31 OK, 5 GAPs, 6 byte-diffs on #642/#601)
**Minimal input:** `.text` / `fmaxv s0, v1.4s` → ccc rejects; both refs accept
**Severity:** medium (missing reductions block SIMD optimization)
**Contract evidence:** inferred (double-reference acceptance)

## Found by

```bash
cd /home/shuhao/fermat-users/leo/github/claudes-c-compiler
printf '.text
fmaxv s0, v1.4s
' > /tmp/t.s
./target/release/ccc-arm -mlittle-endian -c -o /tmp/t.o /tmp/t.s  # error
llvm-mc -triple=aarch64 -filetype=obj /tmp/t.s -o /dev/null  # OK
```
