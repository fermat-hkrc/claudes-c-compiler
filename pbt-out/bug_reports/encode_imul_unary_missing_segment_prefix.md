# Bug: encode_imul 1-op memory path omits segment prefix (via encode_unary_rm)
**Law:** One-operand `imull`/`imulw` on a segmented memory operand must emit the segment override before F6/F7 /5, matching llvm-mc and `emit_segment_prefix`.
**Impact:** `imull %es:(%eax)` assembles without 0x26; unary IMUL (result in EDX:EAX) uses the wrong segment.
**Function:** encode_imul (1-op arm → encode_unary_rm)
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/gp_integer.rs:713
**Detected by:** Differential (encode_imul_diff_unary) + regression encode_imul_regression_unary_segment
**Minimal input:** `imull %es:(%eax)` → SUT `[f7,28]` vs llvm-mc `[26,f7,28]`
**Expected:** `[26, f7, 28]`
**Actual:** `[f7, 28]`
**Severity:** high
**Root cause:** encode_imul delegates 1-op to encode_unary_rm (gp_integer.rs:713). encode_unary_rm's memory arm (gp_integer.rs:794-796) pushes F6/F7 then encode_modrm_mem without `emit_segment_prefix`. (size==2 0x66 is handled, but after where a segment would go — order must be seg then 0x66 then opcode.)
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/gp_integer.rs:794`
```rust
Operand::Memory(mem) => {
    self.bytes.push(if size == 1 { 0xF6 } else { 0xF7 });
    self.encode_modrm_mem(op_ext, mem)
}
```
**Suggested fix:**
```rust
Operand::Memory(mem) => {
    self.emit_segment_prefix(mem);
    if size == 2 { self.bytes.push(0x66); }
    self.bytes.push(if size == 1 { 0xF6 } else { 0xF7 });
    self.encode_modrm_mem(op_ext, mem)
}
```
(Move the existing size==2 push so it sits after the segment prefix.)

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_imul_regression_unary_segment -- --test-threads=1
```
**Raw output:**
```text
assertion `left == right` failed: 1-op mem must emit segment prefix via encode_unary_rm; sut=[f7, 28] mc=[26, f7, 28]
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/encode_imul_pbt.rs (encode_imul_regression_unary_segment)
