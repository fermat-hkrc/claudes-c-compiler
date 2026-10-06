# Bug: encode_mov ignores extra operands
**Law:** A third (or later) operand on `mov` must be rejected; GNU as reports "unexpected characters following instruction"
**Impact:** Trailing junk after a valid two-operand MOV is assembled as if it were not there, so a mistyped extra register is silently dropped
**Function:** encode_mov
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/data_processing.rs:7
**Detected by:** Negative/error contract — gas extra-operand rejection
**Minimal input:** encode_mov([Reg("x0"), Reg("x0"), Reg("x0")])
**Expected:** Err
**Actual:** Ok(Word(0xaa0003e0)) — `mov x0, x0`
**Severity:** medium
**Root cause:** data_processing.rs:7 only rejects `operands.len() < 2`; there is no upper bound, so extra operands are ignored
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/data_processing.rs:7`
```rust
    if operands.len() < 2 {
        return Err("mov requires 2 operands".to_string());
    }
```
**Suggested fix:** Reject `operands.len() != 2` (shifts are not a valid MOV form).
```rust
    if operands.len() != 2 {
        return Err("mov requires 2 operands".to_string());
    }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_mov_regression_extra_operand -- --test-threads=1
```
**Raw output:**
```text
extra operand must Err, got Ok(Word(2852127712))
minimal failing input: rd = 0, rm = 0, extra = Reg("x0")
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_mov_pbt.rs
