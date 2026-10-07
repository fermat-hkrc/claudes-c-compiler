# Bug: encode_v_crypto_vs ignores extra operands
**Law:** ∀ vd, vs2 ∈ {0..31}, extra ∈ Operand. encode_v_crypto_vs([v{vd}, v{vs2}, extra], 0b101001) is Err
**Impact:** A third (or later) operand is not rejected. Callers that pass a stray token get a valid-looking 32-bit vsm4r.vs encoding for the first two operands instead of an assembler error.
**Function:** encode_v_crypto_vs
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/vector.rs:211
**Detected by:** Negative/Error Contract — extra operand
**Minimal input:** encode_v_crypto_vs([Reg("v0"), Reg("v0"), Imm(0)], 0b101001)
**Expected:** Err (complete vd, vs2 form already present)
**Actual:** Ok(Word(0xa6082077)) — same encoding as `vsm4r.vs v0, v0`
**Severity:** medium
**Root cause:** vector.rs:212-215 encode_v_crypto_vs reads only operands 0..1 via get_vreg and never checks operands.len() == 2, so extra tokens are ignored.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/vector.rs:212`
```rust
    let vd = get_vreg(operands, 0)?;
    let vs2 = get_vreg(operands, 1)?;
    let word = (funct6 << 26) | (1u32 << 25) | (vs2 << 20) | (0b10000u32 << 15) | (0b010 << 12) | (vd << 7) | OP_V_CRYPTO;
    Ok(EncodeResult::Word(word))
```
**Suggested fix:** Reject any operand list that is not exactly two operands (vd, vs2).
```rust
    if operands.len() != 2 {
        return Err(format!("vsm4r.vs expects 2 operands, got {}", operands.len()));
    }
    let vd = get_vreg(operands, 0)?;
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_v_crypto_vs_regression_extra_operand -- --test-threads=1
```
**Raw output:**
```text
thread 'backend::riscv::assembler::encoder::encode_v_crypto_vs_pbt::encode_v_crypto_vs_neg_extra' (2893424) panicked at src/backend/riscv/assembler/encoder/encode_v_crypto_vs_pbt.rs:176:1:
Test failed: extra operand Imm(0) must Err for crypto VS; got Ok(Word(2785550455)) at src/backend/riscv/assembler/encoder/encode_v_crypto_vs_pbt.rs:313.
minimal failing input: vd = 0, vs2 = 0, extra = Imm(
    0,
)
	successes: 0
	local rejects: 0
	global rejects: 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/encode_v_crypto_vs_pbt.rs
