# Bug: RISC-V assembler silently accepts out-of-imm12-range immediates (addi a0, a1, 2048 encodes as -2048)
> **Filed as [issue #796](https://github.com/fermat-hkrc/claudes-c-compiler/issues/796)**

**Law:** I-type immediates are SIGNED 12-bit: [-2048, 2047]; load/store offsets likewise. Out-of-range values are physically unencodable and must be rejected (the ISA field width is the contract — RISC-V unprivileged spec, imm[11:0]).

**Impact:** silent WRONG VALUE: `addi a0, a1, 2048` assembles with imm bits 0x800, which decodes as -2048 (verified by encoding bytes 13 85 05 80). Also affects andi/ori/xori (4096), addiw, ld/sd offsets (2048). llvm-mc rejects all; ccc accepts.

**Function:** RISC-V assembler immediate encoding
**Source location:** src/backend/riscv/assembler (no imm12 range validation)
**Detected by:** round-17 deep-sweep hand probes (11/15 invalid forms silently accepted); encoding bytes verified
**Minimal input:** `.text\naddi a0, a1, 2048` → encodes addi a0, a1, -2048
**Expected:** error (imm out of range)
**Actual:** silently encoded with truncated field — the wrong constant
**Severity:** medium (silent wrong value — worst class)
**Contract evidence:** inferred (ISA encoding field width: signed imm[11:0]; llvm-mc rejects; the encoding provably changes the value)
**Fix:** range-check every imm12 slot (addi/andi/ori/xori/slti*/addiw + load/store offsets) before packing.
**Regression test:** src/backend/riscv/mod.rs pbt_asm_deep (red witness)
