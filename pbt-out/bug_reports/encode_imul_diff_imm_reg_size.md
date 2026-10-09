# Bug: encode_imul Imm→Reg ignores size (no 0x66 / imm32 vs imm16)
**Law:** `encode(imul $imm, %dst)` must match llvm-mc for width∈{2,4}.
**Impact:** imulw immediate forms wrong.
**Function:** encode_imul
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/gp_integer.rs:729
**Detected by:** Differential (encode_imul_diff_imm_reg)
**Minimal input:** width=2, imm=-128, dst=ax; also imm=300
**Expected:** 66 6b … / 66 69 … imm16
**Actual:** 6b … / 69 … imm32
**Severity:** high
**Root cause:** gp_integer.rs:729-740 never consults size.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/gp_integer.rs:736`
```rust
self.bytes.push(0x69);
self.bytes.push(self.modrm(3, dst_num, dst_num));
self.bytes.extend_from_slice(&(*val as i32).to_le_bytes());
```
**Suggested fix:**
```rust
if size == 2 { self.bytes.push(0x66); }
// ...
if size == 2 {
    self.bytes.extend_from_slice(&(*val as i16).to_le_bytes());
} else {
    self.bytes.extend_from_slice(&(*val as i32).to_le_bytes());
}
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_imul_diff_imm_reg -- --test-threads=1
cargo test --lib encode_imul_regression_imulw_imm16_width -- --test-threads=1
```
**Raw output:**
```text
minimal failing input: width = 2, di = 0, raw = 0, edge = 0
assertion `left == right` failed: imulw imm16 must be 0x66 + 0x69 + imm16 (not imm32); sut=[69, c0, 2c, 01, 00, 00] mc=[66, 69, c0, 2c, 01]
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/encode_imul_pbt.rs
