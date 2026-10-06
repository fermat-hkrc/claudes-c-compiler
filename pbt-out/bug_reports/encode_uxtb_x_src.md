# Bug: encode_uxtb accepts an X-register source
**Law:** UXTB source is Wn. An X-register source must return Err, matching GNU as (`operand mismatch`) / llvm-mc (`invalid operand for instruction`).
**Impact:** Mixed-width `uxtb w0, x0` is encoded as 32-bit UXTB with Rn taken from the X register number, so illegal assembly becomes a well-formed word.
**Function:** encode_uxtb
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/data_processing.rs:900
**Detected by:** Negative/Error Contract
**Minimal input:** `[Reg("w0"), Reg("x0")]` (shrunk; dest64=false, rd=0, rn=0)
**Expected:** `Err`
**Actual:** `Ok(Word(0x53001C00))`
**Severity:** medium
**Root cause:** data_processing.rs:902 binds `(rn, _)` and discards the source width; ARM C6 source is Wn.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/data_processing.rs:902`
```rust
    let (rn, _) = get_reg(operands, 1)?;
```
**Suggested fix:** Require a W-register source.
```rust
    let (rn, rn_is_64) = get_reg(operands, 1)?;
    if rn_is_64 {
        return Err("uxtb: source must be a W register".to_string());
    }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_uxtb_regression_x_src -- --test-threads=1
```
**Raw output:**
```text
---- backend::arm::assembler::encoder::encode_uxtb_pbt::encode_uxtb_neg_x_src stdout ----
Test failed: UXTB w0, x0 must Err (llvm-mc rejects X-register source) at src/backend/arm/assembler/encoder/encode_uxtb_pbt.rs:323.
minimal failing input: rd = 0, rn = 0, dest64 = false
	successes: 0
	local rejects: 0
	global rejects: 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_uxtb_pbt.rs
