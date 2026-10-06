# Bug: encode_fnmadd_fnmsub ignores extra operands
**Law:** Scalar FNMADD/FNMSUB takes exactly four matching FP registers (Sd, Sn, Sm, Sa or Dd, Dn, Dm, Da); a fifth operand must be rejected.
**Impact:** The assembler silently encodes `fnmadd s0, s0, s0, s0, s0` (and extra Imm/Shift/RegArrangement) as the four-operand scalar form, so invalid GNU-style assembly produces a machine-code word. llvm-mc and gas reject the extra operand.
**Function:** encode_fnmadd_fnmsub
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/fp_scalar.rs:146
**Detected by:** Negative/Error Contract
**Minimal input:** encode_fnmadd_fnmsub([Reg("s0"), Reg("s0"), Reg("s0"), Reg("s0"), Reg("s0")], false)
**Expected:** Err
**Actual:** Ok(Word) — get_reg only reads indices 0..3, so operands beyond index 3 are ignored
**Severity:** medium
**Root cause:** fp_scalar.rs:147-150 four get_reg calls with no operands.len() == 4 check, so a fifth operand is never inspected.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/fp_scalar.rs:147`
```rust
    let (rd, _) = get_reg(operands, 0)?;
    let (rn, _) = get_reg(operands, 1)?;
    let (rm, _) = get_reg(operands, 2)?;
    let (ra, _) = get_reg(operands, 3)?;
```
**Suggested fix:** Reject extra operands before encoding.
```rust
    if operands.len() != 4 {
        return Err("fnmadd/fnmsub requires 4 operands".to_string());
    }
    let (rd, _) = get_reg(operands, 0)?;
    let (rn, _) = get_reg(operands, 1)?;
    let (rm, _) = get_reg(operands, 2)?;
    let (ra, _) = get_reg(operands, 3)?;
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_fnmadd_fnmsub_regression_extra_operand -- --test-threads=1
```
**Raw output:**
```text
---- backend::arm::assembler::encoder::fp_scalar::encode_fnmadd_fnmsub_pbt::encode_fnmadd_fnmsub_neg_extra_operand stdout ----
Test failed: FNMADD/FNMSUB has no 5th operand; extra must Err
minimal failing input: rd = 0, rn = 0, rm = 0, ra = 0, is_d = false, is_sub = false, extra = Reg("s0")
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_fnmadd_fnmsub_pbt.rs
