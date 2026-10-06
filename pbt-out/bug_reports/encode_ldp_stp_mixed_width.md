# Bug: encode_ldp_stp accepts a mixed X/W pair
**Law:** LDP/STP Rt1 and Rt2 must be the same class and width (both Wt, both Xt, or both S/D/Q of one size)
**Impact:** `stp x0, w0, [x0]` is encoded as a 64-bit pair using Rt1's width; the W register is treated as X0
**Function:** encode_ldp_stp
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/load_store.rs:452
**Detected by:** Negative/Error Contract
**Minimal input:** encode_ldp_stp([Reg("x0"), Reg("w0"), Mem{base:"x0", offset:0}], is_load=false)
**Expected:** Err (llvm-mc: invalid operand for instruction)
**Actual:** Ok(Word) with opc=10 (64-bit) and Rt2=0
**Severity:** medium
**Root cause:** load_store.rs:458 discards Rt2's width (`let (rt2, _) = get_reg(operands, 1)?`) and takes opc/shift only from Rt1
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/load_store.rs:458`
```rust
    let (rt2, _) = get_reg(operands, 1)?;
```
**Suggested fix:** Compare Rt1 and Rt2 widths (and FP class) and return Err on mismatch
```rust
    let (rt2, is_64_rt2) = get_reg(operands, 1)?;
    if is_64 != is_64_rt2 {
        return Err("ldp/stp register size mismatch".to_string());
    }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_ldp_stp_regression_mixed_width -- --test-threads=1
```
**Raw output:**
```text
Test failed: accepted invalid register forms (llvm-mc rejects): [..., "mixed X/W pair", ...]
minimal failing input: is_load = false, is_64 = false, rt = 0, rt2 = 0, rn = 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_ldp_stp_pbt.rs
