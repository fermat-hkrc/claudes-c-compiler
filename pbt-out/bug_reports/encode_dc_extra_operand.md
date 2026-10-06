# Bug: encode_dc ignores a third operand
**Law:** If llvm-mc / GNU as reject `dc <op>, Xt, extra`, then encode_dc must return Err
**Impact:** `dc civac, x0, x0` (and any extra third operand) assembles as DC CIVAC, x0. A stray operand is not diagnosed.
**Function:** encode_dc
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/system.rs:564
**Detected by:** Negative/Error Contract (extra operand; llvm-mc rejects)
**Minimal input:** encode_dc([Symbol("civac"), Reg("x0"), Reg("x0")], "civac, x0, x0")  (assembly: `dc civac, x0, x0`)
**Expected:** Err (llvm-mc: unexpected characters following instruction; gas: same)
**Actual:** Ok(Word(0xd50b7e20))  // DC CIVAC, x0
**Severity:** medium
**Root cause:** system.rs:572 takes Rt from operands.get(1) and never checks operands.len(), so a third operand is ignored.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/system.rs:572`
```rust
    let rt = match operands.get(1) {
```
**Suggested fix:** Reject anything other than exactly two operands (op, Xt).
```rust
    if operands.len() != 2 {
        return Err(format!("dc: unexpected extra operand in {}", raw_operands));
    }
    let rt = match operands.get(1) {
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_dc_regression_extra_operand -- --test-threads=1 --nocapture
```
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_dc_pbt::encode_dc_neg_extra_operand' (2568621) panicked at src/backend/arm/assembler/encoder/encode_dc_pbt.rs:455:1:
Test failed: DC with extra operand must Err (llvm-mc rejects dc civac, x0, x0); SUT raw "civac, x0, x0": Word(3574300192).
minimal failing input: op = "civac", xt = "x0", extra = "x0"
	successes: 0
	local rejects: 0
	global rejects: 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_dc_pbt.rs
