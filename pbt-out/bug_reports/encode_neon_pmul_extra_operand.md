# Bug: encode_neon_pmul ignores a fourth operand
**Law:** Vector PMUL is a three-operand instruction; a fourth operand must be rejected
**Impact:** Invalid assembly `pmul Vd.T, Vn.T, Vm.T, Vextra.T` is assembled as if the extra register were absent, so a typo or extra operand is silently dropped instead of failing the assemble
**Function:** encode_neon_pmul
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:336
**Detected by:** Negative/Error Contract
**Minimal input:** encode_neon_pmul([v0.8b, v0.8b, v0.8b, v0.8b])
**Expected:** Err (llvm-mc / gas reject a fourth operand)
**Actual:** Ok(Word(0x2e209c00)) — same encoding as `pmul v0.8b, v0.8b, v0.8b`
**Severity:** medium
**Root cause:** neon.rs:336-345 never checks `operands.len()`; get_neon_reg only reads indices 0..2, so extra operands are ignored and the function returns Ok
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:336`
```rust
pub(crate) fn encode_neon_pmul(operands: &[Operand]) -> Result<EncodeResult, String> {
    let (rd, arr_d) = get_neon_reg(operands, 0)?;
    let (rn, _) = get_neon_reg(operands, 1)?;
    let (rm, _) = get_neon_reg(operands, 2)?;
    let q: u32 = if arr_d == "16b" { 1 } else { 0 };
    // PMUL: 0 Q 1 01110 00 1 Rm 10011 1 Rn Rd (size=00 for bytes, U=1)
    // PMUL encoding: size=00 (bytes) is implicit (zero bits at [23:22])
    let word = (q << 30) | (1 << 29) | (0b01110 << 24) | (1 << 21)
        | (rm << 16) | (0b100111 << 10) | (rn << 5) | rd;
    Ok(EncodeResult::Word(word))
}
```
**Suggested fix:** Reject any operand count other than 3
```rust
if operands.len() != 3 {
    return Err("pmul requires 3 operands".to_string());
}
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_pmul_regression_extra_operand -- --test-threads=1
```
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_neon_pmul_pbt::encode_neon_pmul_neg_extra_operand' panicked at src/backend/arm/assembler/encoder/encode_neon_pmul_pbt.rs:282:1:
Test failed: 4 operands must Err (llvm-mc rejects pmul v0.8b, v0.8b, v0.8b, v0.8b) at src/backend/arm/assembler/encoder/encode_neon_pmul_pbt.rs:301.
minimal failing input: rd = 0, rn = 0, rm = 0, extra = 0, t = "8b"
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_neon_pmul_pbt.rs
