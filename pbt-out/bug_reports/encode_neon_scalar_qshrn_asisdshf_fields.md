# Bug: encode_neon_scalar_qshrn bits[28:24] are 01111 instead of ARM 11111
**Law:** ARM asisdshf scalar shift-by-immediate words have bits[28:24]=11111
**Impact:** Same encoding defect as encode_neon_scalar_qshrn_asisdshf_bit28: every success-path word has bit 28 clear, matching vector Q=1 rather than scalar asisdshf.
**Function:** encode_neon_scalar_qshrn
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:1835
**Detected by:** Algebraic — Invariant
**Minimal input:** encode_neon_scalar_qshrn([Reg("b0"), Reg("h0"), Imm(1)], 0, false)
**Expected:** bits[28:24]=31 (0b11111)
**Actual:** bits[28:24]=15 (0b01111)
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
cargo test --lib encode_neon_scalar_qshrn_invariant_arm_fields -- --test-threads=1
cargo test --lib test_encode_neon_scalar_qshrn_regression_asisdshf_bit28 -- --test-threads=1
```
**Raw output:**
```text
Test failed: assertion failed: `(left == right)`
  left: `15`,
 right: `31`: bits[28:24]=11111 (ARM asisdshf)
minimal failing input: rd = 0, rn = 0, case = ("b", "h", 8, 1), u = 0, round = false
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_neon_scalar_qshrn_pbt.rs
