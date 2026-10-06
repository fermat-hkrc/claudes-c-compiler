# Bug: encode_mneg encodes FP/SIMD registers as GPRs
**Law:** MNEG operands are general-purpose registers. An FP/SIMD name (d/s/q/v/h/b prefix) must return Err, matching GNU as / llvm-mc (`invalid operand for instruction`).
**Impact:** Assembler accepts illegal syntax such as `mneg d0, w1, w2` and emits the encoding for `mneg w0, w1, w2`, so an FP register typo is silently rewritten as the same-numbered GPR.
**Function:** encode_mneg
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/data_processing.rs:677
**Detected by:** Negative/Error Contract
**Minimal input:** `[Reg("d0"), Reg("w1"), Reg("w2")]` (which=0, prefix="d", n=0, is_64=false)
**Expected:** `Err`
**Actual:** `Ok(Word(0x1b02fc20))` — parse_reg_num accepts prefix 'd' and number 0; is_fp_reg is never called
**Severity:** medium
**Root cause:** data_processing.rs:678 calls get_reg / parse_reg_num, which accepts d/s/q/v/h/b prefixes as GPR numbers; encode_mneg never consults is_fp_reg.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/data_processing.rs:678`
```rust
    let (rd, is_64) = get_reg(operands, 0)?;
    let (rn, _) = get_reg(operands, 1)?;
    let (rm, _) = get_reg(operands, 2)?;
```
**Suggested fix:** Reject FP/SIMD register names before encoding.
```rust
    if is_fp_reg(name) {
        return Err(format!("mneg: FP/SIMD register {} is not a valid operand", name));
    }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_mneg_regression_fp -- --test-threads=1
```
**Raw output:**
```text
---- backend::arm::assembler::encoder::encode_mneg_pbt::encode_mneg_neg_fp stdout ----
Test failed: FP/SIMD register d0 is not a valid MNEG operand (which=0) at src/backend/arm/assembler/encoder/encode_mneg_pbt.rs:464.
minimal failing input: which = 0, prefix = "d", n = 0, is_64 = false
	successes: 0
	local rejects: 0
	global rejects: 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_mneg_pbt.rs
