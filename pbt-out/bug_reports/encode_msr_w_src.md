# Bug: encode_msr accepts Wt/SP/FP as Xt
**Law:** If llvm-mc / GNU as reject `msr sysreg, Wt` (MSR requires Xt, a 64-bit GPR, not SP), then encode_msr([Symbol(sysreg), Reg(Wt)]) must return Err
**Impact:** `msr sp_el0, w0` (also `sp`, `wsp`, `d0`, `s0`) assembles as `msr sp_el0, x0`. A width or class error is silently rewritten to Xt=0 / Rt=31.
**Function:** encode_msr
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/system.rs:262
**Detected by:** Negative/Error Contract (non-Xt source; llvm-mc rejects Wt/SP/FP)
**Minimal input:** encode_msr(&[Operand::Symbol("sp_el0".into()), Operand::Reg("w0".into())])  (assembly: `msr sp_el0, w0`)
**Expected:** Err (llvm-mc: "invalid operand for instruction")
**Actual:** Ok(Word(0xd5184100))  // encodes as msr sp_el0, x0
**Severity:** medium
**Root cause:** system.rs:295 `let (rt, _) = get_reg(operands, 1)?` discards `is_64`. `parse_reg_num` accepts w/sp/wsp/d/s/q/v/h/b prefixes and maps sp/wsp to 31.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/system.rs:295`
```rust
    let (rt, _) = get_reg(operands, 1)?;
```
**Suggested fix:** Require a 64-bit GPR (x0–x30 / xzr / lr) and reject SP.
```rust
    let (rt, is_64) = get_reg(operands, 1)?;
    if !is_64 {
        return Err("msr: Xt must be a 64-bit GPR".to_string());
    }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_msr_regression_w0_src -- --test-threads=1 --nocapture
```
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_msr_pbt::encode_msr_neg_wrong_src' panicked at src/backend/arm/assembler/encoder/encode_msr_pbt.rs:444:1:
Test failed: non-Xt source must Err (llvm-mc rejects msr sp_el0, w0), got Ok(Word(3575136512)) at src/backend/arm/assembler/encoder/encode_msr_pbt.rs:717.
minimal failing input: dest = "w0", name = "sp_el0"
	successes: 0
	local rejects: 0
	global rejects: 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_msr_pbt.rs
