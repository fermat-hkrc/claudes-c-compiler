# Bug: encode_neon_movi ignores LSL #8 on .4h/.8h
**Law:** ∀ rd ∈ [0,31], T ∈ {4h,8h}, imm8 ∈ [0,255]. encode_neon_movi([Vd.T, #imm8, lsl #8]) = llvm-mc("movi Vd.T, #imm8, lsl #8")
**Impact:** `movi v0.4h, #0, lsl #8` is a valid GNU/ARM form (cmode=1010, each halfword = imm8<<8). The SUT encodes it as unshifted MOVI (cmode=1000), so the assembled word loads 0 instead of 0x0100 in each 16-bit lane when imm8=1. Any codegen that emits 16-bit MOVI with LSL #8 silently produces the wrong immediate.
**Function:** encode_neon_movi
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:624
**Detected by:** Differential — llvm-mc AArch64 assembler
**Minimal input:** encode_neon_movi([v0.4h, #0, lsl #8])
**Expected:** Ok(Word(0x0f00a400)) — llvm-mc encoding of `movi v0.4h, #0, lsl #8`
**Actual:** Ok(Word(0x0f008400)) — same as `movi v0.4h, #0` (cmode=1000, no shift)
**Severity:** high
**Root cause:** neon.rs:701-710 the .4h/.8h arm hard-codes cmode=1000 and never inspects a third Shift operand. Parser emits Operand::Shift { kind: "lsl", amount: 8 }, so the form is caller-reachable.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:701`
```rust
        "4h" | "8h" => {
            // MOVI Vd.4h/8h, #imm8
            let q: u32 = if arr_d == "8h" { 1 } else { 0 };
            let imm8 = imm as u32 & 0xFF;
            let abc = (imm8 >> 5) & 0x7;
            let defgh = imm8 & 0x1F;
            // cmode=1000 for .4h/.8h with no shift
            let word = (q << 30) | (0b0011110 << 23) | ((abc >> 2) << 18) | (((abc >> 1) & 1) << 17)
                | ((abc & 1) << 16) | (0b1000 << 12) | (0b01 << 10) | (defgh << 5) | rd;
            Ok(EncodeResult::Word(word))
        }
```
**Suggested fix:** Mirror the 2s/4s LSL peek: LSL #0 → cmode=1000, LSL #8 → cmode=1010, other amounts Err.
```rust
        "4h" | "8h" => {
            let q: u32 = if arr_d == "8h" { 1 } else { 0 };
            let imm8 = imm as u32 & 0xFF;
            let abc = (imm8 >> 5) & 0x7;
            let defgh = imm8 & 0x1F;
            let cmode = if let Some(Operand::Shift { kind, amount }) = operands.get(2) {
                if kind == "lsl" {
                    match *amount {
                        0 => 0b1000u32,
                        8 => 0b1010,
                        _ => return Err(format!("movi: unsupported shift amount: {}", amount)),
                    }
                } else {
                    return Err(format!("movi: unsupported shift kind: {}", kind));
                }
            } else if operands.len() > 2 {
                return Err("movi: unexpected extra operand".to_string());
            } else {
                0b1000
            };
            let word = (q << 30) | (0b0011110 << 23) | ((abc >> 2) << 18) | (((abc >> 1) & 1) << 17)
                | ((abc & 1) << 16) | (cmode << 12) | (0b01 << 10) | (defgh << 5) | rd;
            Ok(EncodeResult::Word(word))
        }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_movi_regression_h_lsl8 -- --test-threads=1
```
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_neon_movi_pbt::test_encode_neon_movi_regression_h_lsl8' (2277463) panicked at src/backend/arm/assembler/encoder/encode_neon_movi_pbt.rs:591:5:
assertion `left == right` failed: movi v0.4h, #0, lsl #8 must match llvm-mc (cmode=1010, 0x0f00a400)
  left: 251692032
 right: 251700224
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_neon_movi_pbt.rs
