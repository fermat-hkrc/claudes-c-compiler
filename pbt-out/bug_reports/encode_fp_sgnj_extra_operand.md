# Bug: encode_fp_sgnj ignores a 4th operand
**Law:** A fourth operand after rd, rs1, rs2 must return Err. llvm-mc rejects extra operands ("invalid operand for instruction"); encode_instruction passes the full operand slice through to encode_fp_sgnj.
**Impact:** Malformed `fsgnj.s f0, f0, f0, 0` still assembles as `fsgnj.s f0, f0, f0`. A typo or extra token is silently dropped, so the assembler emits a valid OP-FP FSGNJ word instead of diagnosing the extra operand.
**Function:** encode_fp_sgnj
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/float.rs:95
**Detected by:** Negative/Error Contract
**Minimal input:** encode_fp_sgnj([Reg("f0"), Reg("f0"), Reg("f0"), Imm(0)], 0b0010000, 0b000)
**Expected:** Err
**Actual:** Ok(Word(536870995)) which is 0x20000053, the encoding of fsgnj.s f0, f0, f0
**Severity:** medium
**Root cause:** float.rs:96-99 reads only operands 0, 1, 2 via get_freg and returns Ok without checking operands.len() > 3, so a 4th operand is ignored.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/float.rs:99`
```rust
    Ok(EncodeResult::Word(encode_r(OP_OP_FP, rd, funct3, rs1, rs2, funct7)))
```
**Suggested fix:** Reject more than three operands before packing the R-type word.
```rust
    if operands.len() > 3 {
        return Err("fp sgnj: unexpected extra operand".to_string());
    }
    let rd = get_freg(operands, 0)?;
    let rs1 = get_freg(operands, 1)?;
    let rs2 = get_freg(operands, 2)?;
    Ok(EncodeResult::Word(encode_r(OP_OP_FP, rd, funct3, rs1, rs2, funct7)))
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_fp_sgnj -- --test-threads=1
cargo test --lib test_encode_fp_sgnj_regression_extra_operand -- --test-threads=1
```
**Raw output:**
```text
Test failed: 4th operand must Err for fsgnj.s f0, f0, f0 (llvm-mc rejects extra operands); got Ok(Word(536870995)) at src/backend/riscv/assembler/encoder/encode_fp_sgnj_pbt.rs:468.
minimal failing input: (mn, f7, f3) = (
    "fsgnj.s",
    16,
    0,
), rd = "f0", rs1 = "f0", rs2 = "f0", extra = Imm(
    0,
)
	successes: 0
	local rejects: 0
	global rejects: 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/encode_fp_sgnj_pbt.rs
