# Bug: encode_c_jr encodes rs1=x0 as a reserved halfword instead of rejecting it
**Law:** ∀ name ∈ {x0, zero}. encode_c_jr([Reg(name)]) = Err
**Impact:** `c.jr x0` / `c.jr zero` is not a valid C.JR (RISC-V Unprivileged ISA: C.JR is only valid when rs1≠x0; the code point with rs1=x0 is reserved). The encoder silently emits halfword 0x8002. Handwritten or generated `c.jr zero` therefore becomes a reserved 16-bit encoding rather than an assembler error. llvm-mc `-triple=riscv64 -mattr=+c` rejects the operand (`invalid operand for instruction`). compress.rs:566 requires `rs1 != 0` before emitting C.JR.
**Function:** encode_c_jr
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/compressed.rs:50
**Detected by:** Negative/Error Contract — rs1=x0 (3)
**Minimal input:** encode_c_jr([Reg("x0")])
**Expected:** Err (C.JR requires rs1≠x0; llvm-mc reports `invalid operand for instruction`; the rs1=0 encoding is reserved)
**Actual:** Ok(Half(0x8002)) — reserved CR-type encoding (funct4=1000, rs1=0, rs2=0).
**Severity:** medium
**Root cause:** compressed.rs:50-53 reads rs1 via get_reg and packs the CR-type halfword with no rs1≠0 check, so rs1=x0 falls into the reserved bit pattern (funct4=1000, rs1=0, rs2=0).
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/compressed.rs:50`
```rust
pub(crate) fn encode_c_jr(operands: &[Operand]) -> Result<EncodeResult, String> {
    let rs1 = get_reg(operands, 0)?;
    Ok(EncodeResult::Half(0b10 | ((rs1 as u16) << 7) | (0b100 << 13)))
}
```
**Suggested fix:** Reject rs1 == 0 before packing (C.JR requires rs1≠x0; that encoding is reserved).
```rust
if rs1 == 0 {
    return Err("c.jr: rs1 cannot be x0 (that encoding is reserved)".into());
}
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_c_jr_regression_rs1_x0 -- --test-threads=1
```
**Raw output:**
```text
thread 'backend::riscv::assembler::encoder::encode_c_jr_pbt::encode_c_jr_neg_rs1_x0' (2852478) panicked at src/backend/riscv/assembler/encoder/encode_c_jr_pbt.rs:253:1:
Test failed: rs1=x0 must Err (llvm-mc rejects; encoding is reserved); got Ok(Half(32770)) at src/backend/riscv/assembler/encoder/encode_c_jr_pbt.rs:313.
minimal failing input: name = "x0"
	successes: 0
	local rejects: 0
	global rejects: 0

thread 'backend::riscv::assembler::encoder::encode_c_jr_pbt::test_encode_c_jr_regression_rs1_x0' (2852481) panicked at src/backend/riscv/assembler/encoder/encode_c_jr_pbt.rs:246:5:
c.jr x0 must Err (rs1=x0 is reserved); got Ok(Half(32770))
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/encode_c_jr_pbt.rs
