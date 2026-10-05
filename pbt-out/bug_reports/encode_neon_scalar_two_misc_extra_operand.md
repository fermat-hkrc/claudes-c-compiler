# Bug: encode_neon_scalar_two_misc silently encodes a third operand
**Law:** Scalar SQABS/SQNEG requires exactly two operands; a third operand must be rejected
**Impact:** Invalid assembly such as `sqabs b0, b0, b0` is assembled into a scalar SQABS word instead of an error, so the GNU-style assembler emits machine code that gas/llvm-mc refuse. encode() routes any non-RegArrangement dest into this helper with no arity maximum, so the witness is caller-reachable from the public assembler.
**Function:** encode_neon_scalar_two_misc
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:1819
**Detected by:** Negative/Error Contract
**Minimal input:** encode_neon_scalar_two_misc([Reg("b0"), Reg("b0"), Reg("b0")], u_bit=0, opcode=0b00111)
**Expected:** Err ("scalar two-misc requires 2 operands")
**Actual:** Ok(Word(0x5e207800)) encoding `sqabs b0, b0`
**Severity:** medium
**Root cause:** neon.rs:1820 checks only `operands.len() < 2`, so arity 3+ is treated as a 2-operand encode using the first two operands
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:1820`
```rust
    if operands.len() < 2 { return Err("scalar two-misc requires 2 operands".to_string()); }
```
**Suggested fix:** Reject any arity other than 2
```rust
    if operands.len() != 2 { return Err("scalar two-misc requires 2 operands".to_string()); }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_neon_scalar_two_misc -- --test-threads=1
cargo test --lib encode_neon_scalar_two_misc_neg_extra_operand -- --test-threads=1
```
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_neon_scalar_two_misc_pbt::encode_neon_scalar_two_misc_neg_extra_operand' (2502835) panicked at src/backend/arm/assembler/encoder/encode_neon_scalar_two_misc_pbt.rs:199:1:
Test failed: 3 operands must Err (llvm-mc rejects sqabs b0, b0, b0) at src/backend/arm/assembler/encoder/encode_neon_scalar_two_misc_pbt.rs:347.
minimal failing input: rd = 0, rn = 0, extra = 0, pfx = "b", is_neg = false
	successes: 0
	local rejects: 0
	global rejects: 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_neon_scalar_two_misc_pbt.rs
