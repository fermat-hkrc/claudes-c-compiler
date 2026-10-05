# Bug: encode_neon_across ignores a third operand
**Law:** UMAXV/UMINV/SMAXV/SMINV take exactly two operands; a third operand must be rejected
**Impact:** The assembler silently encodes `umaxv Vd, Vn.T, extra` as `umaxv Vd, Vn.T`, dropping the extra operand instead of diagnosing invalid assembly
**Function:** encode_neon_across
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:445
**Detected by:** Negative/Error Contract
**Minimal input:** encode_neon_across([b0, v0.8b, v0.8b], 1, 0b01010)
**Expected:** Err (llvm-mc/gas reject a third operand)
**Actual:** Ok(Word(0x2e30a800)) — same as `umaxv b0, v0.8b`
**Severity:** medium
**Root cause:** neon.rs:446 uses `operands.len() < 2`, so any extra operands after the first two are ignored
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:446`
```rust
    if operands.len() < 2 {
        return Err("NEON across-vector requires 2 operands".to_string());
    }
```
**Suggested fix:** Reject arity other than 2
```rust
    if operands.len() != 2 {
        return Err("NEON across-vector requires 2 operands".to_string());
    }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_across_regression_extra_operand -- --test-threads=1 --exact
```
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_neon_across_pbt::encode_neon_across_neg_extra' panicked at src/backend/arm/assembler/encoder/encode_neon_across_pbt.rs:297:1:
Test failed: extra operand must Err (llvm-mc rejects umaxv b0, v0.8b, v0.8b) at src/backend/arm/assembler/encoder/encode_neon_across_pbt.rs:417.
minimal failing input: rd = 0, rn = 0, extra = 0, (v, t) = (
    "b",
    "8b",
), (mnem, u_bit, opcode) = (
    "umaxv",
    1,
    10,
)
	successes: 0
	local rejects: 0
	global rejects: 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_neon_across_pbt.rs
