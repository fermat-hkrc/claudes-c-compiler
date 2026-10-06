# Bug: RISC-V assembler silently accepts malformed operand forms (add a0, a1, 2048; beq with raw numeric offset; fence.i with operands)
> **Filed as [issue #798](https://github.com/fermat-hkrc/claudes-c-compiler/issues/798)**

**Law:** register-register ops take exactly two source registers; branch targets are labels (or pc-relative offsets in llvm-mc syntax — either way a raw 4096 immediate in beq is invalid); fence.i takes no operands.

**Impact:** malformed source assembles to something arbitrary instead of erroring. llvm-mc rejects all; ccc accepts.

**Detected by:** round-17 hand probes (`add a0, a1, 2048`, `beq a0, a1, 4096`, `fence.i a0, a1`)
**Minimal input:** `.text\nadd a0, a1, 2048`
**Severity:** medium (silent garbage encodings)
**Contract evidence:** inferred (ISA instruction formats; llvm-mc rejects)
**Fix:** enforce operand kinds per instruction format (reg slots reject immediates; branch requires label; fence.i arity 0).
**Regression test:** (red witness)
