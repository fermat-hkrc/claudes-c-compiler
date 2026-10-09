# Bug: encode_imul Imm,Mem,Reg misses segment prefix and/or 0x66
**Law:** `encode(imul $imm, mem, %dst)` must match llvm-mc including segment overrides.
**Impact:** 3-op memory IMUL wrong under segment or imulw.
**Function:** encode_imul
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/gp_integer.rs:761
**Detected by:** Differential (encode_imul_diff_imm_mem_reg)
**Minimal input:** imull $5, %es:(%eax), %ebx
**Expected:** 26 6b 18 05
**Actual:** 6b 18 05
**Severity:** high
**Root cause:** gp_integer.rs:761-771 skips emit_segment_prefix and size prefix.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/gp_integer.rs:763`
```rust
let dst_num = reg_num(&dst.name).ok_or("bad register")?;
if *val >= -128 && *val <= 127 {
    self.bytes.push(0x6B);
    self.encode_modrm_mem(dst_num, mem)?;
```
**Suggested fix:**
```rust
self.emit_segment_prefix(mem);
if size == 2 { self.bytes.push(0x66); }
if *val >= -128 && *val <= 127 {
    self.bytes.push(0x6B);
    self.encode_modrm_mem(dst_num, mem)?;
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_imul_diff_imm_mem_reg -- --test-threads=1
cargo test --lib encode_imul_regression_seg_imm_mem_reg -- --test-threads=1
```
**Raw output:**
```text
assertion `left == right` failed: 3-op Imm,Mem,Reg must emit segment prefix; sut=[6b, 18, 05] mc=[26, 6b, 18, 05]
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/encode_imul_pbt.rs
