# Bug: encode_load wraps out-of-range I-type offsets
**Law:** A load offset outside signed imm12 [-2048, 2047] must return Err. llvm-mc rejects those offsets; README.md:353 documents I-type imm[11:0].
**Impact:** `ld rd, 2048(rs1)` encodes as `ld rd, -2048(rs1)` (the 12-bit wrap), so a too-large offset becomes a large negative address with no diagnostic.
**Function:** encode_load
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/base.rs:147
**Detected by:** Negative/Error Contract
**Minimal input:** encode_load([Reg("x0"), Mem { base: "x0", offset: 2048 }], funct3=0)  // lb x0, 2048(x0)
**Expected:** Err
**Actual:** Ok(Word(0x80000003))  // lb x0, -2048(x0)
**Severity:** high
**Root cause:** base.rs:152 casts `*offset as i32` with no range check; encode_i then masks with 0xFFF, wrapping 2048 to the signed-12 value -2048.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/base.rs:152`
```rust
            Ok(EncodeResult::Word(encode_i(OP_LOAD, rd, funct3, rs1, *offset as i32)))
```
**Suggested fix:** Reject offsets outside the documented I-type range before packing.
```rust
            if !(-2048..=2047).contains(offset) {
                return Err("load: offset out of signed-12 range".to_string());
            }
            Ok(EncodeResult::Word(encode_i(OP_LOAD, rd, funct3, rs1, *offset as i32)))
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_load_neg_imm_oob -- --test-threads=1
```
**Raw output:**
```text
Test failed: oob imm 2048 must Err (llvm-mc range [-2048, 2047]); got Ok(Word(2147483651)) at src/backend/riscv/assembler/encoder/encode_load_pbt.rs:642.
minimal failing input: (mn, f3) = (
    "lb",
    0,
), rd = "x0", rs1 = "x0", imm = 2048
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/encode_load_pbt.rs
