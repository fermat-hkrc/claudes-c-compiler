# Bug: encode_dsb ignores extra operands
**Law:** If llvm-mc / GNU as reject `dsb <option>, <extra>`, then encode_dsb([Barrier(option), extra]) must return Err
**Impact:** Typos such as `dsb sy, x0` assemble as a silent `dsb sy` instead of being rejected. An extra operand that should have been a parse/encode error is dropped, so the object file contains a full-system barrier the author did not uniquely specify.
**Function:** encode_dsb
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/system.rs:30
**Detected by:** Negative/Error Contract (extra operand; llvm-mc and GNU as reject)
**Minimal input:** encode_dsb(&[Operand::Barrier("sy".into()), Operand::Reg("x0".into())])  (assembly: `dsb sy, x0`)
**Expected:** Err (GNU as: "unexpected characters following instruction"; llvm-mc: "invalid operand")
**Actual:** Ok(Word(0xd5033f9f))  // encodes as dsb sy
**Severity:** medium
**Root cause:** system.rs:31 `operands.first()` — only the first operand is examined; `operands.len()` is never checked, so trailing operands are ignored.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/system.rs:31`
```rust
    let option = match operands.first() {
```
**Suggested fix:** Reject a slice longer than one operand before decoding the option.
```rust
    if operands.len() > 1 {
        return Err("dsb: extra operand".to_string());
    }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_dsb_regression_extra_sy_x0 -- --test-threads=1 --nocapture
```
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_dsb_pbt::encode_dsb_neg_extra' panicked at src/backend/arm/assembler/encoder/encode_dsb_pbt.rs:264:1:
Test failed: extra operand must Err (llvm-mc rejects dsb sy, x0) at src/backend/arm/assembler/encoder/encode_dsb_pbt.rs:368.
minimal failing input: name = "sy", extra = Reg("x0")
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_dsb_pbt.rs
