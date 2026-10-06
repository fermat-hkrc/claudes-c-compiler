# Bug: encode_tbz ignores extra operands
**Law:** TBZ/TBNZ take Rt, #bit, and a label or PC offset; a trailing fourth operand must be rejected.
**Impact:** The assembler silently encodes `tbz x0, #0, labl0, x1` as `tbz x0, #0, labl0`, so invalid GNU-style assembly produces a machine-code word. llvm-mc and gas reject the extra operand.
**Function:** encode_tbz
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/compare_branch.rs:254
**Detected by:** Negative/Error Contract
**Minimal input:** encode_tbz([Reg("x0"), Imm(0), Symbol("labl0"), Reg("x1")], false)
**Expected:** Err
**Actual:** Ok(WordWithReloc { word: 0x36000000, reloc: TstBr14 symbol=labl0 addend=0 }) — extra operand is never read
**Severity:** medium
**Root cause:** compare_branch.rs:254-271 reads only operands 0..2 via get_reg/get_imm/get_symbol and has no operands.len() upper bound.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/compare_branch.rs:257`
```rust
    let (sym, addend) = get_symbol(operands, 2)?;
```
**Suggested fix:** Reject extra operands before encoding.
```rust
    if operands.len() != 3 {
        return Err(format!("tbz: expected 3 operands, got {}", operands.len()));
    }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_tbz_regression_extra_operand -- --test-threads=1
```
**Raw output:**
```text
---- backend::arm::assembler::encoder::encode_tbz_pbt::encode_tbz_neg_extra_operand stdout ----
Test failed: tbz x0, #0, label, extra (which=0) must Err (llvm-mc: invalid operand) at src/backend/arm/assembler/encoder/encode_tbz_pbt.rs:521.
minimal failing input: n = 0, bit = 0, is_nz = false, suffix = 0, which = 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_tbz_pbt.rs
