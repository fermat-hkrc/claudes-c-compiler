# Bug: encode_neon_fcvtl encodes an X-prefixed destination as a V register
**Law:** FCVTL dest must be a NEON V register with arrangement (Vd.Ta); gas/llvm-mc reject `fcvtl x0.4s, v0.4h`
**Impact:** A GPR-prefixed arrangement dest is encoded as the corresponding V register, so a mistyped `x0.4s` silently becomes `v0.4s`
**Function:** encode_neon_fcvtl
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:1640
**Detected by:** Negative/Error Contract
**Minimal input:** encode_neon_fcvtl([RegArrangement{reg:"x0", arrangement:"4s"}, v0.4h], is_high=false)
**Expected:** Err (llvm-mc: invalid operand)
**Actual:** Ok(Word) — parse_reg_num accepts the 'x' prefix and the word is the same as `fcvtl v0.4s, v0.4h`
**Severity:** medium
**Root cause:** neon.rs:1641 calls get_neon_reg, and parse_reg_num treats x/w/d/s/q/h/b prefixes as register numbers; encode_neon_fcvtl never requires a V prefix
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:1641`
```rust
    let (rd, arr_d) = get_neon_reg(operands, 0)?;
```
**Suggested fix:** Reject a destination whose register name is not V-prefixed
```rust
    let (rd, arr_d) = get_neon_reg(operands, 0)?;
    match &operands[0] {
        Operand::RegArrangement { reg, .. } if reg.to_ascii_lowercase().starts_with('v') => {}
        _ => return Err("fcvtl: destination must be a V register with arrangement".to_string()),
    }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_neon_fcvtl_neg_gpr_or_bare -- --test-threads=1
```
**Raw output:**
```text
Test failed: GPR/bare/non-arrangement kind=3 must Err (llvm-mc rejects fcvtl x0.4s, v0.4h) at src/backend/arm/assembler/encoder/encode_neon_fcvtl_pbt.rs:438.
minimal failing input: rd = 0, rn = 0, kind = 3, fp_prefix = "x"
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_neon_fcvtl_pbt.rs
