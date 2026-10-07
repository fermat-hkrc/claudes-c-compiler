# Bug: encode_v_crypto_vi ignores trailing v0.t on a non-maskable instruction
**Law:** ∀ vd, vs2 ∈ {0..31}, uimm ∈ {0..31}, funct6 ∈ {101011,100001}. encode_v_crypto_vi([v{vd}, v{vs2}, Imm(uimm), Symbol("v0.t")], funct6) is Err
**Impact:** Zvksh/Zvksed VI forms are not maskable (vm is required 1). A trailing v0.t is ignored and the encoder still emits vm=1, so `vsm3c.vi v0, v0, 0, v0.t` assembles as unmasked `vsm3c.vi v0, v0, 0` instead of being rejected.
**Function:** encode_v_crypto_vi
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/vector.rs:191
**Detected by:** Negative/Error Contract — trailing v0.t
**Minimal input:** encode_v_crypto_vi([Reg("v0"), Reg("v0"), Imm(0), Symbol("v0.t")], 0b101011)
**Expected:** Err (vsm3c.vi / vsm4k.vi are not maskable)
**Actual:** Ok(Word(0xae002077)) — same encoding as unmasked `vsm3c.vi v0, v0, 0`
**Severity:** medium
**Root cause:** vector.rs:192-196 never inspects operands past index 2, and vm is hardcoded to 1. A trailing v0.t is treated as an ignored extra token.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/vector.rs:192`
```rust
    let vd = get_vreg(operands, 0)?;
    let vs2 = get_vreg(operands, 1)?;
    let uimm5 = get_imm(operands, 2)? as u32 & 0x1F;
    let word = (funct6 << 26) | (1u32 << 25) | (vs2 << 20) | (uimm5 << 15) | (0b010 << 12) | (vd << 7) | OP_V_CRYPTO;
    Ok(EncodeResult::Word(word))
```
**Suggested fix:** Reject a fourth v0.t (and any other extra operand). These instructions are not maskable.
```rust
    if operands.len() != 3 {
        return Err(format!("vsm3c.vi/vsm4k.vi expect 3 operands (not maskable), got {}", operands.len()));
    }
    let vd = get_vreg(operands, 0)?;
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_v_crypto_vi_regression_mask_v0t -- --test-threads=1
```
**Raw output:**
```text
thread 'backend::riscv::assembler::encoder::encode_v_crypto_vi_pbt::encode_v_crypto_vi_neg_mask_v0t' (2891632) panicked at src/backend/riscv/assembler/encoder/encode_v_crypto_vi_pbt.rs:229:1:
Test failed: trailing v0.t must Err (Zvksh/Zvksed VI is not maskable); got Ok(Word(2919243895)) at src/backend/riscv/assembler/encoder/encode_v_crypto_vi_pbt.rs:419.
minimal failing input: vd = 0, vs2 = 0, uimm = 0, kind = 0
	successes: 0
	local rejects: 0
	global rejects: 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/encode_v_crypto_vi_pbt.rs
