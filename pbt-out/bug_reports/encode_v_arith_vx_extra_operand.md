# Bug: encode_v_arith_vx ignores extra operands
**Law:** ∀ vd, vs2, rs1 ∈ 0..31, extra ∉ {v0.t mask}, (mnem, funct6) ∈ opivx_family. encode_v_arith_vx([v{vd}, v{vs2}, x{rs1}, extra], funct6) is Err
**Impact:** A fourth (or later) operand is not rejected. llvm-mc `-triple=riscv64 -mattr=+v` rejects the extra token. Callers that pass a stray token get a valid-looking 32-bit unmasked OPIVX encoding for the first three operands instead of an assembler error.
**Function:** encode_v_arith_vx
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/vector.rs:134
**Detected by:** Negative/Error Contract — extra operand
**Minimal input:** encode_v_arith_vx([Reg("v0"), Reg("v0"), Reg("x0"), Imm(0)], funct6=0b000000)
**Expected:** Err (complete vd, vs2, rs1 form already present; llvm-mc rejects the extra token)
**Actual:** Ok(Word(0x02004057)) — same encoding as `vadd.vx v0, v0, x0`
**Severity:** medium
**Root cause:** vector.rs:135-140 encode_v_arith_vx reads only operands 0, 1, and 2 via get_vreg/get_reg and never checks operands.len() == 3, so extra tokens are ignored.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/vector.rs:135`
```rust
    let vd = get_vreg(operands, 0)?;
    let vs2 = get_vreg(operands, 1)?;
    let rs1 = get_reg(operands, 2)?;
    let vm: u32 = 1;
    let word = (funct6 << 26) | (vm << 25) | (vs2 << 20) | (rs1 << 15) | (0b100 << 12) | (vd << 7) | OP_V;
    Ok(EncodeResult::Word(word))
```
**Suggested fix:** Reject any operand list that is not exactly three operands (mask token v0.t is a separate 4-operand form).
```rust
    if operands.len() != 3 {
        return Err(format!("OPIVX expects 3 operands, got {}", operands.len()));
    }
    let vd = get_vreg(operands, 0)?;
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_v_arith_vx_regression_extra_operand -- --test-threads=1
```
**Raw output:**
```text
thread 'backend::riscv::assembler::encoder::encode_v_arith_vx_pbt::encode_v_arith_vx_neg_extra' (2880308) panicked at src/backend/riscv/assembler/encoder/encode_v_arith_vx_pbt.rs:317:1:
Test failed: extra operand Imm(0) must Err for OPIVX (llvm-mc rejects extra); got Ok(Word(33570903)) at src/backend/riscv/assembler/encoder/encode_v_arith_vx_pbt.rs:478.
minimal failing input: vd = 0, vs2 = 0, rs1 = 0, extra = Imm(
    0,
), kind = 0
	successes: 0
	local rejects: 0
	global rejects: 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/encode_v_arith_vx_pbt.rs
