# Bug: encode_neon_mvni accepts illegal LSL amounts on .4h/.8h
**Law:** LSL amounts other than 0 or 8 on MVNI .4h/.8h must be rejected (llvm-mc/gas reject them)
**Impact:** `mvni v0.4h, #0, lsl #32` is assembled as no-shift MVNI instead of an error
**Function:** encode_neon_mvni
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:1333
**Detected by:** Negative/error contract vs llvm-mc (regression witness of encode_neon_mvni_neg_extra_and_illegal_shift)
**Minimal input:** encode_neon_mvni([v0.4h, #0, lsl #32])
**Expected:** Err (llvm-mc: invalid operand for instruction)
**Actual:** Ok(Word) encoding `mvni v0.4h, #0`
**Severity:** medium
**Root cause:** neon.rs:1375-1378 never inspects the optional shift on the 4h/8h path, so illegal amounts are dropped
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:1377`
```rust
            // MVNI 16-bit: cmode=1000, op=1
            let word = (q << 30) | (1 << 29) | (0b0111100 << 22)
                | (abc << 16) | (0b1000 << 12) | (0b01 << 10) | (defgh << 5) | rd;
            Ok(EncodeResult::Word(word))
```
**Suggested fix:** Parse the optional shift on 4h/8h and reject amounts other than 0 and 8
```rust
            let cmode = if let Some(Operand::Shift { kind, amount }) = operands.get(2) {
                if kind.to_lowercase() == "lsl" {
                    match *amount {
                        0 => 0b1000u32,
                        8 => 0b1010,
                        _ => return Err(format!("mvni: unsupported shift amount: {}", amount)),
                    }
                } else {
                    return Err(format!("mvni: unsupported shift kind: {}", kind));
                }
            } else {
                0b1000
            };
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_mvni_regression_illegal_h_shift -- --test-threads=1
```
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_neon_mvni_pbt::test_encode_neon_mvni_regression_illegal_h_shift' panicked at src/backend/arm/assembler/encoder/encode_neon_mvni_pbt.rs:534:5:
mvni v0.4h, #0, lsl #32 must Err (llvm-mc rejects illegal H LSL amount)
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_neon_mvni_pbt.rs (test_encode_neon_mvni_regression_illegal_h_shift)
