# RISC-V assembler lacks part of the compressed (RVC) subset: c.lwsp/c.sw/c.beqz/c.slli rejected while c.add/c.mv/c.addi/c.li/c.jr work
> **Filed as [issue #821](https://github.com/fermat-hkrc/claudes-c-compiler/issues/821)**

**Law:** llvm-mc -mattr=+c assembles the standard RVC forms; the assembler supports a subset (c.add/c.mv/c.addi/c.li/c.jr work) but rejects others of the same class.

**Impact:** external .s using the compressed load/store/branch/shift forms fails. GAPs (llvm-mc accepts, ccc rejects): `c.lwsp a0, 12(sp)`, `c.sw a0, 8(a1)`, `c.beqz a0, 8`, `c.slli a1, 5`. (c.addi16sp rejected by both — consistent.)

**Detected by:** round-26 RVC probe battery (10 forms; llvm-mc as the RISC-V ISA reference per rounds 17/796-798)
**Severity:** low (completeness)
**Contract evidence:** inferred (llvm-mc +c acceptance; sibling forms already supported)
**Fix:** add the CL/CS/CI-format arms for the missing compressed forms.
