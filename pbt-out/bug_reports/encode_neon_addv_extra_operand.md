# Bug: encode_neon_addv ignores a third operand
**Law:** ADDV takes exactly two operands; a third operand must be rejected
**Impact:** The assembler silently encodes `addv Vd, Vn.T, extra` as `addv Vd, Vn.T`, dropping the extra operand instead of diagnosing invalid assembly
**Function:** encode_neon_addv
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:424
**Detected by:** Negative/Error Contract
**Minimal input:** encode_neon_addv([b0, v0.8b, v0.8b])
**Expected:** Err (llvm-mc/gas reject a third operand)
**Actual:** Ok(Word(0x0e30dc00)) — same as `addv b0, v0.8b`
**Severity:** medium
**Root cause:** neon.rs:425 uses `operands.len() < 2`, so any extra operands after the first two are ignored
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:425`
```rust
    if operands.len() < 2 {
        return Err("addv requires 2 operands".to_string());
    }
```
**Suggested fix:** Reject arity other than 2
```rust
    if operands.len() != 2 {
        return Err("addv requires 2 operands".to_string());
    }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_addv_regression_extra_operand -- --test-threads=1 --exact
```
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_neon_addv_pbt::encode_neon_addv_neg_extra' panicked at src/backend/arm/assembler/encoder/encode_neon_addv_pbt.rs:245:1:
Test failed: extra operand must Err (llvm-mc rejects addv b0, v0.8b, v0.8b) at src/backend/arm/assembler/encoder/encode_neon_addv_pbt.rs:334.
minimal failing input: rd = 0, rn = 0, extra = 0, (v, t) = (
    "b",
    "8b",
)
	successes: 0
	local rejects: 0
	global rejects: 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_neon_addv_pbt.rs
