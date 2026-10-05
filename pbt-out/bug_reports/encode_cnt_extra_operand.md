# Bug: encode_cnt ignores a third operand
**Law:** CNT takes exactly two SIMD operands; a third operand must be rejected
**Impact:** The assembler silently encodes `cnt Vd.T, Vn.T, Vextra.T` as `cnt Vd.T, Vn.T`, dropping the extra operand instead of diagnosing invalid assembly (gas: unexpected characters following instruction)
**Function:** encode_cnt
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:23
**Detected by:** Negative/Error Contract
**Minimal input:** encode_cnt([v0.8b, v0.8b, v0.8b])
**Expected:** Err (llvm-mc/gas reject a third operand)
**Actual:** Ok(Word(0x0e205800)) — same as `cnt v0.8b, v0.8b`
**Severity:** medium
**Root cause:** neon.rs:27 uses `operands.len() < 2`, so any extra operands after the first two are ignored
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:27`
```rust
    if operands.len() < 2 {
        return Err("cnt requires 2 operands".to_string());
    }
```
**Suggested fix:** Reject arity other than 2
```rust
    if operands.len() != 2 {
        return Err("cnt requires 2 operands".to_string());
    }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_cnt_regression_extra_operand -- --test-threads=1 --exact
```
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_cnt_pbt::encode_cnt_neg_extra' panicked at src/backend/arm/assembler/encoder/encode_cnt_pbt.rs:166:1:
Test failed: extra operand must Err (llvm-mc rejects cnt v0.8b, v0.8b, v0.8b) at src/backend/arm/assembler/encoder/encode_cnt_pbt.rs:254.
minimal failing input: rd = 0, rn = 0, extra = 0, t = "8b"
	successes: 0
	local rejects: 0
	global rejects: 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_cnt_pbt.rs
