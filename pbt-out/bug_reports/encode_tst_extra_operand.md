# Bug: encode_tst ignores extra operands
**Law:** TST takes Rn and Rm{, shift} or Rn and #imm; a trailing non-shift operand must be rejected.
**Impact:** The assembler silently encodes `tst w0, w0, x0` as `tst w0, w0`, so invalid GNU-style assembly produces a machine-code word. llvm-mc and gas reject the extra operand.
**Function:** encode_tst
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/compare_branch.rs:37
**Detected by:** Negative/Error Contract
**Minimal input:** encode_tst([Reg("w0"), Reg("w0"), Reg("x0")])
**Expected:** Err
**Actual:** Ok(Word(0x6a00001f)) — extra non-Shift is copied through to encode_logical and ignored
**Severity:** medium
**Root cause:** compare_branch.rs:40 clones every operand into the ANDS alias list with no upper-bound arity check; encode_logical.rs:456 only requires `len < 3` after the ZR prepend, and treats a non-Shift fourth operand as “no shift”.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/compare_branch.rs:40`
```rust
    new_ops.extend(operands.iter().cloned());
```
**Suggested fix:** Reject extra operands before encoding.
```rust
    if operands.len() > 3
        || (operands.len() == 3 && !matches!(operands.get(2), Some(Operand::Shift { .. })))
    {
        return Err("tst: unexpected extra operand".to_string());
    }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_tst_regression_extra_operand -- --test-threads=1
```
**Raw output:**
```text
---- backend::arm::assembler::encoder::encode_tst_pbt::encode_tst_neg_extra_operand stdout ----
Test failed: tst Rn, Rm, extra must Err (llvm-mc: invalid operand) at src/backend/arm/assembler/encoder/encode_tst_pbt.rs:636.
minimal failing input: rn = 0, rm = 0, is_64 = false, extra = Reg("x0")
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_tst_pbt.rs
