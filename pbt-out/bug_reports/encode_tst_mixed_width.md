# Bug: encode_tst accepts mixed W/X register pairs
**Law:** TST Rn, Rm requires matching 32-bit or 64-bit GPRs; mixed W/X must be rejected.
**Impact:** `tst w0, x0` is encoded as `tst w0, w0` (Rm number 0, sf from the prepended WZR). Invalid mixed-width assembly becomes a different well-formed instruction. llvm-mc reports “expected compatible register or logical immediate”.
**Function:** encode_tst
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/compare_branch.rs:37
**Detected by:** Negative/Error Contract
**Minimal input:** encode_tst([Reg("w0"), Reg("x0")])
**Expected:** Err
**Actual:** Ok(Word(0x6a00001f)) — sf from WZR, Rm = parse_reg_num("x0") = 0
**Severity:** medium
**Root cause:** encode_tst derives width only from the first operand (compare_branch.rs:41-49) and encode_logical takes Rm as a 5-bit number with no width check (data_processing.rs:480).
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/data_processing.rs:480`
```rust
        let rm = parse_reg_num(rm_name).ok_or("invalid rm")?;
```
**Suggested fix:** Reject mixed widths in encode_tst.
```rust
    if let (Some(Operand::Reg(rn)), Some(Operand::Reg(rm))) = (operands.get(0), operands.get(1)) {
        if is_32bit_reg(rn) != is_32bit_reg(rm) {
            return Err("tst: mixed register widths".to_string());
        }
    }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_tst_regression_mixed_width -- --test-threads=1
```
**Raw output:**
```text
---- backend::arm::assembler::encoder::encode_tst_pbt::encode_tst_neg_mixed_width stdout ----
Test failed: tst mixed W/X must Err (llvm-mc: expected compatible register) at src/backend/arm/assembler/encoder/encode_tst_pbt.rs:669.
minimal failing input: n = 0, x_first = false
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_tst_pbt.rs
