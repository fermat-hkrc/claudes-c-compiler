# Bug: encode_neon_zip_uzp ignores a fourth operand
**Law:** ZIP/UZP/TRN take exactly three operands; a fourth operand must be rejected
**Impact:** The assembler silently encodes `zip1 Vd.T, Vn.T, Vm.T, extra` as `zip1 Vd.T, Vn.T, Vm.T`, dropping the extra operand instead of diagnosing invalid assembly
**Function:** encode_neon_zip_uzp
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:1094
**Detected by:** Negative/Error Contract
**Minimal input:** encode_neon_zip_uzp([v0.8b, v0.8b, v0.8b, v0.8b], 0b011, false)
**Expected:** Err (llvm-mc/gas reject a fourth operand)
**Actual:** Ok(Word(0x0e003800)) — same as `zip1 v0.8b, v0.8b, v0.8b`
**Severity:** medium
**Root cause:** neon.rs:1095 uses `operands.len() < 3`, so any extra operands after the first three are ignored
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:1095`
```rust
    if operands.len() < 3 {
        return Err("uzp/zip requires 3 operands".to_string());
    }
```
**Suggested fix:** Reject arity other than 3
```rust
    if operands.len() != 3 {
        return Err("uzp/zip requires 3 operands".to_string());
    }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_zip_uzp_regression_extra_operand -- --test-threads=1 --exact
```
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_neon_zip_uzp_pbt::encode_neon_zip_uzp_neg_extra_operand' panicked at src/backend/arm/assembler/encoder/encode_neon_zip_uzp_pbt.rs:187:1:
Test failed: 4 operands must Err (llvm-mc rejects zip1 v0.8b, v0.8b, v0.8b, v0.8b) at src/backend/arm/assembler/encoder/encode_neon_zip_uzp_pbt.rs:317.
minimal failing input: rd = 0, rn = 0, rm = 0, extra = 0, t = "8b", m = "zip1"
	successes: 0
	local rejects: 0
	global rejects: 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_neon_zip_uzp_pbt.rs
