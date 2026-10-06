# Bug: encode_uxtb silently ignores a 3rd operand
**Law:** Scalar UXTB is a 2-operand instruction (`Rd, Wn`). A 3rd operand must return Err, matching GNU as / llvm-mc (`invalid operand for instruction`).
**Impact:** The assembler accepts illegal syntax such as `uxtb w0, w0, x0` and emits the 2-operand encoding, so a typo or leftover operand is silently dropped instead of failing the assemble.
**Function:** encode_uxtb
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/data_processing.rs:900
**Detected by:** Negative/Error Contract
**Minimal input:** `[Reg("w0"), Reg("w0"), Reg("x0")]` (shrunk; any extra Operand kind also accepted)
**Expected:** `Err`
**Actual:** `Ok(Word(0x53001C00))` — extra operand ignored; `get_reg` only reads indices 0 and 1
**Severity:** medium
**Root cause:** data_processing.rs:901-902 reads only operands 0 and 1 via get_reg and has no operands.len() upper bound, then returns Ok(Word) at data_processing.rs:906.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/data_processing.rs:901`
```rust
    let (rd, is_64) = get_reg(operands, 0)?;
    let (rn, _) = get_reg(operands, 1)?;
```
**Suggested fix:** Reject extra operands before encoding.
```rust
    if operands.len() != 2 {
        return Err(format!("uxtb: expected 2 operands, got {}", operands.len()));
    }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_uxtb_regression_extra_operand -- --test-threads=1
```
**Raw output:**
```text
---- backend::arm::assembler::encoder::encode_uxtb_pbt::encode_uxtb_neg_extra_operand stdout ----
Test failed: UXTB has no 3rd operand; extra operand must Err (llvm-mc rejects it) at src/backend/arm/assembler/encoder/encode_uxtb_pbt.rs:310.
minimal failing input: rd = 0, rn = 0, extra = Reg(
    "x0",
)
	successes: 0
	local rejects: 0
	global rejects: 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_uxtb_pbt.rs
