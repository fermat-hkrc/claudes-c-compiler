# Bug: encode_neon_rev64 ignores a third operand
**Law:** REV64 takes exactly two SIMD operands; a third operand must be rejected
**Impact:** The assembler silently encodes `rev64 Vd.T, Vn.T, Vextra.T` as `rev64 Vd.T, Vn.T`, dropping the extra operand instead of diagnosing invalid assembly (gas: unexpected characters following instruction)
**Function:** encode_neon_rev64
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:752
**Detected by:** Negative/Error Contract
**Minimal input:** encode_neon_rev64([v0.8b, v0.8b, v0.8b])
**Expected:** Err (llvm-mc/gas reject a third operand)
**Actual:** Ok(Word(0x0e200800)) — same as `rev64 v0.8b, v0.8b`
**Severity:** medium
**Root cause:** neon.rs:753 uses `operands.len() < 2`, so any extra operands after the first two are ignored
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:753`
```rust
    if operands.len() < 2 {
        return Err("rev64 requires 2 operands".to_string());
    }
```
**Suggested fix:** Reject arity other than 2
```rust
    if operands.len() != 2 {
        return Err("rev64 requires 2 operands".to_string());
    }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_rev64_regression_extra_operand -- --test-threads=1 --exact
```
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_neon_rev64_pbt::encode_neon_rev64_neg_extra' (2294829) panicked at src/backend/arm/assembler/encoder/encode_neon_rev64_pbt.rs:205:1:
Test failed: extra operand must Err (llvm-mc rejects rev64 v0.8b, v0.8b, v0.8b) at src/backend/arm/assembler/encoder/encode_neon_rev64_pbt.rs:301.
minimal failing input: rd = 0, rn = 0, extra = 0, t = "8b"
	successes: 0
	local rejects: 0
	global rejects: 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_neon_rev64_pbt.rs
