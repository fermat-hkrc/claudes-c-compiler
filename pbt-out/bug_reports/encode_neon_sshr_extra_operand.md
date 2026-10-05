# Bug: encode_neon_sshr ignores a fourth operand
**Law:** Vector SSHR takes exactly three operands; a fourth operand must be rejected
**Impact:** The assembler silently encodes `sshr Vd.T, Vn.T, #shift, extra` as `sshr Vd.T, Vn.T, #shift`, dropping the extra operand instead of diagnosing invalid assembly
**Function:** encode_neon_sshr
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:1205
**Detected by:** Negative/Error Contract
**Minimal input:** encode_neon_sshr([v0.8b, v0.8b, #1, v0.8b])
**Expected:** Err (llvm-mc/gas reject a fourth operand)
**Actual:** Ok(Word) — same as `sshr v0.8b, v0.8b, #1`
**Severity:** medium
**Root cause:** neon.rs:1206 checks `operands.len() < 3` only, so any extra operands after the first three are ignored
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:1206`
```rust
    if operands.len() < 3 {
        return Err("sshr requires 3 operands".to_string());
    }
```
**Suggested fix:** Reject arity other than 3
```rust
    if operands.len() != 3 {
        return Err("sshr requires 3 operands".to_string());
    }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_sshr_regression_extra_operand -- --test-threads=1 --exact
```
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_neon_sshr_pbt::encode_neon_sshr_neg_extra_operand' (2343183) panicked at src/backend/arm/assembler/encoder/encode_neon_sshr_pbt.rs:263:1:
Test failed: 4 operands must Err (llvm-mc rejects sshr v0.8b, v0.8b, #1, v0.8b) at src/backend/arm/assembler/encoder/encode_neon_sshr_pbt.rs:366.
minimal failing input: rd = 0, rn = 0, extra = 0, t_shift = (
    "8b",
    1,
)
	successes: 0
	local rejects: 0
	global rejects: 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_neon_sshr_pbt.rs
