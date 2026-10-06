# Bug: encode_jal silently truncates odd and out-of-range immediates
**Law:** ∀ rd ∈ GPR, ∀ imm ∈ i64. (imm odd ∨ imm < -1048576 ∨ imm > 1048574) ⇒ encode_jal([Reg(rd), Imm(imm)]) = Err
**Impact:** An odd or out-of-range JAL offset assembles to a different jump (e.g. `jal x0, 1` becomes `jal x0, 0`) instead of being rejected, so callers get silent wrong machine code.
**Function:** encode_jal
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/base.rs:51
**Detected by:** Negative/Error Contract
**Minimal input:** encode_jal([Reg("x0"), Imm(1)])
**Expected:** Err (llvm-mc: immediate must be a multiple of 2 bytes in the range [-1048576, 1048574])
**Actual:** Ok(Word(0x0000006f)) — encoding of `jal x0, 0`
**Severity:** high
**Root cause:** base.rs:75 casts the i64 immediate to i32 with no range or alignment check; encode_j then drops bit 0 via `(imm >> 1) & 0x3FF`, so odd 1 becomes 0.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/base.rs:75`
```rust
                Ok(EncodeResult::Word(encode_j(OP_JAL, rd, *imm as i32)))
```
**Suggested fix:** Reject immediates that are odd or outside the 21-bit even signed range before packing.
```rust
            let imm = *imm;
            if imm % 2 != 0 || !(-1048576..=1048574).contains(&imm) {
                return Err(format!(
                    "jal: immediate {imm} must be a multiple of 2 in [-1048576, 1048574]"
                ));
            }
            Ok(EncodeResult::Word(encode_j(OP_JAL, rd, imm as i32)))
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_jal_regression_imm_oob -- --test-threads=1
```
**Raw output:**
```text
thread 'backend::riscv::assembler::encoder::encode_jal_pbt::test_encode_jal_regression_imm_oob' panicked at src/backend/riscv/assembler/encoder/encode_jal_pbt.rs:330:5:
jal x0, 1 must Err (odd offset); got Ok(Word(111))
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/encode_jal_pbt.rs
