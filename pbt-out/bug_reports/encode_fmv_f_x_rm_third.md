# Bug: encode_fmv_f_x ignores a 3rd RoundingMode
**Law:** FMV.W.X/D.X has no rounding-mode field (ISA hardwires funct3=000 and rs2=00000). A 3rd RoundingMode operand must return Err. llvm-mc rejects `fmv.w.x fa0, a1, rne` ("invalid operand for instruction"); encode_instruction passes the full operand slice through to encode_fmv_f_x.
**Impact:** Malformed `fmv.w.x f0, x0, rne` still assembles as `fmv.w.x f0, x0`. Unlike FCVT (which consumes rm in funct3), FMV.W.X must not accept rm; silently dropping it hides a real assembly error and can confuse a caller who copied an FCVT-style 3-operand form.
**Function:** encode_fmv_f_x
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/float.rs:168
**Detected by:** Negative/Error Contract
**Minimal input:** encode_fmv_f_x([Reg("f0"), Reg("x0"), RoundingMode("rne")], 0b1111000, 0)
**Expected:** Err
**Actual:** Ok(Word(4026531923)) which is 0xf0000053, the encoding of fmv.w.x f0, x0 (funct3 remains 000, not rne)
**Severity:** medium
**Root cause:** float.rs:170-172 reads only operands 0 and 1 via get_freg/get_reg and returns Ok without checking operands.len() > 2, so a 3rd RoundingMode is ignored and does not overwrite the hardwired funct3=000.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/float.rs:172`
```rust
    Ok(EncodeResult::Word(encode_r(OP_OP_FP, rd, 0b000, rs1, 0, funct7)))
```
**Suggested fix:** Reject more than two operands before packing the R-type word.
```rust
    if operands.len() > 2 {
        return Err("fmv.w.x/d.x: unexpected extra operand".to_string());
    }
    let rd = get_freg(operands, 0)?;
    let rs1 = get_reg(operands, 1)?;
    Ok(EncodeResult::Word(encode_r(OP_OP_FP, rd, 0b000, rs1, 0, funct7)))
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_fmv_f_x -- --test-threads=1
cargo test --lib test_encode_fmv_f_x_regression_rm_third -- --test-threads=1
```
**Raw output:**
```text
Test failed: 3rd RoundingMode must Err for fmv.w.x f0, x0, rne (FMV.W.X has no rm); got Ok(Word(4026531923)) at src/backend/riscv/assembler/encoder/encode_fmv_f_x_pbt.rs:486.
minimal failing input: (mn, f7) = (
    "fmv.w.x",
    120,
), rd = "f0", rs1 = "x0", rm = "rne"
	successes: 0
	local rejects: 0
	global rejects: 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/encode_fmv_f_x_pbt.rs
