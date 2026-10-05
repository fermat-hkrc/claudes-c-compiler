# Bug: encode_neon_across accepts a GPR destination
**Law:** UMAXV/UMINV/SMAXV/SMINV destination must be a scalar SIMD register whose width matches T (B/H/S); GPR, SP, arranged V, and mismatched scalar width must be rejected
**Impact:** `umaxv x0, v0.8b` is assembled as if dest were b0, producing a NEON word for invalid assembly; gas/llvm-mc reject it
**Function:** encode_neon_across
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:445
**Detected by:** Negative/Error Contract
**Minimal input:** encode_neon_across([x0, v0.8b], 1, 0b01010)
**Expected:** Err (llvm-mc/gas reject GPR dest)
**Actual:** Ok(Word(0x2e30a800)) — same Rd field as `umaxv b0, v0.8b`
**Severity:** medium
**Root cause:** neon.rs:449 takes only the register number from dest (`let (rd, _)`) and never checks that dest is a matching B/H/S SIMD scalar
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:449`
```rust
    let (rd, _) = get_neon_reg(operands, 0)?;
```
**Suggested fix:** Require a scalar SIMD dest whose prefix matches T
```rust
    let dest = match &operands[0] {
        Operand::Reg(r) if dest_prefix_matches(r, &arr_n) => parse_reg_num(r).ok_or("invalid dest")?,
        _ => return Err("NEON across-vector: dest must be Bd/Hd/Sd matching T".to_string()),
    };
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_across_regression_gpr_dest -- --test-threads=1 --exact
```
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_neon_across_pbt::encode_neon_across_neg_dest' panicked at src/backend/arm/assembler/encoder/encode_neon_across_pbt.rs:297:1:
Test failed: invalid dest must Err (llvm-mc rejects umaxv x0, v0.8b) at src/backend/arm/assembler/encoder/encode_neon_across_pbt.rs:491.
minimal failing input: rd = 0, rn = 0, (v, t) = (
    "b",
    "8b",
), (mnem, u_bit, opcode) = (
    "umaxv",
    1,
    10,
), kind = 0
	successes: 0
	local rejects: 0
	global rejects: 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_neon_across_pbt.rs
