# Bug: RISC-V assembler silently accepts out-of-range shift amounts (slli a0, a1, 64; slliw a0, a1, 32; negative amounts)
> **Filed as [issue #797](https://github.com/fermat-hkrc/claudes-c-compiler/issues/797)**

**Law:** shamt is 6 bits (0..63) for RV64 I-shifts and 5 bits (0..31) for *W forms; negative or ≥width amounts are unencodable.

**Impact:** silent wrong shift (64 masks into the 6-bit field as 0; *W forms take 32 as 0). llvm-mc rejects; ccc accepts.

**Detected by:** round-17 hand probes (`slli a0, a1, 64`, `slli a0, a1, -1`, `slliw a0, a1, 32`); encoding verified
**Minimal input:** `.text\nslli a0, a1, 64`
**Severity:** medium
**Contract evidence:** inferred (ISA shamt field width; llvm-mc rejects)
**Fix:** validate shamt against the instruction's width (63 / 31).
**Regression test:** (red witness)
