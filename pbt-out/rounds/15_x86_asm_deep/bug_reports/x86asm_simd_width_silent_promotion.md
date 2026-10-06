# Bug: x86 assembler silently promotes SIMD register width mismatches (vaddps %ymm0, %ymm1, %xmm2 assembles as ymm2)
> **Filed as [issue #775](https://github.com/fermat-hkrc/claudes-c-compiler/issues/775)**

**Law:** AVX 3-operand forms require uniform vector register width; a `%xmm` operand in a `%ymm` form has no encoding and must be rejected.

**Impact:** external .s files silently assemble to different instructions than written (the xmm operand is encoded as if it were ymm). GNU as and llvm-mc both reject; ccc accepts.

**Function:** x86 assembler AVX operand validation
**Source location:** src/backend/x86/assembler (AVX 3-op path — width uniformity unvalidated)
**Detected by:** round-15 PBT (mutation width_swap on vector templates); double-reference confirmed
**Minimal input:** `.text\nvaddps %ymm0, %ymm1, %xmm2` → encodes `c5 f4 58 d0 (vaddps %ymm0,%ymm1,%ymm2)`
**Expected:** error
**Actual:** silently encoded with xmm2 promoted to ymm2
**Severity:** medium
**Contract evidence:** inferred (double-reference: both GNU as and llvm-mc reject)
**Fix:** check xmm/ymm uniformity across the operand list for each vector mnemonic form.
**Regression test:** test_x86asm_regression_simd_width_rejected
