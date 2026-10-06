# Bug: encode_ldp_stp accepts SP as Rt1/Rt2
**Law:** LDP/STP Rt must be a GPR (Wt/Xt, 31=WZR/XZR) or SIMD S/D/Q; SP is not a valid pair transfer register
**Impact:** `stp sp, w0, [x0]` is encoded as STP XZR, W0, [X0] (Rt=31), so stack-pointer pair stores assemble as ZR stores
**Function:** encode_ldp_stp
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/load_store.rs:452
**Detected by:** Negative/Error Contract
**Minimal input:** encode_ldp_stp([Reg("sp"), Reg("w0"), Mem{base:"x0", offset:0}], is_load=false)
**Expected:** Err (llvm-mc: invalid operand for instruction)
**Actual:** Ok(Word) with Rt1=31 (XZR), opc from is_64bit_reg("sp")=true
**Severity:** medium
**Root cause:** load_store.rs:457 `get_reg` → `parse_reg_num` maps both "sp" and "xzr" to 31; encode_ldp_stp never distinguishes SP from ZR for Rt
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/load_store.rs:457`
```rust
    let (rt1, is_64) = get_reg(operands, 0)?;
    let (rt2, _) = get_reg(operands, 1)?;
```
**Suggested fix:** Reject Rt names that are SP/WSP before encoding
```rust
    if matches!(name.to_lowercase().as_str(), "sp" | "wsp") {
        return Err("ldp/stp Rt cannot be SP".to_string());
    }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_ldp_stp_regression_sp_dest -- --test-threads=1
```
**Raw output:**
```text
Test failed: accepted invalid register forms (llvm-mc rejects): ["SP as Rt1", "SP as Rt2", ...]
minimal failing input: is_load = false, is_64 = false, rt = 0, rt2 = 0, rn = 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_ldp_stp_pbt.rs
