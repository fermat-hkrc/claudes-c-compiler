# RISC-V atomics (amoadd/amoswap/amoxor/amoor/amoand/amomin/amomax/amominu/amomaxu/lr/sc) encoded without checking the +a extension — 17 forms silently accepted on default rv64i
> **Filed as [issue #833](https://github.com/fermat-hkrc/claudes-c-compiler/issues/833)**

# Bug: RISC-V atomic instructions encoded without checking the +a extension
**Law:** All amo*/lr/sc instructions are the A (atomic) extension; default RISC-V target is rv64i. llvm-mc rejects them without `-mattr=+a`; the assembler must gate on the extension.
**Impact:** ccc-riscv silently encodes all 17 atomic forms on default rv64i — the resulting binaries crash on hardware without the A extension. The encodings are CORRECT (byte-identical to llvm-mc with `-mattr=+a`: `2f25b600` = amoadd.w a0,a1,(a2)). The bug is the missing extension gate (same class as ARM's #832 LSE).
**Function:** RISC-V assembler AMO encoder path
**Detected by:** round-26o 29-form atomic/fence/CSR battery (12 fence/CSR byte-exact ✓; 17 atomics all silently accepted)
**Minimal input:** `.text` / `amoadd.w a0, a1, (a2)` → ccc-riscv assembles `2f25b600`; llvm-mc default rejects, needs `-mattr=+a`
**Expected:** Err without +a; Ok with it (encoding correct)
**Actual:** silently encoded regardless
**Severity:** low (correct bytes, missing gate)
**Regression test:** (to add)

## Found by

```bash
cd /home/shuhao/fermat-users/leo/github/claudes-c-compiler
printf '.text\namoadd.w a0, a1, (a2)\n' > /tmp/t.s
./target/release/ccc-riscv -c -o /tmp/t.o /tmp/t.s  # accepts!
llvm-mc -triple=riscv64 -filetype=obj /tmp/t.s -o /dev/null  # error: instruction requires: a
```

