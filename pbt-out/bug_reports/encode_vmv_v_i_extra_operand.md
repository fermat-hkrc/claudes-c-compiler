# Bug: encode_vmv_v_i ignores extra operands
**Law:** ∀ vd ∈ {0..31}, simm ∈ {-16..15}, extra ∈ Operand. encode_vmv_v_i([Reg("v{vd}"), Imm(simm), extra]) is Err
**Impact:** A third (or later) operand is not rejected. llvm-mc `-triple=riscv64 -mattr=+v` reports `invalid operand for instruction`. Callers that pass a stray token get a valid-looking 32-bit unmasked vmv.v.i encoding for the first two operands instead of an assembler error.
**Function:** encode_vmv_v_i
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/vector.rs:172
**Detected by:** Negative/Error Contract — extra operand
**Minimal input:** encode_vmv_v_i([Reg("v0"), Imm(-16), Imm(0)])
**Expected:** Err (complete vd, simm5 form already present; llvm-mc rejects the extra token)
**Actual:** Ok(Word(0x5e083057)) — same encoding as `vmv.v.i v0, -16`
**Severity:** medium
**Root cause:** vector.rs:173-177 encode_vmv_v_i reads only operands 0 and 1 via get_vreg/get_imm and never checks operands.len() == 2, so extra tokens are ignored.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/vector.rs:173`
```rust
    let vd = get_vreg(operands, 0)?;
    let simm5 = get_imm(operands, 1)? as u32 & 0x1F;
    // funct6=010111, vm=1, vs2=0
    let word = (0b010111u32 << 26) | (1u32 << 25) | (simm5 << 15) | (0b011 << 12) | (vd << 7) | OP_V;
    Ok(EncodeResult::Word(word))
```
**Suggested fix:** Reject any operand list that is not exactly two operands.
```rust
    if operands.len() != 2 {
        return Err(format!("vmv.v.i expects 2 operands, got {}", operands.len()));
    }
    let vd = get_vreg(operands, 0)?;
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_vmv_v_i_regression_extra_operand -- --test-threads=1
```
**Raw output:**
```text
thread 'backend::riscv::assembler::encoder::encode_vmv_v_i_pbt::encode_vmv_v_i_neg_extra' (2888382) panicked at src/backend/riscv/assembler/encoder/encode_vmv_v_i_pbt.rs:266:1:
Test failed: extra operand Imm(0) must Err for vmv.v.i (llvm-mc rejects extra); got Ok(Word(1577594967)) at src/backend/riscv/assembler/encoder/encode_vmv_v_i_pbt.rs:370.
minimal failing input: vd = 0, simm = -16, extra = Imm(
    0,
)
	successes: 0
	local rejects: 0
	global rejects: 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/encode_vmv_v_i_pbt.rs
