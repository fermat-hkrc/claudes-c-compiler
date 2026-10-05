# Bug: encode_neon_scalar_three_same silently encodes a fourth operand
**Law:** Scalar ADD/SUB Dd, Dn, Dm requires exactly three operands; a fourth operand must be rejected
**Impact:** Invalid assembly such as `add d0, d0, d0, d0` is assembled into a scalar ADD word instead of an error, so the GNU-style assembler emits machine code that gas/llvm-mc refuse
**Function:** encode_neon_scalar_three_same
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:1791
**Detected by:** Negative/Error Contract
**Minimal input:** encode_neon_scalar_three_same([d0, d0, d0, d0], u_bit=0, opcode=0b10000, size=0b11)
**Expected:** Err ("scalar three-same requires 3 operands")
**Actual:** Ok(Word(0x5ee08400)) encoding `add d0, d0, d0`
**Severity:** medium
**Root cause:** neon.rs:1792 checks only `operands.len() < 3`, so arity 4+ is treated as a 3-operand encode using the first three registers
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:1792`
```rust
    if operands.len() < 3 { return Err("scalar three-same requires 3 operands".to_string()); }
```
**Suggested fix:** Reject any arity other than 3
```rust
    if operands.len() != 3 { return Err("scalar three-same requires 3 operands".to_string()); }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_neon_scalar_three_same -- --test-threads=1
cargo test --lib encode_neon_scalar_three_same_neg_extra_operand -- --test-threads=1
```
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_neon_scalar_three_same_pbt::encode_neon_scalar_three_same_neg_extra_operand' (2480918) panicked at src/backend/arm/assembler/encoder/encode_neon_scalar_three_same_pbt.rs:182:1:
Test failed: 4 operands must Err (llvm-mc rejects add d0, d0, d0, d0) at src/backend/arm/assembler/encoder/encode_neon_scalar_three_same_pbt.rs:321.
minimal failing input: rd = 0, rn = 0, rm = 0, extra = 0, is_sub = false
	successes: 0
	local rejects: 0
	global rejects: 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_neon_scalar_three_same_pbt.rs
