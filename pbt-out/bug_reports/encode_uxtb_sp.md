# Bug: encode_uxtb treats SP/WSP as ZR
**Law:** UXTB Rd/Rn are ZR at register 31, not SP. SP/WSP must return Err, matching llvm-mc (`invalid operand for instruction`).
**Impact:** `uxtb wsp, w0` encodes as `uxtb wzr, w0`. Using the stack pointer as a UXTB operand is silently rewritten to ZR.
**Function:** encode_uxtb
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/data_processing.rs:900
**Detected by:** Negative/Error Contract
**Minimal input:** `[Reg("wsp"), Reg("w0")]` (shrunk; which=0, is_64_sp=false, a=0)
**Expected:** `Err`
**Actual:** `Ok(Word)` — parse_reg_num maps SP/WSP to 31, same as WZR
**Severity:** medium
**Root cause:** data_processing.rs:901 calls get_reg, which uses parse_reg_num; parse_reg_num maps "sp"|"wsp" and "xzr"|"wzr" both to 31, and encode_uxtb does not distinguish SP from ZR.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/data_processing.rs:901`
```rust
    let (rd, is_64) = get_reg(operands, 0)?;
```
**Suggested fix:** Reject SP/WSP before encoding.
```rust
    fn is_sp(name: &str) -> bool {
        matches!(name.to_ascii_lowercase().as_str(), "sp" | "wsp")
    }
    if operands.iter().any(|op| matches!(op, Operand::Reg(n) if is_sp(n))) {
        return Err("uxtb: SP/WSP is not a valid operand".to_string());
    }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_uxtb_regression_sp -- --test-threads=1
```
**Raw output:**
```text
---- backend::arm::assembler::encoder::encode_uxtb_pbt::encode_uxtb_neg_sp stdout ----
Test failed: SP/WSP is not a valid UXTB operand (which=0 names=["wsp", "w0"]) at src/backend/arm/assembler/encoder/encode_uxtb_pbt.rs:345.
minimal failing input: which = 0, is_64_sp = false, a = 0, dest64 = false
	successes: 0
	local rejects: 0
	global rejects: 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_uxtb_pbt.rs
