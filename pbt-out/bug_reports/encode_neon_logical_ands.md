# Bug: encode_neon_logical encodes ANDS vector form as EOR
**Law:** ANDS is not a NEON instruction; vector operands with opc=0b11 must be rejected
**Impact:** `ands v0.16b, v1.16b, v2.16b` is dispatched through encode_logical(opc=0b11) into encode_neon_logical, which emits EOR instead of failing, so the object file contains a different instruction than the source
**Function:** encode_neon_logical
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:297
**Detected by:** Negative/Error Contract
**Minimal input:** encode_neon_logical([v0.8b, v0.8b, v0.8b], opc=0b11)
**Expected:** Err (llvm-mc: invalid operand for `ands v0.8b, v0.8b, v0.8b`)
**Actual:** Ok(EncodeResult::Word) of `eor v0.8b, v0.8b, v0.8b` (U=1, size=00 — same as EOR)
**Severity:** medium (documented by the author)
**Root cause:** neon.rs:313 maps opc=0b11 to (U=1, size=00), the EOR encoding, with a comment that ANDS is not valid for NEON but still falls through to Word
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:313`
```rust
        0b11 => (1, 0b00),  // ANDS - not valid for NEON, fall back
```
**Suggested fix:** Return Err for opc=0b11 instead of encoding EOR
```rust
        0b11 => return Err("ANDS is not a NEON instruction".to_string()),
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_logical_regression_ands -- --test-threads=1
```
**Raw output:**
```text
Test failed: ANDS is not a NEON instruction (neon.rs:313); must Err (llvm-mc rejects ands v0.8b, v0.8b, v0.8b) at src/backend/arm/assembler/encoder/encode_neon_logical_pbt.rs:506.
minimal failing input: rd = 0, rn = 0, rm = 0, t = "8b"
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_neon_logical_pbt.rs
