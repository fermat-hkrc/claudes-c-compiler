# Bug: encode_ldp_stp accepts invalid register forms that llvm-mc rejects
**Law:** LDP/STP Rt is Wt/Xt (31=WZR/XZR, never SP) or SIMD S/D/Q; Rn is Xn|SP (not XZR, not W); Rt1 and Rt2 must match width; LDP requires Rt1≠Rt2; writeback requires Rn∉{Rt1,Rt2} unless Rn=SP
**Impact:** SP dest encodes as XZR, XZR/x31 base encodes as SP, W base encodes as Xn, mixed X/W uses Rt1 width, writeback overlap and LDP Rt1==Rt2 produce unpredictable encodings
**Function:** encode_ldp_stp
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/load_store.rs:452
**Detected by:** Negative/Error Contract
**Minimal input:** encode_ldp_stp([Reg("sp"), Reg("w0"), Mem{base:"x0", offset:0}], is_load=false) — also XZR/x31/W base, mixed X/W, writeback Rn==Rt1, LDP Rt1==Rt2
**Expected:** Err (llvm-mc: invalid operand / unpredictable LDP/STP)
**Actual:** Ok(Word) for each of those forms
**Severity:** medium
**Root cause:** load_store.rs:457-458 get_reg / parse_reg_num maps SP and XZR both to 31, discards Rt2 width, and the pre/post arms never check writeback overlap or LDP Rt1==Rt2
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/load_store.rs:457`
```rust
    let (rt1, is_64) = get_reg(operands, 0)?;
    let (rt2, _) = get_reg(operands, 1)?;
```
**Suggested fix:** Reject SP as Rt, XZR/W as base, mixed widths, LDP Rt1==Rt2, and writeback overlap
```rust
    if is_load && rt1 == rt2 {
        return Err("ldp Rt1 and Rt2 must differ".to_string());
    }
    // also reject SP dest, XZR/W base, mixed width, writeback overlap
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_ldp_stp_regression_sp_dest -- --test-threads=1
```
**Raw output:**
```text
Test failed: accepted invalid register forms (llvm-mc rejects): ["SP as Rt1", "SP as Rt2", "XZR base", "x31 base", "W base", "mixed X/W pair", "writeback Rn==Rt1"]
minimal failing input: is_load = false, is_64 = false, rt = 0, rt2 = 0, rn = 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_ldp_stp_pbt.rs
