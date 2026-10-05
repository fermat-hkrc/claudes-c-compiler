# Bug: encode_neon_float_elem accepts an X-prefixed destination as Vd
**Law:** Dest must be a V register with arrangement (Vd.T); llvm-mc/gas reject `fmul x0.4s, ...`
**Impact:** `fmul x0.4s, v0.4s, v0.s[0]` encodes as `fmul v0.4s, ...` because parse_reg_num maps x0 to 0
**Function:** encode_neon_float_elem
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:1613
**Detected by:** Negative/Error Contract
**Minimal input:** encode_neon_float_elem([RegArrangement { reg: "x0", arrangement: "4s" }, v0.4s, v0.s[0]], u_bit=0, opcode=0b1001)
**Expected:** Err (llvm-mc: invalid operand)
**Actual:** Ok(Word) with Rd=0
**Severity:** medium
**Root cause:** neon.rs:1616 uses get_neon_reg, which calls parse_reg_num; that helper accepts the x prefix and returns 0, so an X-prefixed arranged dest is encoded as v0
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:1616`
```rust
    let (rd, arr_d) = get_neon_reg(operands, 0)?;
```
**Suggested fix:** Require a V (or uppercase V) prefix on dest and source register names before encoding
```rust
    if !reg.to_ascii_lowercase().starts_with('v') {
        return Err(format!("expected NEON V register, got {}", reg));
    }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_neon_float_elem_neg_gpr_bare_nonv -- --test-threads=1
```
**Raw output:**
```text
Test failed: non-arranged NEON / GPR / SP / non-V prefix must Err (llvm-mc rejects fmul x0.4s, v0.4s, v0.s[0])
minimal failing input: rd = 0, rn = 0, rm = 0, idx = 0, kind = 4
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_neon_float_elem_pbt.rs::test_encode_neon_float_elem_regression_x_prefix
