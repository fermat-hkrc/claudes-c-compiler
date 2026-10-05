# Bug: encode_neon_two_misc_narrow silently encodes a third operand
**Law:** XTN/SQXTN/UQXTN/SQXTUN require exactly two operands; a third operand must be rejected
**Impact:** Invalid assembly such as `xtn v0.8b, v0.8h, v0.8b` is assembled into an XTN word instead of an error, so the GNU-style assembler emits machine code that gas/llvm-mc refuse. encode() routes xtn/xtn2/sqxtn/sqxtn2/uqxtn/uqxtn2/sqxtun/sqxtun2 into this helper with no arity maximum, so the witness is caller-reachable from the public assembler.
**Function:** encode_neon_two_misc_narrow
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:206
**Detected by:** Negative/Error Contract
**Minimal input:** encode_neon_two_misc_narrow([RegArrangement("v0","8b"), RegArrangement("v0","8h"), RegArrangement("v0","8b")], u_bit=0, opcode=0b10010, is_high=false)
**Expected:** Err ("NEON two-reg narrow requires 2 operands")
**Actual:** Ok(Word(0x0e212800)) encoding the first two operands
**Severity:** medium
**Root cause:** neon.rs:207 checks only `operands.len() < 2`, so arity 3+ is treated as a 2-operand encode using the first two operands
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:207`
```rust
    if operands.len() < 2 {
        return Err("NEON two-reg narrow requires 2 operands".to_string());
    }
```
**Suggested fix:** Reject any arity other than 2
```rust
    if operands.len() != 2 {
        return Err("NEON two-reg narrow requires 2 operands".to_string());
    }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_neon_two_misc_narrow_neg_extra_operand -- --test-threads=1
cargo test --lib test_encode_neon_two_misc_narrow_regression_extra_operand -- --test-threads=1
```
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_neon_two_misc_narrow_pbt::encode_neon_two_misc_narrow_neg_extra_operand' (2513865) panicked at src/backend/arm/assembler/encoder/encode_neon_two_misc_narrow_pbt.rs:423:1:
Test failed: 3 operands must Err (llvm-mc rejects xtn v0.8b, v0.8h, v0.8b) at src/backend/arm/assembler/encoder/encode_neon_two_misc_narrow_pbt.rs:444.
minimal failing input: rd = 0, rn = 0, extra = 0, pair = (
    "8b",
    "8h",
    false,
), fam = (
    "xtn",
    0,
    18,
)
	successes: 0
	local rejects: 0
	global rejects: 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_neon_two_misc_narrow_pbt.rs
