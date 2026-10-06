# Bug: encode_ldp_stp encodes LDP with Rt1 == Rt2
**Law:** LDP with Rt1 == Rt2 is ARM UNPREDICTABLE; llvm-mc rejects it (`unpredictable LDP instruction, Rt2==Rt`). STP with equal registers is allowed
**Impact:** `ldp x0, x0, [x1]` is encoded instead of rejected, producing an unpredictable pair load into one register
**Function:** encode_ldp_stp
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/load_store.rs:452
**Detected by:** Negative/Error Contract
**Minimal input:** encode_ldp_stp([Reg("x0"), Reg("x0"), Mem{base:"x1", offset:0}], is_load=true)
**Expected:** Err (llvm-mc: unpredictable LDP instruction, Rt2==Rt)
**Actual:** Ok(Word(0xA9400420)) — LDP X0, X0, [X1]
**Severity:** medium
**Root cause:** load_store.rs:457-458 reads rt1 and rt2 independently and never requires rt1 != rt2 when is_load is true
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/load_store.rs:457`
```rust
    let (rt1, is_64) = get_reg(operands, 0)?;
    let (rt2, _) = get_reg(operands, 1)?;
```
**Suggested fix:** Reject overlapping load destinations
```rust
    if is_load && rt1 == rt2 {
        return Err("ldp Rt1 and Rt2 must differ".to_string());
    }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_ldp_stp_regression_ldp_same_rt -- --test-threads=1
```
**Raw output:**
```text
LDP X0, X0, [X1] must Err; llvm-mc: unpredictable LDP instruction, Rt2==Rt
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_ldp_stp_pbt.rs
