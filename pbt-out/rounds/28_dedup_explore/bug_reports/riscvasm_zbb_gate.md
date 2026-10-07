# RISC-V assembler encodes Zbb bit-manipulation instructions (andn/orn/xnor/clz/ctz/pcnt/rol/ror/rev8/min/max/minu/maxu/sext.b/sext.h/zext.h) without the +zbb extension gate — 15 forms
> **Filed as [issue #1041](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1041)**

# Bug: RISC-V Zbb bit-manipulation instructions encoded without +zbb extension gate
**Law:** These 15 instructions are the Zbb (basic bit-manipulation) extension; default RISC-V target is rv64gc (no Zbb). llvm-mc rejects without `-mattr=+zbb`.
**Impact:** ccc-riscv silently encodes all 15 Zbb forms regardless of target — binaries crash on hardware without Zbb. Affected: andn, orn, xnor, clz, ctz, pcnt, rol, ror, rev8, min, max, minu, maxu, sext.b, sext.h, zext.h. The encodings need verification (llvm-mc with +zbb didn't produce output for comparison — need `-march=rv64gc_zbb`). Same class as #832 (ARM LSE) / #833 (RISC-V atomics).
**Function:** RISC-V assembler Zbb encoder path
**Detected by:** round-28 RISC-V deep battery (31 forms; 15 Zbb BUGs, 15 other forms clean including CSR access)
**Minimal input:** `.text` / `andn a0, a1, a2` → ccc-riscv assembles `40c5f533`; llvm-mc rejects without +zbb
**Severity:** low (correct-encoding verification needed; gate missing)
**Contract evidence:** inferred (llvm-mc requires +zbb)

## Found by

```bash
cd /home/shuhao/fermat-users/leo/github/claudes-c-compiler
printf '.text\nandn a0, a1, a2\n' > /tmp/t.s
./target/release/ccc-riscv -c -o /tmp/t.o /tmp/t.s  # accepts!
llvm-mc -triple=riscv64 -filetype=obj /tmp/t.s -o /dev/null  # error: instruction requires: zbb
```

