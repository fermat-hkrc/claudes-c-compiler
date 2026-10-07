# Bug: encode_v_crypto_vv ignores trailing v0.t on a non-maskable instruction
**Law:** ∀ vd, vs2, vs1 ∈ {0..31}. encode_v_crypto_vv([v{vd}, v{vs2}, v{vs1}, Symbol("v0.t")], 0b100000) is Err
**Impact:** Zvksh VV vsm3me.vv is not maskable (vm is required 1). A trailing v0.t is ignored and the encoder still emits vm=1, so `vsm3me.vv v0, v0, v0, v0.t` assembles as unmasked `vsm3me.vv v0, v0, v0` instead of being rejected.
**Function:** encode_v_crypto_vv
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/vector.rs:201
**Detected by:** Negative/Error Contract — trailing v0.t
**Minimal input:** encode_v_crypto_vv([Reg("v0"), Reg("v0"), Reg("v0"), Symbol("v0.t")], 0b100000)
**Expected:** Err (vsm3me.vv is not maskable)
**Actual:** Ok(Word(0x82002077)) — same encoding as unmasked `vsm3me.vv v0, v0, v0`
**Severity:** medium
**Root cause:** vector.rs:202-206 never inspects operands past index 2, and vm is hardcoded to 1. A trailing v0.t is treated as an ignored extra token.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/vector.rs:202`
```rust
    let vd = get_vreg(operands, 0)?;
    let vs2 = get_vreg(operands, 1)?;
    let vs1 = get_vreg(operands, 2)?;
    let word = (funct6 << 26) | (1u32 << 25) | (vs2 << 20) | (vs1 << 15) | (0b010 << 12) | (vd << 7) | OP_V_CRYPTO;
    Ok(EncodeResult::Word(word))
```
**Suggested fix:** Reject a fourth v0.t (and any other extra operand). These instructions are not maskable.
```rust
    if operands.len() != 3 {
        return Err(format!("vsm3me.vv expects 3 operands (not maskable), got {}", operands.len()));
    }
    let vd = get_vreg(operands, 0)?;
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_v_crypto_vv_regression_mask_v0t -- --test-threads=1
```
**Raw output:**
```text
thread 'backend::riscv::assembler::encoder::encode_v_crypto_vv_pbt::encode_v_crypto_vv_neg_mask_v0t' (2892589) panicked at src/backend/riscv/assembler/encoder/encode_v_crypto_vv_pbt.rs:176:1:
Test failed: trailing v0.t must Err (Zvksh VV is not maskable); got Ok(Word(2181046391)) at src/backend/riscv/assembler/encoder/encode_v_crypto_vv_pbt.rs:347.
minimal failing input: vd = 0, vs2 = 0, vs1 = 0
	successes: 0
	local rejects: 0
	global rejects: 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/encode_v_crypto_vv_pbt.rs
