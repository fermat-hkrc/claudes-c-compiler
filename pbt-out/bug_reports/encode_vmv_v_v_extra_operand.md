# Bug: encode_vmv_v_v ignores extra operands
**Law:** ∀ vd, vs1 ∈ 0..31, extra ∈ Operand. encode_vmv_v_v([Reg(v{vd}), Reg(v{vs1}), extra]) is Err
**Impact:** A third (or later) operand is not rejected. llvm-mc `-triple=riscv64 -mattr=+v` reports `invalid operand for instruction`. Callers that pass a stray token get a valid-looking 32-bit unmasked vmv.v.v encoding for the first two operands instead of an assembler error.
**Function:** encode_vmv_v_v
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/vector.rs:154
**Detected by:** Negative/Error Contract — extra operand
**Minimal input:** encode_vmv_v_v([Reg("v0"), Reg("v0"), Imm(0)])
**Expected:** Err (complete vd, vs1 form already present; llvm-mc rejects the extra token)
**Actual:** Ok(Word(0x5e000057)) — same encoding as `vmv.v.v v0, v0`
**Severity:** medium
**Root cause:** vector.rs:155-159 encode_vmv_v_v reads only operands 0 and 1 via get_vreg and never checks operands.len() == 2, so extra tokens are ignored.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/vector.rs:155`
```rust
    let vd = get_vreg(operands, 0)?;
    let vs1 = get_vreg(operands, 1)?;
    // funct6=010111, vm=1, vs2=0, funct3=000 (OPIVV)
    let word = (0b010111u32 << 26) | (1u32 << 25) | (vs1 << 15) | (vd << 7) | OP_V;
    Ok(EncodeResult::Word(word))
```
**Suggested fix:** Reject any operand list that is not exactly two operands.
```rust
    if operands.len() != 2 {
        return Err(format!("vmv.v.v expects 2 operands, got {}", operands.len()));
    }
    let vd = get_vreg(operands, 0)?;
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_vmv_v_v_regression_extra_operand -- --test-threads=1
```
**Raw output:**
```text
thread 'backend::riscv::assembler::encoder::encode_vmv_v_v_pbt::encode_vmv_v_v_neg_extra' (2884227) panicked at src/backend/riscv/assembler/encoder/encode_vmv_v_v_pbt.rs:204:1:
Test failed: extra operand Imm(0) must Err for vmv.v.v (llvm-mc rejects extra); got Ok(Word(1577058391)) at src/backend/riscv/assembler/encoder/encode_vmv_v_v_pbt.rs:302.
minimal failing input: vd = 0, vs1 = 0, extra = Imm(
    0,
)
	successes: 0
	local rejects: 0
	global rejects: 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/encode_vmv_v_v_pbt.rs
