# Bug: encode_ldop ignores a fourth operand
**Law:** LDADD/LDCLR/LDEOR/LDSET take exactly three operands; a fourth operand must be rejected
**Impact:** `ldadd w0, w0, [x0], x2` is assembled as `ldadd w0, w0, [x0]`. llvm-mc/gas report "invalid operand for instruction". Callers that pass a trailing operand get a silent wrong encoding instead of an assembler error
**Function:** encode_ldop
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/load_store.rs:882
**Detected by:** Negative/Error Contract
**Minimal input:** encode_ldop("ldadd", [Reg("w0"), Reg("w0"), Mem{base:"x0", offset:0}, Reg("x2")])
**Expected:** Err (llvm-mc: "invalid operand for instruction")
**Actual:** Ok(Word) — the extra operand is ignored
**Severity:** medium
**Root cause:** load_store.rs:883 checks `operands.len() < 3` and never rejects `len() > 3`
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/load_store.rs:883`
```rust
    if operands.len() < 3 {
        return Err(format!("{} requires 3 operands", mnemonic));
    }
```
**Suggested fix:** Require exactly three operands
```rust
    if operands.len() != 3 {
        return Err(format!("{} requires 3 operands", mnemonic));
    }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_ldop_regression_extra_operand -- --test-threads=1
```
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_ldop_pbt::test_encode_ldop_regression_extra_operand' panicked at src/backend/arm/assembler/encoder/encode_ldop_pbt.rs:830:5:
ldadd w0, w0, [x0], x2 must Err; llvm-mc/gas reject a 4th operand
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_ldop_pbt.rs
