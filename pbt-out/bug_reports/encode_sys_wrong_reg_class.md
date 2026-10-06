# Bug: encode_sys accepts W/SP/SIMD Xt as if they were X registers
**Law:** If llvm-mc / GNU as reject `sys #<op1>, Cn, Cm, #<op2>, <W/SP/SIMD>`, then encode_sys must return Err
**Impact:** `sys #0, c0, c0, #0, w0` (also sp, wsp, wzr, v0, d0, s0, q0, h0, b0) assembles as SYS with Xt equal to the numeric suffix. Invalid register-class assembly is not diagnosed and encodes as the 64-bit GPR of the same number.
**Function:** encode_sys
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/system.rs:448
**Detected by:** Negative/Error Contract (wrong register class; llvm-mc rejects)
**Minimal input:** encode_sys("#0, c0, c0, #0, w0")  (assembly: `sys #0, c0, c0, #0, w0`)
**Expected:** Err (llvm-mc: invalid operand; gas: operand mismatch / must be an integer register)
**Actual:** Ok(Word(0xd5080000))  // SYS #0, C0, C0, #0, x0
**Severity:** medium
**Root cause:** system.rs:462-463 calls parse_reg_num, which returns Some(n) for w/d/s/q/v/h/b prefixes and for sp/wsp/wzr. encode_sys never checks is_64bit_reg, so a W/SP/SIMD token is packed into Rt as if it were Xn.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/system.rs:462`
```rust
        let reg = parts[4].trim().to_lowercase();
        parse_reg_num(&reg).ok_or_else(|| format!("sys: invalid register: {}", parts[4]))?
```
**Suggested fix:** Require a 64-bit GPR (x0–x30, xzr, x31, lr) before packing Rt.
```rust
        let reg = parts[4].trim().to_lowercase();
        if !(reg.starts_with('x') || reg == "xzr" || reg == "lr" || reg == "fp") {
            return Err(format!("sys: invalid register: {}", parts[4]));
        }
        parse_reg_num(&reg).ok_or_else(|| format!("sys: invalid register: {}", parts[4]))?
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_sys_regression_w0 -- --test-threads=1 --nocapture
```
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_sys_pbt::encode_sys_neg_wrong_reg_class' panicked at src/backend/arm/assembler/encoder/encode_sys_pbt.rs:331:1:
Test failed: SYS with non-X register must Err (llvm-mc rejects sys #0, c0, c0, #0, w0); SUT raw "#0, c0, c0, #0, w0": Word(3574071296).
minimal failing input: op1 = 0, crn = 0, crm = 0, op2 = 0, bad = "w0"
	successes: 0
	local rejects: 0
	global rejects: 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_sys_pbt.rs
