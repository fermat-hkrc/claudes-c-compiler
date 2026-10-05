# Bug: encode_neon_two_misc ignores source arrangement
**Law:** Matching-T two-misc (ABS/NEG/CLS/CLZ/REV16/REV32/SQABS/SQNEG) requires dest and source arrangements to be identical; a mismatch must be rejected
**Impact:** `abs v0.8b, v0.16b` is silently encoded as `abs v0.8b, v0.8b` (Q/size from dest only), so gas/llvm-mc-rejected assembly becomes a different legal instruction
**Function:** encode_neon_two_misc
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:1407
**Detected by:** Negative/Error Contract
**Minimal input:** encode_neon_two_misc([v0.8b, v0.16b], u_bit=0, opcode=0b01011)
**Expected:** Err (llvm-mc/gas: invalid operand / arrangement mismatch)
**Actual:** Ok(Word) — same as `abs v0.8b, v0.8b`
**Severity:** medium
**Root cause:** neon.rs:1409 discards the source arrangement (`let (rn, _)`), so Q and size come only from dest
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:1409`
```rust
    let (rn, _) = get_neon_reg(operands, 1)?;
```
**Suggested fix:** Require dest and source arrangements to match for non-pairwise opcodes
```rust
    let (rn, arr_n) = get_neon_reg(operands, 1)?;
    if arr_d != arr_n {
        return Err(format!("two-misc: arrangement mismatch {} vs {}", arr_d, arr_n));
    }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_two_misc_regression_mismatch_t -- --test-threads=1 --exact
```
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_neon_two_misc_pbt::encode_neon_two_misc_neg_mismatch' (2372499) panicked at src/backend/arm/assembler/encoder/encode_neon_two_misc_pbt.rs:356:1:
Test failed: mismatched T must Err (llvm-mc rejects abs v0.8b, v0.16b) at src/backend/arm/assembler/encoder/encode_neon_two_misc_pbt.rs:492.
minimal failing input: rd = 0, rn = 0, td = "8b", tn = "16b"
	successes: 0
	local rejects: 0
	global rejects: 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_neon_two_misc_pbt.rs
