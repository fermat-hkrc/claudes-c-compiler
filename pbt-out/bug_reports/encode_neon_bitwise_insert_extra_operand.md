# Bug: encode_neon_bitwise_insert ignores a fourth operand
**Law:** BIT/BIF take exactly three SIMD operands; a fourth operand must be rejected
**Impact:** The assembler silently encodes `bit|bif Vd.T, Vn.T, Vm.T, Vextra.T` as `bit|bif Vd.T, Vn.T, Vm.T`, dropping the extra operand instead of diagnosing invalid assembly
**Function:** encode_neon_bitwise_insert
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:1667
**Detected by:** Negative/Error Contract
**Minimal input:** encode_neon_bitwise_insert([v0.8b, v0.8b, v0.8b, v0.8b], size=0b10)
**Expected:** Err (llvm-mc/gas reject a fourth operand)
**Actual:** Ok(Word(0x2ea01c00)) — same as `bit v0.8b, v0.8b, v0.8b`
**Severity:** medium
**Root cause:** neon.rs:1668 uses `operands.len() < 3`, so any extra operands after the first three are ignored
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:1668`
```rust
    if operands.len() < 3 {
        return Err("bit/bif requires 3 operands".to_string());
    }
```
**Suggested fix:** Reject arity other than 3
```rust
    if operands.len() != 3 {
        return Err("bit/bif requires 3 operands".to_string());
    }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_bitwise_insert_regression_extra_operand -- --test-threads=1 --exact
```
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_neon_bitwise_insert_pbt::encode_neon_bitwise_insert_neg_extra' panicked at src/backend/arm/assembler/encoder/encode_neon_bitwise_insert_pbt.rs:185:1:
Test failed: extra operand must Err (llvm-mc rejects bit v0.8b, v0.8b, v0.8b, v0.8b) at src/backend/arm/assembler/encoder/encode_neon_bitwise_insert_pbt.rs:305.
minimal failing input: rd = 0, rn = 0, rm = 0, extra = 0, t = "8b", size = 2
	successes: 0
	local rejects: 0
	global rejects: 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_neon_bitwise_insert_pbt.rs
