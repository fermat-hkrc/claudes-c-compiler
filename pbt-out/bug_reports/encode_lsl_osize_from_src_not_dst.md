# Bug: encode_lsl keys 0x66 operand-size prefix off source register, not destination
**Law:** Intel LSL is `LSL r16, r/m16` or `LSL r32, r/m16`; the operand-size override (0x66) is determined by the destination register width. Source is a selector (r/m16) regardless of the name used.
**Impact:** Mixed-width AT&T forms such as `lsl %ax, %ebx` (32-bit dest) incorrectly get 0x66, and `lsl %eax, %bx` (16-bit dest) miss 0x66. Assembled code has the wrong operand size and writes the limit into the wrong register width.
**Function:** encode_lsl
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/system.rs:152
**Detected by:** Differential — llvm-mc i686 (encode_lsl_diff_mixed_width_dest_drives_osize)
**Minimal input:** `lsl %eax, %ax` (src=eax, dst=ax) and `lsl %ax, %ebx` (src=ax, dst=ebx)
**Expected:** `lsl %eax, %ax` → `[0x66, 0x0f, 0x03, 0xc0]`; `lsl %ax, %ebx` → `[0x0f, 0x03, 0xd8]`
**Actual:** `lsl %eax, %ax` → `[0x0f, 0x03, 0xc0]` (missing 0x66); `lsl %ax, %ebx` → `[0x66, 0x0f, 0x03, 0xd8]` (spurious 0x66)
**Severity:** high
**Root cause:** system.rs:152 computes `is_16` from `src.name` instead of `dst.name`.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/system.rs:152`
```rust
let is_16 = matches!(src.name.as_str(), "ax"|"bx"|"cx"|"dx"|"si"|"di"|"sp"|"bp");
if is_16 {
    self.bytes.push(0x66);
}
```
**Suggested fix:** Key operand size off the destination:
```rust
let is_16 = matches!(dst.name.as_str(), "ax"|"bx"|"cx"|"dx"|"si"|"di"|"sp"|"bp");
if is_16 {
    self.bytes.push(0x66);
}
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_lsl_regression_osize_from_src_eax_bx -- --test-threads=1
cargo test --lib test_encode_lsl_regression_osize_from_src_ax_ebx -- --test-threads=1
```
**Raw output:**
```text
assertion `left == right` failed: lsl %eax, %bx must be [66, 0f, 03, d8] (dest-driven), got [0f, 03, d8]
  left: [15, 3, 216]
 right: [102, 15, 3, 216]

assertion `left == right` failed: lsl %ax, %ebx must be [0f, 03, d8] (dest-driven), got [66, 0f, 03, d8]
  left: [102, 15, 3, 216]
 right: [15, 3, 216]
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/encode_lsl_pbt.rs
