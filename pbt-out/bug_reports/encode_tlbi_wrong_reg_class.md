# Bug: encode_tlbi accepts W/SP/SIMD registers as TLBI Xt
**Law:** TLBI Xt must be a 64-bit GPR, XZR, or LR; W, SP, WSP, WZR, and SIMD registers are invalid
**Impact:** `tlbi vale1is, w0` encodes identically to `tlbi vale1is, x0`. A 32-bit or SIMD operand is silently treated as the corresponding X register number, so the assembler produces a 64-bit SYS instruction GNU gas / llvm-mc refuse
**Function:** encode_tlbi
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/system.rs:477
**Detected by:** Negative/Error Contract
**Minimal input:** encode_tlbi([], "vale1is, w0")
**Expected:** Err (llvm-mc: "invalid operand for instruction")
**Actual:** Ok(Word(0xd50883a0)) — same encoding as `vale1is, x0`
**Severity:** medium
**Root cause:** system.rs:482 calls parse_reg_num, which accepts w/sp/wsp/wzr and SIMD prefixes, and encode_tlbi never checks is_64bit_reg
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/system.rs:482`
```rust
        parse_reg_num(reg_str).ok_or_else(|| format!("tlbi: invalid register '{}'", reg_str))?
```
**Suggested fix:** After parsing, require a 64-bit GPR name (x0–x30, xzr, x31, lr); reject W/SP/SIMD
```rust
        let rt = parse_reg_num(reg_str).ok_or_else(|| format!("tlbi: invalid register '{}'", reg_str))?;
        if !is_64bit_gpr(reg_str) {
            return Err(format!("tlbi: Xt must be a 64-bit GPR, got '{}'", reg_str));
        }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_tlbi_regression_w0 -- --test-threads=1
```
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_tlbi_pbt::test_encode_tlbi_regression_w0' panicked at src/backend/arm/assembler/encoder/encode_tlbi_pbt.rs:383:36:
TLBI Xt must be a 64-bit GPR (llvm-mc: invalid operand for instruction): Word(3574104864)
Test failed: TLBI with non-X register must Err (llvm-mc rejects tlbi vale1is, w0); SUT raw "vale1is, w0": Word(3574104992).
minimal failing input: op = "vale1is", bad = "w0"
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_tlbi_pbt.rs
