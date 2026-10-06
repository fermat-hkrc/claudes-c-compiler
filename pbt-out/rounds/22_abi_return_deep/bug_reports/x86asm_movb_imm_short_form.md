# Bug: builtin x86 assembler encodes `movb $imm, %r8` in the 3-byte C6 form — never the 2-byte B0+ short form (1-byte bloat per mov-imm-to-8bit-reg)
> **Filed as [issue #807](https://github.com/fermat-hkrc/claudes-c-compiler/issues/807)**

**Law:** x86 has a canonical short encoding for `mov $imm8, %al..%bl` etc. (opcode B0+r, 2 bytes); GNU as uses it. ccc's builtin assembler always emits the 3-byte C6 /0 form.

**Impact:** systematic code-size bloat: every mov-imm-to-8-bit-register in generated code is 1 byte larger than gcc's (found via the -S round-trip: ccc -S output assembled by GNU as produces SHORTER .text than ccc -c's own object — byte-identical otherwise). Not a correctness bug; affects all generated code density.

**Function:** x86 assembler mov-immediate encoder
**Source location:** src/backend/x86/assembler/encoder/gp_integer.rs:98/187 (`self.bytes.push(0xC6)` — no B0+r short-form arm for imm8→r8 without REX)
**Detected by:** round-22 -S round-trip differential (rt.c: direct -c .text = `... c6 c0 01 ...` vs GNU as on ccc's own -S output = `... b0 01 ...`; rest of the function byte-identical, shifted by the 1-byte delta)
**Minimal input:** `.text\nmovb $1, %al` → ccc: `c6 c0 01` (3B); as: `b0 01` (2B)
**Expected:** 2-byte B0+r form for non-REX 8-bit registers
**Actual:** 3-byte C6 /0 form always
**Severity:** low (code size only; execution identical)
**Contract evidence:** inferred (double-reference: GNU as encodes the short form for ccc's own emitted text)
**Fix:** in the imm8 path, when the destination is a legacy 8-bit register (no REX needed), emit B0+reg instead of C6 /0 ib.
**Regression test:** assemble `movb $1, %al` and assert the 2-byte encoding.
