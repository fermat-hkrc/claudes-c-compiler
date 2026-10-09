# Bug: encode_imul ignores size on 2/3-operand forms (missing 0x66 + wrong imm width)
**Law:** For `imulw` (size=2), every 2- and 3-operand form must emit the 0x66 operand-size override, and the 0x69 form must append imm16 (not imm32), matching Intel SDM IMUL and llvm-mc `-triple=i686`.
**Impact:** 16-bit signed multiply encodings are wrong; assemblers emit bytes that execute as 32-bit IMUL, corrupting codegen for `imulw`.
**Function:** encode_imul
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/gp_integer.rs:716
**Detected by:** Differential — llvm-mc i686 (encode_imul_diff_rr_same_width, encode_imul_diff_imm_reg, encode_imul_invariant_rr_opcode)
**Minimal input:** `imulw %ax, %bx` → SUT `[0f,af,d8]` vs llvm-mc `[66,0f,af,d8]`; `imulw $300, %ax` → SUT `[69,c0,2c,01,00,00]` vs `[66,69,c0,2c,01]`
**Expected:** `66 0f af d8` / `66 69 c0 2c 01`
**Actual:** `0f af d8` / `69 c0 2c 01 00 00`
**Severity:** high
**Root cause:** gp_integer.rs:716-771 — 2-op and 3-op arms never consult `size`; no `if size == 2 { push 0x66 }`, and 0x69 always uses `(*val as i32).to_le_bytes()` (4 bytes) instead of imm16 when size==2.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/gp_integer.rs:719`
```rust
self.bytes.extend_from_slice(&[0x0F, 0xAF]);
self.bytes.push(self.modrm(3, dst_num, src_num));
```
and
```rust
self.bytes.push(0x69);
self.bytes.push(self.modrm(3, dst_num, dst_num));
self.bytes.extend_from_slice(&(*val as i32).to_le_bytes());
```
**Suggested fix:** Emit 0x66 when `size == 2` before the opcode on every 2/3-op arm; for the 0x69 path use imm16 when size==2:
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
cargo test --lib encode_imul_regression_imulw_rr_missing_66 -- --test-threads=1
cargo test --lib encode_imul_regression_imulw_imm16_width -- --test-threads=1
```
**Raw output:**
```text
assertion `left == right` failed: imulw RR must emit 0x66 operand-size prefix; sut=[0f, af, d8] mc=[66, 0f, af, d8]
assertion `left == right` failed: imulw imm16 must be 0x66 + 0x69 + imm16 (not imm32); sut=[69, c0, 2c, 01, 00, 00] mc=[66, 69, c0, 2c, 01]
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/encode_imul_pbt.rs (encode_imul_regression_imulw_rr_missing_66, encode_imul_regression_imulw_imm16_width)
