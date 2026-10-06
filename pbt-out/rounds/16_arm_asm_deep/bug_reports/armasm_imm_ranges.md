# Bug: AArch64 assembler silently accepts out-of-range immediates (movz/movk imm16, ldur [-256,255], ldp/stp imm7×8 offsets)
> **Filed as [issue #792](https://github.com/fermat-hkrc/claudes-c-compiler/issues/792)**

**Law:** imm16 for movz/movk/movn; unscaled ldur/stur offsets within [-256,255]; ldp/stp offsets within the imm7×8 (x-pair) range. Out-of-range immediates must be rejected.

**Impact:** silent wrong encodings of external .s (e.g. movk #325050 encodes low bits). Both references reject; ccc accepts.

**Function:** AArch64 assembler immediate encoding paths
**Source location:** src/backend/arm/assembler (no range checks on imm16 / unscaled / pair offsets)
**Detected by:** round-16 hand probes (`movk x0, #65536`, `movz w0, #65536`, `ldur x0, [x1, #-257]`, `ldp x23, x27, [sp, #1936]`) + P2 imm-bump mutation; all double-reference confirmed
**Minimal input:** `.text\nmovk x0, #65536`
**Expected:** error
**Actual:** silently assembled (low bits taken)
**Severity:** medium
**Contract evidence:** inferred (double-reference)
**Fix:** range-check each immediate slot per the ARM ARM encoding limits.
**Regression test:** (red witness to be added)
