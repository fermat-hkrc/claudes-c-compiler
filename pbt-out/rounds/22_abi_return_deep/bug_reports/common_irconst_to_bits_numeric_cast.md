# Bug: irconst_to_bits does a NUMERIC cast, not a bitcast — signbit(-0.0) folds to 0 on constants
> **Filed as [issue #806](https://github.com/fermat-hkrc/claudes-c-compiler/issues/806)**

**Law:** the function's own documentation says it converts an IrConst "to raw u64 bits for bit-level operations" (src/common/README.md). A float constant's bit-level value is its IEEE 754 pattern: bits(-0.0) = 0x8000000000000000.

**Impact:** every compile-time-folded float bit operation is silently wrong. Visible symptom: `signbit(-0.0)` folds to 0 when the operand is a literal/local/computed constant (works correctly through a global variable's runtime load — the runtime bitcast path is fine). The pattern: `IrConst::F64(v) => *v as i64 as u64` converts the VALUE (-0.0 → 0, 3.5 → 3) instead of the bits. signbit(-3.5) survives only by accident (-3 >> 63 = 1); signbit(-0.0) is the canonical falsifier; any const bit-trick (union punning of const floats, frexp-style masks) is corrupted.

**Function:** shared_const_eval::irconst_to_bits
**Source location:** src/common/const_eval.rs:298-305 (`IrConst::F32(v) => *v as i64 as u64, IrConst::F64(v) => *v as i64 as u64`)
**Detected by:** round-22 battery (NaN/-0.0 semantics probe vs gcc; isolated: global=1, local/const/computed=0 at -O0 AND -O2; gcc gives 1 everywhere)
**Minimal input:** `double lit = -0.0; printf("%d %d\n", signbit(lit), signbit(-0.0));` → gcc: `1 1`; ccc -O0/-O2: `0 0` (signbit via glibc math.h → __builtin_signbit → the SignBit intrinsic's constant path)
**Expected:** bitcast semantics (f64::to_bits / f32::to_bits widened)
**Actual:** numeric cast (`as i64`) — -0.0 → 0, 3.5 → 3, inf saturates
**Severity:** medium (silent wrong constant for bit-level float folding)
**Contract evidence:** documented-and-violated — src/common/README.md: "IrConst-to-bits conversion (irconst_to_bits) — Converts an IrConst to raw u64 bits for bit-level operations."
**Fix:** `IrConst::F64(v) => v.to_bits()`, `IrConst::F32(v) => (v.to_bits() as u32) as u64` (sign-extend per the bool you already return); keep the integer arm as is.
**Regression test:** the minimal input above asserting signbit(-0.0)==1 in all four positions (global/local/computed/bare-const).
