# Bug: encode_brk masks immediates outside 0..=65535 instead of rejecting
**Law:** If llvm-mc / GNU as reject `brk #imm` for imm outside 0..=65535, then encode_brk([Imm(imm)]) must return Err
**Impact:** `brk #-1` encodes as `brk #65535`; `brk #65536` encodes as `brk #0`. An out-of-range breakpoint immediate silently wraps, so the assembled instruction carries the wrong comment/imm16. ARM codegen uses `brk #0` as the trap instruction; a wrong immediate still traps but reports a different BRK number to a debugger.
**Function:** encode_brk
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/system.rs:471
**Detected by:** Negative/Error Contract (oob imm; llvm-mc rejects imm16 out of range)
**Minimal input:** encode_brk(&[Operand::Imm(-1)])  (assembly: `brk #-1`)
**Expected:** Err (llvm-mc: "immediate must be an integer in range [0, 65535]")
**Actual:** Ok(Word(0xd43fffe0))  // encodes as brk #65535
**Severity:** medium
**Root cause:** system.rs:473 `let word = 0xd4200000 | ((imm as u32 & 0xFFFF) << 5);` truncates instead of range-checking.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/system.rs:473`
```rust
    let word = 0xd4200000 | ((imm as u32 & 0xFFFF) << 5);
```
**Suggested fix:** Reject immediates outside 0..=65535.
```rust
    let imm = get_imm(operands, 0)?;
    if !(0..=65535).contains(&imm) {
        return Err("brk: immediate must be in 0..=65535".to_string());
    }
    let word = 0xd4200000 | ((imm as u32) << 5);
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_brk_regression_imm_neg1 -- --test-threads=1 --nocapture
```
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_brk_pbt::encode_brk_neg_oob_imm' (2547093) panicked at src/backend/arm/assembler/encoder/encode_brk_pbt.rs:218:1:
Test failed: imm -1 outside 0..=65535 must Err (llvm-mc rejects brk #-1) at src/backend/arm/assembler/encoder/encode_brk_pbt.rs:286.
minimal failing input: imm = -1
	successes: 0
	local rejects: 0
	global rejects: 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_brk_pbt.rs
