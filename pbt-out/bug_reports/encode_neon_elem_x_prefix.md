# Bug: encode_neon_elem accepts an X-prefixed arranged destination as a V register
**Law:** GNU-style by-element MUL/MLA/MLS requires a V-register destination (Vd.T); an X/W/S-prefixed arranged dest must be rejected
**Impact:** `mul x0.4h, v0.4h, v0.h[0]` encodes as `mul v0.4h, ...` because parse_reg_num treats the `x` prefix as a register number, so illegal GPR-looking assembly becomes a NEON word
**Function:** encode_neon_elem
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:1591
**Detected by:** Negative/Error Contract
**Minimal input:** encode_neon_elem([x0.4h, v0.4h, v0.h[0]], u=0, opcode=0b1000)
**Expected:** Err
**Actual:** Ok(Word) encoding Rd=0 as if dest were v0.4h
**Severity:** medium
**Root cause:** neon.rs:1593 extracts dest via get_neon_reg, which calls parse_reg_num; parse_reg_num accepts prefix x/w/d/s/q/h/b as well as v, so `x0.4h` becomes Rd=0
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:1593`
```rust
    let (rd, arr_d) = get_neon_reg(operands, 0)?;
```
**Suggested fix:** Require a `v` prefix on arranged NEON registers before encoding
```rust
    let (rd, arr_d) = get_neon_reg(operands, 0)?;
    let dest_name = match &operands[0] {
        Operand::RegArrangement { reg, .. } => reg,
        _ => return Err("expected NEON register".to_string()),
    };
    if !dest_name.to_lowercase().starts_with('v') {
        return Err(format!("expected V register, got {dest_name}"));
    }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_elem_regression_x_prefix -- --test-threads=1
```
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_neon_elem_pbt::test_encode_neon_elem_regression_x_prefix' panicked at src/backend/arm/assembler/encoder/encode_neon_elem_pbt.rs:631:5:
mul x0.4h, v0.4h, v0.h[0] must Err (llvm-mc/gas require Vd)
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_neon_elem_pbt.rs
