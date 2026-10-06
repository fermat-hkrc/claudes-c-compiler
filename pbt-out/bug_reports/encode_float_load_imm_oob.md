# Bug: encode_float_load silently truncates out-of-range I-type immediates
**Law:** For every FLW/FLD with a memory offset outside signed imm12 [-2048, 2047], encode_float_load must return Err, matching llvm-mc and the RISC-V I-type imm[11:0] layout.
**Impact:** An assembler that accepts `flw f0, 2048(x0)` emits the encoding of `flw f0, -2048(x0)` (offset off by 4096). Any compiler or handwritten asm that slips past the 12-bit window gets a silently wrong load address.
**Function:** encode_float_load
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/float.rs:5
**Detected by:** Negative/Error Contract
**Minimal input:** encode_float_load([Reg("f0"), Mem { base: "x0", offset: 2048 }], 0b010)
**Expected:** Err (llvm-mc: "operand must be a symbol with %lo/%pcrel_lo/%tprel_lo modifier or an integer in the range [-2048, 2047]")
**Actual:** Ok(Word(2147491847)) which is 0x80002007, the encoding of flw f0, -2048(x0)
**Severity:** high
**Root cause:** float.rs:10 passes `*offset as i32` into encode_i, which masks to 12 bits (`(imm as u32) & 0xFFF`) with no range check on the LOAD-FP path.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/float.rs:10`
```rust
            Ok(EncodeResult::Word(encode_i(OP_LOAD_FP, rd, funct3, rs1, *offset as i32)))
```
**Suggested fix:** Reject offsets outside signed imm12 before packing.
```rust
            if !(-2048..=2047).contains(offset) {
                return Err(format!("float load immediate out of range: {}", offset));
            }
            Ok(EncodeResult::Word(encode_i(OP_LOAD_FP, rd, funct3, rs1, *offset as i32)))
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_float_load_neg_imm_oob -- --test-threads=1
cargo test --lib test_encode_float_load_regression_imm_oob -- --test-threads=1
```
**Raw output:**
```text
Test failed: oob imm 2048 must Err (llvm-mc range [-2048, 2047]); got Ok(Word(2147491847)) at src/backend/riscv/assembler/encoder/encode_float_load_pbt.rs:481.
minimal failing input: (mn, f3) = (
    "flw",
    2,
), rd = "f0", rs1 = "x0", imm = 2048
	successes: 0
	local rejects: 0
	global rejects: 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/encode_float_load_pbt.rs
