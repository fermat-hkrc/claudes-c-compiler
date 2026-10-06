# Bug: encode_hint masks immediates outside 0..=127 instead of rejecting
**Law:** If llvm-mc / GNU as reject `hint #imm` for imm outside 0..=127, then encode_hint([Imm(imm)]) must return Err
**Impact:** `hint #-1` encodes as `hint #127`; `hint #128` encodes as `hint #0` (NOP). An out-of-range hint immediate silently wraps into CRm:op2, so the assembled instruction is a different hint (or an alias such as NOP) than the source asked for.
**Function:** encode_hint
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/system.rs:554
**Detected by:** Negative/Error Contract (oob imm; llvm-mc rejects imm out of range [0, 127])
**Minimal input:** encode_hint(&[Operand::Imm(-1)])  (assembly: `hint #-1`)
**Expected:** Err (llvm-mc: "immediate must be an integer in range [0, 127]")
**Actual:** Ok(Word(0xd5032fff))  // encodes as hint #127
**Severity:** medium
**Root cause:** system.rs:558-560 pack `((imm as u32) >> 3) & 0xF` and `(imm as u32) & 0x7` with no range check, so values outside 0..=127 wrap into the 7-bit CRm:op2 field.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/system.rs:558`
```rust
    let crm = ((imm as u32) >> 3) & 0xF;
```
**Suggested fix:** Reject immediates outside 0..=127.
```rust
    let imm = get_imm(operands, 0)?;
    if !(0..=127).contains(&imm) {
        return Err("hint: immediate must be in 0..=127".to_string());
    }
    let crm = (imm as u32) >> 3;
    let op2 = (imm as u32) & 0x7;
    let word = 0xd503201f | (crm << 8) | (op2 << 5);
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_hint_regression_imm_neg1 -- --test-threads=1 --nocapture
```
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_hint_pbt::encode_hint_neg_oob_imm' (2551453) panicked at src/backend/arm/assembler/encoder/encode_hint_pbt.rs:227:1:
Test failed: imm -1 outside 0..=127 must Err (llvm-mc rejects hint #-1) at src/backend/arm/assembler/encoder/encode_hint_pbt.rs:295.
minimal failing input: imm = -1
	successes: 0
	local rejects: 0
	global rejects: 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_hint_pbt.rs
