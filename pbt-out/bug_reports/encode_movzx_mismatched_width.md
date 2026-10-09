# Bug: encode_movzx accepts mismatched register widths
**Law:** ∀ form ∈ {movzbl,movzbw,movzwl}, src/dst registers must match the mnemonic's source and destination sizes; invalid width pairs must be rejected (as llvm-mc / Intel SDM require)
**Impact:** Assembler silently accepts invalid AT&T such as `movzbl %ax, %eax` or `movzbl %eax, %ebx` and emits 0F B6 with reg_num of the wrong-width name, producing a GP encoding that does not match the written operands — silent wrong machine code for any frontend that passes through mismatched sizes
**Function:** encode_movzx
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/gp_integer.rs:302
**Detected by:** Negative/error contract — llvm-mc rejects; SUT Ok
**Minimal input:** `movzbl %ax, %eax` (also `movzbl %eax, %ebx` → Ok([0f, b6, d8]))
**Expected:** Err (size mismatch)
**Actual:** Ok([0x0f, 0xb6, 0xc0]) for movzbl %ax, %eax — treats ax as if it were al (same reg_num 0)
**Severity:** high
**Root cause:** encode_movzx uses only the mnemonic-derived src_size/dst_size for opcode selection and never checks `reg_size` of the actual register operands. reg_num maps ax and al to the same 3-bit code, so width mismatches encode as if the correct-width alias were used.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/gp_integer.rs:316`
```rust
            (Operand::Register(src), Operand::Register(dst)) => {
                let src_num = reg_num(&src.name).ok_or("bad src register")?;
                let dst_num = reg_num(&dst.name).ok_or("bad dst register")?;
                self.bytes.extend_from_slice(&opcode);
                self.bytes.push(self.modrm(3, dst_num, src_num));
            }
```
**Suggested fix:** Gate register operands on `reg_size` matching src_size/dst_size before encoding:
```rust
            (Operand::Register(src), Operand::Register(dst)) => {
                if reg_size(&src.name) != src_size {
                    return Err(format!("movzx src size mismatch: {}", src.name));
                }
                if reg_size(&dst.name) != dst_size {
                    return Err(format!("movzx dst size mismatch: {}", dst.name));
                }
                let src_num = reg_num(&src.name).ok_or("bad src register")?;
                let dst_num = reg_num(&dst.name).ok_or("bad dst register")?;
                self.bytes.extend_from_slice(&opcode);
                self.bytes.push(self.modrm(3, dst_num, src_num));
            }
```
**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_movzx_neg_mismatched_width -- --test-threads=1
cargo test --lib encode_movzx_regression_mismatched_width_eax_src -- --test-threads=1
```
**Raw output:**
```text
Test failed: SUT accepted size-mismatched MOVZX `movzbl %ax, %eax` → [0f, b6, c0]; MOVZX requires matching operand sizes (Intel SDM; llvm-mc rejects). encode_movzx ignores register widths and only uses mnemonic sizes..
minimal failing input: mode = 0, si = 0, di = 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/encode_movzx_pbt.rs (`encode_movzx_regression_mismatched_width_eax_src`)
