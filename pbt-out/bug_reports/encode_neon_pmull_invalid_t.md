# Bug: encode_neon_pmull accepts invalid arrangements
**Law:** PMULL{2} must reject Ta/Tb outside ARM's pairs (8H←8B/16B, 1Q←1D/2D); llvm-mc/gas reject those forms
**Impact:** Invalid assembly such as `pmull v0.8b, v0.8b, v0.8b` is encoded as 64-bit PMULL instead of being diagnosed, so a typo silently becomes a different instruction
**Function:** encode_neon_pmull
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:1128
**Detected by:** Negative/Error Contract
**Minimal input:** encode_neon_pmull([v0.8b, v0.8b, v0.8b], is_pmull2=false)
**Expected:** Err (ARM PMULL Ta is 8H or 1Q; llvm-mc/gas reject `.8b`)
**Actual:** Ok(Word(0x0ee0e000)) — same as `pmull v0.1q, v0.1d, v0.1d`
**Severity:** medium
**Root cause:** neon.rs:1132-1134 discards all three arrangements (`let (rd, _)`), so Ta/Tb are never checked
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:1132`
```rust
    let (rd, _) = get_neon_reg(operands, 0)?;
    let (rn, _) = get_neon_reg(operands, 1)?;
    let (rm, _) = get_neon_reg(operands, 2)?;
```
**Suggested fix:** Keep arrangements and reject triples that are not the four ARM-legal PMULL{2} pairs
```rust
    let (rd, arr_d) = get_neon_reg(operands, 0)?;
    let (rn, arr_n) = get_neon_reg(operands, 1)?;
    let (rm, arr_m) = get_neon_reg(operands, 2)?;
    let legal = matches!(
        (arr_d.as_str(), arr_n.as_str(), arr_m.as_str(), is_pmull2),
        ("1q", "1d", "1d", false) | ("1q", "2d", "2d", true)
            | ("8h", "8b", "8b", false) | ("8h", "16b", "16b", true)
    );
    if !legal {
        return Err(format!("unsupported pmull arrangement: {}/{}/{}", arr_d, arr_n, arr_m));
    }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_pmull_regression_invalid_t -- --test-threads=1 --exact
```
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_neon_pmull_pbt::encode_neon_pmull_neg_invalid_t' (2327751) panicked at src/backend/arm/assembler/encoder/encode_neon_pmull_pbt.rs:210:1:
Test failed: invalid Ta/Tb must Err (ARM PMULL Ta in {8H,1Q} with matching Tb; llvm-mc rejects pmull v0.8b, v0.8b, v0.8b) at src/backend/arm/assembler/encoder/encode_neon_pmull_pbt.rs:393.
minimal failing input: rd = 0, rn = 0, rm = 0, is_pmull2 = false, td = "8b", tn = "8b", tm = "8b"
	successes: 0
	local rejects: 0
	global rejects: 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_neon_pmull_pbt.rs
