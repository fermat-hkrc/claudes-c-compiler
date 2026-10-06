# Bug: encode_auipc silently truncates immediates outside [0, 1048575]
**Law:** ∀ rd, ∀ imm ∉ [0, 1048575]. encode_auipc([Reg(rd), Imm(imm)]) = Err
**Impact:** Out-of-range AUIPC immediates assemble to a different instruction (e.g. `auipc x0, -1` becomes `auipc x0, 0xFFFFF`) instead of being rejected, so callers get silent wrong machine code.
**Function:** encode_auipc
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/base.rs:30
**Detected by:** Negative/Error Contract
**Minimal input:** encode_auipc([Reg("x0"), Imm(-1)])
**Expected:** Err (llvm-mc: operand must be an integer in the range [0, 1048575])
**Actual:** Ok(Word(0xFFFFF017)) — encoding of `auipc x0, 1048575`
**Severity:** high
**Root cause:** base.rs:34 casts the i64 immediate to u32 and shifts, wrapping negatives and dropping bits above 20, then encode_u masks to imm[31:12].
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/base.rs:34`
```rust
            Ok(EncodeResult::Word(encode_u(OP_AUIPC, rd, (*imm as u32) << 12)))
```
**Suggested fix:** Reject immediates outside the 20-bit unsigned range before shifting.
```rust
            let imm = *imm;
            if !(0..=1048575).contains(&imm) {
                return Err(format!("auipc: immediate {imm} out of range [0, 1048575]"));
            }
            Ok(EncodeResult::Word(encode_u(OP_AUIPC, rd, (imm as u32) << 12)))
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_auipc_regression_imm_oob -- --test-threads=1
```
**Raw output:**
```text
thread 'backend::riscv::assembler::encoder::encode_auipc_pbt::test_encode_auipc_regression_imm_oob' panicked at src/backend/riscv/assembler/encoder/encode_auipc_pbt.rs:348:5:
auipc x0, -1 must Err (llvm-mc range [0, 1048575]); got Ok(Word(4294963223))
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/encode_auipc_pbt.rs
