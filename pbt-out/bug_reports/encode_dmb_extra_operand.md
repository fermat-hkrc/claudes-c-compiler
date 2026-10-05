# Bug: encode_dmb ignores extra operands
**Law:** If llvm-mc / GNU as reject `dmb <option>, <extra>`, then encode_dmb([Barrier(option), extra]) must return Err
**Impact:** Typos such as `dmb sy, x0` assemble as a silent `dmb sy` instead of being rejected. An extra operand that should have been a parse/encode error is dropped, so the object file contains a full-system barrier the author did not uniquely specify.
**Function:** encode_dmb
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/system.rs:6
**Detected by:** Negative/Error Contract (extra operand; llvm-mc and GNU as reject)
**Minimal input:** encode_dmb(&[Operand::Barrier("sy".into()), Operand::Reg("x0".into())])  (assembly: `dmb sy, x0`)
**Expected:** Err (GNU as: "unexpected characters following instruction"; llvm-mc: "invalid operand")
**Actual:** Ok(Word(0xd5033fbf))  // encodes as dmb sy
**Severity:** medium
**Root cause:** system.rs:7 `operands.first()` — only the first operand is examined; `operands.len()` is never checked, so trailing operands are ignored.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/system.rs:7`
```rust
    let option = match operands.first() {
```
**Suggested fix:** Reject a slice longer than one operand before decoding the option.
```rust
    if operands.len() > 1 {
        return Err("dmb: extra operand".to_string());
    }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_dmb_regression_extra_sy_x0 -- --test-threads=1 --nocapture
```
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_dmb_pbt::encode_dmb_neg_extra' panicked at src/backend/arm/assembler/encoder/encode_dmb_pbt.rs:264:1:
Test failed: extra operand must Err (llvm-mc rejects dmb sy, x0) at src/backend/arm/assembler/encoder/encode_dmb_pbt.rs:368.
minimal failing input: name = "sy", extra = Reg("x0")
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_dmb_pbt.rs
