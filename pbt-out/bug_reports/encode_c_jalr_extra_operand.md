# Bug: encode_c_jalr ignores extra operands
**Law:** ∀ rs1 ∈ GPR\{x0}, ∀ extra. encode_c_jalr([Reg(rs1), extra]) = Err
**Impact:** A second (or later) operand is silently dropped, so `c.jalr x1, 0` encodes as `c.jalr x1` (halfword 0x9082) instead of being rejected. Callers and handwritten assembly that accidentally pass extra tokens get a valid-looking 16-bit linked jump rather than an assembler error. llvm-mc `-triple=riscv64 -mattr=+c` rejects the extra operand (`invalid operand for instruction`).
**Function:** encode_c_jalr
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/compressed.rs:56
**Detected by:** Negative/Error Contract — extra operand (3)
**Minimal input:** encode_c_jalr([Reg("x1"), Imm(0)])
**Expected:** Err (C.JALR is one-operand `c.jalr rs1`; llvm-mc reports `invalid operand for instruction`)
**Actual:** Ok(Half(0x9082)) — the encoding of `c.jalr x1` / `jalr ra`.
**Severity:** medium
**Root cause:** compressed.rs:56-59 reads only operands[0] via get_reg and never checks operands.len(), so a second token is ignored and the CR-type halfword is still returned.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/compressed.rs:57`
```rust
    let rs1 = get_reg(operands, 0)?;
    Ok(EncodeResult::Half(0b10 | ((rs1 as u16) << 7) | (1 << 12) | (0b100 << 13)))
```
**Suggested fix:** Reject any operand list whose length is not exactly 1 before packing.
```rust
if operands.len() != 1 {
    return Err(format!("c.jalr: expected 1 operand, got {}", operands.len()));
}
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_c_jalr_regression_extra_operand -- --test-threads=1
```
**Raw output:**
```text
proptest: Saving this and future failures in /home/toan/github/claudes-c-compiler/proptest-regressions/backend/riscv/assembler/encoder/encode_c_jalr_pbt.txt
proptest: If this test was run on a CI system, you may wish to add the following line to your copy of the file. (You may need to create it.)
cc e3d16e17f40f453d0c80bfd3ae37b5bcb03515a09b54d73108935401b3ef5c12

thread 'backend::riscv::assembler::encoder::encode_c_jalr_pbt::encode_c_jalr_neg_extra' (2854338) panicked at src/backend/riscv/assembler/encoder/encode_c_jalr_pbt.rs:253:1:
Test failed: extra operand must Err for c.jalr x1 (llvm-mc rejects extra operands); got Ok(Half(36994)) at src/backend/riscv/assembler/encoder/encode_c_jalr_pbt.rs:325.
minimal failing input: rs1 = "x1", extra = Imm(
    0,
)
	successes: 0
	local rejects: 0
	global rejects: 0

thread 'backend::riscv::assembler::encoder::encode_c_jalr_pbt::test_encode_c_jalr_regression_extra_operand' (2854342) panicked at src/backend/riscv/assembler/encoder/encode_c_jalr_pbt.rs:235:5:
c.jalr x1 with a second operand must Err; got Ok(Half(36994))
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/encode_c_jalr_pbt.rs
