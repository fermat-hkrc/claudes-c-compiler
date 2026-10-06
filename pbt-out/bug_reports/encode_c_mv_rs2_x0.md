# Bug: encode_c_mv encodes rs2=x0 as C.JR instead of rejecting it
**Law:** ∀ rd ∈ GPR. encode_c_mv([Reg(rd), Reg("x0")]) = Err
**Impact:** `c.mv rd, x0` is not a valid C.MV (RISC-V CR-type with rs2=x0 is C.JR). The encoder silently emits the C.JR halfword: `c.mv x1, x0` becomes `c.jr x1` (0x8082), a control-flow instruction instead of a register copy. Handwritten or generated `c.mv rd, zero` therefore jumps rather than moving zero. llvm-mc `-triple=riscv64 -mattr=+c` rejects the operand (`invalid operand for instruction`). compress.rs:205 requires `rs2 != 0` before emitting C.MV.
**Function:** encode_c_mv
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/compressed.rs:36
**Detected by:** Negative/Error Contract — rs2=x0 (3)
**Minimal input:** encode_c_mv([Reg("x1"), Reg("x0")])
**Expected:** Err (C.MV requires rs2≠x0; llvm-mc reports `invalid operand for instruction`; the rs2=0 encoding is C.JR)
**Actual:** Ok(Half(0x8082)) — the encoding of `c.jr x1`. Shrunk PBT witness encode_c_mv([Reg("x0"), Reg("x0")]) returned Ok(Half(0x8002)) (reserved C.JR x0).
**Severity:** high
**Root cause:** compressed.rs:36-39 reads rd and rs2 via get_reg and packs the CR-type halfword with no rs2≠0 check, so rs2=x0 falls into the C.JR bit pattern (funct4=1000, rs2=0).
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/compressed.rs:36`
```rust
pub(crate) fn encode_c_mv(operands: &[Operand]) -> Result<EncodeResult, String> {
    let rd = get_reg(operands, 0)?;
    let rs2 = get_reg(operands, 1)?;
    Ok(EncodeResult::Half(0b10 | ((rs2 as u16) << 2) | ((rd as u16) << 7) | (0b100 << 13)))
}
```
**Suggested fix:** Reject rs2 == 0 before packing (C.MV requires rs2≠x0; that encoding is C.JR).
```rust
if rs2 == 0 {
    return Err("c.mv: rs2 cannot be x0 (that encoding is c.jr)".into());
}
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_c_mv_regression_rs2_x0 -- --test-threads=1
```
**Raw output:**
```text
thread 'backend::riscv::assembler::encoder::encode_c_mv_pbt::encode_c_mv_neg_rs2_x0' (2846585) panicked at src/backend/riscv/assembler/encoder/encode_c_mv_pbt.rs:267:1:
Test failed: rs2=x0 must Err (llvm-mc rejects; encoding is C.JR); got Ok(Half(32770)) at src/backend/riscv/assembler/encoder/encode_c_mv_pbt.rs:334.
minimal failing input: rd = "x0"
	successes: 0
	local rejects: 0
	global rejects: 0

thread 'backend::riscv::assembler::encoder::encode_c_mv_pbt::test_encode_c_mv_regression_rs2_x0' (2846597) panicked at src/backend/riscv/assembler/encoder/encode_c_mv_pbt.rs:260:5:
c.mv x1, x0 must Err (rs2=x0 is C.JR); got Ok(Half(32898))
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/encode_c_mv_pbt.rs
