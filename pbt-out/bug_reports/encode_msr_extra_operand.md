# Bug: encode_msr ignores extra operands
**Law:** If llvm-mc / GNU as reject `msr sysreg, Xt, extra`, then encode_msr([Symbol(sysreg), Reg(Xt), extra]) must return Err
**Impact:** Typos such as `msr sp_el0, x0, x0` assemble as a silent `msr sp_el0, x0` instead of being rejected. An extra operand that should have been a parse/encode error is dropped.
**Function:** encode_msr
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/system.rs:262
**Detected by:** Negative/Error Contract (extra operand; llvm-mc rejects arity > 2)
**Minimal input:** encode_msr(&[Operand::Symbol("sp_el0".into()), Operand::Reg("x0".into()), Operand::Reg("x0".into())])  (assembly: `msr sp_el0, x0, x0`)
**Expected:** Err (llvm-mc: "invalid operand")
**Actual:** Ok(Word)  // encodes as msr sp_el0, x0
**Severity:** medium
**Root cause:** system.rs:263-295 examine only operand 0 (sysreg) and operand 1 (Xt or imm); `operands.len()` is never checked, so trailing operands are ignored.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/system.rs:295`
```rust
    let (rt, _) = get_reg(operands, 1)?;
```
**Suggested fix:** Reject a slice longer than two operands before encoding.
```rust
    if operands.len() != 2 {
        return Err("msr: expected system_reg, Xt".to_string());
    }
    let (rt, _) = get_reg(operands, 1)?;
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_msr_regression_extra_x0 -- --test-threads=1 --nocapture
```
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_msr_pbt::encode_msr_neg_extra' panicked at src/backend/arm/assembler/encoder/encode_msr_pbt.rs:444:1:
Test failed: extra operand must Err (llvm-mc rejects msr sp_el0, x0, x0) at src/backend/arm/assembler/encoder/encode_msr_pbt.rs:578.
minimal failing input: name = "sp_el0", xt = "x0", extra = Reg(
    "x0",
)
	successes: 0
	local rejects: 0
	global rejects: 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_msr_pbt.rs
