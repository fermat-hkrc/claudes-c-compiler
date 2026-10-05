# Bug: encode_neon_shrn ignores a fourth operand
**Law:** Vector SHRN/RSHRN takes exactly three operands; a fourth operand must be rejected
**Impact:** The assembler silently encodes `shrn Vd.Tb, Vn.Ta, #shift, extra` as `shrn Vd.Tb, Vn.Ta, #shift`, dropping the extra operand instead of diagnosing invalid assembly
**Function:** encode_neon_shrn
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:1436
**Detected by:** Negative/Error Contract
**Minimal input:** encode_neon_shrn([v0.8b, v0.8h, #1, v0.8b], opcode=0b100001, is_high=false)
**Expected:** Err (llvm-mc/gas reject a fourth operand)
**Actual:** Ok(Word) — same as `shrn v0.8b, v0.8h, #1`
**Severity:** medium
**Root cause:** neon.rs:1437 checks `operands.len() < 3` only, so any extra operands after the first three are ignored
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:1437`
```rust
    if operands.len() < 3 { return Err("shrn/rshrn requires 3 operands".to_string()); }
```
**Suggested fix:** Reject arity other than 3
```rust
    if operands.len() != 3 {
        return Err("shrn/rshrn requires 3 operands".to_string());
    }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_shrn_regression_extra_operand -- --test-threads=1 --exact
```
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_neon_shrn_pbt::encode_neon_shrn_neg_extra_operand' (2365137) panicked at src/backend/arm/assembler/encoder/encode_neon_shrn_pbt.rs:393:1:
Test failed: 4 operands must Err (llvm-mc rejects shrn v0.8b, v0.8h, #1, v0.8b) at src/backend/arm/assembler/encoder/encode_neon_shrn_pbt.rs:419.
minimal failing input: rd = 0, rn = 0, extra = 0, ta_shift = (
    "8h",
    1,
), is_high = false, opcode = 33
	successes: 0
	local rejects: 0
	global rejects: 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_neon_shrn_pbt.rs
