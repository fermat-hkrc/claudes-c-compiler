# Bug: encode_bt metamorphic segment‖bare fails (missing emit_segment_prefix)
**Law:** For any BT-family memory encoding, the segmented form must equal `[seg_prefix_byte(seg)] ‖ bare_encoding` — i.e. the only difference introduced by a segment override is the documented i686 prefix byte from `core.rs::emit_segment_prefix`.
**Impact:** Same as the differential segment bug: segmented BT memory forms omit the override prefix, so the instruction addresses the wrong segment. This metamorphic property independently witnesses the same defective arms without relying on llvm-mc.
**Function:** encode_bt
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/gp_integer.rs:1033
**Detected by:** Algebraic — Metamorphic (seg ‖ bare)
**Minimal input:** bare `btl %eax, (%eax)` → `[0f,a3,00]`; segmented `btl %eax, %es:(%eax)` → SUT `[0f,a3,00]` (expect `[26,0f,a3,00]`)
**Expected:** `[26, 0f, a3, 00]`
**Actual:** `[0f, a3, 00]`
**Severity:** high
**Root cause:** Reg→Mem and Imm→Mem arms never call `emit_segment_prefix(mem)` before the opcode (`gp_integer.rs:1033-1037`, `1045-1050`).
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/gp_integer.rs:1033`
```rust
            (Operand::Register(src), Operand::Memory(mem)) => {
                let src_num = reg_num(&src.name).ok_or("bad register")?;
                self.bytes.extend_from_slice(&[0x0F, opcode_rr]);
                self.encode_modrm_mem(src_num, mem)
            }
```
**Suggested fix:** Emit the segment prefix before the opcode on both memory arms.
```rust
                self.emit_segment_prefix(mem);
                self.bytes.extend_from_slice(&[0x0F, opcode_rr]);
                self.encode_modrm_mem(src_num, mem)
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_bt_meta_segment_prefix -- --test-threads=1
```
**Raw output:**
```text
metamorphic seg||bare for btl %es:(%eax): got [0f, a3, 00] expect [26, 0f, a3, 00]
minimal failing input: mi = 0, si = 0, bi = 0, ri = 0, form = 0, imm = 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/encode_bt_pbt.rs (`encode_bt_regression_segment_es_reg_mem`)
