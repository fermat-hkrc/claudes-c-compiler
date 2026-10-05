# Bug: encode_neon_rev64 treats a GPR source as a NEON register
**Law:** NEON REV64 source must be an arranged SIMD register; a GPR (xN/wN) must be rejected
**Impact:** `rev64 v0.8b, x0` is encoded as `rev64 v0.8b, v0.8b`. gas rejects this (`operand 2 must be a SIMD vector register`). The assembler silently retargets a scalar register number into Vn
**Function:** encode_neon_rev64
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:752
**Detected by:** Negative/Error Contract
**Minimal input:** encode_neon_rev64([v0.8b, x0])
**Expected:** Err (gas: operand 2 must be a SIMD vector register; llvm-mc: invalid operand)
**Actual:** Ok(Word(0x0e200800)) — same as `rev64 v0.8b, v0.8b`
**Severity:** medium
**Root cause:** get_neon_reg accepts Operand::Reg and returns an empty arrangement; encode_neon_rev64 then discards that arrangement and uses only the register number as Rn
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:757`
```rust
    let (rn, _) = get_neon_reg(operands, 1)?;
```
**Suggested fix:** Require a RegArrangement source (non-empty arrangement matching dest)
```rust
    let (rn, arr_n) = get_neon_reg(operands, 1)?;
    if arr_n.is_empty() || arr_n != arr_d {
        return Err("rev64: source must be a SIMD vector register with matching arrangement".to_string());
    }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_rev64_regression_gpr_src -- --test-threads=1 --exact
```
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_neon_rev64_pbt::encode_neon_rev64_neg_non_neon' (2294860) panicked at src/backend/arm/assembler/encoder/encode_neon_rev64_pbt.rs:205:1:
Test failed: non-arranged NEON / GPR / SP / FP must Err (gas rejects rev64 v0.8b, x0) at src/backend/arm/assembler/encoder/encode_neon_rev64_pbt.rs:405.
minimal failing input: rd = 0, rn = 0, t = "8b", kind = 7
	successes: 10
	local rejects: 0
	global rejects: 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_neon_rev64_pbt.rs
