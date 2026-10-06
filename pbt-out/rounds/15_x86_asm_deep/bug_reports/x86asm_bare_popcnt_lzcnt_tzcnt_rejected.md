# Bug: x86 assembler rejects suffix-less popcnt/lzcnt/tzcnt (bare forms valid in GNU as and llvm-mc)
> **Filed as [issue #777](https://github.com/fermat-hkrc/claudes-c-compiler/issues/777)**

**Law:** AT&T mnemonics whose width is inferable from register operands accept the suffix-less form (GNU as and llvm-mc both assemble `popcnt %rax, %rbx`). ccc only matches the suffixed variants.

**Impact:** external .s files using the canonical bare forms fail to assemble ("unhandled instruction: popcnt"). Functionality/completeness gap (no silent wrong code).

**Function:** x86 assembler mnemonic table
**Source location:** src/backend/x86/assembler (popcnt/lzcnt/tzcnt matched only with q/l suffixes)
**Detected by:** round-15 P1 (REJECT-VALID class, GNU as accepted in-run)
**Minimal input:** `.text\npopcnt %rax, %rbx` → ccc: error: unhandled instruction; as/llvm-mc: assemble fine
**Expected:** assembles (suffix inferred from register width)
**Actual:** rejected
**Severity:** low
**Contract evidence:** inferred (double-reference acceptance)
**Fix:** accept bare popcnt/lzcnt/tzcnt, inferring the width from the first register operand (same as bswap already does).
**Regression test:** test_x86asm_regression_bare_popcnt_assembles
