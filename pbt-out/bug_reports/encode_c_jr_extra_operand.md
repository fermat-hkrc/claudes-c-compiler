# Bug: encode_c_jr ignores extra operands
**Law:** ∀ rs1 ∈ GPR\{x0}, ∀ extra. encode_c_jr([Reg(rs1), extra]) = Err
**Impact:** A second (or later) operand is silently dropped, so `c.jr x1, 0` encodes as `c.jr x1` (halfword 0x8082) instead of being rejected. Callers and handwritten assembly that accidentally pass extra tokens get a valid-looking 16-bit jump rather than an assembler error. llvm-mc `-triple=riscv64 -mattr=+c` rejects the extra operand (`invalid operand for instruction`).
**Function:** encode_c_jr
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/compressed.rs:50
**Detected by:** Negative/Error Contract — extra operand (3)
**Minimal input:** encode_c_jr([Reg("x1"), Imm(0)])
**Expected:** Err (C.JR is one-operand `c.jr rs1`; llvm-mc reports `invalid operand for instruction`)
**Actual:** Ok(Half(0x8082)) — the encoding of `c.jr x1` / `ret`.
**Severity:** medium
**Root cause:** compressed.rs:50-53 reads only operands[0] via get_reg and never checks operands.len(), so a second token is ignored and the CR-type halfword is still returned.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/compressed.rs:50`
```rust
pub(crate) fn encode_c_jr(operands: &[Operand]) -> Result<EncodeResult, String> {
    let rs1 = get_reg(operands, 0)?;
    Ok(EncodeResult::Half(0b10 | ((rs1 as u16) << 7) | (0b100 << 13)))
}
```
**Suggested fix:** Reject any operand list whose length is not exactly 1 before packing.
```rust
if operands.len() != 1 {
    return Err(format!("c.jr: expected 1 operand, got {}", operands.len()));
}
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_c_jr_regression_extra_operand -- --test-threads=1
```
**Raw output:**
```text
proptest: Saving this and future failures in /home/toan/github/claudes-c-compiler/proptest-regressions/backend/riscv/assembler/encoder/encode_c_jr_pbt.txt
proptest: If this test was run on a CI system, you may wish to add the following line to your copy of the file. (You may need to create it.)
cc 76196dcd79a75097626af2f2f10d1b194c259b99dee69d29bb8415247c6fc7ab

thread 'backend::riscv::assembler::encoder::encode_c_jr_pbt::encode_c_jr_neg_extra' (2852477) panicked at src/backend/riscv/assembler/encoder/encode_c_jr_pbt.rs:253:1:
Test failed: extra operand must Err for c.jr x1 (llvm-mc rejects extra operands); got Ok(Half(32898)) at src/backend/riscv/assembler/encoder/encode_c_jr_pbt.rs:325.
minimal failing input: rs1 = "x1", extra = Imm(
    0,
)
	successes: 0
	local rejects: 0
	global rejects: 0

thread 'backend::riscv::assembler::encoder::encode_c_jr_pbt::test_encode_c_jr_regression_extra_operand' (2852480) panicked at src/backend/riscv/assembler/encoder/encode_c_jr_pbt.rs:235:5:
c.jr x1 with a second operand must Err; got Ok(Half(32898))
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/encode_c_jr_pbt.rs
