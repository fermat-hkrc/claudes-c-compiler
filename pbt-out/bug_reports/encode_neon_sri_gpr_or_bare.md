# Bug: encode_neon_sri accepts a bare V source without arrangement
**Law:** Vector SRI requires NEON arrangement operands Vd.T and Vn.T; a bare `Vn` (or GPR / xN.T) must be rejected
**Impact:** `sri v0.8b, v0, #1` is encoded as `sri v0.8b, v0.8b, #1`. llvm-mc/gas reject the bare source. GPR destinations written as `x0.8b` are similarly treated as `v0.8b` via parse_reg_num.
**Function:** encode_neon_sri
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:1285
**Detected by:** Negative/Error Contract
**Minimal input:** encode_neon_sri([v0.8b, Reg(v0), #1])
**Expected:** Err (llvm-mc rejects `sri v0.8b, v0, #1`)
**Actual:** Ok(Word) — same as `sri v0.8b, v0.8b, #1`
**Severity:** medium
**Root cause:** get_neon_reg accepts Operand::Reg and returns an empty arrangement, which encode_neon_sri then discards (`let (rn, _)`), so a bare V/X/W source is encoded as a NEON register
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:1290`
```rust
    let (rn, _) = get_neon_reg(operands, 1)?;
```
**Suggested fix:** Require RegArrangement with a v-prefix on both operands
```rust
    let (rn, arr_n) = get_neon_reg(operands, 1)?;
    if arr_n.is_empty() {
        return Err("sri requires Vn.T arrangement".to_string());
    }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_sri_regression_bare_src -- --test-threads=1 --exact
```
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_neon_sri_pbt::encode_neon_sri_neg_gpr_or_bare' (2357969) panicked at src/backend/arm/assembler/encoder/encode_neon_sri_pbt.rs:267:1:
Test failed: GPR/bare/non-arrangement kind=1 must Err (llvm-mc rejects sri v0.8b, v0, #1) at src/backend/arm/assembler/encoder/encode_neon_sri_pbt.rs:486.
minimal failing input: rd = 0, rn = 0, kind = 1, fp_prefix = "x"
	successes: 0
	local rejects: 0
	global rejects: 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_neon_sri_pbt.rs
