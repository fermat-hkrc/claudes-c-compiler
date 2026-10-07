# Bug: encode_v_arith_vv ignores extra operands
**Law:** ∀ vd, vs2, vs1 ∈ 0..31, extra ∈ Operand \ {v0.t mask token}, (mnem, funct6) ∈ opivv_family. encode_v_arith_vv([Reg(v{vd}), Reg(v{vs2}), Reg(v{vs1}), extra], funct6) is Err
**Impact:** A fourth (or later) operand is not rejected. llvm-mc `-triple=riscv64 -mattr=+v` reports `operand must be v0.t` / `expected '.t' suffix`. Callers that pass a stray token get a valid-looking 32-bit unmasked OPIVV encoding for the first three operands instead of an assembler error.
**Function:** encode_v_arith_vv
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/vector.rs:123
**Detected by:** Negative/Error Contract — extra operand
**Minimal input:** encode_v_arith_vv([Reg("v0"), Reg("v0"), Reg("v0"), Imm(0)], funct6=0b000000)
**Expected:** Err (complete vd, vs2, vs1 form already present; llvm-mc rejects the extra token)
**Actual:** Ok(Word(0x02000057)) — same encoding as `vadd.vv v0, v0, v0`
**Severity:** medium
**Root cause:** vector.rs:124-130 encode_v_arith_vv reads only operands 0, 1, and 2 via get_vreg and never checks operands.len() == 3, so extra tokens are ignored.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/vector.rs:124`
```rust
    let vd = get_vreg(operands, 0)?;
    let vs2 = get_vreg(operands, 1)?;
    let vs1 = get_vreg(operands, 2)?;
    let vm: u32 = 1; // unmasked
    // funct3=000 (OPIVV)
    let word = (funct6 << 26) | (vm << 25) | (vs2 << 20) | (vs1 << 15) | (vd << 7) | OP_V;
    Ok(EncodeResult::Word(word))
```
**Suggested fix:** Reject any operand list that is not exactly three operands (mask token v0.t is a separate 4-operand form).
```rust
    if operands.len() != 3 {
        return Err(format!("OPIVV expects 3 operands, got {}", operands.len()));
    }
    let vd = get_vreg(operands, 0)?;
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_v_arith_vv_regression_extra_operand -- --test-threads=1
```
**Raw output:**
```text
thread 'backend::riscv::assembler::encoder::encode_v_arith_vv_pbt::encode_v_arith_vv_neg_extra' (2877953) panicked at src/backend/riscv/assembler/encoder/encode_v_arith_vv_pbt.rs:260:1:
Test failed: extra operand Imm(0) must Err for OPIVV (llvm-mc rejects extra); got Ok(Word(33554519)) at src/backend/riscv/assembler/encoder/encode_v_arith_vv_pbt.rs:394.
minimal failing input: vd = 0, vs2 = 0, vs1 = 0, extra = Imm(
    0,
), kind = 0
	successes: 0
	local rejects: 0
	global rejects: 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/encode_v_arith_vv_pbt.rs
