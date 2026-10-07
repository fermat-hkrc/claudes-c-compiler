# Bug: encode_vmv_v_x ignores extra operands
**Law:** ∀ vd, rs1 ∈ 0..31, extra ∈ Operand. encode_vmv_v_x([Reg(v{vd}), Reg(x{rs1}), extra]) is Err
**Impact:** A third (or later) operand is not rejected. llvm-mc `-triple=riscv64 -mattr=+v` reports `invalid operand for instruction`. Callers that pass a stray token get a valid-looking 32-bit unmasked vmv.v.x encoding for the first two operands instead of an assembler error.
**Function:** encode_vmv_v_x
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/vector.rs:163
**Detected by:** Negative/Error Contract — extra operand
**Minimal input:** encode_vmv_v_x([Reg("v0"), Reg("x0"), Imm(0)])
**Expected:** Err (complete vd, rs1 form already present; llvm-mc rejects the extra token)
**Actual:** Ok(Word(0x5e004057)) — same encoding as `vmv.v.x v0, x0`
**Severity:** medium
**Root cause:** vector.rs:164-169 encode_vmv_v_x reads only operands 0 and 1 via get_vreg/get_reg and never checks operands.len() == 2, so extra tokens are ignored.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/vector.rs:164`
```rust
    let vd = get_vreg(operands, 0)?;
    let rs1 = get_reg(operands, 1)?;
    // funct6=010111, vm=1, vs2=0
    let word = (0b010111u32 << 26) | (1u32 << 25) | (rs1 << 15) | (0b100 << 12) | (vd << 7) | OP_V;
    Ok(EncodeResult::Word(word))
```
**Suggested fix:** Reject any operand list that is not exactly two operands.
```rust
    if operands.len() != 2 {
        return Err(format!("vmv.v.x expects 2 operands, got {}", operands.len()));
    }
    let vd = get_vreg(operands, 0)?;
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_vmv_v_x_regression_extra_operand -- --test-threads=1
```
**Raw output:**
```text
thread 'backend::riscv::assembler::encoder::encode_vmv_v_x_pbt::encode_vmv_v_x_neg_extra' (2886141) panicked at src/backend/riscv/assembler/encoder/encode_vmv_v_x_pbt.rs:240:1:
Test failed: extra operand Imm(0) must Err for vmv.v.x (llvm-mc rejects extra); got Ok(Word(1577074775)) at src/backend/riscv/assembler/encoder/encode_vmv_v_x_pbt.rs:360.
minimal failing input: vd = 0, rs1 = 0, extra = Imm(
    0,
)
	successes: 0
	local rejects: 0
	global rejects: 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/encode_vmv_v_x_pbt.rs
