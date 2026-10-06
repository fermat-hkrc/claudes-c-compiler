# Bug: encode_swp ignores a nonzero memory offset
**Law:** SWP optional offset may only be #0; any nonzero offset must be rejected
**Impact:** `swp w0, w0, [x0, #-1]` is assembled as `swp w0, w0, [x0]`. llvm-mc/gas report "invalid operand for instruction". The offset is dropped, so the encoded address is not the one written
**Function:** encode_swp
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/load_store.rs:849
**Detected by:** Negative/Error Contract
**Minimal input:** encode_swp("swp", [Reg("w0"), Reg("w0"), Mem{base:"x0", offset:-1}])
**Expected:** Err (llvm-mc: "invalid operand for instruction")
**Actual:** Ok(Word) — offset is ignored; encodes as [x0]
**Severity:** medium
**Root cause:** load_store.rs:856 matches `Operand::Mem { base, .. }` and discards the offset field
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/load_store.rs:856`
```rust
        Some(Operand::Mem { base, .. }) => parse_reg_num(base).ok_or("swp: invalid base")?,
```
**Suggested fix:** Require offset == 0 on the Mem operand
```rust
        Some(Operand::Mem { base, offset }) if *offset == 0 => {
            parse_reg_num(base).ok_or("swp: invalid base")?
        }
        Some(Operand::Mem { .. }) => return Err("swp: optional offset can only be 0".to_string()),
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_swp_regression_nonzero_offset -- --test-threads=1
```
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_swp_pbt::test_encode_swp_regression_nonzero_offset' panicked at src/backend/arm/assembler/encoder/encode_swp_pbt.rs:863:5:
swp w0, w0, [x0, #-1] must Err; gas: optional immediate offset can only be 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_swp_pbt.rs
