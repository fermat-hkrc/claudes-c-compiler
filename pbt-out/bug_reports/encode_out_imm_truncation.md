# Bug: encode_out silently truncates out-of-range port immediate to u8
**Law:** OUT imm-port form takes an imm8 port; values outside a valid imm8 must be rejected, not truncated
**Impact:** `outb %al, $256` encodes as `E6 00` (port 0) instead of an error, so I/O goes to the wrong port with no diagnostic
**Function:** encode_out
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/system.rs:68
**Detected by:** Negative/Error Contract + Differential (llvm-mc)
**Minimal input:** `outb` with operands `%al, $256`
**Expected:** `Err` (imm not imm8), matching llvm-mc
**Actual:** `Ok([0xE6, 0x00])`
**Severity:** high
**Root cause:** `system.rs:68` does `self.bytes.push(*val as u8)` with no range check, truncating any i64 to the low 8 bits
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/system.rs:64`
```rust
            (Operand::Register(_src), Operand::Immediate(ImmediateValue::Integer(val))) => {
                if size == 2 { self.bytes.push(0x66); }
                self.bytes.push(if size == 1 { 0xE6 } else { 0xE7 });
                self.bytes.push(*val as u8);
                Ok(())
            }
```
**Suggested fix:** Accept only values in 0..=255 (and optionally signed bytes that fit imm8 the same way llvm-mc does); reject others
```rust
            (Operand::Register(src), Operand::Immediate(ImmediateValue::Integer(val))) => {
                let expect = match size { 1 => "al", 2 => "ax", 4 => "eax", _ => unreachable!() };
                if src.name != expect {
                    return Err(format!("unsupported {} operands", mnemonic));
                }
                if *val < i64::from(i8::MIN) || *val > 255 {
                    return Err(format!("{} port immediate out of imm8 range", mnemonic));
                }
                if size == 2 { self.bytes.push(0x66); }
                self.bytes.push(if size == 1 { 0xE6 } else { 0xE7 });
                self.bytes.push(*val as u8);
                Ok(())
            }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_out_regression_imm_256_truncated -- --test-threads=1
cargo test --lib encode_out_neg_imm_out_of_range -- --test-threads=1
```
**Raw output:**
```text
Test failed: SUT silently encoded out-of-range port `outb %al, $256` → [e6, 00] (truncated?); llvm-mc rejected: llvm-mc error: <stdin>:1:11: error: invalid operand for instruction
outb %al, $256
          ^~~~
.
minimal failing input: mnemonic = "outb", imm_v = 256
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/encode_out_pbt.rs::test_encode_out_regression_imm_256_truncated
