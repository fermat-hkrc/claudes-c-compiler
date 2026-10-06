# Bug: AArch64 assembler silently accepts invalid addressing forms (ldp non-multiple-of-8 offsets, ldr/str index scale lsl #4)
> **Filed as [issue #794](https://github.com/fermat-hkrc/claudes-c-compiler/issues/794)**

**Law:** ldp/stp offsets must be multiples of the pair element size; the register-index extend/scale option for 64-bit ldr/str is limited to lsl #2/#3 (per size), not arbitrary shifts.

**Impact:** silent wrong encodings. Both references reject; ccc accepts.

**Detected by:** round-16 hand probes (`ldp x0, x1, [sp, #3]`, `str x0, [x1, x2, lsl #4]`); double-reference confirmed
**Minimal input:** `.text\nldp x0, x1, [sp, #3]`
**Severity:** medium
**Contract evidence:** inferred (double-reference)
**Fix:** validate pair-offset alignment and index-scale bounds per access size.
**Regression test:** (red witness to be added)
