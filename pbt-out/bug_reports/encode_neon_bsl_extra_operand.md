# Bug: encode_neon_bsl ignores a fourth operand
**Law:** BSL takes exactly three SIMD operands; a fourth operand must be rejected
**Impact:** The assembler silently encodes `bsl Vd.T, Vn.T, Vm.T, Vextra.T` as `bsl Vd.T, Vn.T, Vm.T`, dropping the extra operand instead of diagnosing invalid assembly
**Function:** encode_neon_bsl
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:735
**Detected by:** Negative/Error Contract
**Minimal input:** encode_neon_bsl([v0.8b, v0.8b, v0.8b, v0.8b])
**Expected:** Err (llvm-mc/gas reject a fourth operand)
**Actual:** Ok(Word(0x2e601c00)) — same as `bsl v0.8b, v0.8b, v0.8b`
**Severity:** medium
**Root cause:** neon.rs:736 uses `operands.len() < 3`, so any extra operands after the first three are ignored
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:736`
```rust
    if operands.len() < 3 {
        return Err("bsl requires 3 operands".to_string());
    }
```
**Suggested fix:** Reject arity other than 3
```rust
    if operands.len() != 3 {
        return Err("bsl requires 3 operands".to_string());
    }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_bsl_regression_extra_operand -- --test-threads=1 --exact
```
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_neon_bsl_pbt::encode_neon_bsl_neg_extra' panicked at src/backend/arm/assembler/encoder/encode_neon_bsl_pbt.rs:165:1:
Test failed: extra operand must Err (llvm-mc rejects bsl v0.8b, v0.8b, v0.8b, v0.8b) at src/backend/arm/assembler/encoder/encode_neon_bsl_pbt.rs:271.
minimal failing input: rd = 0, rn = 0, rm = 0, extra = 0, t = "8b"
	successes: 0
	local rejects: 0
	global rejects: 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_neon_bsl_pbt.rs
