# Bug: encode_mneg encodes SP/WSP as ZR
**Law:** ARM ARM Data-processing (3 source) register 31 is XZR/WZR, not SP/WSP. MNEG with sp or wsp in any slot must return Err, matching GNU as / llvm-mc (`invalid operand for instruction`).
**Impact:** Assembler accepts illegal syntax such as `mneg wsp, w0, w0` and emits the encoding for `mneg wzr, w0, w0`, so a stack-pointer operand is silently rewritten as the zero register.
**Function:** encode_mneg
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/data_processing.rs:677
**Detected by:** Negative/Error Contract
**Minimal input:** `[Reg("wsp"), Reg("w0"), Reg("w0")]` (which=0, is_64=false, a=0, b=0)
**Expected:** `Err`
**Actual:** `Ok(Word(0x1b00fc1f))` — parse_reg_num maps wsp to 31, same as wzr
**Severity:** medium
**Root cause:** data_processing.rs:678 calls get_reg, which uses parse_reg_num; that helper maps both "sp"/"wsp" and "xzr"/"wzr" to 31, and encode_mneg never distinguishes them.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/data_processing.rs:678`
```rust
    let (rd, is_64) = get_reg(operands, 0)?;
    let (rn, _) = get_reg(operands, 1)?;
    let (rm, _) = get_reg(operands, 2)?;
```
**Suggested fix:** Reject SP/WSP before encoding.
```rust
    fn reject_sp(name: &str) -> Result<(), String> {
        let n = name.to_ascii_lowercase();
        if n == "sp" || n == "wsp" {
            Err("mneg: SP/WSP is not a valid operand (use XZR/WZR)".into())
        } else {
            Ok(())
        }
    }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_mneg_regression_sp -- --test-threads=1
```
**Raw output:**
```text
---- backend::arm::assembler::encoder::encode_mneg_pbt::encode_mneg_neg_sp stdout ----
Test failed: SP/WSP is not a valid MNEG operand (which=0 names=["wsp", "w0", "w0"]) at src/backend/arm/assembler/encoder/encode_mneg_pbt.rs:442.
minimal failing input: which = 0, is_64 = false, a = 0, b = 0
	successes: 0
	local rejects: 0
	global rejects: 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_mneg_pbt.rs
