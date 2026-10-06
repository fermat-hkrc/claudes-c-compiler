# Bug: x86 assembler silently coerces GPR operand width mismatches (movl %eax, %r8w assembles as r8d; addq %r10d, %r10 accepted)
> **Filed as [issue #774](https://github.com/fermat-hkrc/claudes-c-compiler/issues/774)**

**Law:** an AT&T instruction form with a size suffix must reject operands whose register width disagrees with that suffix — `movl %eax, %r8w` has no encoding (16-bit dest in 32-bit mov).

**Impact:** hand-written or external .s files (asm sources, inline-asm edge cases) silently assemble to DIFFERENT instructions than written. Both GNU as and llvm-mc reject these forms; ccc accepts and encodes its own interpretation. ccc's internal C pipeline is unaffected (its codegen emits consistent forms — round-09 E2E vs gcc is fully green).

**Function:** x86 assembler parser/encoder (register-width validation absent)
**Source location:** src/backend/x86/assembler/parser.rs (operand parsing) — no width-vs-suffix check
**Detected by:** round-15 PBT, mutation class "width_swap", double-reference confirmed (GNU as exit 1 AND llvm-mc error; ccc exit 0 + object)
**Minimal input:** `.text\nmovl %eax, %r8w` → encodes `41 89 c0 (mov %eax,%r8d)`; also `addq %r10d, %r10`; also mem form `movl 596(%rsp), %r13`
**Expected:** error (both references reject)
**Actual:** silently assembled with the register reinterpreted at the opcode's width
**Severity:** medium
**Contract evidence:** inferred (double-reference convention: GNU as AND llvm-mc both reject; silent reinterpretation of the source text is a silent wrong-code path for external asm)
**Fix:** validate each register operand's width against the mnemonic suffix (and against the other operand) at parse time; error on mismatch.
**Regression test:** src/backend/x86/mod.rs pbt_regression_asm (test_x86asm_regression_gpr_width_rejected)
