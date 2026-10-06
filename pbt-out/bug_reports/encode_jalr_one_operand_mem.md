# Bug: encode_jalr rejects 1-operand mem form jalr off(rs1)
**Law:** ∀ rs1 ∈ GPR, ∀ off ∈ [-2048, 2047]. encode_jalr([Mem{base: rs1, offset: off}]) = encode_jalr([Reg("ra"), Reg(rs1), Imm(off)]) = Word(llvm-mc("jalr off(rs1)"))
**Impact:** Valid textual assembly `jalr 8(x2)` (llvm-mc: jalr ra, 8(sp)) is rejected, so the assembler cannot encode the implicit-rd mem form of JALR.
**Function:** encode_jalr
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/base.rs:92
**Detected by:** Algebraic — Metamorphic (plus llvm-mc differential)
**Minimal input:** encode_jalr([Mem { base: "x2", offset: 8 }])
**Expected:** Ok(Word) equal to encode_jalr([Reg("ra"), Reg("x2"), Imm(8)]) / llvm-mc("jalr 8(x2)")
**Actual:** Err("expected register at operand 0, got Some(Mem { base: \"x2\", offset: 8 })")
**Severity:** medium
**Root cause:** base.rs:97 the 1-operand arm always calls get_reg; Mem (the documented 2-operand mem form with implicit rd=ra) is not matched.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/base.rs:97`
```rust
            let rs1 = get_reg(operands, 0)?;
```
**Suggested fix:** Accept a Mem operand in the 1-operand arm with implicit rd=ra.
```rust
        1 => {
            match &operands[0] {
                Operand::Reg(name) => {
                    let rs1 = reg_num(name).ok_or("invalid register")?;
                    Ok(EncodeResult::Word(encode_i(OP_JALR, 1, 0, rs1, 0)))
                }
                Operand::Mem { base, offset } => {
                    let rs1 = reg_num(base).ok_or("invalid base register")?;
                    if !(-2048..=2047).contains(offset) {
                        return Err(format!(
                            "jalr: immediate {offset} must be in [-2048, 2047]"
                        ));
                    }
                    Ok(EncodeResult::Word(encode_i(OP_JALR, 1, 0, rs1, *offset as i32)))
                }
                _ => Err("jalr: invalid operands".to_string()),
            }
        }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_jalr_regression_one_operand_mem -- --test-threads=1
```
**Raw output:**
```text
thread 'backend::riscv::assembler::encoder::encode_jalr_pbt::test_encode_jalr_regression_one_operand_mem' (2734247) panicked at src/backend/riscv/assembler/encoder/encode_jalr_pbt.rs:342:18:
expected Word for jalr 8(x2), got Err("expected register at operand 0, got Some(Mem { base: \"x2\", offset: 8 })")
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/encode_jalr_pbt.rs
