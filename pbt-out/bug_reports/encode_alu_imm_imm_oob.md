# Bug: encode_alu_imm wraps out-of-range OP-IMM immediates
**Law:** An OP-IMM immediate outside signed 12-bit range [-2048, 2047] must return Err, matching llvm-mc which requires an integer in that range.
**Impact:** Immediates such as 2048 are encoded as the wrapped 12-bit pattern (2048 → −2048), so an addi/andi/xori that the source wrote with a large constant silently computes the wrong value.
**Function:** encode_alu_imm
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/base.rs:223
**Detected by:** Negative/Error Contract
**Minimal input:** encode_alu_imm([Reg("x0"), Reg("x0"), Imm(2048)], funct3=0)  // addi x0, x0, 2048
**Expected:** Err
**Actual:** Ok(Word(0x80000013))  // encoding of addi x0, x0, -2048
**Severity:** high
**Root cause:** base.rs:228 casts `*imm as i32` into encode_i, which keeps only imm[11:0] (`& 0xFFF`) and never range-checks the 12-bit signed field.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/base.rs:228`
```rust
            Ok(EncodeResult::Word(encode_i(OP_OP_IMM, rd, funct3, rs1, *imm as i32)))
```
**Suggested fix:** Reject immediates outside [-2048, 2047] before packing.
```rust
            if !(-2048..=2047).contains(imm) {
                return Err("alu_imm: immediate out of range [-2048, 2047]".to_string());
            }
            Ok(EncodeResult::Word(encode_i(OP_OP_IMM, rd, funct3, rs1, *imm as i32)))
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_alu_imm_neg_imm_oob -- --test-threads=1
```
**Raw output:**
```text
Test failed: oob imm 2048 must Err (llvm-mc range [-2048, 2047]); got Ok(Word(2147483667)) at src/backend/riscv/assembler/encoder/encode_alu_imm_pbt.rs:501.
minimal failing input: (mn, f3) = (
    "addi",
    0,
), rd = "x0", rs1 = "x0", imm = 2048
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/encode_alu_imm_pbt.rs
