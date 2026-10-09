# Bug: encode_double_shift silently truncates Imm counts outside Imm8

**Law:** The SHLD/SHRD immediate count is Imm8. Values outside the Imm8 domain (llvm-mc rejects `$256`) must be rejected, not truncated with `as u8`.
**Impact:** `shldl $256, %eax, %edx` encodes as `shldl $0, %eax, %edx` (`[0x0f, 0xa4, 0xc2, 0x00]`). A wrong shift count is emitted with no error — silent miscompilation of double-precision shifts (i128 codegen uses Imm forms).
**Function:** encode_double_shift
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/gp_integer.rs:931
**Detected by:** Differential / negative Imm8 domain (`encode_double_shift_neg_imm_out_of_u8`)
**Minimal input:** `shldl $256, %eax, %edx` (also `shldl $256, %eax, %eax`)
**Expected:** `Err` (Imm outside Imm8; llvm-mc rejects)
**Actual:** `Ok([0x0f, 0xa4, 0xc2, 0x00])` — 256 truncated to 0
**Severity:** medium
**Root cause:** `gp_integer.rs:931` pushes `*count as u8` with no range check on the `i64` immediate.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/gp_integer.rs:931`
```rust
                self.bytes.push(*count as u8);
```
**Suggested fix:** Reject counts that do not fit in a signed or unsigned byte before encoding (match llvm-mc / Imm8).
```rust
                if !(-128..=255).contains(count) {
                    return Err(format!("double shift immediate out of Imm8 range: {count}"));
                }
                self.bytes.push(*count as u8);
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_double_shift_regression_imm_truncate -- --test-threads=1
```
**Raw output:**
```text
encode_double_shift must reject Imm count 256 (Imm8 domain), got Ok([15, 164, 194, 0])
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/encode_double_shift_pbt.rs (`encode_double_shift_regression_imm_truncate`)
