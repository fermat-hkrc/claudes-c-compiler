# Bug: encode_neon_eor3 ignores a fifth operand
**Law:** EOR3 takes exactly four operands; a fifth operand must be rejected
**Impact:** The assembler silently encodes `eor3 Vd.16b, Vn.16b, Vm.16b, Vk.16b, extra` as `eor3 Vd.16b, Vn.16b, Vm.16b, Vk.16b`, dropping the extra operand instead of diagnosing invalid assembly
**Function:** encode_neon_eor3
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:1112
**Detected by:** Negative/Error Contract
**Minimal input:** encode_neon_eor3([v0.16b, v0.16b, v0.16b, v0.16b, v0.16b])
**Expected:** Err (llvm-mc/gas reject a fifth operand)
**Actual:** Ok(Word(0xce000000)) — same as `eor3 v0.16b, v0.16b, v0.16b, v0.16b`
**Severity:** medium
**Root cause:** neon.rs:1113 uses `operands.len() < 4`, so any extra operands after the first four are ignored
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:1113`
```rust
    if operands.len() < 4 {
        return Err("eor3 requires 4 operands".to_string());
    }
```
**Suggested fix:** Reject arity other than 4
```rust
    if operands.len() != 4 {
        return Err("eor3 requires 4 operands".to_string());
    }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_eor3_regression_extra_operand -- --test-threads=1 --exact
```
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_neon_eor3_pbt::encode_neon_eor3_neg_extra_operand' panicked at src/backend/arm/assembler/encoder/encode_neon_eor3_pbt.rs:146:1:
Test failed: 5 operands must Err (llvm-mc rejects eor3 v0.16b, v0.16b, v0.16b, v0.16b, v0.16b) at src/backend/arm/assembler/encoder/encode_neon_eor3_pbt.rs:276.
minimal failing input: rd = 0, rn = 0, rm = 0, rk = 0, extra = 0
	successes: 0
	local rejects: 0
	global rejects: 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_neon_eor3_pbt.rs
