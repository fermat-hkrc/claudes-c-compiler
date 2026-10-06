# Bug: encode_msr masks PSTATE immediates outside 0..=15 instead of rejecting
**Law:** If llvm-mc / GNU as reject `msr daifset, #imm` with imm ∉ 0..=15, then encode_msr([Symbol("daifset"), Imm(imm)]) must return Err. The same holds for daifclr and spsel.
**Impact:** `msr daifset, #16` encodes as `msr daifset, #0`; `msr daifset, #-2` encodes as CRm=14. An out-of-range PSTATE immediate silently wraps in the 4-bit CRm field, so the assembled instruction sets/clears the wrong DAIF bits.
**Function:** encode_msr
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/system.rs:262
**Detected by:** Negative/Error Contract (oob PSTATE imm; llvm-mc requires [0, 15])
**Minimal input:** encode_msr(&[Operand::Symbol("daifset".into()), Operand::Imm(-2)])  (assembly: `msr daifset, #-2`)
**Expected:** Err (llvm-mc: "immediate must be an integer in range [0, 15]")
**Actual:** Ok(Word)  // CRm = (-2 as u32) & 0xF = 14
**Severity:** medium
**Root cause:** system.rs:273 `let imm = get_imm(operands, 1)? as u32 & 0xF;` (same mask at daifclr and spsel immediate arms) truncates instead of range-checking.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/system.rs:273`
```rust
            let imm = get_imm(operands, 1)? as u32 & 0xF;
```
**Suggested fix:** Reject immediates outside 0..=15.
```rust
            let imm = get_imm(operands, 1)?;
            if !(0..=15).contains(&imm) {
                return Err("msr: PSTATE immediate must be in 0..=15".to_string());
            }
            let imm = imm as u32;
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_msr_regression_daifset_imm16 -- --test-threads=1 --nocapture
```
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_msr_pbt::encode_msr_neg_oob_imm' panicked at src/backend/arm/assembler/encoder/encode_msr_pbt.rs:444:1:
Test failed: oob PSTATE imm must Err (llvm-mc rejects msr daifset, #-2) at src/backend/arm/assembler/encoder/encode_msr_pbt.rs:733.
minimal failing input: field = "daifset", imm = -2
	successes: 0
	local rejects: 0
	global rejects: 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_msr_pbt.rs
