# Bug: encode_neon_bsl ignores mismatched Vn/Vm arrangements
**Law:** BSL requires Vd, Vn, and Vm to share the same arrangement T
**Impact:** `bsl v0.8b, v0.8b, v0.16b` is encoded as Q=0 BSL using only the destination T, so mixed-width assembly is accepted and the Q bit does not reflect the sources
**Function:** encode_neon_bsl
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:735
**Detected by:** Negative/Error Contract
**Minimal input:** encode_neon_bsl([v0.8b, v0.8b, v0.16b])
**Expected:** Err (llvm-mc: invalid operand on the mismatched source)
**Actual:** Ok(Word(0x2e601c00)) — same as `bsl v0.8b, v0.8b, v0.8b`
**Severity:** medium
**Root cause:** neon.rs:740-741 discard source arrangements (`let (rn, _)`, `let (rm, _)`) and Q is taken only from dest
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:740`
```rust
    let (rn, _) = get_neon_reg(operands, 1)?;
    let (rm, _) = get_neon_reg(operands, 2)?;
```
**Suggested fix:** Require Vn and Vm arrangements to equal dest T
```rust
    let (rn, arr_n) = get_neon_reg(operands, 1)?;
    let (rm, arr_m) = get_neon_reg(operands, 2)?;
    if arr_n != arr_d || arr_m != arr_d {
        return Err("bsl: operand arrangement mismatch".to_string());
    }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_bsl_regression_mismatch_t -- --test-threads=1 --exact
```
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_neon_bsl_pbt::encode_neon_bsl_neg_mismatch_t' panicked at src/backend/arm/assembler/encoder/encode_neon_bsl_pbt.rs:165:1:
Test failed: mismatched T must Err (llvm-mc rejects bsl v0.8b, v0.8b, v0.16b) at src/backend/arm/assembler/encoder/encode_neon_bsl_pbt.rs:316.
minimal failing input: rd = 0, rn = 0, rm = 0, td = "8b", tn = "8b", tm = "16b"
	successes: 0
	local rejects: 0
	global rejects: 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_neon_bsl_pbt.rs
