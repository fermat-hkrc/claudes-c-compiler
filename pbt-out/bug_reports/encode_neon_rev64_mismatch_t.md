# Bug: encode_neon_rev64 ignores source arrangement mismatch
**Law:** REV64 Vd.T, Vn.T requires both operands to share the same arrangement T
**Impact:** `rev64 v0.8b, v0.16b` is encoded as `rev64 v0.8b, v0.8b` (dest T only). llvm-mc and gas report operand mismatch. Callers can assemble illegal mixed-width reverse and get a well-formed but wrong instruction
**Function:** encode_neon_rev64
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:752
**Detected by:** Negative/Error Contract
**Minimal input:** encode_neon_rev64([v0.8b, v0.16b])
**Expected:** Err (llvm-mc/gas: operand mismatch)
**Actual:** Ok(Word(0x0e200800)) — same as `rev64 v0.8b, v0.8b`
**Severity:** medium
**Root cause:** neon.rs:757 discards the source arrangement (`let (rn, _)`), then Q/size come only from dest
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:757`
```rust
    let (rn, _) = get_neon_reg(operands, 1)?;
```
**Suggested fix:** Require source arrangement to equal dest
```rust
    let (rn, arr_n) = get_neon_reg(operands, 1)?;
    if arr_n != arr_d {
        return Err(format!("rev64: operand mismatch .{arr_d} vs .{arr_n}"));
    }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_rev64_regression_mismatch_t -- --test-threads=1 --exact
```
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_neon_rev64_pbt::encode_neon_rev64_neg_mismatch_t' (2294852) panicked at src/backend/arm/assembler/encoder/encode_neon_rev64_pbt.rs:205:1:
Test failed: mismatched T must Err (llvm-mc rejects rev64 v0.8b, v0.16b) at src/backend/arm/assembler/encoder/encode_neon_rev64_pbt.rs:341.
minimal failing input: rd = 0, rn = 0, (td, tn) = (
    "8b",
    "16b",
)
	successes: 0
	local rejects: 0
	global rejects: 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_neon_rev64_pbt.rs
