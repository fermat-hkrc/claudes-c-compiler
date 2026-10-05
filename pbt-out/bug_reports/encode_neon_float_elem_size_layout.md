# Bug: encode_neon_float_elem ARM size field is 00/01 instead of 10/11
**Law:** The 32-bit word must pack ARM size[1:0] at bits[23:22] as 10 for S and 11 for D
**Impact:** Same as the llvm-mc disagreement: every FP by-element word has reserved size 00 (S) or 01 (D)
**Function:** encode_neon_float_elem
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:1613
**Detected by:** Algebraic — Invariant (ARM size field)
**Minimal input:** encode_neon_float_elem([v0.2s, v0.2s, v0.s[0]], u_bit=0, opcode=0b0001)
**Expected:** bits[23:22] = 10
**Actual:** bits[23:22] = 00
**Severity:** high
**Root cause:** neon.rs:1633 shifts `sz` to bit 22 only, leaving bit 23 clear
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:1633`
```rust
    let word = (q << 30) | (u_bit << 29) | (0b01111 << 24) | (sz << 22)
```
**Suggested fix:** OR the high size bit so S=0b10 and D=0b11
```rust
    let word = (q << 30) | (u_bit << 29) | (0b01111 << 24) | ((0b10 | sz) << 22)
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_neon_float_elem_inv_layout -- --test-threads=1
```
**Raw output:**
```text
Test failed: assertion failed: `(left == right)`
  left: `0`,
 right: `2`: size must be 10 (S) or 11 (D)
minimal failing input: rd = 0, rn = 0, rm = 0, idx_raw = 0, shape = ("2s", "s", 3), u = 0, opcode = 1
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_neon_float_elem_pbt.rs::test_encode_neon_float_elem_regression_size_bit23
