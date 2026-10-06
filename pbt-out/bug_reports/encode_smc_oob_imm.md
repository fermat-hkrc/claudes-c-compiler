# Bug: encode_smc masks immediates outside 0..=65535 instead of rejecting
**Law:** If llvm-mc / GNU as reject `smc #imm` for imm outside 0..=65535, then encode_smc([Imm(imm)]) must return Err
**Impact:** `smc #-1` encodes as `smc #65535`; `smc #65536` encodes as `smc #0`. An out-of-range Secure Monitor Call immediate silently wraps, so the assembled instruction invokes the wrong SMC number.
**Function:** encode_smc
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/system.rs:420
**Detected by:** Negative/Error Contract (oob imm; llvm-mc rejects imm16 out of range)
**Minimal input:** encode_smc(&[Operand::Imm(-1)])  (assembly: `smc #-1`)
**Expected:** Err (llvm-mc: "immediate must be an integer in range [0, 65535]")
**Actual:** Ok(Word(0xd41fffe3))  // encodes as smc #65535
**Severity:** medium
**Root cause:** system.rs:422 `let word = 0xd4000003 | ((imm as u32 & 0xFFFF) << 5);` truncates instead of range-checking.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/system.rs:422`
```rust
    let word = 0xd4000003 | ((imm as u32 & 0xFFFF) << 5);
```
**Suggested fix:** Reject immediates outside 0..=65535.
```rust
    let imm = get_imm(operands, 0)?;
    if !(0..=65535).contains(&imm) {
        return Err("smc: immediate must be in 0..=65535".to_string());
    }
    let word = 0xd4000003 | ((imm as u32) << 5);
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_smc_regression_imm_neg1 -- --test-threads=1 --nocapture
```
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_smc_pbt::encode_smc_neg_oob_imm' (2542909) panicked at src/backend/arm/assembler/encoder/encode_smc_pbt.rs:217:1:
Test failed: imm -1 outside 0..=65535 must Err (llvm-mc rejects smc #-1) at src/backend/arm/assembler/encoder/encode_smc_pbt.rs:285.
minimal failing input: imm = -1
	successes: 0
	local rejects: 0
	global rejects: 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_smc_pbt.rs
