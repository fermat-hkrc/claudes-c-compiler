# Bug: encode_bt truncates out-of-range bit-offset immediates with `as u8`
**Law:** The imm8 bit-offset form (`0F BA /ext ib`) accepts only values that fit in an 8-bit immediate as accepted by the assembler contract (llvm-mc rejects `$256` and values outside the imm8 domain). Values outside that domain must return `Err`, not silently truncate.
**Impact:** `btl $256, %eax` encodes as `btl $0, %eax` (`[0f,ba,e0,00]`). A wrong bit index is tested/set/reset/complemented with no diagnostic — silent wrong code generation for any OOR immediate.
**Function:** encode_bt
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/gp_integer.rs:1041
**Detected by:** Negative/error contract — llvm-mc rejects `$256`; SUT truncates
**Minimal input:** `btl $256, %eax` → SUT `Ok([0f,ba,e0,00])`; llvm-mc: `invalid operand for instruction`
**Expected:** `Err(...)`
**Actual:** `Ok([0f, ba, e0, 00])` because `256u8 as truncated == 0`
**Severity:** high
**Root cause:** All three Imm arms push `*val as u8` with no range check (`gp_integer.rs:1041`, `1048`, `1058`). Rust `as u8` truncates; llvm-mc requires a true imm8.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/gp_integer.rs:1038`
```rust
            (Operand::Immediate(ImmediateValue::Integer(val)), Operand::Register(dst)) => {
                let dst_num = reg_num(&dst.name).ok_or("bad register")?;
                self.bytes.extend_from_slice(&[0x0F, 0xBA]);
                self.bytes.push(self.modrm(3, ext, dst_num));
                self.bytes.push(*val as u8);
                Ok(())
            }
```
**Suggested fix:** Reject values outside the imm8 domain before casting (e.g. require `(-128..=255).contains(val)` or `0..=255` per assembler policy), then push `(val as u8)`.
```rust
            (Operand::Immediate(ImmediateValue::Integer(val)), Operand::Register(dst)) => {
                if !(-128..=255).contains(val) {
                    return Err(format!("{mnemonic} immediate out of imm8 range: {val}"));
                }
                let dst_num = reg_num(&dst.name).ok_or("bad register")?;
                self.bytes.extend_from_slice(&[0x0F, 0xBA]);
                self.bytes.push(self.modrm(3, ext, dst_num));
                self.bytes.push(*val as u8);
                Ok(())
            }
```
(Apply the same check on Imm→Mem and Imm→Label arms.)

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_bt_regression_imm_oor_256 -- --test-threads=1
cargo test --lib encode_bt_neg_imm_oor -- --test-threads=1
```
**Raw output:**
```text
SUT accepted btl $256, %eax → Ok([15, 186, 224, 0]); imm must fit imm8
SUT accepted OOR imm `btl $256, %eax` → [0f, ba, e0, 00]; llvm-mc rejects; `*val as u8` truncates (imm8 contract broken).
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/encode_bt_pbt.rs (`encode_bt_regression_imm_oor_256`)
