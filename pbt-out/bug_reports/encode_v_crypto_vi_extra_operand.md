# Bug: encode_v_crypto_vi ignores extra operands
**Law:** ∀ vd, vs2 ∈ {0..31}, uimm ∈ {0..31}, extra ∈ Operand, funct6 ∈ {101011,100001}. encode_v_crypto_vi([v{vd}, v{vs2}, Imm(uimm), extra], funct6) is Err
**Impact:** A fourth (or later) operand is not rejected. Callers that pass a stray token get a valid-looking 32-bit vsm3c.vi / vsm4k.vi encoding for the first three operands instead of an assembler error.
**Function:** encode_v_crypto_vi
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/vector.rs:191
**Detected by:** Negative/Error Contract — extra operand
**Minimal input:** encode_v_crypto_vi([Reg("v0"), Reg("v0"), Imm(0), Imm(0)], 0b101011)
**Expected:** Err (complete vd, vs2, uimm5 form already present)
**Actual:** Ok(Word(0xae002077)) — same encoding as `vsm3c.vi v0, v0, 0`
**Severity:** medium
**Root cause:** vector.rs:192-196 encode_v_crypto_vi reads only operands 0..2 via get_vreg/get_imm and never checks operands.len() == 3, so extra tokens are ignored.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/vector.rs:192`
```rust
    let vd = get_vreg(operands, 0)?;
    let vs2 = get_vreg(operands, 1)?;
    let uimm5 = get_imm(operands, 2)? as u32 & 0x1F;
    let word = (funct6 << 26) | (1u32 << 25) | (vs2 << 20) | (uimm5 << 15) | (0b010 << 12) | (vd << 7) | OP_V_CRYPTO;
    Ok(EncodeResult::Word(word))
```
**Suggested fix:** Reject any operand list that is not exactly three operands (vd, vs2, uimm5).
```rust
    if operands.len() != 3 {
        return Err(format!("vsm3c.vi/vsm4k.vi expect 3 operands, got {}", operands.len()));
    }
    let vd = get_vreg(operands, 0)?;
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_v_crypto_vi_regression_extra_operand -- --test-threads=1
```
**Raw output:**
```text
thread 'backend::riscv::assembler::encoder::encode_v_crypto_vi_pbt::encode_v_crypto_vi_neg_extra' (2891631) panicked at src/backend/riscv/assembler/encoder/encode_v_crypto_vi_pbt.rs:229:1:
Test failed: extra operand Imm(0) must Err for crypto VI; got Ok(Word(2919243895)) at src/backend/riscv/assembler/encoder/encode_v_crypto_vi_pbt.rs:385.
minimal failing input: vd = 0, vs2 = 0, uimm = 0, extra = Imm(
    0,
), kind = 0
	successes: 0
	local rejects: 0
	global rejects: 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/encode_v_crypto_vi_pbt.rs
