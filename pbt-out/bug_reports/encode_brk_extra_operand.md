# Bug: encode_brk ignores extra operands
**Law:** If llvm-mc / GNU as reject `brk #imm, extra`, then encode_brk([Imm(imm), extra]) must return Err
**Impact:** Typos such as `brk #0, x0` assemble as a silent `brk #0` instead of being rejected. An extra operand that should have been a parse/encode error is dropped, so a second intended argument never reaches the encoding. ARM codegen emits `brk #0` as the trap instruction; a mistyped extra operand would still produce a trap with the wrong awareness that the line was malformed.
**Function:** encode_brk
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/system.rs:471
**Detected by:** Negative/Error Contract (extra operand; llvm-mc rejects arity > 1)
**Minimal input:** encode_brk(&[Operand::Imm(0), Operand::Reg("x0".into())])  (assembly: `brk #0, x0`)
**Expected:** Err (llvm-mc: "invalid operand for instruction")
**Actual:** Ok(Word(0xd4200000))  // encodes as brk #0
**Severity:** medium
**Root cause:** system.rs:472 reads only operand 0 via get_imm; `operands.len()` is never checked, so trailing operands are ignored.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/system.rs:472`
```rust
    let imm = get_imm(operands, 0)?;
```
**Suggested fix:** Reject a slice longer than one operand before encoding.
```rust
    if operands.len() != 1 {
        return Err("brk: expected a single immediate".to_string());
    }
    let imm = get_imm(operands, 0)?;
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_brk_regression_extra_x0 -- --test-threads=1 --nocapture
```
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_brk_pbt::encode_brk_neg_extra' (2547057) panicked at src/backend/arm/assembler/encoder/encode_brk_pbt.rs:218:1:
Test failed: extra operand must Err (llvm-mc rejects brk #0, x0) at src/backend/arm/assembler/encoder/encode_brk_pbt.rs:270.
minimal failing input: imm = 0, extra = Reg(
    "x0",
)
	successes: 0
	local rejects: 0
	global rejects: 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_brk_pbt.rs
