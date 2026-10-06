# Bug: encode_c_mv ignores extra operands
**Law:** ∀ rd ∈ GPR, ∀ rs2 ∈ GPR\{x0}, ∀ extra. encode_c_mv([Reg(rd), Reg(rs2), extra]) = Err
**Impact:** A third (or later) operand is silently dropped, so `c.mv x0, x1, 0` encodes as `c.mv x0, x1` (HINT, halfword 0x8006) instead of being rejected. Callers and handwritten assembly that accidentally pass extra tokens get a valid-looking 16-bit instruction rather than an assembler error. llvm-mc `-triple=riscv64 -mattr=+c` rejects the extra operand (`invalid operand for instruction`).
**Function:** encode_c_mv
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/compressed.rs:36
**Detected by:** Negative/Error Contract — extra operand (3)
**Minimal input:** encode_c_mv([Reg("x0"), Reg("x1"), Imm(0)])
**Expected:** Err (C.MV is two-operand `c.mv rd, rs2`; llvm-mc reports `invalid operand for instruction`)
**Actual:** Ok(Half(0x8006)) — the encoding of `c.mv x0, x1` (HINT)
**Severity:** medium
**Root cause:** compressed.rs:36-39 reads only operands[0] and operands[1] via get_reg and never checks operands.len(), so a third token is ignored and the CR-type halfword is still returned.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/compressed.rs:36`
```rust
pub(crate) fn encode_c_mv(operands: &[Operand]) -> Result<EncodeResult, String> {
    let rd = get_reg(operands, 0)?;
    let rs2 = get_reg(operands, 1)?;
    Ok(EncodeResult::Half(0b10 | ((rs2 as u16) << 2) | ((rd as u16) << 7) | (0b100 << 13)))
}
```
**Suggested fix:** Reject any operand list whose length is not exactly 2 before packing.
```rust
if operands.len() != 2 {
    return Err(format!("c.mv: expected 2 operands, got {}", operands.len()));
}
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_c_mv_regression_extra_operand -- --test-threads=1
```
**Raw output:**
```text
proptest: Saving this and future failures in /home/toan/github/claudes-c-compiler/proptest-regressions/backend/riscv/assembler/encoder/encode_c_mv_pbt.txt
proptest: If this test was run on a CI system, you may wish to add the following line to your copy of the file. (You may need to create it.)
cc be4fc2cb2f7296db33b556ba026253c6be8167a6e769d89425b6dc20ec152591

thread 'backend::riscv::assembler::encoder::encode_c_mv_pbt::encode_c_mv_neg_extra' (2846584) panicked at src/backend/riscv/assembler/encoder/encode_c_mv_pbt.rs:267:1:
Test failed: extra operand must Err for c.mv x0, x1 (llvm-mc rejects extra operands); got Ok(Half(32774)) at src/backend/riscv/assembler/encoder/encode_c_mv_pbt.rs:351.
minimal failing input: rd = "x0", rs2 = "x1", extra = Imm(
    0,
)
	successes: 0
	local rejects: 0
	global rejects: 0

thread 'backend::riscv::assembler::encoder::encode_c_mv_pbt::test_encode_c_mv_regression_extra_operand' (2846596) panicked at src/backend/riscv/assembler/encoder/encode_c_mv_pbt.rs:249:5:
c.mv x1, x2 with a third operand must Err; got Ok(Half(32906))
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/encode_c_mv_pbt.rs
