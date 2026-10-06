# Bug: encode_c_lui ignores extra operands
**Law:** ∀ rd ∈ GPR\{x0,x2}, ∀ imm ∈ [1,31], ∀ extra. encode_c_lui([Reg(rd), Imm(imm), extra]) = Err
**Impact:** A third (or later) operand is silently dropped, so `c.lui x3, 1, 0` encodes as `c.lui x3, 1` instead of being rejected. Callers and handwritten assembly that accidentally pass extra tokens get a valid-looking 16-bit instruction rather than an assembler error. llvm-mc `-triple=riscv64 -mattr=+c` rejects the extra operand.
**Function:** encode_c_lui
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/compressed.rs:6
**Detected by:** Negative/Error Contract — extra operand (3)
**Minimal input:** encode_c_lui([Reg("x3"), Imm(1), Imm(0)])
**Expected:** Err (C.LUI is two-operand `c.lui rd, nzimm`; llvm-mc reports `invalid operand for instruction`)
**Actual:** Ok(Half(0x6185)) — the encoding of `c.lui x3, 1`
**Severity:** medium
**Root cause:** compressed.rs:6-14 reads only operands[0] and operands[1] via get_reg/get_imm and never checks operands.len(), so a third token is ignored and the CI-type halfword is still returned.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/compressed.rs:6`
```rust
pub(crate) fn encode_c_lui(operands: &[Operand]) -> Result<EncodeResult, String> {
    let rd = get_reg(operands, 0)?;
    if rd == 0 || rd == 2 { return Err("c.lui: rd cannot be x0 or x2".into()); }
    let imm = get_imm(operands, 1)?;
    let nzimm = imm as i32;
    if nzimm == 0 { return Err("c.lui: nzimm must not be zero".into()); }
    let bit17 = ((nzimm >> 5) & 1) as u16;
    let bits16_12 = (nzimm & 0x1F) as u16;
    Ok(EncodeResult::Half(0b01 | ((bits16_12 & 0x1F) << 2) | ((rd as u16) << 7) | (bit17 << 12) | (0b011 << 13)))
}
```
**Suggested fix:** Reject any operand list whose length is not exactly 2 before packing.
```rust
if operands.len() != 2 {
    return Err(format!("c.lui: expected 2 operands, got {}", operands.len()));
}
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_c_lui_neg_extra -- --test-threads=1
```
**Raw output:**
```text
proptest: Saving this and future failures in /home/toan/github/claudes-c-compiler/proptest-regressions/backend/riscv/assembler/encoder/encode_c_lui_pbt.txt
proptest: If this test was run on a CI system, you may wish to add the following line to your copy of the file. (You may need to create it.)
cc d61994242bf0b142134434df88fff935c338d29ed9044b435e1065818557a9d7

thread 'backend::riscv::assembler::encoder::encode_c_lui_pbt::encode_c_lui_neg_extra' (2838212) panicked at src/backend/riscv/assembler/encoder/encode_c_lui_pbt.rs:295:1:
Test failed: extra operand must Err for c.lui x3, 1 (llvm-mc rejects extra operands); got Ok(Half(24965)) at src/backend/riscv/assembler/encoder/encode_c_lui_pbt.rs:382.
minimal failing input: rd = "x3", imm = 1, extra = Imm(
    0,
)
	successes: 0
	local rejects: 0
	global rejects: 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/encode_c_lui_pbt.rs
