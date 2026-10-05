# Bug: encode_neon_logical ignores a fourth operand
**Law:** Vector AND/ORR/EOR take exactly three arranged registers; a fourth operand must be rejected
**Impact:** The assembler silently drops extra operands and emits a well-formed AND/ORR/EOR, so a typo such as `and v0.8b, v1.8b, v2.8b, v3.8b` assembles instead of failing like gas/llvm-mc
**Function:** encode_neon_logical
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:297
**Detected by:** Negative/Error Contract
**Minimal input:** encode_neon_logical([v0.8b, v0.8b, v0.8b, v0.8b], opc=0)
**Expected:** Err (llvm-mc/gas: invalid operand for a fourth register)
**Actual:** Ok(EncodeResult::Word) of `and v0.8b, v0.8b, v0.8b`
**Severity:** medium
**Root cause:** neon.rs:297-318 never checks `operands.len()`; it reads indices 0..2 via get_neon_reg and returns Word, so any extra operands are ignored
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:298`
```rust
    let (rd, arr_d) = get_neon_reg(operands, 0)?;
```
**Suggested fix:** Reject anything other than exactly three operands before packing the word
```rust
if operands.len() != 3 {
    return Err("NEON logical requires 3 operands".to_string());
}
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_logical_regression_extra_operand -- --test-threads=1
```
**Raw output:**
```text
Test failed: extra operand must Err (llvm-mc rejects and v0.8b, v0.8b, v0.8b, v0.8b) at src/backend/arm/assembler/encoder/encode_neon_logical_pbt.rs:373.
minimal failing input: rd = 0, rn = 0, rm = 0, extra = 0, t = "8b", opc = 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_neon_logical_pbt.rs
