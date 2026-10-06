# Bug: encode_fp_cmp ignores a 4th operand
**Law:** A fourth operand after rd, rs1, rs2 must return Err. llvm-mc rejects extra operands ("invalid operand for instruction"); encode_instruction passes the full operand slice through to encode_fp_cmp.
**Impact:** Malformed `feq.s x0, f0, f0, 0` still assembles as `feq.s x0, f0, f0`. A typo or extra token is silently dropped, so the assembler emits a valid OP-FP compare word instead of diagnosing the extra operand.
**Function:** encode_fp_cmp
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/float.rs:102
**Detected by:** Negative/Error Contract
**Minimal input:** encode_fp_cmp([Reg("x0"), Reg("f0"), Reg("f0"), Imm(0)], 0b1010000, 0b010)
**Expected:** Err
**Actual:** Ok(Word(2684362835)) which is 0xa0002053, the encoding of feq.s x0, f0, f0
**Severity:** medium
**Root cause:** float.rs:104-107 reads only operands 0, 1, 2 via get_reg/get_freg and returns Ok without checking operands.len() > 3, so a 4th operand is ignored.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/float.rs:107`
```rust
    Ok(EncodeResult::Word(encode_r(OP_OP_FP, rd, funct3, rs1, rs2, funct7)))
```
**Suggested fix:** Reject more than three operands before packing the R-type word.
```rust
    if operands.len() > 3 {
        return Err("fp cmp: unexpected extra operand".to_string());
    }
    let rd = get_reg(operands, 0)?;
    let rs1 = get_freg(operands, 1)?;
    let rs2 = get_freg(operands, 2)?;
    Ok(EncodeResult::Word(encode_r(OP_OP_FP, rd, funct3, rs1, rs2, funct7)))
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_fp_cmp -- --test-threads=1
cargo test --lib test_encode_fp_cmp_regression_extra_operand -- --test-threads=1
```
**Raw output:**
```text
Test failed: 4th operand must Err for feq.s x0, f0, f0 (llvm-mc rejects extra operands); got Ok(Word(2684362835)) at src/backend/riscv/assembler/encoder/encode_fp_cmp_pbt.rs:469.
minimal failing input: (mn, f7, f3) = (
    "feq.s",
    80,
    2,
), rd = "x0", rs1 = "f0", rs2 = "f0", extra = Imm(
    0,
)
	successes: 0
	local rejects: 0
	global rejects: 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/encode_fp_cmp_pbt.rs
