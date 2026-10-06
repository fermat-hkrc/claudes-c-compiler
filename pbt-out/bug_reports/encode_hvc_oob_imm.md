# Bug: encode_hvc masks immediates outside 0..=65535 instead of rejecting
**Law:** If llvm-mc / GNU as reject `hvc #imm` with imm ∉ 0..=65535, then encode_hvc([Imm(imm)]) must return Err
**Impact:** `hvc #-1` encodes as `hvc #65535`; `hvc #65536` encodes as `hvc #0`. An out-of-range hypervisor-call immediate silently wraps in the 16-bit imm16 field, so the assembled instruction invokes the wrong HVC number.
**Function:** encode_hvc
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/system.rs:395
**Detected by:** Negative/Error Contract (oob imm16; llvm-mc requires [0, 65535])
**Minimal input:** encode_hvc(&[Operand::Imm(-1)])  (assembly: `hvc #-1`)
**Expected:** Err (llvm-mc: "immediate must be an integer in range [0, 65535]")
**Actual:** Ok(Word(0xd41fffe2))  // imm16 = (-1 as u32) & 0xFFFF = 65535
**Severity:** medium
**Root cause:** system.rs:397 `let word = 0xd4000002 | ((imm as u32 & 0xFFFF) << 5);` truncates instead of range-checking.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/system.rs:397`
```rust
    let word = 0xd4000002 | ((imm as u32 & 0xFFFF) << 5);
```
**Suggested fix:** Reject immediates outside 0..=65535.
```rust
    let imm = get_imm(operands, 0)?;
    if !(0..=65535).contains(&imm) {
        return Err("hvc: immediate must be in 0..=65535".to_string());
    }
    let word = 0xd4000002 | ((imm as u32) << 5);
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_hvc_regression_imm_neg1 -- --test-threads=1 --nocapture
```
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_hvc_pbt::encode_hvc_neg_oob_imm' (2538416) panicked at src/backend/arm/assembler/encoder/encode_hvc_pbt.rs:217:1:
Test failed: imm -1 outside 0..=65535 must Err (llvm-mc rejects hvc #-1) at src/backend/arm/assembler/encoder/encode_hvc_pbt.rs:285.
minimal failing input: imm = -1
	successes: 0
	local rejects: 0
	global rejects: 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_hvc_pbt.rs
