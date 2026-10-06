# Bug: x86 assembler silently truncates out-of-range immediates (addq $4294967296, %rax assembles as add $0x0)
> **Filed as [issue #776](https://github.com/fermat-hkrc/claudes-c-compiler/issues/776)**

**Law:** `addq $imm, %r64` accepts imm32 (sign-extended); an immediate outside [-2^31, 2^31-1] has no encoding and must be rejected (or require movabsq).

**Impact:** the WORST of the round-15 classes — the assembled instruction computes something entirely different from the source text (`add $4294967296` becomes `add $0`). GNU as and llvm-mc both reject; ccc accepts and truncates to the low 32 bits.

**Function:** x86 assembler immediate encoding
**Source location:** src/backend/x86/assembler (imm64 parsed, cast to i32 without range check)
**Detected by:** round-15 hand probe + PBT mutation imm_overflow; double-reference confirmed
**Minimal input:** `.text\naddq $4294967296, %rax` → encodes `48 05 00 00 00 00 (add $0x0,%rax)`
**Expected:** error (imm out of imm32 range)
**Actual:** silently truncated to 0
**Severity:** medium (silent wrong constant)
**Contract evidence:** inferred (double-reference rejection)
**Fix:** range-check immediates against the opcode's imm width; error (or suggest movabsq) on overflow.
**Regression test:** test_x86asm_regression_imm32_overflow_rejected
