# Bug: encode_push16 rejects r16 register operands (PUSH r16)
**Law:** ∀ r ∈ {ax,cx,dx,bx,sp,bp,si,di}. encode_push16([Reg(r)]) must equal llvm-mc `pushw %r` = `[0x66, 0x50+reg_num(r)]`.
**Impact:** AT&T `pushw %ax` (and any 16-bit GP push) fails to assemble; 16-bit stack code and OS/boot stubs that use operand-size override cannot be emitted.
**Function:** encode_push16
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/gp_integer.rs:382
**Detected by:** Differential — llvm-mc i686 (encode_push16_diff_r16)
**Minimal input:** `pushw %ax` → Operand::Register("ax")
**Expected:** `[0x66, 0x50]`
**Actual:** `Err("unsupported pushw operand")`
**Severity:** high
**Root cause:** `/home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/gp_integer.rs:389-400` — match only handles `Immediate(Integer)`; Register falls through to the catch-all Err. Sibling `encode_push` emits `0x50+n` for r32 without 0x66; pushw needs the same short form with a leading operand-size override.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/gp_integer.rs:389`
```rust
            Operand::Immediate(ImmediateValue::Integer(val)) => {
                self.bytes.push(0x66);
                if *val >= -128 && *val <= 127 {
                    self.bytes.push(0x6A);
                    self.bytes.push(*val as u8);
                } else {
                    self.bytes.push(0x68);
                    self.bytes.extend_from_slice(&(*val as i16).to_le_bytes());
                }
                Ok(())
            }
            _ => Err("unsupported pushw operand".to_string()),
```
**Suggested fix:** Add a Register arm gated on `reg_size == 2` (r16 only), emit `0x66` then `0x50 + reg_num`:
```rust
            Operand::Register(reg) => {
                if reg_size(&reg.name) != 2 {
                    return Err(format!("pushw requires r16 register, got {}", reg.name));
                }
                let num = reg_num(&reg.name).ok_or("bad register")?;
                self.bytes.push(0x66);
                self.bytes.push(0x50 + num);
                Ok(())
            }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_push16_diff_r16 -- --test-threads=1
cargo test --lib test_encode_push16_regression_r16_unsupported -- --test-threads=1
```
**Raw output:**
```text
Test failed: SUT rejected valid r16 form `pushw %ax`: unsupported pushw operand; Intel PUSH r16 / AT&T pushw %r16 must encode as 0x66 0x50+rw (sibling encode_push handles r32; encode_pop16 handles r16)..
minimal failing input: r16 = "ax"
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/encode_push16_pbt.rs::test_encode_push16_regression_r16_unsupported
