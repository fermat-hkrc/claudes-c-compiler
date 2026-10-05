# Bug: encode_neon_addv places ADDV opcode bits incorrectly
**Law:** ADDV encoding is 0 Q 0 01110 size 11000 11011 10 Rn Rd, matching llvm-mc and the function's own encoding comment
**Impact:** Every ADDV the compiler emits (including `addv b0, v0.8b` from codegen/intrinsics.rs) assembles to the wrong 32-bit word, so object files contain a different or unallocated instruction
**Function:** encode_neon_addv
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:424
**Detected by:** Differential vs llvm-mc; Algebraic — Invariant (ARM layout)
**Minimal input:** encode_neon_addv([b0, v0.8b])
**Expected:** Word(0x0e31b800) — llvm-mc `addv b0, v0.8b`
**Actual:** Word(0x0e30dc00)
**Severity:** high
**Root cause:** neon.rs:434-435 uses `0b110111 << 10` (bits[15:10]=110111) instead of opcode 11011 at bits[16:12] and 10 at bits[11:10]
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:434`
```rust
    let word = (q << 30) | (0b001110 << 24) | (size << 22) | (0b11000 << 17)
        | (0b110111 << 10) | (rn << 5) | rd;
```
**Suggested fix:** Place opcode and op2 at the ARM bit positions (same pattern as encode_neon_across)
```rust
    let word = (q << 30) | (0b001110 << 24) | (size << 22) | (0b11000 << 17)
        | (0b11011 << 12) | (0b10 << 10) | (rn << 5) | rd;
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_addv_regression_encoding -- --test-threads=1 --exact
```
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_neon_addv_pbt::encode_neon_addv_diff_llvm_mc' panicked at src/backend/arm/assembler/encoder/encode_neon_addv_pbt.rs:245:1:
Test failed: assertion failed: `(left == right)`
  left: `238083072`,
 right: `238139392`: SUT vs llvm-mc for addv b0, v0.8b at src/backend/arm/assembler/encoder/encode_neon_addv_pbt.rs:257.
minimal failing input: rd = 0, rn = 0, (v, t) = (
    "b",
    "8b",
)
	successes: 0
	local rejects: 0
	global rejects: 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_neon_addv_pbt.rs
