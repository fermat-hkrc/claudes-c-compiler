# Bug: encode_fmv_x_f ignores a 3rd operand
**Law:** A third operand after rd, rs1 must return Err. llvm-mc rejects extra operands ("invalid operand for instruction"); encode_instruction passes the full operand slice through to encode_fmv_x_f.
**Impact:** Malformed `fmv.x.w x0, f0, 0` still assembles as `fmv.x.w x0, f0`. A typo or extra token is silently dropped, so the assembler emits a valid OP-FP FMV.X word instead of diagnosing the extra operand.
**Function:** encode_fmv_x_f
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/float.rs:161
**Detected by:** Negative/Error Contract
**Minimal input:** encode_fmv_x_f([Reg("x0"), Reg("f0"), Imm(0)], 0b1110000, 0)
**Expected:** Err
**Actual:** Ok(Word(3758096467)) which is 0xe0000053, the encoding of fmv.x.w x0, f0
**Severity:** medium
**Root cause:** float.rs:163-165 reads only operands 0 and 1 via get_reg/get_freg and returns Ok without checking operands.len() > 2, so a 3rd operand is ignored.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/float.rs:165`
```rust
    Ok(EncodeResult::Word(encode_r(OP_OP_FP, rd, 0b000, rs1, 0, funct7)))
```
**Suggested fix:** Reject more than two operands before packing the R-type word.
```rust
    if operands.len() > 2 {
        return Err("fmv.x: unexpected extra operand".to_string());
    }
    let rd = get_reg(operands, 0)?;
    let rs1 = get_freg(operands, 1)?;
    Ok(EncodeResult::Word(encode_r(OP_OP_FP, rd, 0b000, rs1, 0, funct7)))
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_fmv_x_f -- --test-threads=1
cargo test --lib test_encode_fmv_x_f_regression_extra_operand -- --test-threads=1
```
**Raw output:**
```text
Test failed: 3rd operand must Err for fmv.x.w x0, f0 (llvm-mc rejects extra operands); got Ok(Word(3758096467)) at src/backend/riscv/assembler/encoder/encode_fmv_x_f_pbt.rs:468.
minimal failing input: (mn, f7) = (
    "fmv.x.w",
    112,
), rd = "x0", rs1 = "f0", extra = Imm(
    0,
)
	successes: 0
	local rejects: 0
	global rejects: 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/encode_fmv_x_f_pbt.rs
