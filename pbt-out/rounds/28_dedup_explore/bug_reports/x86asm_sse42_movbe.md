# x86-64 assembler lacks SSE4.2 string comparison (pcmpestri/pcmpestrm/pcmpistri/pcmpistrm) and MOVBE — 6 instructions missing
> **Filed as [issue #1051](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1051)**

# Bug: x86-64 assembler lacks SSE4.2 string comparison and MOVBE
**Law:** pcmpestri/pcmpestrm/pcmpistri/pcmpistrm are SSE4.2 string comparison instructions; MOVBE is a byte-swap load/store. GNU as and llvm-mc assemble all six.
**Impact:** External .s files using these fail. 6 GAPs: pcmpestri, pcmpestrm, pcmpistri, pcmpistrm (4 SSE4.2 string cmp), movbeq load, movbeq store. 14/20 other forms work (pcmpgtq, crc32 all widths, aesenc/dec/enclast/eclast/keygenassist/imc, pclmulqdq both imm forms, popcnt).
**Function:** x86 assembler mnemonic table
**Detected by:** round-28 SSE4.2/AES battery (20 forms; 14 OK, 6 GAPs)
**Minimal input:**  /  → ccc rejects; as accepts
**Severity:** low (completeness)
**Contract evidence:** inferred (double-reference acceptance)

## Found by

```bash
cd /home/shuhao/fermat-users/leo/github/claudes-c-compiler
printf '.text
pcmpestri /bin/bash, %%xmm0, %%xmm1
' > /tmp/t.s
./target/release/ccc -c -o /tmp/t.o /tmp/t.s  # error
as /tmp/t.s -o /dev/null  # OK
```
