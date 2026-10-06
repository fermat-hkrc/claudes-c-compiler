# Bug: encode_alu_imm_w wraps out-of-range immediates
**Law:** An OP-IMM-32 addiw immediate must be a signed 12-bit integer in [-2048, 2047]; values outside that range must return Err, matching llvm-mc and the I-type imm[11:0] layout.
**Impact:** addiw x0, x0, 2048 is encoded as addiw x0, x0, -2048 (the 12-bit wrap), so a too-large immediate silently becomes a different instruction.
**Function:** encode_alu_imm_w
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/base.rs:267
**Detected by:** Negative/Error Contract
**Minimal input:** encode_alu_imm_w([Reg("x0"), Reg("x0"), Imm(2048)], funct3=0)  // addiw x0, x0, 2048
**Expected:** Err
**Actual:** Ok(Word(0x8000001b))  // addiw x0, x0, -2048
**Severity:** high
**Root cause:** base.rs:270 casts get_imm to i32 with no range check; encode_i then masks with 0xFFF, wrapping 2048 to the signed-12 encoding of -2048.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/base.rs:270`
```rust
    let imm = get_imm(operands, 2)? as i32;
    Ok(EncodeResult::Word(encode_i(OP_OP_IMM_32, rd, funct3, rs1, imm)))
```
**Suggested fix:** Reject immediates outside the signed 12-bit range before packing.
```rust
    let imm = get_imm(operands, 2)?;
    if !(-2048..=2047).contains(&imm) {
        return Err(format!("alu_imm_w: immediate {imm} out of [-2048, 2047]"));
    }
    Ok(EncodeResult::Word(encode_i(OP_OP_IMM_32, rd, funct3, rs1, imm as i32)))
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_alu_imm_w_neg_imm_oob -- --test-threads=1
```
**Raw output:**
```text
Test failed: oob imm 2048 must Err (llvm-mc range [-2048, 2047]); got Ok(Word(2147483675)) at src/backend/riscv/assembler/encoder/encode_alu_imm_w_pbt.rs:459.
minimal failing input: rd = "x0", rs1 = "x0", imm = 2048
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/encode_alu_imm_w_pbt.rs
