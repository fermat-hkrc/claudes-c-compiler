# Bug: encode_smc ignores extra operands
**Law:** If llvm-mc / GNU as reject `smc #imm, extra`, then encode_smc([Imm(imm), extra]) must return Err
**Impact:** Typos such as `smc #0, x0` assemble as a silent `smc #0` instead of being rejected. An extra operand that should have been a parse/encode error is dropped, so a second intended argument never reaches the encoding.
**Function:** encode_smc
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/system.rs:420
**Detected by:** Negative/Error Contract (extra operand; llvm-mc rejects arity > 1)
**Minimal input:** encode_smc(&[Operand::Imm(0), Operand::Reg("x0".into())])  (assembly: `smc #0, x0`)
**Expected:** Err (llvm-mc: "invalid operand for instruction")
**Actual:** Ok(Word(0xd4000003))  // encodes as smc #0
**Severity:** medium
**Root cause:** system.rs:421 reads only operand 0 via get_imm; `operands.len()` is never checked, so trailing operands are ignored.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/system.rs:421`
```rust
    let imm = get_imm(operands, 0)?;
```
**Suggested fix:** Reject a slice longer than one operand before encoding.
```rust
    if operands.len() != 1 {
        return Err("smc: expected a single immediate".to_string());
    }
    let imm = get_imm(operands, 0)?;
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_smc_regression_extra_x0 -- --test-threads=1 --nocapture
```
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_smc_pbt::encode_smc_neg_extra' (2542879) panicked at src/backend/arm/assembler/encoder/encode_smc_pbt.rs:217:1:
Test failed: extra operand must Err (llvm-mc rejects smc #0, x0) at src/backend/arm/assembler/encoder/encode_smc_pbt.rs:269.
minimal failing input: imm = 0, extra = Reg(
    "x0",
)
	successes: 0
	local rejects: 0
	global rejects: 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_smc_pbt.rs
