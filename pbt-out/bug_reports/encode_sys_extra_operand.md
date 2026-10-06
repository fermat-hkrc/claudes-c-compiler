# Bug: encode_sys ignores a sixth operand
**Law:** If llvm-mc / GNU as reject `sys #<op1>, Cn, Cm, #<op2>, Xt, extra`, then encode_sys must return Err
**Impact:** `sys #0, c0, c0, #0, x0, x0` (and any extra sixth operand) assembles as SYS with the first five fields. A stray operand is not diagnosed, so typos after Xt silently drop.
**Function:** encode_sys
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/system.rs:448
**Detected by:** Negative/Error Contract (extra operand; llvm-mc rejects)
**Minimal input:** encode_sys("#0, c0, c0, #0, x0, x0")  (assembly: `sys #0, c0, c0, #0, x0, x0`)
**Expected:** Err (llvm-mc: invalid operand; gas: unexpected characters following instruction at operand 5)
**Actual:** Ok(Word(0xd5080000))  // SYS #0, C0, C0, #0, x0
**Severity:** medium
**Root cause:** system.rs:451 only rejects parts.len() < 4. When parts.len() >= 5 the fifth token is used as Xt and every later comma-field is ignored.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/system.rs:451`
```rust
    if parts.len() < 4 {
        return Err(format!("sys needs at least 4 operands, got: {}", raw_operands));
    }
```
**Suggested fix:** Also reject more than five operands.
```rust
    if parts.len() < 4 || parts.len() > 5 {
        return Err(format!("sys needs 4 or 5 operands, got: {}", raw_operands));
    }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_sys_regression_extra_operand -- --test-threads=1 --nocapture
```
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_sys_pbt::encode_sys_neg_extra_operand' panicked at src/backend/arm/assembler/encoder/encode_sys_pbt.rs:429:37:
Test failed: SYS with extra operand must Err (llvm-mc rejects sys #0, c0, c0, #0, x0, x0); SUT raw "#0, c0, c0, #0, x0, x0": Word(3574071296).
minimal failing input: op1 = 0, crn = 0, crm = 0, op2 = 0, t = 0, extra = "x0"
	successes: 0
	local rejects: 0
	global rejects: 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_sys_pbt.rs
