# Bug: encode_svc masks immediates outside 0..=65535 instead of rejecting
**Law:** If llvm-mc / GNU as reject `svc #imm` with imm ∉ 0..=65535, then encode_svc([Imm(imm)]) must return Err
**Impact:** `svc #-1` encodes as `svc #65535`; `svc #65536` encodes as `svc #0`. An out-of-range supervisor-call immediate silently wraps in the 16-bit imm16 field, so the assembled instruction invokes the wrong syscall number.
**Function:** encode_svc
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/system.rs:389
**Detected by:** Negative/Error Contract (oob imm16; llvm-mc requires [0, 65535])
**Minimal input:** encode_svc(&[Operand::Imm(-1)])  (assembly: `svc #-1`)
**Expected:** Err (llvm-mc: "immediate must be an integer in range [0, 65535]")
**Actual:** Ok(Word(0xd41fffe1))  // imm16 = (-1 as u32) & 0xFFFF = 65535
**Severity:** medium
**Root cause:** system.rs:391 `let word = 0xd4000001 | ((imm as u32 & 0xFFFF) << 5);` truncates instead of range-checking.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/system.rs:391`
```rust
    let word = 0xd4000001 | ((imm as u32 & 0xFFFF) << 5);
```
**Suggested fix:** Reject immediates outside 0..=65535.
```rust
    let imm = get_imm(operands, 0)?;
    if !(0..=65535).contains(&imm) {
        return Err("svc: immediate must be in 0..=65535".to_string());
    }
    let word = 0xd4000001 | ((imm as u32) << 5);
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_svc_regression_imm_neg1 -- --test-threads=1 --nocapture
```
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_svc_pbt::encode_svc_neg_oob_imm' (2534286) panicked at src/backend/arm/assembler/encoder/encode_svc_pbt.rs:218:1:
Test failed: imm -1 outside 0..=65535 must Err (llvm-mc rejects svc #-1) at src/backend/arm/assembler/encoder/encode_svc_pbt.rs:286.
minimal failing input: imm = -1
	successes: 0
	local rejects: 0
	global rejects: 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_svc_pbt.rs
