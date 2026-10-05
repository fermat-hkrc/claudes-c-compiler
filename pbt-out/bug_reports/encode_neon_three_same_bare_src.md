# Bug: encode_neon_three_same accepts a bare Vn/Vm without arrangement
**Law:** GNU/llvm-mc three-same operands are `Vd.T, Vn.T, Vm.T`. A bare register (`v0` without `.8b`/`.16b`/…) is not a valid operand.
**Impact:** `cmeq v0.8b, v0, v0.8b` encodes using the destination arrangement for Q/size and the bare register's number for Rn, so invalid assembly is assembled.
**Function:** encode_neon_three_same
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:65
**Detected by:** Negative/Error Contract (4e)
**Minimal input:** encode_neon_three_same([v0.8b, Reg("v0"), v0.8b], u=1, opcode=0b10001)
**Expected:** Err (llvm-mc: "invalid operand for instruction")
**Actual:** Ok(Word) — get_neon_reg on Operand::Reg returns (num, "") and the empty arrangement is discarded
**Severity:** medium
**Root cause:** neon.rs:70-71 call get_neon_reg, which accepts Operand::Reg, then ignore the empty source arrangement.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:70`
```rust
    let (rn, _arr_n) = get_neon_reg(operands, 1)?;
    let (rm, _arr_m) = get_neon_reg(operands, 2)?;
```
**Suggested fix:** Require Operand::RegArrangement on all three slots (or reject an empty source arrangement).
```rust
    let (rn, arr_n) = get_neon_reg(operands, 1)?;
    let (rm, arr_m) = get_neon_reg(operands, 2)?;
    if arr_n.is_empty() || arr_m.is_empty() {
        return Err("NEON three-same requires Vn.T and Vm.T".to_string());
    }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_three_same_regression_bare_src -- --test-threads=1
```
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_neon_three_same_pbt::test_encode_neon_three_same_regression_bare_src' panicked at src/backend/arm/assembler/encoder/encode_neon_three_same_pbt.rs:519:5:
cmeq v0.8b, v0, v0.8b must Err (gas/llvm-mc require Vn.T)
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_neon_three_same_pbt.rs
