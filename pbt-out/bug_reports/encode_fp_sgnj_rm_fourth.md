# Bug: encode_fp_sgnj ignores a 4th RoundingMode
**Law:** FSGNJ/FSGNJN/FSGNJX/FMIN/FMAX have no rounding-mode field (ISA uses funct3 as the operation). A 4th RoundingMode operand must return Err. llvm-mc rejects `fsgnj.s fa0, fa1, fa2, rne` ("invalid operand for instruction"); encode_instruction passes the full operand slice through to encode_fp_sgnj.
**Impact:** Malformed `fsgnj.s f0, f0, f0, rne` still assembles as `fsgnj.s f0, f0, f0`. Unlike FADD (which consumes rm in funct3), this family must not accept rm; silently dropping it hides a real assembly error and can confuse a caller who copied an FADD-style 4-operand form.
**Function:** encode_fp_sgnj
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/float.rs:95
**Detected by:** Negative/Error Contract
**Minimal input:** encode_fp_sgnj([Reg("f0"), Reg("f0"), Reg("f0"), RoundingMode("rne")], 0b0010000, 0b000)
**Expected:** Err
**Actual:** Ok(Word(536870995)) which is 0x20000053, the encoding of fsgnj.s f0, f0, f0 (funct3 remains 000, not rne)
**Severity:** medium
**Root cause:** float.rs:96-99 reads only operands 0, 1, 2 via get_freg and returns Ok without checking operands.len() > 3, so a 4th RoundingMode is ignored and does not overwrite funct3.
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
cargo test --lib test_encode_fp_sgnj_regression_rm_fourth -- --test-threads=1
```
**Raw output:**
```text
Test failed: 4th RoundingMode must Err for fsgnj.s f0, f0, f0, rne (FSGNJ/FMIN have no rm); got Ok(Word(536870995)) at src/backend/riscv/assembler/encoder/encode_fp_sgnj_pbt.rs:487.
minimal failing input: (mn, f7, f3) = (
    "fsgnj.s",
    16,
    0,
), rd = "f0", rs1 = "f0", rs2 = "f0", rm = "rne"
	successes: 0
	local rejects: 0
	global rejects: 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/encode_fp_sgnj_pbt.rs
