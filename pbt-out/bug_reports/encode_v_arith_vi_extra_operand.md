# Bug: encode_v_arith_vi ignores extra operands
**Law:** ∀ vd, vs2 ∈ 0..31, extra ∉ {v0.t mask}, (mnem, funct6) ∈ opivi_family. encode_v_arith_vi([v{vd}, v{vs2}, Imm(imm), extra], funct6) is Err
**Impact:** A fourth (or later) operand is not rejected. llvm-mc `-triple=riscv64 -mattr=+v` rejects the extra token ("operand must be v0.t"). Callers that pass a stray token get a valid-looking 32-bit unmasked OPIVI encoding for the first three operands instead of an assembler error.
**Function:** encode_v_arith_vi
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/vector.rs:144
**Detected by:** Negative/Error Contract — extra operand
**Minimal input:** encode_v_arith_vi([Reg("v0"), Reg("v0"), Imm(-16), Imm(0)], funct6=0b000000)
**Expected:** Err (complete vd, vs2, imm form already present; llvm-mc rejects the extra token)
**Actual:** Ok(Word(0x02083057)) — same encoding as `vadd.vi v0, v0, -16`
**Severity:** medium
**Root cause:** vector.rs:145-150 encode_v_arith_vi reads only operands 0, 1, and 2 via get_vreg/get_imm and never checks operands.len() == 3, so extra tokens are ignored.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/vector.rs:145`
```rust
    let vd = get_vreg(operands, 0)?;
    let vs2 = get_vreg(operands, 1)?;
    let simm5 = get_imm(operands, 2)? as u32 & 0x1F;
    let vm: u32 = 1;
    let word = (funct6 << 26) | (vm << 25) | (vs2 << 20) | (simm5 << 15) | (0b011 << 12) | (vd << 7) | OP_V;
    Ok(EncodeResult::Word(word))
```
**Suggested fix:** Reject any operand list that is not exactly three operands (mask token v0.t is a separate 4-operand form).
```rust
    if operands.len() != 3 {
        return Err(format!("OPIVI expects 3 operands, got {}", operands.len()));
    }
    let vd = get_vreg(operands, 0)?;
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_v_arith_vi_regression_extra_operand -- --test-threads=1
```
**Raw output:**
```text
thread 'backend::riscv::assembler::encoder::encode_v_arith_vi_pbt::encode_v_arith_vi_neg_extra' (2882352) panicked at src/backend/riscv/assembler/encoder/encode_v_arith_vi_pbt.rs:356:1:
Test failed: extra operand Imm(0) must Err for OPIVI (llvm-mc rejects extra); got Ok(Word(34091095)) at src/backend/riscv/assembler/encoder/encode_v_arith_vi_pbt.rs:498.
minimal failing input: vd = 0, vs2 = 0, simm = -16, extra = Imm(
    0,
), kind = 0
	successes: 0
	local rejects: 0
	global rejects: 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/encode_v_arith_vi_pbt.rs
