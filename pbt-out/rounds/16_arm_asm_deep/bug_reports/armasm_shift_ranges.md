# Bug: AArch64 assembler silently accepts out-of-range shift amounts (lsl #64, ror w #32, movk w lsl #32/#48, imm12 shift lsl #13)
> **Filed as [issue #793](https://github.com/fermat-hkrc/claudes-c-compiler/issues/793)**

**Law:** shift amounts must be < register width (63 for X, 31 for W); the mov-variant hw field is 0/16 for W and 0/16/32/48 for X; the add/sub imm12 shift may only be lsl #0 or #12.

**Impact:** silent wrong encodings. Both references reject; ccc accepts.

**Detected by:** round-16 hand probes (`lsl x0, x1, #64`, `ror w0, w1, #32`, `movk w14, #25050, lsl #32`, `add x0, x1, #1, lsl #13`); double-reference confirmed
**Minimal input:** `.text\nlsl x0, x1, #64`
**Severity:** medium
**Contract evidence:** inferred (double-reference)
**Fix:** validate shift/hw amounts against operand width and opcode form.
**Regression test:** (red witness to be added)
