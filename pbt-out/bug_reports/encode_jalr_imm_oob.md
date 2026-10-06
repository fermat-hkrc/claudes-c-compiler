# Bug: encode_jalr silently truncates out-of-range immediates
**Law:** ∀ rd, rs1 ∈ GPR, ∀ off ∈ ℤ \ [-2048, 2047]. llvm-mc("jalr rd, rs1, off") errors ∧ encode_jalr([Reg(rd), Reg(rs1), Imm(off)]) = Err(_)
**Impact:** An out-of-range JALR offset assembles to a different jump (e.g. `jalr x0, x1, 2048` becomes `jalr x0, x1, -2048`) instead of being rejected, so callers get silent wrong machine code.
**Function:** encode_jalr
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/base.rs:92
**Detected by:** Negative/Error Contract
**Minimal input:** encode_jalr([Reg("x0"), Reg("x1"), Imm(2048)])
**Expected:** Err (llvm-mc: operand must be an integer in the range [-2048, 2047])
**Actual:** Ok(Word(0x80008067)) — encoding of `jalr x0, x1, -2048`
**Severity:** high
**Root cause:** base.rs:119 casts the i64 immediate to i32 with no range check; encode_i then masks with 0xFFF, so 2048 becomes the 12-bit pattern of -2048.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/base.rs:119`
```rust
            Ok(EncodeResult::Word(encode_i(OP_JALR, rd, 0, rs1, imm as i32)))
```
**Suggested fix:** Reject immediates outside the signed 12-bit range before packing.
```rust
            let imm = get_imm(operands, 2)?;
            if !(-2048..=2047).contains(&imm) {
                return Err(format!(
                    "jalr: immediate {imm} must be in [-2048, 2047]"
                ));
            }
            Ok(EncodeResult::Word(encode_i(OP_JALR, rd, 0, rs1, imm as i32)))
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_jalr_regression_imm_oob -- --test-threads=1
```
**Raw output:**
```text
thread 'backend::riscv::assembler::encoder::encode_jalr_pbt::test_encode_jalr_regression_imm_oob' (2734246) panicked at src/backend/riscv/assembler/encoder/encode_jalr_pbt.rs:311:5:
jalr x0, x1, 2048 must Err (imm12 range); got Ok(Word(2147516519))
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/encode_jalr_pbt.rs
