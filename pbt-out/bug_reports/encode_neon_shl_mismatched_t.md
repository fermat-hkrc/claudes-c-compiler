# Bug: encode_neon_shl ignores the source arrangement
**Law:** SHL is `Vd.T, Vn.T, #shift` with matching T; mismatched or reserved arrangements must be rejected
**Impact:** Invalid assembly `shl v0.2s, v0.4s, #0` is encoded as `shl v0.2s, v0.2s, #0` (size and Q taken only from the destination), so the assembler emits a different instruction than the source text
**Function:** encode_neon_shl
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:1231
**Detected by:** Negative/Error Contract
**Minimal input:** encode_neon_shl([v0.2s, v0.4s, #0])
**Expected:** Err (ARM/llvm-mc require matching T; llvm-mc: "invalid operand for instruction")
**Actual:** Ok(Word) — encoded as if both were .2s
**Severity:** medium
**Root cause:** neon.rs:1236 discards the source arrangement (`let (rn, _)`), so Td/Tn never compared and 1d/1q on the source never rejected
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:1236`
```rust
    let (rn, _) = get_neon_reg(operands, 1)?;
```
**Suggested fix:** Require matching arrangements and a valid SHL T
```rust
    let (rn, arr_n) = get_neon_reg(operands, 1)?;
    if arr_n != arr_d {
        return Err(format!("shl arrangement mismatch: {} vs {}", arr_d, arr_n));
    }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_shl_regression_mismatched_t -- --test-threads=1 --exact
```
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_neon_shl_pbt::encode_neon_shl_neg_invalid_t' (2353139) panicked at src/backend/arm/assembler/encoder/encode_neon_shl_pbt.rs:265:1:
Test failed: invalid/mismatched/reserved T must Err (ARM SHL T in {8B,16B,4H,8H,2S,4S,2D} matching; llvm-mc rejects shl v0.2s, v0.4s, #0) at src/backend/arm/assembler/encoder/encode_neon_shl_pbt.rs:394.
minimal failing input: rd = 0, rn = 0, td = "2s", tn = "4s", shift = 0
	successes: 0
	local rejects: 0
	global rejects: 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_neon_shl_pbt.rs
