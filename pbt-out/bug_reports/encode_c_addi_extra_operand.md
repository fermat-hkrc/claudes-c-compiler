# Bug: encode_c_addi ignores extra operands
**Law:** ∀ rd ∈ GPR, ∀ imm ∈ [-32,31], ∀ extra. encode_c_addi([Reg(rd), Imm(imm), extra]) = Err
**Impact:** A third (or later) operand is silently dropped, so `c.addi x0, 0, 0` encodes as `c.addi x0, 0` (C.NOP, halfword 0x0001) instead of being rejected. Callers and handwritten assembly that accidentally pass extra tokens get a valid-looking 16-bit instruction rather than an assembler error. llvm-mc `-triple=riscv64 -mattr=+c` rejects the extra operand (`invalid operand for instruction`).
**Function:** encode_c_addi
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/compressed.rs:27
**Detected by:** Negative/Error Contract — extra operand (3)
**Minimal input:** encode_c_addi([Reg("x0"), Imm(0), Imm(0)])
**Expected:** Err (C.ADDI is two-operand `c.addi rd, nzimm`; llvm-mc reports `invalid operand for instruction`)
**Actual:** Ok(Half(0x0001)) — the encoding of `c.addi x0, 0` / C.NOP
**Severity:** medium
**Root cause:** compressed.rs:27-32 reads only operands[0] and operands[1] via get_reg/get_imm and never checks operands.len(), so a third token is ignored and the CI-type halfword is still returned.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/compressed.rs:27`
```rust
pub(crate) fn encode_c_addi(operands: &[Operand]) -> Result<EncodeResult, String> {
    let rd = get_reg(operands, 0)?;
    let imm = get_imm(operands, 1)? as i32;
    let bit5 = ((imm >> 5) & 1) as u16;
    let bits4_0 = (imm & 0x1F) as u16;
    Ok(EncodeResult::Half(0b01 | (bits4_0 << 2) | ((rd as u16) << 7) | (bit5 << 12)))
}
```
**Suggested fix:** Reject any operand list whose length is not exactly 2 before packing.
```rust
if operands.len() != 2 {
    return Err(format!("c.addi: expected 2 operands, got {}", operands.len()));
}
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_c_addi_neg_extra -- --test-threads=1
```
**Raw output:**
```text
proptest: Saving this and future failures in /home/toan/github/claudes-c-compiler/proptest-regressions/backend/riscv/assembler/encoder/encode_c_addi_pbt.txt
proptest: If this test was run on a CI system, you may wish to add the following line to your copy of the file. (You may need to create it.)
cc f827f82f4e423a5506f15f1ade1e05e203f90ab1266a22c346f843ab35587923

thread 'backend::riscv::assembler::encoder::encode_c_addi_pbt::encode_c_addi_neg_extra' (2844740) panicked at src/backend/riscv/assembler/encoder/encode_c_addi_pbt.rs:289:1:
Test failed: extra operand must Err for c.addi x0, 0 (llvm-mc rejects extra operands); got Ok(Half(1)) at src/backend/riscv/assembler/encoder/encode_c_addi_pbt.rs:364.
minimal failing input: rd = "x0", imm = 0, extra = Imm(
    0,
)
	successes: 0
	local rejects: 0
	global rejects: 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/encode_c_addi_pbt.rs
