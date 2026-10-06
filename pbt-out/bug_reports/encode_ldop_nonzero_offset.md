# Bug: encode_ldop ignores a nonzero Mem offset
**Law:** LDADD memory operand is [Xn|SP] with optional immediate offset only #0
**Impact:** `ldadd w0, w0, [x0, #-1]` is assembled as `ldadd w0, w0, [x0]`. llvm-mc/gas report "invalid operand for instruction". A nonzero offset is silently dropped
**Function:** encode_ldop
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/load_store.rs:882
**Detected by:** Negative/Error Contract
**Minimal input:** encode_ldop("ldadd", [Reg("w0"), Reg("w0"), Mem{base:"x0", offset:-1}])
**Expected:** Err (llvm-mc: "invalid operand for instruction")
**Actual:** Ok(Word) — offset is ignored
**Severity:** medium
**Root cause:** load_store.rs:888-889 matches `Operand::Mem { base, .. }` and never inspects offset
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/load_store.rs:888`
```rust
    let rn = match operands.get(2) {
        Some(Operand::Mem { base, .. }) => parse_reg_num(base).ok_or("ldop: invalid base")?,
        _ => return Err(format!("{} requires memory operand [Xn]", mnemonic)),
    };
```
**Suggested fix:** Reject a nonzero offset
```rust
    let rn = match operands.get(2) {
        Some(Operand::Mem { base, offset }) if *offset == 0 => {
            parse_reg_num(base).ok_or("ldop: invalid base")?
        }
        Some(Operand::Mem { .. }) => return Err(format!("{}: offset must be #0", mnemonic)),
        _ => return Err(format!("{} requires memory operand [Xn]", mnemonic)),
    };
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_ldop_regression_nonzero_offset -- --test-threads=1
```
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_ldop_pbt::test_encode_ldop_regression_nonzero_offset' panicked at src/backend/arm/assembler/encoder/encode_ldop_pbt.rs:942:5:
ldadd w0, w0, [x0, #-1] must Err; gas: optional immediate offset can only be 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_ldop_pbt.rs
