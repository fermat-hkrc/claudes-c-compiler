# RISC-V: FP-bearing struct returns ({float}/{double}/{float,float}) delivered in integer a0 — RV64gc psABI requires fa0(/fa1); {float,float} also DROPS the second member
> **Filed as [issue #1119](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1119)**

# Bug: RISC-V: FP-bearing struct returns ({float}/{double}/{float,float}) delivered in integer a0 — RV64gc psABI requires fa0(/fa1); {float,float} also DROPS the second member
**Law:** ∀ struct whose members are all floating-point (RV64gc psABI §18.3): returns are delivered one FP member per register in fa0, fa1. FALSIFIED — ccc-riscv moves only the first member's BITS into integer a0; {float,float} loses member b entirely.
**Impact:** Any FP-struct return crossing compilation units breaks:
```
mkf (returns {float})   : ld t0,-24(s0); mv s1,t0; mv a0,t0; ret   ← a0 integer, fa0 never loaded
mkd (returns {double})  : mv a0, t0                                ← same
mk2 (returns {f,f})     : sw t0,-24/-20(s0); ld t0,-24(s0); mv a0,t0 ← only a's bits; b stays in memory
```
Args are CORRECT (take prologue reads `fsw fa0/-24; fsw fa1/-20` — verified), so pure-ccc programs self-consistent but the return convention deviates; clang/qemu interop witness (freestanding ecall exit-code, /tmp/rv_m.c) shows all three return shapes BAD (code stayed at base).
**Function:** RISC-V FP-struct return path (call_abi classification → riscv codegen epilogue); ARG side already correct.
**Detected by:** Differential — round-38 high-value battery RISC-V extension: disassembly + clang-21 --target=riscv64-linux-gnu -march=rv64gc interop witness under qemu-riscv64-static
**Minimal input:**
```c
struct SF { float v; };
struct SF mkf(void) { struct SF s = { 2.5f }; return s; }
```
**Expected:** `fmv.w.x fa0, t0` (or flw fa0) at return; {f,f} → fa0 AND fa1.
**Actual:** `mv a0, t0` — integer register only; fa0 untouched; {f,f} drops b.
**Fix sketch:** in the RISC-V return classification, mark all-FP aggregates as FP-class (mirroring the already-correct ARGUMENT path that uses fa0/fa1) and emit per-member fmv.w.x/fmv.d.x into fa0/fa1.

## Found by

```bash
cd /home/shuhao/fermat-users/leo/github/claudes-c-compiler
printf 'struct SF { float v; };\nstruct SF mkf(void){ struct SF s={2.5f}; return s; }' > /tmp/r.c
./target/debug/ccc-riscv -S /tmp/r.c -o /tmp/r.s && grep -B1 ret /tmp/r.s   # mv a0, t0 — no fa0
```
