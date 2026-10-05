# Bug: encode_neon_logical ignores mismatched source arrangements
**Law:** AND/ORR/EOR Vd.T, Vn.T, Vm.T require the same arrangement T on all three operands
**Impact:** `and v0.8b, v0.8b, v0.16b` assembles as 64-bit AND (Q from dest only), so a mixed-width typo is not diagnosed and the object file contains a different instruction than the source text
**Function:** encode_neon_logical
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:297
**Detected by:** Negative/Error Contract
**Minimal input:** encode_neon_logical([v0.8b, v0.8b, v0.16b], opc=0)
**Expected:** Err (llvm-mc/gas: operand mismatch)
**Actual:** Ok(EncodeResult::Word) of `and v0.8b, v0.8b, v0.8b` (Q=0 from dest)
**Severity:** medium
**Root cause:** neon.rs:299-300 bind source arrangements as `_arr_n` / `_arr_m` and never compare them to dest T; Q is taken only from arr_d
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:299`
```rust
    let (rn, _arr_n) = get_neon_reg(operands, 1)?;
```
**Suggested fix:** Require dest, Rn, and Rm arrangements to match
```rust
    let (rn, arr_n) = get_neon_reg(operands, 1)?;
    let (rm, arr_m) = get_neon_reg(operands, 2)?;
    if arr_d != arr_n || arr_n != arr_m {
        return Err(format!("NEON logical arrangement mismatch: .{arr_d}, .{arr_n}, .{arr_m}"));
    }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_logical_regression_mismatch_t -- --test-threads=1
```
**Raw output:**
```text
Test failed: mismatched T must Err (llvm-mc rejects and v0.8b, v0.8b, v0.16b) at src/backend/arm/assembler/encoder/encode_neon_logical_pbt.rs:399.
minimal failing input: rd = 0, rn = 0, rm = 0, td = "8b", tn = "8b", tm = "16b", opc = 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_neon_logical_pbt.rs
