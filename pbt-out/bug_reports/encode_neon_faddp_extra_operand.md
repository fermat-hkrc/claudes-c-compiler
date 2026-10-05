# Bug: encode_neon_faddp silently encodes a fourth operand
**Law:** FADDP accepts exactly 2 (scalar) or 3 (vector) operands; a fourth operand must be rejected
**Impact:** Invalid assembly such as `faddp v0.2s, v0.2s, v0.2s, v0.2s` is assembled into a vector FADDP word instead of an error, so gas/llvm-mc-incompatible source produces a silent encoding
**Function:** encode_neon_faddp
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:1686
**Detected by:** Negative/Error Contract
**Minimal input:** encode_neon_faddp([v0.2s, v0.2s, v0.2s, v0.2s])
**Expected:** Err ("faddp requires 2 or 3 operands"); llvm-mc rejects the same text
**Actual:** Ok(Word) encoding vector FADDP v0.2s, v0.2s, v0.2s (fourth operand ignored)
**Severity:** medium
**Root cause:** neon.rs:1687 uses `operands.len() >= 3` for the vector form, so arity 4+ is treated as a 3-operand vector encode; the "requires 2 or 3" error only fires for arity < 2
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:1687`
```rust
    if operands.len() >= 3 {
```
**Suggested fix:** Dispatch on exact arity 3 (vector) or 2 (scalar); reject anything else
```rust
    match operands.len() {
        3 => { /* vector form */ }
        2 => { /* scalar form */ }
        _ => Err("faddp requires 2 or 3 operands".to_string()),
    }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_neon_faddp_neg_extra -- --test-threads=1
```
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_neon_faddp_pbt::encode_neon_faddp_neg_extra' (2471457) panicked at src/backend/arm/assembler/encoder/encode_neon_faddp_pbt.rs:217:1:
Test failed: extra operand must Err (llvm-mc rejects faddp v0.2s, v0.2s, v0.2s, v0.2s) at src/backend/arm/assembler/encoder/encode_neon_faddp_pbt.rs:347.
minimal failing input: rd = 0, rn = 0, rm = 0, extra = 0, t = "2s", n = 0
	successes: 0
	local rejects: 0
	global rejects: 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_neon_faddp_pbt.rs
