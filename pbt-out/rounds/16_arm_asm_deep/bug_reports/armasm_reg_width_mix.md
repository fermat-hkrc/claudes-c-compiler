# Bug: AArch64 assembler silently accepts W/X register width mixing (add w0, x1, x2; W register as memory base)
> **Filed as [issue #791](https://github.com/fermat-hkrc/claudes-c-compiler/issues/791)**

**Law:** an AArch64 instruction form mixing 32-bit (W) and 64-bit (X) registers in the same operand role is invalid and must be rejected. Also: a W register may not be used as a memory-address base.

**Impact:** hand-written/external .s files with width typos assemble silently to a different instruction than written (the W/X bit is taken from one operand, reinterpreting the others). Both GNU as (aarch64-linux-gnu-as) and llvm-mc reject every case; ccc accepts.

**Function:** AArch64 assembler operand validation (non-NEON paths)
**Source location:** src/backend/arm/assembler (no width-uniformity check between operands / base-register width check)
**Detected by:** round-16 deep sweep, hand probes + P2 mutations; double-reference confirmed
**Minimal input:** `.text\nadd w0, x1, x2` (also `add x0, w1, #5`; `str w1, [w28, #8]` — W base)
**Expected:** error
**Actual:** silently assembled
**Severity:** medium
**Contract evidence:** inferred (double-reference: aarch64-linux-gnu-as AND llvm-mc both reject)
**Fix:** validate register width uniformity across operands and require X (or SP) bases for memory addressing.
**Regression test:** src/backend/arm/mod.rs pbt_asm_deep (red witness to be added)
