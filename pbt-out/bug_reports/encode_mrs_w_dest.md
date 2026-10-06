# Bug: encode_mrs accepts a 32-bit Wt destination
**Law:** MRS requires Xt (a 64-bit GPR). If llvm-mc rejects `mrs Wt, sysreg`, then encode_mrs([Reg(Wt), Symbol(sysreg)]) must return Err. The function comment "MRS Xt, system_reg" asserts this form.
**Impact:** `mrs w0, sp_el0` assembles as `mrs x0, sp_el0`. A width typo is silently rewritten to the 64-bit encoding, so the object file does not match the source the author wrote.
**Function:** encode_mrs
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/system.rs:54
**Detected by:** Negative/Error Contract (non-Xt dest; llvm-mc rejects W/SP/FP)
**Minimal input:** encode_mrs(&[Operand::Reg("w0".into()), Operand::Symbol("sp_el0".into())])  (assembly: `mrs w0, sp_el0`)
**Expected:** Err (llvm-mc: "invalid operand for instruction")
**Actual:** Ok(Word(0xd5384100))  // same word as mrs x0, sp_el0
**Severity:** medium
**Root cause:** system.rs:56 `let (rt, _) = get_reg(operands, 0)?` discards the is_64 flag. parse_reg_num also accepts w/sp/d/s/q/v/h/b prefixes, so Wt, SP, and SIMD registers encode as Rt without a class check.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/system.rs:56`
```rust
    let (rt, _) = get_reg(operands, 0)?;
```
**Suggested fix:** Require a 64-bit GPR that is not SP.
```rust
    let (rt, is_64) = get_reg(operands, 0)?;
    if !is_64 || matches!(operands.get(0), Some(Operand::Reg(n)) if n.eq_ignore_ascii_case("sp")) {
        return Err("mrs: destination must be Xt".to_string());
    }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_mrs_regression_w0_dest -- --test-threads=1 --nocapture
```
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_mrs_pbt::encode_mrs_neg_wrong_dest' panicked at src/backend/arm/assembler/encoder/encode_mrs_pbt.rs:419:1:
Test failed: non-Xt dest must Err (llvm-mc rejects mrs w0, sp_el0), got Ok(Word(3577233664)) at src/backend/arm/assembler/encoder/encode_mrs_pbt.rs:604.
minimal failing input: dest = "w0", name = "sp_el0"
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_mrs_pbt.rs
