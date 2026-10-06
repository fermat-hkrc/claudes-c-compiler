# Bug: encode_c_li ignores extra operands
**Law:** ∀ rd ∈ GPR, ∀ imm ∈ [-32,31], ∀ extra. encode_c_li([Reg(rd), Imm(imm), extra]) = Err
**Impact:** A third (or later) operand is silently dropped, so `c.li x0, 0, 0` encodes as `c.li x0, 0` instead of being rejected. Callers and handwritten assembly that accidentally pass extra tokens get a valid-looking 16-bit instruction rather than an assembler error. llvm-mc `-triple=riscv64 -mattr=+c` rejects the extra operand (`invalid operand for instruction`).
**Function:** encode_c_li
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/compressed.rs:18
**Detected by:** Negative/Error Contract — extra operand (3)
**Minimal input:** encode_c_li([Reg("x0"), Imm(0), Imm(0)])
**Expected:** Err (C.LI is two-operand `c.li rd, imm`; llvm-mc reports `invalid operand for instruction`)
**Actual:** Ok(Half(0x4001)) — the encoding of `c.li x0, 0`
**Severity:** medium
**Root cause:** compressed.rs:18-23 reads only operands[0] and operands[1] via get_reg/get_imm and never checks operands.len(), so a third token is ignored and the CI-type halfword is still returned.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/compressed.rs:18`
```rust
pub(crate) fn encode_c_li(operands: &[Operand]) -> Result<EncodeResult, String> {
    let rd = get_reg(operands, 0)?;
    let imm = get_imm(operands, 1)? as i32;
    let bit5 = ((imm >> 5) & 1) as u16;
    let bits4_0 = (imm & 0x1F) as u16;
    Ok(EncodeResult::Half(0b01 | (bits4_0 << 2) | ((rd as u16) << 7) | (bit5 << 12) | (0b010 << 13)))
}
```
**Suggested fix:** Reject any operand list whose length is not exactly 2 before packing.
```rust
if operands.len() != 2 {
    return Err(format!("c.li: expected 2 operands, got {}", operands.len()));
}
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_c_li_neg_extra -- --test-threads=1
```
**Raw output:**
```text
proptest: Saving this and future failures in /home/toan/github/claudes-c-compiler/proptest-regressions/backend/riscv/assembler/encoder/encode_c_li_pbt.txt
proptest: If this test was run on a CI system, you may wish to add the following line to your copy of the file. (You may need to create it.)
cc 5723fdcb91ba698431c9caaab531629e955639ac7b68bc9725af8d2eb8b7ba98

thread 'backend::riscv::assembler::encoder::encode_c_li_pbt::encode_c_li_neg_extra' (2842760) panicked at src/backend/riscv/assembler/encoder/encode_c_li_pbt.rs:289:1:
Test failed: extra operand must Err for c.li x0, 0 (llvm-mc rejects extra operands); got Ok(Half(16385)) at src/backend/riscv/assembler/encoder/encode_c_li_pbt.rs:364.
minimal failing input: rd = "x0", imm = 0, extra = Imm(
    0,
)
	successes: 0
	local rejects: 0
	global rejects: 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/encode_c_li_pbt.rs
