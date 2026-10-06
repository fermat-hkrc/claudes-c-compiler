# Bug: encode_smaddl silently ignores a 5th operand
**Law:** Scalar SMADDL is a 4-operand instruction (`Xd, Wn, Wm, Xa`). A 5th operand must return Err, matching GNU as / llvm-mc (`invalid operand for instruction`).
**Impact:** Assembler accepts illegal syntax such as `smaddl x0, w0, w0, x0, x0` and emits the 4-operand encoding, so a typo or extra operand is silently dropped instead of failing the assemble.
**Function:** encode_smaddl
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/data_processing.rs:653
**Detected by:** Negative/Error Contract
**Minimal input:** `[Reg("x0"), Reg("w0"), Reg("w0"), Reg("x0"), Reg("x0")]` (shrunk; any extra Operand kind also accepted)
**Expected:** `Err`
**Actual:** `Ok(Word(0x9b200000))` — extra operand ignored; `get_reg` only reads indices 0..3
**Severity:** medium
**Root cause:** data_processing.rs:654-657 reads only operands 0..3 via get_reg and has no operands.len() upper bound, then returns Ok(Word) at data_processing.rs:659-661.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/data_processing.rs:654`
```rust
    let (rd, _) = get_reg(operands, 0)?;
    let (rn, _) = get_reg(operands, 1)?;
    let (rm, _) = get_reg(operands, 2)?;
    let (ra, _) = get_reg(operands, 3)?;
```
**Suggested fix:** Reject extra operands before encoding.
```rust
    if operands.len() != 4 {
        return Err(format!("smaddl: expected 4 operands, got {}", operands.len()));
    }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_smaddl_regression_extra_operand -- --test-threads=1
```
**Raw output:**
```text
---- backend::arm::assembler::encoder::encode_smaddl_pbt::encode_smaddl_neg_extra_operand stdout ----
Test failed: SMADDL has no 5th operand; extra operand must Err (llvm-mc rejects it) at src/backend/arm/assembler/encoder/encode_smaddl_pbt.rs:403.
minimal failing input: rd = 0, rn = 0, rm = 0, ra = 0, extra = Reg(
    "x0",
)
	successes: 0
	local rejects: 0
	global rejects: 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_smaddl_pbt.rs
