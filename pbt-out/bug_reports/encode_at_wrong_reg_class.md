# Bug: encode_at accepts W/SP/SIMD registers as AT Xt
**Law:** If llvm-mc / GNU as reject `at s1e1r, Wn` / `at s1e1r, sp` / `at s1e1r, Dn` (Xt must be a 64-bit GPR), then encode_at([], "s1e1r, <non-X>") must return Err
**Impact:** `at s1e1r, w0` encodes identically to `at s1e1r, x0` (0xd5087800). A 32-bit, SP, or SIMD register is silently treated as the corresponding 5-bit encoding, so a width/class typo is not diagnosed.
**Function:** encode_at
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/system.rs:426
**Detected by:** Negative/Error Contract (wrong register class on AT; llvm-mc rejects)
**Minimal input:** encode_at(&[], "s1e1r, w0")  (assembly: `at s1e1r, w0`)
**Expected:** Err (llvm-mc: "invalid operand for instruction"; gas: "operand mismatch")
**Actual:** Ok(Word(0xd5087800))  // same encoding as `at s1e1r, x0`
**Severity:** medium
**Root cause:** system.rs:431 uses parse_reg_num, which accepts W/SP/WZR/WSP and SIMD/FP prefixes (d/s/q/v/h/b) as 5-bit numbers, with no 64-bit GPR check for AT Xt.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/system.rs:431`
```rust
        parse_reg_num(reg_str).ok_or_else(|| format!("at: invalid register '{}'", reg_str))?
```
**Suggested fix:** Restrict AT Xt to 64-bit integer registers (Xn / XZR / LR).
```rust
        let rt = parse_reg_num(reg_str).ok_or_else(|| format!("at: invalid register '{}'", reg_str))?;
        let low = reg_str.trim().to_lowercase();
        let is_x = low.starts_with('x') || low == "xzr" || low == "lr";
        if !is_x {
            return Err(format!("at: Xt must be a 64-bit GPR, got '{}'", reg_str));
        }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_at_regression_w0 -- --test-threads=1 --nocapture
```
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_at_pbt::encode_at_neg_wrong_reg_class' (2584808) panicked at src/backend/arm/assembler/encoder/encode_at_pbt.rs:337:1:
Test failed: AT with non-X register must Err (llvm-mc rejects at s1e1r, w0); SUT raw "s1e1r, w0": Word(3574102016).
minimal failing input: op = "s1e1r", bad = "w0"
	successes: 0
	local rejects: 0
	global rejects: 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_at_pbt.rs
