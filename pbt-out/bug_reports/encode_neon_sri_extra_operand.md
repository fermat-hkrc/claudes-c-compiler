# Bug: encode_neon_sri ignores a fourth operand
**Law:** Vector SRI takes exactly three operands; a fourth operand must be rejected
**Impact:** The assembler silently encodes `sri Vd.T, Vn.T, #shift, extra` as `sri Vd.T, Vn.T, #shift`, dropping the extra operand instead of diagnosing invalid assembly
**Function:** encode_neon_sri
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:1285
**Detected by:** Negative/Error Contract
**Minimal input:** encode_neon_sri([v0.8b, v0.8b, #1, v0.8b])
**Expected:** Err (llvm-mc/gas reject a fourth operand)
**Actual:** Ok(Word) — same as `sri v0.8b, v0.8b, #1`
**Severity:** medium
**Root cause:** neon.rs:1286 checks `operands.len() < 3` only, so any extra operands after the first three are ignored
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:1286`
```rust
    if operands.len() < 3 {
        return Err("sri requires 3 operands".to_string());
    }
```
**Suggested fix:** Reject arity other than 3
```rust
    if operands.len() != 3 {
        return Err("sri requires 3 operands".to_string());
    }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_sri_regression_extra_operand -- --test-threads=1 --exact
```
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_neon_sri_pbt::encode_neon_sri_neg_extra_operand' (2357942) panicked at src/backend/arm/assembler/encoder/encode_neon_sri_pbt.rs:267:1:
Test failed: 4 operands must Err (llvm-mc rejects sri v0.8b, v0.8b, #1, v0.8b) at src/backend/arm/assembler/encoder/encode_neon_sri_pbt.rs:370.
minimal failing input: rd = 0, rn = 0, extra = 0, t_shift = (
    "8b",
    1,
)
	successes: 0
	local rejects: 0
	global rejects: 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_neon_sri_pbt.rs
