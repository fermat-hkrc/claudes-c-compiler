# Bug: encode_bt omits segment override prefix on memory forms
**Law:** Valid BT/BTS/BTR/BTC memory operands with a segment override (`%es:`, `%fs:`, …) must emit the corresponding i686 segment-override prefix byte before the opcode, matching llvm-mc and `core.rs::emit_segment_prefix`.
**Impact:** Any AT&T instruction such as `btl %eax, %es:(%ebx)` or `btsl $7, %fs:(%eax)` assembles without the segment prefix, so the encoded instruction addresses the wrong segment (DS default) — silent wrong machine code for every segmented BT-family memory form.
**Function:** encode_bt
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/gp_integer.rs:1033
**Detected by:** Differential — llvm-mc i686 (segment forms) + Algebraic metamorphic (seg ‖ bare)
**Minimal input:** `btl %eax, %es:(%ebx)` → SUT `[0f,a3,03]`, expected `[26,0f,a3,03]`; also `btsl $7, %fs:(%eax)` → SUT omits `0x64`
**Expected:** Segment prefix byte (`0x26` for ES, `0x64` for FS, …) then `0F opc …`
**Actual:** Opcode bytes only; no prefix
**Severity:** high
**Root cause:** `gp_integer.rs:1033-1037` (Reg→Mem) and `1045-1050` (Imm→Mem) call `encode_modrm_mem` without first calling `self.emit_segment_prefix(mem)`. The x86-64 sibling (`x87_misc.rs` encode_bt) does call `emit_segment_prefix` on both mem arms.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/gp_integer.rs:1033`
```rust
            (Operand::Register(src), Operand::Memory(mem)) => {
                let src_num = reg_num(&src.name).ok_or("bad register")?;
                self.bytes.extend_from_slice(&[0x0F, opcode_rr]);
                self.encode_modrm_mem(src_num, mem)
            }
```
and
```rust
            (Operand::Immediate(ImmediateValue::Integer(val)), Operand::Memory(mem)) => {
                self.bytes.extend_from_slice(&[0x0F, 0xBA]);
                self.encode_modrm_mem(ext, mem)?;
                self.bytes.push(*val as u8);
                Ok(())
            }
```
**Suggested fix:** Call `emit_segment_prefix` before the opcode on both memory arms (same shape as x86-64 sibling and other fixed i686 encoders).
```rust
            (Operand::Register(src), Operand::Memory(mem)) => {
                let src_num = reg_num(&src.name).ok_or("bad register")?;
                self.emit_segment_prefix(mem);
                self.bytes.extend_from_slice(&[0x0F, opcode_rr]);
                self.encode_modrm_mem(src_num, mem)
            }
            (Operand::Immediate(ImmediateValue::Integer(val)), Operand::Memory(mem)) => {
                self.emit_segment_prefix(mem);
                self.bytes.extend_from_slice(&[0x0F, 0xBA]);
                self.encode_modrm_mem(ext, mem)?;
                self.bytes.push(*val as u8);
                Ok(())
            }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_bt_regression_segment_es_reg_mem -- --test-threads=1
cargo test --lib encode_bt_diff_segment -- --test-threads=1
```
**Raw output:**
```text
assertion `left == right` failed: segment ES prefix missing on BT reg→mem
  left: [15, 163, 3]
 right: [38, 15, 163, 3]
segment diff `btl %eax, %es:(%eax)`: sut=[0f, a3, 00] mc=[26, 0f, a3, 00]; expected prefix 0x26
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/encode_bt_pbt.rs (`encode_bt_regression_segment_es_reg_mem`, `encode_bt_regression_segment_fs_imm_mem`)
