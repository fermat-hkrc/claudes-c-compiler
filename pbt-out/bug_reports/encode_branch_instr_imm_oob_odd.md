# Bug: encode_branch_instr silently truncates odd and out-of-range immediates
**Law:** ∀ rs1, rs2 ∈ GPR, ∀ imm ∈ i64. (imm odd ∨ imm < -4096 ∨ imm > 4094) ⇒ encode_branch_instr([Reg(rs1), Reg(rs2), Imm(imm)], 0) = Err
**Impact:** An odd or out-of-range B-type offset assembles to a different branch (e.g. `beq x0, x0, 1` becomes `beq x0, x0, 0`) instead of being rejected, so callers get silent wrong machine code.
**Function:** encode_branch_instr
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/base.rs:125
**Detected by:** Negative/Error Contract
**Minimal input:** encode_branch_instr([Reg("x0"), Reg("x0"), Imm(1)], 0)
**Expected:** Err (llvm-mc: immediate must be a multiple of 2 bytes in the range [-4096, 4094])
**Actual:** Ok(Word(0x00000063)) — encoding of `beq x0, x0, 0`
**Severity:** high
**Root cause:** base.rs:131 casts the i64 immediate to i32 with no range or alignment check; encode_b then drops bit 0 via `(imm >> 1) & 0xF`, so odd 1 becomes 0.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/base.rs:131`
```rust
            Ok(EncodeResult::Word(encode_b(OP_BRANCH, funct3, rs1, rs2, *imm as i32)))
```
**Suggested fix:** Reject immediates that are odd or outside the 13-bit even signed range before packing.
```rust
            let imm = *imm;
            if imm % 2 != 0 || !(-4096..=4094).contains(&imm) {
                return Err(format!(
                    "branch: immediate {imm} must be a multiple of 2 in [-4096, 4094]"
                ));
            }
            Ok(EncodeResult::Word(encode_b(OP_BRANCH, funct3, rs1, rs2, imm as i32)))
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_branch_instr_regression_imm_oob -- --test-threads=1
```
**Raw output:**
```text
thread 'backend::riscv::assembler::encoder::encode_branch_instr_pbt::test_encode_branch_instr_regression_imm_oob' (2737315) panicked at src/backend/riscv/assembler/encoder/encode_branch_instr_pbt.rs:368:5:
beq x0, x1, 1 must Err (odd offset); got Ok(Word(1048675))
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/encode_branch_instr_pbt.rs
