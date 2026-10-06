# Bug: encode_sxtb treats SP/WSP as ZR
**Law:** ARM C6 SXTB/SBFM register 31 is ZR, not SP. llvm-mc rejects `sxtb wsp, w0` and `sxtb sp, x0` (`invalid operand for instruction`).
**Impact:** `sxtb wsp, w0` is encoded as `sxtb wzr, w0` (rd=31). Stack-pointer operands are silently rewritten to the zero register, so illegal assembly assembles to a different instruction.
**Function:** encode_sxtb
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/data_processing.rs:872
**Detected by:** Negative/Error Contract
**Minimal input:** `[Reg("wsp"), Reg("w0")]` (sxtb wsp, w0)
**Expected:** `Err`
**Actual:** `Ok(Word(0x13001c1f))` — parse_reg_num maps SP/WSP and XZR/WZR both to 31
**Severity:** medium
**Root cause:** encode_sxtb does not distinguish SP from ZR; get_reg/parse_reg_num map both to 31, and data_processing.rs:878 returns Ok(Word) with rd=31.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/data_processing.rs:873`
```rust
    let (rd, is_64) = get_reg(operands, 0)?;
    let (rn, _) = get_reg(operands, 1)?;
```
**Suggested fix:** Reject SP/WSP by name before encoding.
```rust
    if matches!(name.to_lowercase().as_str(), "sp" | "wsp") {
        return Err("sxtb: SP/WSP is not a valid operand (use ZR)".to_string());
    }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_sxtb_regression_sp -- --test-threads=1
```
**Raw output:**
```text
---- backend::arm::assembler::encoder::encode_sxtb_pbt::encode_sxtb_neg_sp stdout ----
Test failed: SP/WSP is not a valid SXTB operand (which=0 names=["wsp", "w0"]) at src/backend/arm/assembler/encoder/encode_sxtb_pbt.rs:360.
minimal failing input: which = 0, is_64_sp = false, a = 0, dest64 = false
	successes: 0
	local rejects: 0
	global rejects: 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_sxtb_pbt.rs
