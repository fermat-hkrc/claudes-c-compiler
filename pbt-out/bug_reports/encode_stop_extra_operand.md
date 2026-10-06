# Bug: encode_stop ignores a third operand
**Law:** STADD/STCLR/STEOR/STSET take exactly two operands; a third operand must be rejected
**Impact:** `stadd w0, [x0], x2` is assembled as `stadd w0, [x0]`. llvm-mc/gas report "invalid operand for instruction". Callers that pass a trailing operand get a silent wrong encoding instead of an assembler error
**Function:** encode_stop
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/load_store.rs:927
**Detected by:** Negative/Error Contract
**Minimal input:** encode_stop("stadd", [Reg("w0"), Mem{base:"x0", offset:0}, Reg("x2")])
**Expected:** Err (llvm-mc: "invalid operand for instruction")
**Actual:** Ok(Word) — the extra operand is ignored
**Severity:** medium
**Root cause:** load_store.rs:928 checks `operands.len() < 2` and never rejects `len() > 2`
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/load_store.rs:928`
```rust
    if operands.len() < 2 {
        return Err(format!("{} requires 2 operands", mnemonic));
    }
```
**Suggested fix:** Require exactly two operands
```rust
    if operands.len() != 2 {
        return Err(format!("{} requires 2 operands", mnemonic));
    }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_stop_regression_extra_operand -- --test-threads=1
```
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_stop_pbt::test_encode_stop_regression_extra_operand' (2610720) panicked at src/backend/arm/assembler/encoder/encode_stop_pbt.rs:657:5:
stadd w0, [x0], x2 must Err; llvm-mc/gas reject a 3rd operand
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_stop_pbt.rs
