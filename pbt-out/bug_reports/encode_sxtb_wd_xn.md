# Bug: encode_sxtb accepts SXTB Wd, Xn
**Law:** ARM C6 SXTB source is Wn; SXTB Wd, Xn is invalid and llvm-mc rejects it (`invalid operand for instruction`).
**Impact:** Mixed-width `sxtb w0, x0` is encoded as 32-bit SXTB with Rn taken from the X register number, producing a well-formed word for illegal assembly. Callers that mix W dest and X source get silent wrong code instead of an error.
**Function:** encode_sxtb
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/data_processing.rs:872
**Detected by:** Negative/Error Contract
**Minimal input:** `[Reg("w0"), Reg("x0")]` (sxtb w0, x0)
**Expected:** `Err`
**Actual:** `Ok(Word(0x13001c00))` — sf taken only from Rd; Rn width discarded
**Severity:** medium
**Root cause:** data_processing.rs:874 binds `(rn, _)` and discards the source width; sf comes only from Rd at data_processing.rs:873-875.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/data_processing.rs:874`
```rust
    let (rn, _) = get_reg(operands, 1)?;
```
**Suggested fix:** Require a W-register source when the destination is 32-bit (and reject Wd+Xn).
```rust
    let (rn, rn_is_64) = get_reg(operands, 1)?;
    if !is_64 && rn_is_64 {
        return Err("sxtb: 32-bit dest requires Wn source".to_string());
    }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_sxtb_regression_wd_xn -- --test-threads=1
```
**Raw output:**
```text
---- backend::arm::assembler::encoder::encode_sxtb_pbt::encode_sxtb_neg_wd_xn stdout ----
Test failed: SXTB Wd, Xn must Err (llvm-mc rejects sxtb w0, x0) at src/backend/arm/assembler/encoder/encode_sxtb_pbt.rs:338.
minimal failing input: rd = 0, rn = 0
	successes: 0
	local rejects: 0
	global rejects: 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_sxtb_pbt.rs
