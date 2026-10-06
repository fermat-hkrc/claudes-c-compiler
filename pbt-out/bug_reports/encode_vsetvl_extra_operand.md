# Bug: encode_vsetvl ignores extra operands
**Law:** ∀ rd, rs1, rs2 ∈ GPR names, extra ∈ Operand. encode_vsetvl([Reg(rd), Reg(rs1), Reg(rs2), extra]) = Err(_)
**Impact:** A fourth (or later) operand is not rejected. llvm-mc `-triple=riscv64 -mattr=+v` reports `invalid operand for instruction`. Callers that pass a stray token get a valid-looking 32-bit word for the first three GPRs instead of an assembler error.
**Function:** encode_vsetvl
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/vector.rs:70
**Detected by:** Negative/Error Contract — extra operand
**Minimal input:** encode_vsetvl([Reg("x0"), Reg("x0"), Reg("x0"), Imm(0)])
**Expected:** Err (complete 3-GPR form already present; llvm-mc rejects the extra token)
**Actual:** Ok(Word(0x80007057)) — same encoding as `vsetvl x0, x0, x0`
**Severity:** medium
**Root cause:** vector.rs:71-75 encode_vsetvl reads only operands 0, 1, and 2 via get_reg and never checks operands.len() == 3, so extra tokens are ignored.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/vector.rs:71`
```rust
    let rd = get_reg(operands, 0)?;
    let rs1 = get_reg(operands, 1)?;
    let rs2 = get_reg(operands, 2)?;
    let word = (0b1000000u32 << 25) | (rs2 << 20) | (rs1 << 15) | (0b111 << 12) | (rd << 7) | OP_V;
    Ok(EncodeResult::Word(word))
```
**Suggested fix:** Reject any operand list that is not exactly three registers.
```rust
    if operands.len() != 3 {
        return Err(format!("vsetvl expects 3 operands, got {}", operands.len()));
    }
    let rd = get_reg(operands, 0)?;
    let rs1 = get_reg(operands, 1)?;
    let rs2 = get_reg(operands, 2)?;
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_vsetvl_regression_extra_operand -- --test-threads=1
```
**Raw output:**
```text
thread 'backend::riscv::assembler::encoder::encode_vsetvl_pbt::encode_vsetvl_neg_extra' panicked at src/backend/riscv/assembler/encoder/encode_vsetvl_pbt.rs:259:1:
Test failed: extra operand Imm(0) must Err for vsetvl (llvm-mc rejects extra); got Ok(Word(2147512407)) at src/backend/riscv/assembler/encoder/encode_vsetvl_pbt.rs:392.
minimal failing input: rd = "x0", rs1 = "x0", rs2 = "x0", extra = Imm(
    0,
)
	successes: 0
	local rejects: 0
	global rejects: 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/encode_vsetvl_pbt.rs
