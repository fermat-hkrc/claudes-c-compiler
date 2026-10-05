# Bug: encode_neon_shl ignores a fourth operand
**Law:** Vector SHL takes exactly three operands; a fourth operand must be rejected
**Impact:** The assembler silently encodes `shl Vd.T, Vn.T, #shift, extra` as `shl Vd.T, Vn.T, #shift`, dropping the extra operand instead of diagnosing invalid assembly
**Function:** encode_neon_shl
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:1231
**Detected by:** Negative/Error Contract
**Minimal input:** encode_neon_shl([v0.8b, v0.8b, #0, v0.8b])
**Expected:** Err (llvm-mc/gas reject a fourth operand)
**Actual:** Ok(Word) — same as `shl v0.8b, v0.8b, #0`
**Severity:** medium
**Root cause:** neon.rs:1232 checks `operands.len() < 3` only, so any extra operands after the first three are ignored
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:1232`
```rust
    if operands.len() < 3 {
        return Err("shl requires 3 operands".to_string());
    }
```
**Suggested fix:** Reject arity other than 3
```rust
    if operands.len() != 3 {
        return Err("shl requires 3 operands".to_string());
    }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_shl_regression_extra_operand -- --test-threads=1 --exact
```
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_neon_shl_pbt::encode_neon_shl_neg_extra_operand' (2353101) panicked at src/backend/arm/assembler/encoder/encode_neon_shl_pbt.rs:265:1:
Test failed: 4 operands must Err (llvm-mc rejects shl v0.8b, v0.8b, #0, v0.8b) at src/backend/arm/assembler/encoder/encode_neon_shl_pbt.rs:368.
minimal failing input: rd = 0, rn = 0, extra = 0, t_shift = (
    "8b",
    0,
)
	successes: 0
	local rejects: 0
	global rejects: 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_neon_shl_pbt.rs
