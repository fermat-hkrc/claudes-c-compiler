# Bug: encode_store wraps out-of-range store offsets
**Law:** A store immediate outside signed 12-bit range [-2048, 2047] must return Err, matching llvm-mc which requires an integer in that range.
**Impact:** Offsets such as 2048 are encoded as the wrapped 12-bit pattern (2048 → −2048), so a store that the source wrote as a large displacement silently hits the wrong address.
**Function:** encode_store
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/base.rs:194
**Detected by:** Negative/Error Contract
**Minimal input:** encode_store([Reg("x0"), Mem { base: "x0", offset: 2048 }], funct3=0)  // sb x0, 2048(x0)
**Expected:** Err
**Actual:** Ok(Word(0x80000023))  // encoding of sb x0, -2048(x0)
**Severity:** high
**Root cause:** base.rs:199 casts `*offset as i32` into encode_s, which keeps only imm[11:5]|imm[4:0] and never range-checks the 12-bit signed field.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/base.rs:199`
```rust
            Ok(EncodeResult::Word(encode_s(OP_STORE, funct3, rs1, rs2, *offset as i32)))
```
**Suggested fix:** Reject immediates outside [-2048, 2047] before packing.
```rust
            if !(-2048..=2047).contains(&offset) {
                return Err("store: immediate out of range [-2048, 2047]".to_string());
            }
            Ok(EncodeResult::Word(encode_s(OP_STORE, funct3, rs1, rs2, *offset as i32)))
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_store_neg_imm_oob -- --test-threads=1
```
**Raw output:**
```text
Test failed: oob imm 2048 must Err (llvm-mc range [-2048, 2047]); got Ok(Word(2147483683)) at src/backend/riscv/assembler/encoder/encode_store_pbt.rs:517.
minimal failing input: (mn, f3) = (
    "sb",
    0,
), rs2 = "x0", rs1 = "x0", imm = 2048
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/encode_store_pbt.rs
