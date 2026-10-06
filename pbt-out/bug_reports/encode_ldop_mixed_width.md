# Bug: encode_ldop accepts mixed W/X, FP registers, and X registers on byte/half variants
**Law:** LDADD Rs and Rt must be the same integer width; byte/half variants require W registers; FP/SIMD registers are invalid
**Impact:** `ldadd x0, w0, [x1]` encodes using Rs's 64-bit size and ignores Rt's width. `ldadd s0, s1, [x2]` and `ldaddb x0, x1, [x2]` likewise assemble. llvm-mc/gas report "invalid operand for instruction"
**Function:** encode_ldop
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/load_store.rs:882
**Detected by:** Negative/Error Contract
**Minimal input:** encode_ldop("ldadd", [Reg("x0"), Reg("w0"), Mem{base:"x1", offset:0}])
**Expected:** Err (llvm-mc: "invalid operand for instruction")
**Actual:** Ok(Word) — size comes from Rs (or the b/h suffix); Rt width and register class are ignored
**Severity:** medium
**Root cause:** load_store.rs:886-887 take is_64 only from Rs and discard Rt's width; parse_reg_num accepts d/s/q/v/h/b prefixes; size for ldaddb/ldaddh is taken from the suffix, not the register class
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/load_store.rs:886`
```rust
    let (rs, is_64) = get_reg(operands, 0)?;
    let (rt, _) = get_reg(operands, 1)?;
```
**Suggested fix:** Require matching GPR widths, reject FP prefixes, and require W registers for byte/half variants
```rust
    let (rs, is_64) = get_reg(operands, 0)?;
    let (rt, rt_64) = get_reg(operands, 1)?;
    if is_64 != rt_64 {
        return Err("ldop: Rs and Rt must be the same width".to_string());
    }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_ldop_regression_mixed_width -- --test-threads=1
```
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_ldop_pbt::test_encode_ldop_regression_mixed_width' panicked at src/backend/arm/assembler/encoder/encode_ldop_pbt.rs:894:5:
ldadd x0, w0, [x1] must Err; llvm-mc/gas reject mixed W/X
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_ldop_pbt.rs
