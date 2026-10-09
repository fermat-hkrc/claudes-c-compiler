# Bug: encode_bsr_bsf_16 accepts r32/r8 via reg_num aliasing

**Law:** `bsfw`/`bsrw` encode 16-bit BSF/BSR (Intel SDM: r16, r/m16). Operand registers must be r16; r32 and r8 forms must be rejected (llvm-mc rejects them).
**Impact:** Assembler silently accepts `bsfw %eax, %bx` / `bsrw %ax, %al` and emits the same bytes as the r16 form, so a typo that should fail assembly produces a 16-bit bit-scan with the wrong architectural width — misassembled code with no diagnostic.
**Function:** encode_bsr_bsf_16
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/system.rs:353
**Detected by:** Negative/error contract — llvm-mc rejection + SUT must Err (encode_bsr_bsf_16_neg_wrong_width)
**Minimal input:** `bsfw %eax, %bx` (also `bsfw %ax, %eax`, `bsrw %ax, %al`)
**Expected:** `Err(...)` (llvm-mc: invalid operand for instruction)
**Actual:** `Ok([0x66, 0x0f, 0xbc, 0xd8])` — same as `bsfw %ax, %bx`
**Severity:** medium
**Root cause:** `/home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/system.rs:365-368` — uses `reg_num` which aliases `eax`/`al`→0, `ebx`/`bl`→3 with no `reg_size(...) == 2` gate on either register operand.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/system.rs:365`
```rust
            (Operand::Register(src), Operand::Register(dst)) => {
                let src_num = reg_num(&src.name).ok_or("bad register")?;
                let dst_num = reg_num(&dst.name).ok_or("bad register")?;
                self.bytes.extend_from_slice(&opcode);
                self.bytes.push(self.modrm(3, dst_num, src_num));
                Ok(())
            }
            (Operand::Memory(mem), Operand::Register(dst)) => {
                let dst_num = reg_num(&dst.name).ok_or("bad register")?;
                self.bytes.extend_from_slice(&opcode);
                self.encode_modrm_mem(dst_num, mem)
            }
```
**Suggested fix:** Require `reg_size == 2` on both register names (and on the destination of the memory form) before encoding.
```rust
            (Operand::Register(src), Operand::Register(dst)) => {
                if reg_size(&src.name) != 2 || reg_size(&dst.name) != 2 {
                    return Err(format!("unsupported {} operands", mnemonic));
                }
                let src_num = reg_num(&src.name).ok_or("bad register")?;
                let dst_num = reg_num(&dst.name).ok_or("bad register")?;
                self.bytes.push(0x66);
                self.bytes.extend_from_slice(&opcode);
                self.bytes.push(self.modrm(3, dst_num, src_num));
                Ok(())
            }
            (Operand::Memory(mem), Operand::Register(dst)) => {
                if reg_size(&dst.name) != 2 {
                    return Err(format!("unsupported {} operands", mnemonic));
                }
                let dst_num = reg_num(&dst.name).ok_or("bad register")?;
                self.emit_segment_prefix(mem);
                self.bytes.push(0x66);
                self.bytes.extend_from_slice(&opcode);
                self.encode_modrm_mem(dst_num, mem)
            }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_bsr_bsf_16_regression_r32_src_accepted -- --test-threads=1
cargo test --lib test_encode_bsr_bsf_16_regression_r8_dst_accepted -- --test-threads=1
```
**Raw output:**
```text
regression: bsfw %eax, %bx must be Err, got Ok([66, 0f, bc, d8])
regression: bsrw %ax, %al must be Err, got Ok([66, 0f, bd, c0])
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/encode_bsr_bsf_16_pbt.rs (test_encode_bsr_bsf_16_regression_r32_src_accepted, test_encode_bsr_bsf_16_regression_r8_dst_accepted)
