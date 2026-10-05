# Bug: encode_neon_eor3 encodes arrangements other than .16B
**Law:** EOR3 is defined only for .16B; any other arrangement must be rejected
**Impact:** Illegal SHA3 assembly such as `eor3 v0.8b, v0.8b, v0.8b, v0.8b` is encoded as a valid 16B EOR3 word, so a typo produces silent wrong machine code instead of an assembler error
**Function:** encode_neon_eor3
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:1112
**Detected by:** Negative/Error Contract
**Minimal input:** encode_neon_eor3([v0.8b, v0.8b, v0.8b, v0.8b])
**Expected:** Err (ARM SHA3 EOR3 is 16B only; llvm-mc/gas reject .8b)
**Actual:** Ok(Word(0xce000000)) — same encoding as `eor3 v0.16b, v0.16b, v0.16b, v0.16b`
**Severity:** medium
**Root cause:** neon.rs:1115-1118 discards every arrangement (`let (rd, _)`), then always emits the 16B SHA3 word
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:1115`
```rust
    let (rd, _) = get_neon_reg(operands, 0)?;
    let (rn, _) = get_neon_reg(operands, 1)?;
    let (rm, _) = get_neon_reg(operands, 2)?;
    let (rk, _) = get_neon_reg(operands, 3)?;
```
**Suggested fix:** Require arrangement `16b` on every operand
```rust
    let (rd, arr_d) = get_neon_reg(operands, 0)?;
    let (rn, arr_n) = get_neon_reg(operands, 1)?;
    let (rm, arr_m) = get_neon_reg(operands, 2)?;
    let (rk, arr_k) = get_neon_reg(operands, 3)?;
    if arr_d != "16b" || arr_n != "16b" || arr_m != "16b" || arr_k != "16b" {
        return Err("eor3 requires .16b arrangement".to_string());
    }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_eor3_regression_invalid_t -- --test-threads=1 --exact
```
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_neon_eor3_pbt::encode_neon_eor3_neg_invalid_t' panicked at src/backend/arm/assembler/encoder/encode_neon_eor3_pbt.rs:146:1:
Test failed: T=8b must Err (ARM SHA3 EOR3 is 16B only; llvm-mc rejects eor3 v0.8b, v0.8b, v0.8b, v0.8b) at src/backend/arm/assembler/encoder/encode_neon_eor3_pbt.rs:297.
minimal failing input: rd = 0, rn = 0, rm = 0, rk = 0, t = "8b"
	successes: 0
	local rejects: 0
	global rejects: 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_neon_eor3_pbt.rs
