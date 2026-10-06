# Bug: encode_dc accepts W, SP, and SIMD registers as Xt
**Law:** If llvm-mc / GNU as reject `dc <op>, <non-X-GPR>`, then encode_dc must return Err
**Impact:** `dc civac, w0` encodes as DC CIVAC, x0; `dc civac, sp` encodes as DC CIVAC, xzr. A 32-bit, stack, or SIMD register is silently treated as the corresponding 5-bit number, so a width/class typo is not diagnosed.
**Function:** encode_dc
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/system.rs:564
**Detected by:** Negative/Error Contract (wrong register class; llvm-mc/gas reject)
**Minimal input:** encode_dc([Symbol("civac"), Reg("w0")], "civac, w0")  (assembly: `dc civac, w0`)
**Expected:** Err (llvm-mc: invalid operand; gas: "operand mismatch" / "operand 2 must be an integer register")
**Actual:** Ok(Word(0xd50b7e20))  // DC CIVAC, x0
**Severity:** medium
**Root cause:** system.rs:573 calls parse_reg_num, which accepts w/sp/wsp/d/s/q/v/h/b, and encode_dc never checks that Xt is a 64-bit integer GPR.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/system.rs:573`
```rust
        Some(Operand::Reg(name)) => parse_reg_num(name).ok_or("invalid register for dc")?,
```
**Suggested fix:** Reject non-X GPRs (W, SP, SIMD) before encoding.
```rust
        Some(Operand::Reg(name)) => {
            if !is_64bit_reg(name) || name.eq_ignore_ascii_case("sp") {
                return Err(format!("dc: Xt must be a 64-bit GPR, got {name}"));
            }
            parse_reg_num(name).ok_or("invalid register for dc")?
        }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_dc_regression_w0 -- --test-threads=1 --nocapture
```
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_dc_pbt::encode_dc_neg_wrong_reg_class' (2568678) panicked at src/backend/arm/assembler/encoder/encode_dc_pbt.rs:455:1:
Test failed: DC with non-X register must Err (llvm-mc rejects dc civac, w0); SUT raw "civac, w0": Word(3574300192).
minimal failing input: op = "civac", bad = "w0"
	successes: 0
	local rejects: 0
	global rejects: 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_dc_pbt.rs
