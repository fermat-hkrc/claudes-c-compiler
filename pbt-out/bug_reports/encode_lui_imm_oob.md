# Bug: encode_lui silently truncates immediates outside [0, 1048575]
**Law:** ∀ rd, ∀ imm ∉ [0, 1048575]. encode_lui([Reg(rd), Imm(imm)]) = Err
**Impact:** Out-of-range LUI immediates assemble to a different instruction (e.g. `lui x0, -1` becomes `lui x0, 0xFFFFF`) instead of being rejected, so callers get silent wrong machine code.
**Function:** encode_lui
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/base.rs:5
**Detected by:** Negative/Error Contract
**Minimal input:** encode_lui([Reg("x0"), Imm(-1)])
**Expected:** Err (llvm-mc: operand must be an integer in the range [0, 1048575])
**Actual:** Ok(Word(0xFFFFF037)) — encoding of `lui x0, 1048575`
**Severity:** high
**Root cause:** base.rs:10 casts the i64 immediate to u32 and shifts, wrapping negatives and dropping bits above 20, then encode_u masks to imm[31:12].
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/base.rs:10`
```rust
            Ok(EncodeResult::Word(encode_u(OP_LUI, rd, (*imm as u32) << 12)))
```
**Suggested fix:** Reject immediates outside the 20-bit unsigned range before shifting.
```rust
            let imm = *imm;
            if !(0..=1048575).contains(&imm) {
                return Err(format!("lui: immediate {imm} out of range [0, 1048575]"));
            }
            Ok(EncodeResult::Word(encode_u(OP_LUI, rd, (imm as u32) << 12)))
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_lui_regression_imm_oob -- --test-threads=1
```
**Raw output:**
```text
thread 'backend::riscv::assembler::encoder::encode_lui_pbt::test_encode_lui_regression_imm_oob' panicked at src/backend/riscv/assembler/encoder/encode_lui_pbt.rs:333:5:
lui x0, -1 must Err (llvm-mc range [0, 1048575]); got Ok(Word(4294963255))
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/encode_lui_pbt.rs
