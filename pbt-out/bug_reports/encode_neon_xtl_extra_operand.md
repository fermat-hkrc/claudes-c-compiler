# Bug: encode_neon_xtl ignores extra operands
**Law:** UXTL/SXTL is a two-operand instruction; a third operand must be rejected
**Impact:** A typo or extra register in GNU-style `uxtl`/`sxtl` assembly is silently encoded as the two-operand form, so the assembler emits machine code that gas and llvm-mc refuse
**Function:** encode_neon_xtl
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:163
**Detected by:** Negative/Error Contract
**Minimal input:** encode_neon_xtl([v0.8h, v0.8b, v0.8h], u_bit=0, is_high=false)  (asm `sxtl v0.8h, v0.8b, v0.8h`)
**Expected:** Err
**Actual:** Ok(Word) — same encoding as `sxtl v0.8h, v0.8b`
**Severity:** medium
**Root cause:** neon.rs:164 checks only `operands.len() < 2`, so any extra operands past index 1 are ignored
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:164`
```rust
    if operands.len() < 2 {
        return Err("NEON uxtl/sxtl requires 2 operands".to_string());
    }
```
**Suggested fix:** Reject any arity other than 2
```rust
    if operands.len() != 2 {
        return Err("NEON uxtl/sxtl requires 2 operands".to_string());
    }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_neon_xtl -- --test-threads=1
cargo test --lib test_encode_neon_xtl_regression_extra_operand -- --test-threads=1
```
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_neon_xtl_pbt::encode_neon_xtl_neg_extra_operand' panicked at src/backend/arm/assembler/encoder/encode_neon_xtl_pbt.rs:366:1:
Test failed: 3 operands must Err (llvm-mc rejects sxtl v0.8h, v0.8b, v0.8h) at src/backend/arm/assembler/encoder/encode_neon_xtl_pbt.rs:386.
minimal failing input: rd = 0, rn = 0, extra = 0, pair = (
    "8h",
    "8b",
    false,
), u_bit = 0
	successes: 0
	local rejects: 0
	global rejects: 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_neon_xtl_pbt.rs
