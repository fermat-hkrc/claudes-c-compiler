# Bug: encode_neon_two_misc ignores a third operand
**Law:** Integer two-register misc (ABS/NEG/CLS/…) takes exactly two operands; a third operand must be rejected
**Impact:** The assembler silently encodes `abs Vd.T, Vn.T, extra` as `abs Vd.T, Vn.T`, dropping the extra operand instead of diagnosing invalid assembly
**Function:** encode_neon_two_misc
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:1407
**Detected by:** Negative/Error Contract
**Minimal input:** encode_neon_two_misc([v0.8b, v0.8b, v0.8b], u_bit=0, opcode=0b01011)
**Expected:** Err (llvm-mc/gas reject a third operand)
**Actual:** Ok(Word) — same as `abs v0.8b, v0.8b`
**Severity:** medium
**Root cause:** neon.rs:1408-1409 only reads operands[0] and operands[1]; there is no arity check, so extra operands are ignored
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:1408`
```rust
    let (rd, arr_d) = get_neon_reg(operands, 0)?;
    let (rn, _) = get_neon_reg(operands, 1)?;
```
**Suggested fix:** Reject arity other than 2
```rust
    if operands.len() != 2 {
        return Err("NEON two-misc requires 2 operands".to_string());
    }
    let (rd, arr_d) = get_neon_reg(operands, 0)?;
    let (rn, _) = get_neon_reg(operands, 1)?;
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_two_misc_regression_extra_operand -- --test-threads=1 --exact
```
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_neon_two_misc_pbt::encode_neon_two_misc_neg_extra' (2372485) panicked at src/backend/arm/assembler/encoder/encode_neon_two_misc_pbt.rs:356:1:
Test failed: extra operand must Err (llvm-mc rejects abs v0.8b, v0.8b, v0.8b) at src/backend/arm/assembler/encoder/encode_neon_two_misc_pbt.rs:470.
minimal failing input: rd = 0, rn = 0, extra = 0, t = "8b"
	successes: 0
	local rejects: 0
	global rejects: 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_neon_two_misc_pbt.rs
