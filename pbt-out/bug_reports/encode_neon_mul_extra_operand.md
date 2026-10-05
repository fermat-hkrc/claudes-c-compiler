# Bug: encode_neon_mul ignores a fourth operand
**Law:** Vector MUL is a three-operand instruction; a fourth operand must be rejected
**Impact:** Invalid assembly `mul Vd.T, Vn.T, Vm.T, Vextra.T` is assembled as if the extra register were absent, so a typo or extra operand is silently dropped instead of failing the assemble
**Function:** encode_neon_mul
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:323
**Detected by:** Negative/Error Contract
**Minimal input:** encode_neon_mul([v0.8b, v0.8b, v0.8b, v0.8b])
**Expected:** Err (llvm-mc / gas reject a fourth operand)
**Actual:** Ok(Word(0x0e209c00)) — same encoding as `mul v0.8b, v0.8b, v0.8b`
**Severity:** medium
**Root cause:** neon.rs:323-332 never checks `operands.len()`; get_neon_reg only reads indices 0..2, so extra operands are ignored and the function returns Ok
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:323`
```rust
pub(crate) fn encode_neon_mul(operands: &[Operand]) -> Result<EncodeResult, String> {
    let (rd, arr_d) = get_neon_reg(operands, 0)?;
    let (rn, _) = get_neon_reg(operands, 1)?;
    let (rm, _) = get_neon_reg(operands, 2)?;
    let (q, size) = neon_arr_to_q_size(&arr_d)?;

    // MUL (vector): 0 Q 0 01110 size 1 Rm 10011 1 Rn Rd
    let word = (q << 30) | (0b001110 << 24) | (size << 22) | (1 << 21)
        | (rm << 16) | (0b100111 << 10) | (rn << 5) | rd;
    Ok(EncodeResult::Word(word))
}
```
**Suggested fix:** Reject any operand count other than 3
```rust
if operands.len() != 3 {
    return Err("mul requires 3 operands".to_string());
}
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_mul_regression_extra_operand -- --test-threads=1
```
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_neon_mul_pbt::encode_neon_mul_neg_extra_operand' panicked at src/backend/arm/assembler/encoder/encode_neon_mul_pbt.rs:300:1:
Test failed: 4 operands must Err (llvm-mc rejects mul v0.8b, v0.8b, v0.8b, v0.8b) at src/backend/arm/assembler/encoder/encode_neon_mul_pbt.rs:319.
minimal failing input: rd = 0, rn = 0, rm = 0, extra = 0, t = "8b"
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_neon_mul_pbt.rs
