# Bug: encode_fclass ignores a 3rd operand
**Law:** A third operand after rd, rs1 must return Err. llvm-mc rejects extra operands ("invalid operand for instruction"); encode_instruction passes the full operand slice through to encode_fclass.
**Impact:** Malformed `fclass.s x0, f0, 0` still assembles as `fclass.s x0, f0`. A typo or extra token is silently dropped, so the assembler emits a valid OP-FP FCLASS word instead of diagnosing the extra operand.
**Function:** encode_fclass
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/float.rs:110
**Detected by:** Negative/Error Contract
**Minimal input:** encode_fclass([Reg("x0"), Reg("f0"), Imm(0)], 0b1110000)
**Expected:** Err
**Actual:** Ok(Word(3758100563)) which is 0xe0001053, the encoding of fclass.s x0, f0
**Severity:** medium
**Root cause:** float.rs:111-113 reads only operands 0 and 1 via get_reg/get_freg and returns Ok without checking operands.len() > 2, so a 3rd operand is ignored.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/float.rs:113`
```rust
    Ok(EncodeResult::Word(encode_r(OP_OP_FP, rd, 0b001, rs1, 0, funct7)))
```
**Suggested fix:** Reject more than two operands before packing the R-type word.
```rust
    if operands.len() > 2 {
        return Err("fclass: unexpected extra operand".to_string());
    }
    let rd = get_reg(operands, 0)?;
    let rs1 = get_freg(operands, 1)?;
    Ok(EncodeResult::Word(encode_r(OP_OP_FP, rd, 0b001, rs1, 0, funct7)))
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_fclass -- --test-threads=1
cargo test --lib test_encode_fclass_regression_extra_operand -- --test-threads=1
```
**Raw output:**
```text
Test failed: 3rd operand must Err for fclass.s x0, f0 (llvm-mc rejects extra operands); got Ok(Word(3758100563)) at src/backend/riscv/assembler/encoder/encode_fclass_pbt.rs:439.
minimal failing input: (mn, f7) = (
    "fclass.s",
    112,
), rd = "x0", rs1 = "f0", extra = Imm(
    0,
)
	successes: 0
	local rejects: 0
	global rejects: 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/encode_fclass_pbt.rs
