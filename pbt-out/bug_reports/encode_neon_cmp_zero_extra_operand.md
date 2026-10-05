# Bug: encode_neon_cmp_zero ignores a fourth operand
**Law:** ∀ rd,rn,extra ∈ {0..31}, T ∈ {8b,16b,4h,8h,2s,4s,2d}, (U,opc,mnem) ∈ cmp_zero_table. llvm-mc(mnem Vd.T, Vn.T, #0, Vextra.T) = Err ∧ encode_neon_cmp_zero([Vd.T, Vn.T, Imm(0), Vextra.T], U, opc) = Err
**Impact:** The assembler emits a compare-to-zero encoding for assembly gas and llvm-mc reject, so a typo or extra operand is silently assembled instead of diagnosed.
**Function:** encode_neon_cmp_zero
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:189
**Detected by:** Negative/Error Contract
**Minimal input:** encode_neon_cmp_zero([v0.8b, v0.8b, Imm(0), v0.8b], u=0, opcode=0b01001)  // cmeq v0.8b, v0.8b, #0, v0.8b
**Expected:** Err
**Actual:** Ok(Word) — same encoding as `cmeq v0.8b, v0.8b, #0`
**Severity:** medium
**Root cause:** neon.rs:190 only rejects `operands.len() < 2`; operands at index 2 and beyond are never inspected, so a fourth operand is ignored.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:190`
```rust
    if operands.len() < 2 {
        return Err("NEON compare-zero requires at least 2 operands".to_string());
    }
    let (rd, arr_d) = get_neon_reg(operands, 0)?;
    let (rn, _) = get_neon_reg(operands, 1)?;
```
**Suggested fix:** Reject more than two NEON registers; if a third operand is present it must be Imm(0), and a fourth operand must Err.
```rust
    if operands.len() < 2 {
        return Err("NEON compare-zero requires at least 2 operands".to_string());
    }
    if operands.len() > 3 {
        return Err("NEON compare-zero: extra operand".to_string());
    }
    if operands.len() == 3 && !matches!(operands.get(2), Some(Operand::Imm(0))) {
        return Err("NEON compare-zero: expected #0".to_string());
    }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_neon_cmp_zero_neg_extra -- --test-threads=1
```
**Raw output:**
```text
Test failed: extra operand must Err (llvm-mc rejects cmeq v0.8b, v0.8b, #0, v0.8b) at src/backend/arm/assembler/encoder/encode_neon_cmp_zero_pbt.rs:302.
minimal failing input: rd = 0, rn = 0, extra = 0, t = "8b", insn = (
    0,
    9,
    "cmeq",
)
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_neon_cmp_zero_pbt.rs
