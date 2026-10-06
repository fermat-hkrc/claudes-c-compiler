# Bug: encode_sxtb accepts FP/SIMD registers as GPRs
**Law:** SXTB operands are general-purpose registers. llvm-mc rejects `sxtb d0, w1` (`invalid operand for instruction`).
**Impact:** FP/SIMD names (`d0`, `s0`, `q0`, `v0`, `h0`, `b0`) are parsed as register number N and encoded as Wd/Wn. `sxtb d0, w1` becomes the same word as `sxtb w0, w1`, so illegal SIMD operands assemble as GPR SXTB.
**Function:** encode_sxtb
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/data_processing.rs:872
**Detected by:** Negative/Error Contract
**Minimal input:** `[Reg("d0"), Reg("w1")]` (sxtb d0, w1)
**Expected:** `Err`
**Actual:** `Ok(Word(0x13001c20))` — parse_reg_num accepts d/s/q/v/h/b prefixes
**Severity:** medium
**Root cause:** encode_sxtb does not reject FP/SIMD names; parse_reg_num maps prefix+N to N, and data_processing.rs:878 returns Ok(Word) with rd=0.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/data_processing.rs:873`
```rust
    let (rd, is_64) = get_reg(operands, 0)?;
    let (rn, _) = get_reg(operands, 1)?;
```
**Suggested fix:** Reject floating-point / SIMD register names.
```rust
    if is_fp_reg(name) {
        return Err(format!("sxtb: FP/SIMD register {name} is not a valid operand"));
    }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_sxtb_regression_fp -- --test-threads=1
```
**Raw output:**
```text
---- backend::arm::assembler::encoder::encode_sxtb_pbt::encode_sxtb_neg_fp stdout ----
Test failed: FP/SIMD register d0 is not a valid SXTB operand (which=0) at src/backend/arm/assembler/encoder/encode_sxtb_pbt.rs:380.
minimal failing input: which = 0, prefix = "d", n = 0
	successes: 0
	local rejects: 0
	global rejects: 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_sxtb_pbt.rs
