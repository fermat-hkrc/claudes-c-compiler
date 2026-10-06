# Bug: encode_crc32 ignores extra operands
**Law:** CRC32 instructions take exactly three registers (Wd, Wn, Wm|Xm); a trailing fourth operand must be rejected.
**Impact:** The assembler silently encodes `crc32b w0, w1, w2, w3` as `crc32b w0, w1, w2`, so invalid GNU-style assembly produces a machine-code word. llvm-mc and gas reject the extra operand.
**Function:** encode_crc32
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/bitfield.rs:226
**Detected by:** Negative/Error Contract
**Minimal input:** encode_crc32("crc32b", [Reg("w0"), Reg("w0"), Reg("w0"), Reg("x0")])
**Expected:** Err
**Actual:** Ok(Word(0x1ac04000)) — extra operand is never read
**Severity:** medium
**Root cause:** bitfield.rs:227-229 reads only operands 0..2 via get_reg and has no operands.len() upper bound, then returns Ok(Word) at bitfield.rs:244-245.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/bitfield.rs:227`
```rust
    let (rd, _) = get_reg(operands, 0)?;
    let (rn, _) = get_reg(operands, 1)?;
    let (rm, _) = get_reg(operands, 2)?;
```
**Suggested fix:** Reject extra operands before encoding.
```rust
    if operands.len() != 3 {
        return Err(format!("crc32: expected 3 operands, got {}", operands.len()));
    }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_crc32_regression_extra_operand -- --test-threads=1
```
**Raw output:**
```text
---- backend::arm::assembler::encoder::encode_crc32_pbt::encode_crc32_neg_extra_operand stdout ----
Test failed: CRC32 has no 4th operand; extra operand must Err (llvm-mc rejects it) at src/backend/arm/assembler/encoder/encode_crc32_pbt.rs:391.
minimal failing input: m = "crc32b", rd = 0, rn = 0, rm = 0, extra = Reg(
    "x0",
)
	successes: 0
	local rejects: 0
	global rejects: 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_crc32_pbt.rs
