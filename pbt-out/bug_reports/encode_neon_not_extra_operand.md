# Bug: encode_neon_not ignores a third operand
**Law:** NOT takes exactly two SIMD operands; a third operand must be rejected
**Impact:** The assembler silently encodes `not Vd.T, Vn.T, Vextra.T` as `not Vd.T, Vn.T`, dropping the extra operand instead of diagnosing invalid assembly (gas: unexpected characters following instruction)
**Function:** encode_neon_not
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:608
**Detected by:** Negative/Error Contract
**Minimal input:** encode_neon_not([v0.8b, v0.8b, v0.8b])
**Expected:** Err (llvm-mc/gas reject a third operand)
**Actual:** Ok(Word(0x2e205800)) — same as `not v0.8b, v0.8b`
**Severity:** medium
**Root cause:** neon.rs:609 uses `operands.len() < 2`, so any extra operands after the first two are ignored
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:609`
```rust
    if operands.len() < 2 {
        return Err("not requires 2 operands".to_string());
    }
```
**Suggested fix:** Reject arity other than 2
```rust
    if operands.len() != 2 {
        return Err("not requires 2 operands".to_string());
    }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_not_regression_extra_operand -- --test-threads=1 --exact
```
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_neon_not_pbt::encode_neon_not_neg_extra' panicked at src/backend/arm/assembler/encoder/encode_neon_not_pbt.rs:167:1:
Test failed: extra operand must Err (llvm-mc rejects not v0.8b, v0.8b, v0.8b) at src/backend/arm/assembler/encoder/encode_neon_not_pbt.rs:258.
minimal failing input: rd = 0, rn = 0, extra = 0, t = "8b"
	successes: 0
	local rejects: 0
	global rejects: 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_neon_not_pbt.rs
