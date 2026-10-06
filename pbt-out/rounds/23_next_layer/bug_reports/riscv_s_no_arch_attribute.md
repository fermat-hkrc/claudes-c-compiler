# Bug: ccc-riscv -S output lacks the .attribute arch directive — external assemblers (llvm-mc/GNU as) reject it (fsd/mul need D/M declared)
> **Filed as [issue #808](https://github.com/fermat-hkrc/claudes-c-compiler/issues/808)**

**Law:** an assembler source that uses instructions beyond the base ISA must declare the module set; GNU/LLVM assemblers default to rv64i and REQUIRE `.attribute arch, "rv64..."` (or -march) before accepting fsd (D) / mul (M) / float instructions. gcc/clang -S always emit the attribute.

**Impact:** ccc-riscv's -S output cannot be assembled by ANY external assembler (llvm-mc: "instruction requires 'D' / 'M'"), breaking the -S → external-tool → link workflow (asm inspection, hand-tuning, objdump roundtrips). The internal -c path is unaffected.

**Function:** riscv codegen -S emission
**Source location:** src/backend/riscv/ (-S output has no .attribute arch line; verified: grep -c attribute on -S output = 0)
**Detected by:** round-23 -S roundtrip differential (llvm-mc failed on ccc's own -S output; prepending `.attribute arch, "rv64gc"` fixes it)
**Minimal input:** compile any program using a double or multiplication with `ccc-riscv -S`; then `llvm-mc -triple=riscv64 -filetype=obj` on the .s → "error: instruction requires the following: 'D'"
**Expected:** .s starts with (or contains) `.attribute arch, "rv64gc"` (or the configured march)
**Actual:** no attribute line at all
**Severity:** low (interop; internal pipeline fine)
**Contract evidence:** inferred (both reference assemblers require it; the -march= flag exists on the CLI so the arch string is known at emit time)
**Fix:** emit `.attribute arch, "rv64<i|m|f|d...>"` at the top of -S output per the selected -march (default gc).
