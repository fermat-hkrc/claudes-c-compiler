# Bug: encode_mneg silently ignores a 4th operand
**Law:** Scalar MNEG is a 3-operand instruction (`Rd, Rn, Rm`). A 4th operand must return Err, matching GNU as / llvm-mc (`invalid operand for instruction`).
**Impact:** Assembler accepts illegal syntax such as `mneg w0, w0, w0, x0` and emits the 3-operand encoding, so a typo or extra operand is silently dropped instead of failing the assemble.
**Function:** encode_mneg
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/data_processing.rs:677
**Detected by:** Negative/Error Contract
**Minimal input:** `[Reg("w0"), Reg("w0"), Reg("w0"), Reg("x0")]` (shrunk; any extra Operand kind also accepted)
**Expected:** `Err`
**Actual:** `Ok(Word(0x1b00fc00))` — extra operand ignored; `get_reg` only reads indices 0..2
**Severity:** medium
**Root cause:** data_processing.rs:678-680 reads only operands 0..2 via get_reg and has no operands.len() upper bound, then returns Ok(Word) at data_processing.rs:685.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/data_processing.rs:678`
```rust
    let (rd, is_64) = get_reg(operands, 0)?;
    let (rn, _) = get_reg(operands, 1)?;
    let (rm, _) = get_reg(operands, 2)?;
```
**Suggested fix:** Reject extra operands before encoding.
```rust
    if operands.len() != 3 {
        return Err(format!("mneg: expected 3 operands, got {}", operands.len()));
    }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_mneg_regression_extra_operand -- --test-threads=1
```
**Raw output:**
```text
---- backend::arm::assembler::encoder::encode_mneg_pbt::encode_mneg_neg_extra_operand stdout ----
Test failed: MNEG has no 4th operand; extra operand must Err (llvm-mc rejects it) at src/backend/arm/assembler/encoder/encode_mneg_pbt.rs:393.
minimal failing input: rd = 0, rn = 0, rm = 0, is_64 = false, extra = Reg(
    "x0",
)
	successes: 0
	local rejects: 0
	global rejects: 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_mneg_pbt.rs
