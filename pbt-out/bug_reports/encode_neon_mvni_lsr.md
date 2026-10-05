# Bug: encode_neon_mvni treats LSR (and other non-LSL/MSL shifts) as no-shift
**Law:** Only LSL and MSL are legal MVNI shift kinds; llvm-mc/gas reject LSR
**Impact:** `mvni v0.4s, #0, lsr #8` is assembled as `mvni v0.4s, #0` instead of an error
**Function:** encode_neon_mvni
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:1333
**Detected by:** Negative/error contract vs llvm-mc (regression witness of encode_neon_mvni_neg_extra_and_illegal_shift)
**Minimal input:** encode_neon_mvni([v0.4s, #0, lsr #8])
**Expected:** Err (llvm-mc: invalid operand for instruction)
**Actual:** Ok(Word) encoding `mvni v0.4s, #0` (cmode=0000)
**Severity:** medium
**Root cause:** neon.rs:1364-1366 maps any non-lsl/non-msl shift kind to cmode=0000 instead of returning Err
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:1364`
```rust
                } else {
                    0b0000
                }
```
**Suggested fix:** Reject unknown shift kinds
```rust
                } else {
                    return Err(format!("mvni: unsupported shift kind: {}", kind));
                }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_mvni_regression_lsr -- --test-threads=1
```
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_neon_mvni_pbt::test_encode_neon_mvni_regression_lsr' panicked at src/backend/arm/assembler/encoder/encode_neon_mvni_pbt.rs:543:5:
mvni v0.4s, #0, lsr #8 must Err (llvm-mc rejects LSR on MVNI)
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_neon_mvni_pbt.rs (test_encode_neon_mvni_regression_lsr)
