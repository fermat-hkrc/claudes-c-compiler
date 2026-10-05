# Bug: encode_neon_mvni ignores LSL #8 on .4h/.8h (wrong cmode)
**Law:** Valid `mvni Vd.{4h,8h}, #imm8, lsl #8` must encode cmode=1010 and match llvm-mc/gas
**Impact:** Assembler emits the no-shift encoding, so `mvni v0.4h, #0, lsl #8` becomes `mvni v0.4h, #0` in the object file; callers get the wrong vector immediate
**Function:** encode_neon_mvni
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:1333
**Detected by:** Differential vs llvm-mc -triple=aarch64 -show-encoding
**Minimal input:** encode_neon_mvni([v0.4h, #0, lsl #8])
**Expected:** 0x2f00a400 (llvm-mc encoding of `mvni v0.4h, #0, lsl #8`)
**Actual:** 0x2f008400 (encoding of `mvni v0.4h, #0` with cmode=1000)
**Severity:** high
**Root cause:** neon.rs:1375-1378 hard-codes cmode=1000 and never reads operands[2], so ARM-valid LSL #8 (cmode=1010) is dropped
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:1377`
```rust
            // MVNI 16-bit: cmode=1000, op=1
            let word = (q << 30) | (1 << 29) | (0b0111100 << 22)
                | (abc << 16) | (0b1000 << 12) | (0b01 << 10) | (defgh << 5) | rd;
            Ok(EncodeResult::Word(word))
```
**Suggested fix:** Decode optional LSL on the 4h/8h path: amount 0 → cmode=1000, amount 8 → cmode=1010, anything else Err
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
            let word = (q << 30) | (1 << 29) | (0b0111100 << 22)
                | (abc << 16) | (cmode << 12) | (0b01 << 10) | (defgh << 5) | rd;
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_neon_mvni_diff_h_lsl8 -- --test-threads=1
```
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_neon_mvni_pbt::encode_neon_mvni_diff_h_lsl8' panicked at src/backend/arm/assembler/encoder/encode_neon_mvni_pbt.rs:278:1:
Test failed: assertion failed: `(left == right)`
  left: `788562944`,
 right: `788571136`: mismatch for mvni v0.4h, #0, lsl #8
minimal failing input: rd = 0, t = "4h", imm8 = 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_neon_mvni_pbt.rs (test_encode_neon_mvni_regression_h_lsl8)
