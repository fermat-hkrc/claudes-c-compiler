# AArch64 prfm with register-index addressing produces wrong encoding — bits 23-21 encode wrong opc field (all indexed forms wrong; bare [x0] correct)
> **Filed as [issue #1063](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1063)**

# Bug: AArch64 prfm register-index addressing wrong encoding
**Law:** PRFM (prefetch memory) with register-index addressing `[x0, x1, lsl #N]` has a specific encoding where bits 23-21 encode the opc (operation type) and options (shift type). Both refs produce exact bytes.
**Impact:** `prfm pldl1keep, [x0, x1, lsl #3]` → ccc `007821f9` vs mc `0078a1f8` — wrong opc bits. Also `prfm pldl3strm, [x0, x1]` → ccc `056821f9` vs mc `0568a1f8` and `prfm #0, [x0, x1, lsl #3]` → same error. The bare form `prfm pldl1keep, [x0]` (no index) IS correct — the bug only fires when a register index is present. This is an ENCODING bug (wrong bytes for valid input), distinct from the VALIDATION bugs in #345-#347.
**Function:** AArch64 assembler prfm encoder (register-offset path)
**Detected by:** round-28 ARM data_processing + load_store battery (33 forms; prfm indexed forms all wrong)
**Minimal input:** `.text` / `prfm pldl1keep, [x0, x1, lsl #3]` → ccc `007821f9`; mc `0078a1f8`
**Severity:** medium (wrong prefetch hint — performance bug, not correctness)
**Contract evidence:** inferred (double-reference byte comparison; encoding vs validation distinction from #345-#347)

## Found by

```bash
cd /home/shuhao/fermat-users/leo/github/claudes-c-compiler
printf '.text
prfm pldl1keep, [x0, x1, lsl #3]
' > /tmp/t.s
./target/release/ccc-arm -mlittle-endian -c -o /tmp/t.o /tmp/t.s
readelf -x .text /tmp/t.o  # 007821f9 (WRONG: should be 0078a1f8)
```
