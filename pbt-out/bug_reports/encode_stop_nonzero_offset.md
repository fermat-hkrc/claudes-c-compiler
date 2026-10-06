# Bug: encode_stop ignores a nonzero Mem offset
**Law:** STADD memory operand is [Xn|SP] or [Xn|SP, #0]; a nonzero offset must be rejected
**Impact:** `stadd w0, [x0, #-1]` is assembled as `stadd w0, [x0]`. llvm-mc/gas report "invalid operand for instruction". An offset the programmer wrote is silently dropped
**Function:** encode_stop
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/load_store.rs:927
**Detected by:** Negative/Error Contract
**Minimal input:** encode_stop("stadd", [Reg("w0"), Mem{base:"x0", offset:-1}])
**Expected:** Err (llvm-mc: "invalid operand for instruction")
**Actual:** Ok(Word) — the offset is ignored
**Severity:** medium
**Root cause:** load_store.rs:933 matches `Operand::Mem { base, .. }` and never reads `offset`
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/load_store.rs:933`
```rust
        Some(Operand::Mem { base, .. }) => parse_reg_num(base).ok_or_else(|| format!("{}: invalid base", mnemonic))?,
```
**Suggested fix:** Reject a nonzero offset
```rust
        Some(Operand::Mem { base, offset }) if *offset == 0 => parse_reg_num(base).ok_or_else(|| format!("{}: invalid base", mnemonic))?,
        Some(Operand::Mem { offset, .. }) => return Err(format!("{}: offset must be #0, got {}", mnemonic, offset)),
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_stop_regression_nonzero_offset -- --test-threads=1
```
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_stop_pbt::test_encode_stop_regression_nonzero_offset' (2610722) panicked at src/backend/arm/assembler/encoder/encode_stop_pbt.rs:702:5:
stadd w0, [x0, #-1] must Err; gas: optional immediate offset can only be 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_stop_pbt.rs
