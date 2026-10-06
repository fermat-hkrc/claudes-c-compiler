# Bug: encode_sxtb silently ignores a 3rd operand
**Law:** Scalar SXTB is a 2-operand instruction (`Rd, Wn`). A 3rd operand must return Err, matching GNU as / llvm-mc (`invalid operand for instruction`).
**Impact:** The assembler accepts illegal syntax such as `sxtb w0, w0, x0` and emits the 2-operand encoding, so a typo or leftover operand is silently dropped instead of failing the assemble.
**Function:** encode_sxtb
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/data_processing.rs:872
**Detected by:** Negative/Error Contract
**Minimal input:** `[Reg("w0"), Reg("w0"), Reg("x0")]` (shrunk; any extra Operand kind also accepted)
**Expected:** `Err`
**Actual:** `Ok(Word(0x13001c00))` — extra operand ignored; `get_reg` only reads indices 0 and 1
**Severity:** medium
**Root cause:** data_processing.rs:873-874 reads only operands 0 and 1 via get_reg and has no operands.len() upper bound, then returns Ok(Word) at data_processing.rs:878.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/data_processing.rs:873`
```rust
    let (rd, is_64) = get_reg(operands, 0)?;
    let (rn, _) = get_reg(operands, 1)?;
```
**Suggested fix:** Reject extra operands before encoding.
```rust
    if operands.len() != 2 {
        return Err(format!("sxtb: expected 2 operands, got {}", operands.len()));
    }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_sxtb_regression_extra_operand -- --test-threads=1
```
**Raw output:**
```text
---- backend::arm::assembler::encoder::encode_sxtb_pbt::encode_sxtb_neg_extra_operand stdout ----
Test failed: SXTB has no 3rd operand; extra operand must Err (llvm-mc rejects it) at src/backend/arm/assembler/encoder/encode_sxtb_pbt.rs:326.
minimal failing input: rd = 0, rn = 0, is_64 = false, extra = Reg(
    "x0",
)
	successes: 0
	local rejects: 0
	global rejects: 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_sxtb_pbt.rs
