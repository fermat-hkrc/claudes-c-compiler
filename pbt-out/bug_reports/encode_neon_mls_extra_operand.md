# Bug: encode_neon_mls ignores a fourth operand
**Law:** Vector MLS is a three-operand instruction; a fourth operand must be rejected
**Impact:** Invalid assembly `mls Vd.T, Vn.T, Vm.T, Vextra.T` is assembled as if the extra register were absent, so a typo or extra operand is silently dropped instead of failing the assemble
**Function:** encode_neon_mls
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:361
**Detected by:** Negative/Error Contract
**Minimal input:** encode_neon_mls([v0.8b, v0.8b, v0.8b, v0.8b])
**Expected:** Err (llvm-mc / gas reject a fourth operand)
**Actual:** Ok(Word(0x2e209400)) — same encoding as `mls v0.8b, v0.8b, v0.8b`
**Severity:** medium
**Root cause:** neon.rs:361-369 never checks `operands.len()`; get_neon_reg only reads indices 0..2, so extra operands are ignored and the function returns Ok
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:369`
```rust
    Ok(EncodeResult::Word(word))
```
**Suggested fix:** Reject any operand count other than 3
```rust
if operands.len() != 3 {
    return Err("mls requires 3 operands".to_string());
}
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_mls_regression_extra_operand -- --test-threads=1
```
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_neon_mls_pbt::encode_neon_mls_neg_extra_operand' panicked at src/backend/arm/assembler/encoder/encode_neon_mls_pbt.rs:303:1:
Test failed: 4 operands must Err (llvm-mc rejects mls v0.8b, v0.8b, v0.8b, v0.8b) at src/backend/arm/assembler/encoder/encode_neon_mls_pbt.rs:322.
minimal failing input: rd = 0, rn = 0, rm = 0, extra = 0, t = "8b"
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_neon_mls_pbt.rs
