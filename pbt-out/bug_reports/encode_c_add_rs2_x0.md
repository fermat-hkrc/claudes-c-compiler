# Bug: encode_c_add encodes rs2=x0 as C.JALR / C.EBREAK instead of rejecting it
**Law:** ∀ rd ∈ GPR. encode_c_add([Reg(rd), Reg("x0")]) = Err
**Impact:** `c.add rd, x0` is not a valid C.ADD (RISC-V CR-type with funct4=1001 and rs2=x0 is C.JALR when rd≠0, C.EBREAK when rd=0). The encoder silently emits that halfword: `c.add x1, x0` becomes `c.jalr x1` (0x9082), a control-flow instruction instead of a register add; `c.add x0, x0` becomes `c.ebreak` (0x9002). Handwritten or generated `c.add rd, zero` therefore jumps or traps rather than adding zero. llvm-mc `-triple=riscv64 -mattr=+c` rejects the operand (`invalid operand for instruction`). compress.rs:200 requires `rs2 != 0` before emitting C.ADD.
**Function:** encode_c_add
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/compressed.rs:43
**Detected by:** Negative/Error Contract — rs2=x0 (3)
**Minimal input:** encode_c_add([Reg("x1"), Reg("x0")])
**Expected:** Err (C.ADD requires rs2≠x0; llvm-mc reports `invalid operand for instruction`; the rs2=0 encoding is C.JALR / C.EBREAK)
**Actual:** Ok(Half(0x9082)) — the encoding of `c.jalr x1`. Shrunk PBT witness encode_c_add([Reg("x0"), Reg("x0")]) returned Ok(Half(0x9002)) (C.EBREAK).
**Severity:** high
**Root cause:** compressed.rs:43-47 reads rd and rs2 via get_reg and packs the CR-type halfword with no rs2≠0 check, so rs2=x0 falls into the C.JALR / C.EBREAK bit pattern (funct4=1001, rs2=0).
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/compressed.rs:43`
```rust
pub(crate) fn encode_c_add(operands: &[Operand]) -> Result<EncodeResult, String> {
    let rd = get_reg(operands, 0)?;
    let rs2 = get_reg(operands, 1)?;
    Ok(EncodeResult::Half(0b10 | ((rs2 as u16) << 2) | ((rd as u16) << 7) | (1 << 12) | (0b100 << 13)))
}
```
**Suggested fix:** Reject rs2 == 0 before packing (C.ADD requires rs2≠x0; that encoding is C.JALR or C.EBREAK).
```rust
if rs2 == 0 {
    return Err("c.add: rs2 cannot be x0 (that encoding is c.jalr / c.ebreak)".into());
}
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_c_add_regression_rs2_x0 -- --test-threads=1
```
**Raw output:**
```text
thread 'backend::riscv::assembler::encoder::encode_c_add_pbt::encode_c_add_neg_rs2_x0' (2850635) panicked at src/backend/riscv/assembler/encoder/encode_c_add_pbt.rs:268:1:
Test failed: rs2=x0 must Err (llvm-mc rejects; encoding is C.JALR/C.EBREAK); got Ok(Half(36866)) at src/backend/riscv/assembler/encoder/encode_c_add_pbt.rs:335.
minimal failing input: rd = "x0"
	successes: 0
	local rejects: 0
	global rejects: 0

thread 'backend::riscv::assembler::encoder::encode_c_add_pbt::test_encode_c_add_regression_rs2_x0' (2850645) panicked at src/backend/riscv/assembler/encoder/encode_c_add_pbt.rs:261:5:
c.add x1, x0 must Err (rs2=x0 is C.JALR); got Ok(Half(36994))
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/encode_c_add_pbt.rs
