# Bug: encode_mneg accepts mixed W/X register widths
**Law:** MNEG Rd, Rn, Rm requires all three GPRs to be the same width (all X or all W). Mixed width must return Err, matching GNU as / llvm-mc (`invalid operand for instruction`) and the ARM ARM Data-processing (3 source) sf bit.
**Impact:** Assembler accepts illegal syntax such as `mneg w0, w0, x0` and encodes it as 32-bit MNEG using only Rd's width, so a mixed-width typo assembles to the wrong instruction instead of failing.
**Function:** encode_mneg
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/data_processing.rs:677
**Detected by:** Negative/Error Contract
**Minimal input:** `[Reg("w0"), Reg("w0"), Reg("x0")]` (rd64=false, rn64=false, rm64=true)
**Expected:** `Err`
**Actual:** `Ok(Word(0x1b00fc00))` — sf taken only from Rd; Rn/Rm widths discarded
**Severity:** medium
**Root cause:** data_processing.rs:678-681 binds is_64 only from operand 0 and discards the bool from get_reg on Rn/Rm, then sets sf from that dest-only flag.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/data_processing.rs:678`
```rust
    let (rd, is_64) = get_reg(operands, 0)?;
    let (rn, _) = get_reg(operands, 1)?;
    let (rm, _) = get_reg(operands, 2)?;
    let sf = sf_bit(is_64);
```
**Suggested fix:** Require matching widths before encoding.
```rust
    let (rd, rd64) = get_reg(operands, 0)?;
    let (rn, rn64) = get_reg(operands, 1)?;
    let (rm, rm64) = get_reg(operands, 2)?;
    if rd64 != rn64 || rn64 != rm64 {
        return Err("mneg: register size mismatch".into());
    }
    let sf = sf_bit(rd64);
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_mneg_regression_mixed_width -- --test-threads=1
```
**Raw output:**
```text
---- backend::arm::assembler::encoder::encode_mneg_pbt::encode_mneg_neg_mixed_width stdout ----
Test failed: mixed-width MNEG registers must Err (rd64=false rn64=false rm64=true) at src/backend/arm/assembler/encoder/encode_mneg_pbt.rs:414.
minimal failing input: rd = 0, rn = 0, rm = 0, rd64 = false, rn64 = false, rm64 = true
	successes: 0
	local rejects: 0
	global rejects: 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_mneg_pbt.rs
