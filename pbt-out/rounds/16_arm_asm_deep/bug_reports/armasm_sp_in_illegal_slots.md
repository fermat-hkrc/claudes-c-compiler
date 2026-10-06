# Bug: AArch64 assembler silently accepts SP/WSP in register-file-only operand slots (and sp, sp, x28; mul w0, wsp, w2)
> **Filed as [issue #795](https://github.com/fermat-hkrc/claudes-c-compiler/issues/795)**

**Law:** SP/WSP is only encodable in the dedicated SP-legal forms (add/sub immediate and shifted, loads/stores base, mov); logical ops, mul-family and most data-processing ops take X0-X30/W0-W30 only.

**Impact:** silent wrong encodings (SP encodes as reg 31 = XZR in those forms — `and sp, sp, x28` silently becomes `and xzr, xzr, x28`). Both references reject; ccc accepts.

**Detected by:** round-16 hand probe (`mul w0, wsp, w2`) + P1 domain finding (`and sp, sp, x28`); double-reference confirmed
**Minimal input:** `.text\nmul w0, wsp, w2`
**Severity:** medium
**Contract evidence:** inferred (double-reference)
**Fix:** reject SP/WSP operands outside the SP-legal opcode forms.
**Regression test:** (red witness to be added)
