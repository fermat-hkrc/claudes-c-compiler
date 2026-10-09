# Bug: encode_imul Imm,Reg,Reg ignores size (no 0x66)
**Law:** `encode(imul $imm, %src, %dst)` must match llvm-mc for width∈{2,4}.
**Impact:** 3-op imulw wrong.
**Function:** encode_imul
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/gp_integer.rs:747
**Detected by:** Differential (encode_imul_diff_imm_reg_reg)
**Minimal input:** width=2, imm=-128, src=ax, dst=ax
**Expected:** 66 6b c0 80
**Actual:** 6b c0 80
**Severity:** high
**Root cause:** gp_integer.rs:747-758 never consults size.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/gp_integer.rs:751`
```rust
if *val >= -128 && *val <= 127 {
    self.bytes.push(0x6B);
    self.bytes.push(self.modrm(3, dst_num, src_num));
    self.bytes.push(*val as u8);
```
**Suggested fix:**
```rust
if size == 2 { self.bytes.push(0x66); }
if *val >= -128 && *val <= 127 {
    self.bytes.push(0x6B);
    ...
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_imul_diff_imm_reg_reg -- --test-threads=1
```
**Raw output:**
```text
minimal failing input: width = 2, si = 0, di = 0, raw = 0, edge = 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/encode_imul_pbt.rs
