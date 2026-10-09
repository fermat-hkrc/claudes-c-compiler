# Bug: encode_in silently truncates out-of-range port immediates

**Law:** The IN imm8-port form accepts only an 8-bit port immediate; values outside the imm8 domain must be rejected, not truncated to a low byte.
**Impact:** Assembler input such as `inb $256, %al` encodes as `E4 00` (port 0) instead of failing. Generated or hand-written code that intends a wider port constant silently targets the wrong I/O port.
**Function:** encode_in
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/system.rs:102
**Detected by:** Negative error + differential vs llvm-mc (encode_in_neg_imm_out_of_range)
**Minimal input:** `inb $256, %al` (ops = [Immediate(256), Register("al")])
**Expected:** `Err(...)` (llvm-mc: invalid operand for instruction)
**Actual:** `Ok([0xe4, 0x00])`
**Severity:** high
**Root cause:** system.rs:102 does `self.bytes.push(*val as u8)`, which wraps any i64 into a byte without a range check.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/system.rs:99`
```rust
(Operand::Immediate(ImmediateValue::Integer(val)), Operand::Register(_dst)) => {
    if size == 2 { self.bytes.push(0x66); }
    self.bytes.push(if size == 1 { 0xE4 } else { 0xE5 });
    self.bytes.push(*val as u8);
    Ok(())
}
```
**Suggested fix:** Reject values outside the signed/unsigned imm8 range accepted by gas/llvm-mc (e.g. not in -128..=255, or not in 0..=255 per chosen convention) before emitting.
```rust
(Operand::Immediate(ImmediateValue::Integer(val)), Operand::Register(dst)) => {
    if *val < -128 || *val > 255 {
        return Err(format!("IN port immediate out of imm8 range: {val}"));
    }
    // also validate dst is the canonical data reg
    if size == 2 { self.bytes.push(0x66); }
    self.bytes.push(if size == 1 { 0xE4 } else { 0xE5 });
    self.bytes.push(*val as u8);
    Ok(())
}
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_in_pbt::test_encode_in_regression_imm_256_truncated -- --test-threads=1
cargo test --lib encode_in_pbt::encode_in_neg_imm_out_of_range -- --test-threads=1
```
**Raw output:**
```text
Test failed: SUT silently encoded out-of-range port `inb $256, %al` → [e4, 00] (truncated?); llvm-mc rejected: llvm-mc error: <stdin>:1:5: error: invalid operand for instruction
inb $256, %al
    ^~~~
.
minimal failing input: mnemonic = "inb", imm_v = 256
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/encode_in_pbt.rs (test_encode_in_regression_imm_256_truncated)
