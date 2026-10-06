# Bug: encode_svc ignores extra operands
**Law:** If llvm-mc / GNU as reject `svc #imm, extra`, then encode_svc([Imm(imm), extra]) must return Err
**Impact:** Typos such as `svc #0, x0` assemble as a silent `svc #0` instead of being rejected. An extra operand that should have been a parse/encode error is dropped, so a second intended argument never reaches the encoding.
**Function:** encode_svc
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/system.rs:389
**Detected by:** Negative/Error Contract (extra operand; llvm-mc rejects arity > 1)
**Minimal input:** encode_svc(&[Operand::Imm(0), Operand::Reg("x0".into())])  (assembly: `svc #0, x0`)
**Expected:** Err (llvm-mc: "invalid operand for instruction")
**Actual:** Ok(Word(0xd4000001))  // encodes as svc #0
**Severity:** medium
**Root cause:** system.rs:390-391 read only operand 0 via get_imm; `operands.len()` is never checked, so trailing operands are ignored.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/system.rs:390`
```rust
    let imm = get_imm(operands, 0)?;
```
**Suggested fix:** Reject a slice longer than one operand before encoding.
```rust
    if operands.len() != 1 {
        return Err("svc: expected a single immediate".to_string());
    }
    let imm = get_imm(operands, 0)?;
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_svc_regression_extra_x0 -- --test-threads=1 --nocapture
```
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_svc_pbt::encode_svc_neg_extra' (2534251) panicked at src/backend/arm/assembler/encoder/encode_svc_pbt.rs:218:1:
Test failed: extra operand must Err (llvm-mc rejects svc #0, x0) at src/backend/arm/assembler/encoder/encode_svc_pbt.rs:270.
minimal failing input: imm = 0, extra = Reg(
    "x0",
)
	successes: 0
	local rejects: 0
	global rejects: 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_svc_pbt.rs
