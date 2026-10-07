# Bug: encode_v_crypto_vv ignores extra operands
**Law:** ∀ vd, vs2, vs1 ∈ {0..31}, extra ∈ Operand. encode_v_crypto_vv([v{vd}, v{vs2}, v{vs1}, extra], 0b100000) is Err
**Impact:** A fourth (or later) operand is not rejected. Callers that pass a stray token get a valid-looking 32-bit vsm3me.vv encoding for the first three operands instead of an assembler error.
**Function:** encode_v_crypto_vv
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/vector.rs:201
**Detected by:** Negative/Error Contract — extra operand
**Minimal input:** encode_v_crypto_vv([Reg("v0"), Reg("v0"), Reg("v0"), Imm(0)], 0b100000)
**Expected:** Err (complete vd, vs2, vs1 form already present)
**Actual:** Ok(Word(0x82002077)) — same encoding as `vsm3me.vv v0, v0, v0`
**Severity:** medium
**Root cause:** vector.rs:202-206 encode_v_crypto_vv reads only operands 0..2 via get_vreg and never checks operands.len() == 3, so extra tokens are ignored.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/vector.rs:202`
```rust
    let vd = get_vreg(operands, 0)?;
    let vs2 = get_vreg(operands, 1)?;
    let vs1 = get_vreg(operands, 2)?;
    let word = (funct6 << 26) | (1u32 << 25) | (vs2 << 20) | (vs1 << 15) | (0b010 << 12) | (vd << 7) | OP_V_CRYPTO;
    Ok(EncodeResult::Word(word))
```
**Suggested fix:** Reject any operand list that is not exactly three operands (vd, vs2, vs1).
```rust
    if operands.len() != 3 {
        return Err(format!("vsm3me.vv expects 3 operands, got {}", operands.len()));
    }
    let vd = get_vreg(operands, 0)?;
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_v_crypto_vv_regression_extra_operand -- --test-threads=1
```
**Raw output:**
```text
thread 'backend::riscv::assembler::encoder::encode_v_crypto_vv_pbt::encode_v_crypto_vv_neg_extra' (2892588) panicked at src/backend/riscv/assembler/encoder/encode_v_crypto_vv_pbt.rs:176:1:
Test failed: extra operand Imm(0) must Err for crypto VV; got Ok(Word(2181046391)) at src/backend/riscv/assembler/encoder/encode_v_crypto_vv_pbt.rs:331.
minimal failing input: vd = 0, vs2 = 0, vs1 = 0, extra = Imm(
    0,
)
	successes: 0
	local rejects: 0
	global rejects: 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/encode_v_crypto_vv_pbt.rs
