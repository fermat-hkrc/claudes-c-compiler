# Bug: encode_c_add ignores extra operands
**Law:** ∀ rd ∈ GPR, ∀ rs2 ∈ GPR\{x0}, ∀ extra. encode_c_add([Reg(rd), Reg(rs2), extra]) = Err
**Impact:** A third (or later) operand is silently dropped, so `c.add x0, x1, 0` encodes as `c.add x0, x1` (HINT, halfword 0x9006) instead of being rejected. Callers and handwritten assembly that accidentally pass extra tokens get a valid-looking 16-bit instruction rather than an assembler error. llvm-mc `-triple=riscv64 -mattr=+c` rejects the extra operand (`invalid operand for instruction`).
**Function:** encode_c_add
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/compressed.rs:43
**Detected by:** Negative/Error Contract — extra operand (3)
**Minimal input:** encode_c_add([Reg("x0"), Reg("x1"), Imm(0)])
**Expected:** Err (C.ADD is two-operand `c.add rd, rs2`; llvm-mc reports `invalid operand for instruction`)
**Actual:** Ok(Half(0x9006)) — the encoding of `c.add x0, x1` (HINT). Regression encode_c_add([Reg("x1"), Reg("x2"), Imm(0)]) returned Ok(Half(0x908a)).
**Severity:** medium
**Root cause:** compressed.rs:43-47 reads only operands[0] and operands[1] via get_reg and never checks operands.len(), so a third token is ignored and the CR-type halfword is still returned.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/compressed.rs:43`
```rust
pub(crate) fn encode_c_add(operands: &[Operand]) -> Result<EncodeResult, String> {
    let rd = get_reg(operands, 0)?;
    let rs2 = get_reg(operands, 1)?;
    Ok(EncodeResult::Half(0b10 | ((rs2 as u16) << 2) | ((rd as u16) << 7) | (1 << 12) | (0b100 << 13)))
}
```
**Suggested fix:** Reject any operand list whose length is not exactly 2 before packing.
```rust
if operands.len() != 2 {
    return Err(format!("c.add: expected 2 operands, got {}", operands.len()));
}
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_c_add_regression_extra_operand -- --test-threads=1
```
**Raw output:**
```text
proptest: Saving this and future failures in /home/toan/github/claudes-c-compiler/proptest-regressions/backend/riscv/assembler/encoder/encode_c_add_pbt.txt
proptest: If this test was run on a CI system, you may wish to add the following line to your copy of the file. (You may need to create it.)
cc 90bcb15308bb091474f8c16e69b1127e88b3a129476d372024005ea6230c2d9a

thread 'backend::riscv::assembler::encoder::encode_c_add_pbt::encode_c_add_neg_extra' (2850634) panicked at src/backend/riscv/assembler/encoder/encode_c_add_pbt.rs:268:1:
Test failed: extra operand must Err for c.add x0, x1 (llvm-mc rejects extra operands); got Ok(Half(36870)) at src/backend/riscv/assembler/encoder/encode_c_add_pbt.rs:352.
minimal failing input: rd = "x0", rs2 = "x1", extra = Imm(
    0,
)
	successes: 0
	local rejects: 0
	global rejects: 0

thread 'backend::riscv::assembler::encoder::encode_c_add_pbt::test_encode_c_add_regression_extra_operand' (2850644) panicked at src/backend/riscv/assembler/encoder/encode_c_add_pbt.rs:250:5:
c.add x1, x2 with a third operand must Err; got Ok(Half(37002))
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/encode_c_add_pbt.rs
