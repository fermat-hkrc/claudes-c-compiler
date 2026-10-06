# Bug: encode_ldnp_stnp accepts invalid register forms that llvm-mc rejects
**Law:** LDNP/STNP Rt is Wt/Xt (31=WZR/XZR, never SP); Rn is Xn|SP (not XZR, not W, not WSP, not x31); Rt1 and Rt2 must match width
**Impact:** SP dest encodes as XZR, XZR/x31 base encodes as SP, W/WSP base encodes as Xn, mixed X/W uses Rt1 width — silent wrong encodings for assembler-invalid input
**Function:** encode_ldnp_stnp
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/load_store.rs:518
**Detected by:** Negative/Error Contract
**Minimal input:** encode_ldnp_stnp([Reg("sp"), Reg("w0"), Mem{base:"x0", offset:0}], is_load=false) — also XZR/x31/W/WSP base, mixed X/W
**Expected:** Err (llvm-mc: invalid operand)
**Actual:** Ok(Word) for each of those forms
**Severity:** medium
**Root cause:** load_store.rs:523-524 get_reg / parse_reg_num maps SP and XZR both to 31, accepts W-prefixed bases, and discards Rt2 width
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/load_store.rs:523`
```rust
    let (rt1, is_64) = get_reg(operands, 0)?;
    let (rt2, _) = get_reg(operands, 1)?;
```
**Suggested fix:** Reject SP as Rt, XZR/W/WSP/x31 as base, and mixed widths before packing the word
```rust
    if name_is_sp(operands[0]) || name_is_sp(operands[1]) {
        return Err("ldnp/stnp Rt cannot be SP".to_string());
    }
    // also reject XZR/W/WSP/x31 base and mixed Rt widths
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_ldnp_stnp_regression_sp_dest -- --test-threads=1
```
**Raw output:**
```text
Test failed: accepted invalid register forms (llvm-mc rejects): ["SP as Rt1", "SP as Rt2", "XZR base", "x31 base", "W base", "WSP base", "mixed X/W pair"]
minimal failing input: is_load = false, is_64 = false, rt = 0, rt2 = 0, rn = 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_ldnp_stnp_pbt.rs
