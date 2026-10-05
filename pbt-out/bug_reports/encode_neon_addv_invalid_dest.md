# Bug: encode_neon_addv accepts a GPR destination
**Law:** ADDV destination must be a scalar SIMD register whose width matches T (B/H/S); GPR, SP, arranged V, and mismatched scalar width must be rejected
**Impact:** `addv x0, v0.8b` is assembled as if dest were b0, producing a NEON word for invalid assembly; gas/llvm-mc reject it
**Function:** encode_neon_addv
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:424
**Detected by:** Negative/Error Contract
**Minimal input:** encode_neon_addv([x0, v0.8b])
**Expected:** Err (llvm-mc/gas reject GPR dest)
**Actual:** Ok(Word(0x0e30dc00)) — same Rd field as `addv b0, v0.8b`
**Severity:** medium
**Root cause:** neon.rs:428 takes only the register number from dest (`let (rd, _)`) and never checks that dest is a matching B/H/S SIMD scalar
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:428`
```rust
    let (rd, _) = get_neon_reg(operands, 0)?;
```
**Suggested fix:** Require a scalar SIMD dest whose prefix matches T
```rust
    let dest = match &operands[0] {
        Operand::Reg(r) if dest_prefix_matches(r, &arr_n) => parse_reg_num(r).ok_or("invalid dest")?,
        _ => return Err("addv: dest must be Bd/Hd/Sd matching T".to_string()),
    };
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_addv_regression_gpr_dest -- --test-threads=1 --exact
```
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_neon_addv_pbt::encode_neon_addv_neg_dest' panicked at src/backend/arm/assembler/encoder/encode_neon_addv_pbt.rs:245:1:
Test failed: invalid dest must Err (llvm-mc rejects addv x0, v0.8b) at src/backend/arm/assembler/encoder/encode_neon_addv_pbt.rs:406.
minimal failing input: rd = 0, rn = 0, (v, t) = (
    "b",
    "8b",
), kind = 0
	successes: 0
	local rejects: 0
	global rejects: 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_neon_addv_pbt.rs
