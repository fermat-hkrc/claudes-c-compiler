# Bug: encode_neon_elem_long accepts a GPR destination as a NEON register
**Law:** Long by-element SMULL/UMULL/SMLAL/… require a NEON Vd.Ta destination; a GPR (x/w/sp/s/…) dest must be rejected
**Impact:** `smull x0, v0.4h, v0.h[0]` is encoded as if dest were v0, so illegal assembly becomes a NEON instruction targeting the same register number
**Function:** encode_neon_elem_long
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:235
**Detected by:** Negative/Error Contract
**Minimal input:** encode_neon_elem_long([x0, v0.4h, v0.h[0]], u=0, opcode=0b1010, is_high=false)
**Expected:** Err
**Actual:** Ok(Word) with Rd=0
**Severity:** medium
**Root cause:** neon.rs:239 extracts dest via get_neon_reg, which accepts Operand::Reg and any parse_reg_num prefix (x/w/s/sp), and then discards the arrangement
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:239`
```rust
    let (rd, _arr_d) = get_neon_reg(operands, 0)?;
```
**Suggested fix:** Require dest to be Operand::RegArrangement with a `v` prefix (and a matching Ta); reject Operand::Reg / x/w/sp/s prefixes
```rust
    let (rd, arr_d) = match operands.get(0) {
        Some(Operand::RegArrangement { reg, arrangement }) if reg.to_lowercase().starts_with('v') => {
            let num = parse_reg_num(reg).ok_or_else(|| format!("invalid NEON register: {}", reg))?;
            (num, arrangement.clone())
        }
        other => return Err(format!("expected NEON Vd.Ta at operand 0, got {:?}", other)),
    };
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_elem_long_regression_gpr_dest -- --test-threads=1
```
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_neon_elem_long_pbt::test_encode_neon_elem_long_regression_gpr_dest' panicked at src/backend/arm/assembler/encoder/encode_neon_elem_long_pbt.rs:547:5:
smull x0, v0.4h, v0.h[0] must Err (llvm-mc/gas require Vd.4s)
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_neon_elem_long_pbt.rs
