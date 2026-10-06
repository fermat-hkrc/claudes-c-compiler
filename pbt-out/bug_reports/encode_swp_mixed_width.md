# Bug: encode_swp accepts mixed W/X data registers
**Law:** SWP Rs and Rt must have the same width (both W or both X)
**Impact:** `swp x0, w0, [x1]` is assembled as a 64-bit SWP using Rs from X0. llvm-mc/gas report "invalid operand for instruction". Mixed-width assembly is silently coerced to the Rs width
**Function:** encode_swp
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/load_store.rs:849
**Detected by:** Negative/Error Contract
**Minimal input:** encode_swp("swp", [Reg("x0"), Reg("w0"), Mem{base:"x1", offset:0}])
**Expected:** Err (llvm-mc: "invalid operand for instruction")
**Actual:** Ok(Word) — Rt width is discarded; size follows Rs
**Severity:** medium
**Root cause:** load_store.rs:854 binds `let (rt, _) = get_reg(operands, 1)?` and never compares Rt width to Rs
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/load_store.rs:854`
```rust
    let (rt, _) = get_reg(operands, 1)?;
```
**Suggested fix:** Require Rs and Rt to share the same GPR width
```rust
    let (rt, rt_64) = get_reg(operands, 1)?;
    if is_64 != rt_64 {
        return Err("swp: Rs and Rt must be the same width".to_string());
    }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_swp_regression_mixed_width -- --test-threads=1
```
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_swp_pbt::test_encode_swp_regression_mixed_width' panicked at src/backend/arm/assembler/encoder/encode_swp_pbt.rs:815:5:
swp x0, w0, [x1] must Err; llvm-mc/gas reject mixed W/X
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_swp_pbt.rs
