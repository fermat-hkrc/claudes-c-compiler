# Bug: encode_neon_pmull ignores a fourth operand
**Law:** PMULL/PMULL2 take exactly three operands; a fourth operand must be rejected
**Impact:** The assembler silently encodes `pmull Vd.1q, Vn.1d, Vm.1d, extra` as `pmull Vd.1q, Vn.1d, Vm.1d`, dropping the extra operand instead of diagnosing invalid assembly
**Function:** encode_neon_pmull
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:1128
**Detected by:** Negative/Error Contract
**Minimal input:** encode_neon_pmull([v0.1q, v0.1d, v0.1d, v0.1d], is_pmull2=false)
**Expected:** Err (llvm-mc/gas reject a fourth operand)
**Actual:** Ok(Word(0x0ee0e000)) — same as `pmull v0.1q, v0.1d, v0.1d`
**Severity:** medium
**Root cause:** neon.rs:1129 uses `operands.len() < 3`, so any extra operands after the first three are ignored
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:1129`
```rust
    if operands.len() < 3 {
        return Err("pmull requires 3 operands".to_string());
    }
```
**Suggested fix:** Reject arity other than 3
```rust
    if operands.len() != 3 {
        return Err("pmull requires 3 operands".to_string());
    }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_pmull_regression_extra_operand -- --test-threads=1 --exact
```
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_neon_pmull_pbt::encode_neon_pmull_neg_extra_operand' (2327734) panicked at src/backend/arm/assembler/encoder/encode_neon_pmull_pbt.rs:210:1:
Test failed: 4 operands must Err (llvm-mc rejects pmull v0.1q, v0.1d, v0.1d, v0.1d) at src/backend/arm/assembler/encoder/encode_neon_pmull_pbt.rs:363.
minimal failing input: rd = 0, rn = 0, rm = 0, extra = 0, is_pmull2 = false
	successes: 0
	local rejects: 0
	global rejects: 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_neon_pmull_pbt.rs
