# Bug: encode_neon_shrn truncates i64 shift via `as u32`
**Law:** The SHRN shift immediate is the assembler immediate in [1, dest element size]. An i64 Imm outside that range must be rejected even if its low 32 bits look like a valid shift.
**Impact:** `Imm(4294967297)` (`1 + 2^32`) encodes as shift `#1`. An out-of-range immediate is silently rewritten to a different legal shift, producing the wrong instruction.
**Function:** encode_neon_shrn
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:1436
**Detected by:** Negative/Error Contract
**Minimal input:** encode_neon_shrn([v0.8b, v0.8h, #4294967297], opcode=0b100001, is_high=false)
**Expected:** Err (immediate not in [1, 8]; llvm-mc: "immediate must be an integer in range [1, 8]"; gas: "immediate value out of range 1 to 64")
**Actual:** Ok(Word) — `let shift = get_imm(operands, 2)? as u32` yields 1, which passes the half_bits range check
**Severity:** medium
**Root cause:** neon.rs:1440 truncates the i64 immediate to u32 before the range check at neon.rs:1444, so values congruent to a valid shift modulo 2^32 are accepted
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:1440`
```rust
    let shift = get_imm(operands, 2)? as u32;
```
**Suggested fix:** Range-check the i64 immediate before narrowing
```rust
    let shift_i = get_imm(operands, 2)?;
    if shift_i < 1 || shift_i > half_bits as i64 {
        return Err(format!("shrn: shift {} out of range", shift_i));
    }
    let shift = shift_i as u32;
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_shrn_regression_shift_i64_trunc -- --test-threads=1 --exact
```
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_neon_shrn_pbt::encode_neon_shrn_neg_shift_oob' (2365184) panicked at src/backend/arm/assembler/encoder/encode_neon_shrn_pbt.rs:459:1:
Test failed: shift 4294967297 not in [1, 8] must Err (llvm-mc rejects shrn v0.8b, v0.8h, #4294967297) at src/backend/arm/assembler/encoder/encode_neon_shrn_pbt.rs:482.
minimal failing input: rd = 0, rn = 0, ta_shift = (
    "8h",
    4294967297,
), is_high = false, opcode = 33
	successes: 1
	local rejects: 0
	global rejects: 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_neon_shrn_pbt.rs
