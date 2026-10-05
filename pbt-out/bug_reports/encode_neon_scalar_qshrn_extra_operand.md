# Bug: encode_neon_scalar_qshrn silently encodes a fourth operand
**Law:** Scalar SQSHRN requires exactly three operands; a fourth operand must be rejected
**Impact:** Invalid assembly such as `sqshrn b0, h0, #1, b0` is assembled into a scalar SQSHRN word instead of an error, so the GNU-style assembler emits machine code that gas/llvm-mc refuse. encode() routes any non-RegArrangement dest into this helper with no arity maximum, so the witness is caller-reachable from the public assembler.
**Function:** encode_neon_scalar_qshrn
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:1835
**Detected by:** Negative/Error Contract
**Minimal input:** encode_neon_scalar_qshrn([Reg("b0"), Reg("h0"), Imm(1), Reg("b0")], u_bit=0, is_rounding=false)
**Expected:** Err ("scalar qshrn requires 3 operands")
**Actual:** Ok(Word) encoding the first three operands
**Severity:** medium
**Root cause:** neon.rs:1836 checks only `operands.len() < 3`, so arity 4+ is treated as a 3-operand encode using the first three operands
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:1836`
```rust
    if operands.len() < 3 { return Err("scalar qshrn requires 3 operands".to_string()); }
```
**Suggested fix:** Reject any arity other than 3
```rust
    if operands.len() != 3 { return Err("scalar qshrn requires 3 operands".to_string()); }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_neon_scalar_qshrn_neg_extra_operand -- --test-threads=1
cargo test --lib test_encode_neon_scalar_qshrn_regression_extra_operand -- --test-threads=1
```
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_neon_scalar_qshrn_pbt::encode_neon_scalar_qshrn_neg_extra_operand' (2507286) panicked at src/backend/arm/assembler/encoder/encode_neon_scalar_qshrn_pbt.rs:207:1:
Test failed: 4 operands must Err (llvm-mc rejects sqshrn b0, h0, #1, b0) at src/backend/arm/assembler/encoder/encode_neon_scalar_qshrn_pbt.rs:364.
minimal failing input: rd = 0, rn = 0, extra = 0, case = (
    "b",
    "h",
    8,
    1,
), u = 0, round = false
	successes: 0
	local rejects: 0
	global rejects: 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_neon_scalar_qshrn_pbt.rs
