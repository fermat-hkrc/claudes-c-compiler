# Bug: encode_lsl SIB/abs memory form omits 0x66 for 16-bit destination
**Law:** `LSL r16, m16` (including SIB and absolute memory) requires the 0x66 operand-size override before 0F 03.
**Impact:** SIB-addressed 16-bit dest LSL assembles as 32-bit form; same root cause as plain base+disp mem16.
**Function:** encode_lsl
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/system.rs:160
**Detected by:** Differential — llvm-mc i686 (encode_lsl_diff_sib / encode_lsl_diff_edges)
**Minimal input:** `lsl (,%eax,1), %ax` → base=None, index=eax, scale=1, disp=0, dst=ax
**Expected:** `[0x66, 0x0f, 0x03, 0x04, 0x05, 0x00, 0x00, 0x00, 0x00]`
**Actual:** `[0x0f, 0x03, 0x04, 0x05, 0x00, 0x00, 0x00, 0x00]`
**Severity:** high
**Root cause:** system.rs:160-164 memory arm never inspects destination width for 0x66 (same defective arm as base+disp mem16).
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/system.rs:160`
```rust
(Operand::Memory(mem), Operand::Register(dst)) => {
    let dst_num = reg_num(&dst.name).ok_or("bad register")?;
    self.bytes.extend_from_slice(&[0x0F, 0x03]);
    self.encode_modrm_mem(dst_num, mem)
}
```
**Suggested fix:**
```rust
let is_16 = matches!(dst.name.as_str(), "ax"|"bx"|"cx"|"dx"|"si"|"di"|"sp"|"bp");
self.emit_segment_prefix(mem);
if is_16 { self.bytes.push(0x66); }
self.bytes.extend_from_slice(&[0x0F, 0x03]);
self.encode_modrm_mem(dst_num, mem)
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_lsl_diff_sib -- --test-threads=1
```
**Raw output:**
```text
minimal failing input: base = None, index = "eax", scale = 1, disp = 0, dst = "ax"
SUT=[0f, 03, 04, 05, 00, 00, 00, 00] llvm-mc=[66, 0f, 03, 04, 05, 00, 00, 00, 00]
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/encode_lsl_pbt.rs
