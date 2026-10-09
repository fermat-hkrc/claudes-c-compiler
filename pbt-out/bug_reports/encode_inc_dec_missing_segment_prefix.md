# Bug: encode_inc_dec omits segment-override prefix on memory operands

**Law:** Memory INC/DEC with a segment override must emit the documented segment prefix byte (ES=0x26, CS=0x2E, SS=0x36, DS=0x3E, FS=0x64, GS=0x65) before the FE/FF opcode, matching llvm-mc and `core.rs::emit_segment_prefix`.
**Impact:** Assembler silently drops segment overrides on `incl`/`decl`/`incw`/`decw`/`incb`/`decb` memory forms, so `%fs:(%eax)` encodes as bare `(%eax)`. Generated code reads/writes the wrong segment — silent data corruption for TLS, kernel, and far-data code.
**Function:** encode_inc_dec
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/gp_integer.rs:829
**Detected by:** Differential — llvm-mc i686 (and algebraic.metamorphic segment = prefix ‖ bare)
**Minimal input:** `incl %es:(%eax)` → SUT `[0xff, 0x00]`, llvm-mc `[0x26, 0xff, 0x00]`
**Expected:** `[0x26, 0xff, 0x00]` (ES override then FF /0)
**Actual:** `[0xff, 0x00]` (no override)
**Severity:** high
**Root cause:** `gp_integer.rs:829-832` memory arm pushes size prefix and opcode then calls `encode_modrm_mem` without `self.emit_segment_prefix(mem)`.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/gp_integer.rs:829`
```rust
            Operand::Memory(mem) => {
                if size == 2 { self.bytes.push(0x66); }
                self.bytes.push(if size == 1 { 0xFE } else { 0xFF });
                self.encode_modrm_mem(op_ext, mem)
            }
```
**Suggested fix:** Call `emit_segment_prefix` before the operand-size / opcode bytes (same order as llvm-mc: seg, then 0x66 if any, then opcode).
```rust
            Operand::Memory(mem) => {
                self.emit_segment_prefix(mem);
                if size == 2 { self.bytes.push(0x66); }
                self.bytes.push(if size == 1 { 0xFE } else { 0xFF });
                self.encode_modrm_mem(op_ext, mem)
            }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_inc_dec_regression_missing_es_prefix -- --test-threads=1
```
**Raw output:**
```text
assertion `left == right` failed: regression: encode_inc_dec must emit segment override before 0xFF /0
  left: [255, 0]
 right: [38, 255, 0]
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/encode_inc_dec_pbt.rs (encode_inc_dec_regression_missing_es_prefix)
