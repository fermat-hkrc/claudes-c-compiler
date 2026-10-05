# Bug: encode_neon_scalar_qshrn uppercase SQSHRN disagrees with llvm-mc
**Law:** Uppercase scalar SQSHRN must encode the same ARM asisdshf word as llvm-mc
**Impact:** Same encoding defect as encode_neon_scalar_qshrn_asisdshf_bit28: `SQSHRN B0, H0, #1` yields 0x4f0f9400 instead of llvm-mc 0x5f0f9400.
**Function:** encode_neon_scalar_qshrn
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:1835
**Detected by:** Differential — llvm-mc AArch64 assembler
**Minimal input:** encode_neon_scalar_qshrn([Reg("B0"), Reg("H0"), Imm(1)], 0, false)
**Expected:** Ok(Word(0x5f0f9400))
**Actual:** Ok(Word(0x4f0f9400))
**Severity:** high
**Root cause:** neon.rs:1849 ORs vector asimdshf fixed bits `(0b011110 << 23)` onto `(0b01 << 30)`, leaving bit 28 = 0
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:1849`
```rust
    let word = (0b01 << 30) | (u_bit << 29) | (0b011110 << 23) | ((immhb >> 3) << 19) | ((immhb & 7) << 16)
        | (opcode_bits << 10) | (rn << 5) | rd;
```
**Suggested fix:** Place scalar asisdshf fixed bits at [28:24]=11111
```rust
    let word = (0b01 << 30) | (u_bit << 29) | (0b11111 << 24) | ((immhb >> 3) << 19) | ((immhb & 7) << 16)
        | (opcode_bits << 10) | (rn << 5) | rd;
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_neon_scalar_qshrn_diff_alt_spellings -- --test-threads=1
cargo test --lib test_encode_neon_scalar_qshrn_regression_asisdshf_bit28 -- --test-threads=1
```
**Raw output:**
```text
Test failed: assertion failed: `(left == right)`
  left: `1326420992`,
 right: `1594856448`: mismatch for alt-spelling SQSHRN B0, H0, #1
minimal failing input: rd = 0, rn = 0, case = ("b", "h", 8, 1), u = 0, round = false
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_neon_scalar_qshrn_pbt.rs
