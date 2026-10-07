# Bug: encode_v_crypto_vs ignores trailing v0.t on a non-maskable instruction
**Law:** ∀ vd, vs2 ∈ {0..31}. encode_v_crypto_vs([v{vd}, v{vs2}, Symbol("v0.t")], 0b101001) is Err
**Impact:** Zvksed VS forms are not maskable (vm is required 1). A trailing v0.t is ignored and the encoder still emits vm=1, so `vsm4r.vs v0, v0, v0.t` assembles as unmasked `vsm4r.vs v0, v0` instead of being rejected.
**Function:** encode_v_crypto_vs
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/vector.rs:211
**Detected by:** Negative/Error Contract — trailing v0.t
**Minimal input:** encode_v_crypto_vs([Reg("v0"), Reg("v0"), Symbol("v0.t")], 0b101001)
**Expected:** Err (vsm4r.vs is not maskable)
**Actual:** Ok(Word(0xa6082077)) — same encoding as unmasked `vsm4r.vs v0, v0`
**Severity:** medium
**Root cause:** vector.rs:212-215 never inspects operands past index 1, and vm is hardcoded to 1. A trailing v0.t is treated as an ignored extra token.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/vector.rs:212`
```rust
    let vd = get_vreg(operands, 0)?;
    let vs2 = get_vreg(operands, 1)?;
    let word = (funct6 << 26) | (1u32 << 25) | (vs2 << 20) | (0b10000u32 << 15) | (0b010 << 12) | (vd << 7) | OP_V_CRYPTO;
    Ok(EncodeResult::Word(word))
```
**Suggested fix:** Reject a third v0.t (and any other extra operand). These instructions are not maskable.
```rust
    if operands.len() != 2 {
        return Err(format!("vsm4r.vs expects 2 operands (not maskable), got {}", operands.len()));
    }
    let vd = get_vreg(operands, 0)?;
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_v_crypto_vs_regression_mask_v0t -- --test-threads=1
```
**Raw output:**
```text
thread 'backend::riscv::assembler::encoder::encode_v_crypto_vs_pbt::encode_v_crypto_vs_neg_mask_v0t' (2893425) panicked at src/backend/riscv/assembler/encoder/encode_v_crypto_vs_pbt.rs:176:1:
Test failed: trailing v0.t must Err (Zvksed VS is not maskable); got Ok(Word(2785550455)) at src/backend/riscv/assembler/encoder/encode_v_crypto_vs_pbt.rs:328.
minimal failing input: vd = 0, vs2 = 0
	successes: 0
	local rejects: 0
	global rejects: 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/encode_v_crypto_vs_pbt.rs
