# Bug: encode_neon_scalar_addp silently encodes a third operand
**Law:** Scalar ADDP Dd, Vn.2D requires exactly two operands; a third operand must be rejected
**Impact:** Invalid assembly such as `addp d0, v0.2d, d0` is assembled into a scalar ADDP word instead of an error, so the GNU-style assembler emits machine code that gas/llvm-mc refuse. encode() currently routes only arity==2 into this helper, so the public dispatcher sanitizes this witness; the helper itself still violates its documented two-operand contract.
**Function:** encode_neon_scalar_addp
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:1802
**Detected by:** Negative/Error Contract
**Minimal input:** encode_neon_scalar_addp([Reg("d0"), RegArrangement { reg: "v0", arrangement: "2d" }, Reg("d0")])
**Expected:** Err ("scalar addp requires 2 operands")
**Actual:** Ok(Word(0x5ef1b800)) encoding `addp d0, v0.2d`
**Severity:** medium
**Root cause:** neon.rs:1803 checks only `operands.len() < 2`, so arity 3+ is treated as a 2-operand encode using the first two operands
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:1803`
```rust
    if operands.len() < 2 { return Err("scalar addp requires 2 operands".to_string()); }
```
**Suggested fix:** Reject any arity other than 2
```rust
    if operands.len() != 2 { return Err("scalar addp requires 2 operands".to_string()); }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_neon_scalar_addp -- --test-threads=1
cargo test --lib encode_neon_scalar_addp_neg_extra_operand -- --test-threads=1
```
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_neon_scalar_addp_pbt::encode_neon_scalar_addp_neg_extra_operand' (2493589) panicked at src/backend/arm/assembler/encoder/encode_neon_scalar_addp_pbt.rs:170:1:
Test failed: 3 operands must Err (llvm-mc rejects addp d0, v0.2d, d0) at src/backend/arm/assembler/encoder/encode_neon_scalar_addp_pbt.rs:278.
minimal failing input: rd = 0, rn = 0, extra = 0
	successes: 0
	local rejects: 0
	global rejects: 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_neon_scalar_addp_pbt.rs
