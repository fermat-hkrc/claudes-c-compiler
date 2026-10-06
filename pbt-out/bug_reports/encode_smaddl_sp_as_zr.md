# Bug: encode_smaddl encodes SP/WSP as XZR/WZR
**Law:** SMADDL register 31 is XZR/WZR, not SP/WSP. SP or WSP in any operand slot must return Err, matching GNU as / llvm-mc (`invalid operand for instruction`).
**Impact:** Assembler accepts `smaddl wsp, w0, w0, x0` and emits the encoding for `smaddl xzr, w0, w0, x0`. Using the stack pointer as a multiply-add operand is silently rewritten to the zero register.
**Function:** encode_smaddl
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/data_processing.rs:653
**Detected by:** Negative/Error Contract
**Minimal input:** `[Reg("wsp"), Reg("w0"), Reg("w0"), Reg("x0")]`
**Expected:** `Err`
**Actual:** `Ok(Word(0x9b20001f))` — wsp maps to 31 (XZR)
**Severity:** medium
**Root cause:** encoder/mod.rs:270 parse_reg_num maps "sp" and "wsp" to 31, the same number as xzr/wzr; encode_smaddl:654-657 does not distinguish SP from ZR.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/data_processing.rs:654`
```rust
    let (rd, _) = get_reg(operands, 0)?;
    let (rn, _) = get_reg(operands, 1)?;
    let (rm, _) = get_reg(operands, 2)?;
    let (ra, _) = get_reg(operands, 3)?;
```
**Suggested fix:** Reject SP/WSP in every SMADDL slot.
```rust
    fn not_sp(name: &str) -> Result<(), String> {
        let n = name.to_lowercase();
        if n == "sp" || n == "wsp" {
            Err("smaddl: SP/WSP is not a valid operand".into())
        } else {
            Ok(())
        }
    }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_smaddl_regression_sp -- --test-threads=1
```
**Raw output:**
```text
---- backend::arm::assembler::encoder::encode_smaddl_pbt::encode_smaddl_neg_sp stdout ----
Test failed: SP/WSP is not a valid SMADDL operand (which=0 names=["wsp", "w0", "w0", "x0"]) at src/backend/arm/assembler/encoder/encode_smaddl_pbt.rs:453.
minimal failing input: which = 0, is_64 = false, a = 0, b = 0
	successes: 0
	local rejects: 0
	global rejects: 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_smaddl_pbt.rs
