# Bug: i686 assembler silently accepts GPR width mismatches (movl %eax, %ax; mov %al, %bx; xorl %eax, %bx)
> **Filed as [issue #800](https://github.com/fermat-hkrc/claudes-c-compiler/issues/800)**

**Law:** an AT&T form with a size suffix must reject operands of a different register width (i686: mixing l/w/b classes under a fixed suffix is unencodable).

**Impact:** external .s files silently assemble to different instructions than written — the i686 assembler's own path (sibling of the x86-64 #774 class; separate code path under src/backend/i686/assembler).

**Function:** i686 assembler operand validation
**Source location:** src/backend/i686/assembler (no width-vs-suffix check)
**Detected by:** round-19 PBT witness property p_i686_width_mix_rejected (FAILING, shrunk counterexample `movl %ax, %eax`-class); double-reference confirmed (as --32 AND llvm-mc -triple=i386 reject every case)
**Minimal input:** `.text\nmovl %eax, %ax` (also `mov %al, %bx`, `xorl %eax, %bx`)
**Expected:** error
**Actual:** silently assembled
**Severity:** medium
**Contract evidence:** inferred (double-reference rejection)
**Fix:** validate register width class against the mnemonic suffix, as proposed for #774 on the x86-64 side.
**Regression test:** src/backend/i686/mod.rs pbt_witness::test_i686asm_regression_width_mix_rejected
