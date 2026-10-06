# Bug: encode_ic accepts W/SP/SIMD registers as IVAU Xt
**Law:** If llvm-mc / GNU as reject `ic ivau, Wn` / `ic ivau, sp` / `ic ivau, Dn` (Xt must be a 64-bit GPR), then encode_ic("ivau, <non-X>") must return Err
**Impact:** `ic ivau, w0` encodes identically to `ic ivau, x0` (0xd50b7520). A 32-bit, SP, or SIMD register is silently treated as the corresponding 5-bit encoding, so a width/class typo is not diagnosed.
**Function:** encode_ic
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/system.rs:401
**Detected by:** Negative/Error Contract (wrong register class on IVAU; llvm-mc rejects)
**Minimal input:** encode_ic("ivau, w0")  (assembly: `ic ivau, w0`)
**Expected:** Err (llvm-mc: "invalid operand for instruction"; gas: "operand mismatch")
**Actual:** Ok(Word(0xd50b7520))  // same encoding as `ic ivau, x0`
**Severity:** medium
**Root cause:** system.rs:406 uses parse_reg_num, which accepts W/SP/WZR/WSP and SIMD/FP prefixes (d/s/q/v/h/b) as 5-bit numbers, with no 64-bit GPR check for IVAU Xt.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/system.rs:406`
```rust
        parse_reg_num(reg_str).ok_or_else(|| format!("ic: invalid register '{}'", reg_str))?
```
**Suggested fix:** Restrict IVAU Xt to 64-bit integer registers (Xn / XZR / LR).
```rust
        let rt = parse_reg_num(reg_str).ok_or_else(|| format!("ic: invalid register '{}'", reg_str))?;
        let low = reg_str.trim().to_lowercase();
        let is_x = low.starts_with('x') || low == "xzr" || low == "lr";
        if !is_x {
            return Err(format!("ic: Xt must be a 64-bit GPR, got '{}'", reg_str));
        }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_ic_regression_ivau_w0 -- --test-threads=1 --nocapture
```
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_ic_pbt::encode_ic_neg_wrong_reg_class' (2566626) panicked at src/backend/arm/assembler/encoder/encode_ic_pbt.rs:345:1:
Test failed: IVAU with non-X register must Err (llvm-mc rejects ic ivau, w0); SUT raw "ivau, w0": Word(3574297888).
minimal failing input: bad = "w0"
	successes: 0
	local rejects: 0
	global rejects: 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_ic_pbt.rs
