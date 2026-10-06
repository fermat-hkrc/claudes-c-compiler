# Bug: encode_float_store silently truncates out-of-range S-type immediates
**Law:** For every FSW/FSD with a memory offset outside signed imm12 [-2048, 2047], encode_float_store must return Err, matching llvm-mc and the RISC-V S-type imm[11:5]|imm[4:0] layout.
**Impact:** An assembler that accepts `fsw f0, 2048(x0)` emits the encoding of `fsw f0, -2048(x0)` (offset off by 4096). Any compiler or handwritten asm that slips past the 12-bit window gets a silently wrong store address.
**Function:** encode_float_store
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/float.rs:33
**Detected by:** Negative/Error Contract
**Minimal input:** encode_float_store([Reg("f0"), Mem { base: "x0", offset: 2048 }], 0b010)
**Expected:** Err (llvm-mc: "operand must be a symbol with %lo/%pcrel_lo/%tprel_lo modifier or an integer in the range [-2048, 2047]")
**Actual:** Ok(Word(2147491879)) which is 0x80002027, the encoding of fsw f0, -2048(x0)
**Severity:** high
**Root cause:** float.rs:38 passes `*offset as i32` into encode_s, which takes imm[11:5] and imm[4:0] from the low 12 bits with no range check on the STORE-FP path.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/float.rs:38`
```rust
            Ok(EncodeResult::Word(encode_s(OP_STORE_FP, funct3, rs1, rs2, *offset as i32)))
```
**Suggested fix:** Reject offsets outside signed imm12 before packing.
```rust
            if !(-2048..=2047).contains(offset) {
                return Err(format!("float store immediate out of range: {}", offset));
            }
            Ok(EncodeResult::Word(encode_s(OP_STORE_FP, funct3, rs1, rs2, *offset as i32)))
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_float_store_neg_imm_oob -- --test-threads=1
cargo test --lib test_encode_float_store_regression_imm_oob -- --test-threads=1
```
**Raw output:**
```text
Test failed: oob imm 2048 must Err (llvm-mc range [-2048, 2047]); got Ok(Word(2147491879)) at src/backend/riscv/assembler/encoder/encode_float_store_pbt.rs:512.
minimal failing input: (mn, f3) = (
    "fsw",
    2,
), rs2 = "f0", rs1 = "x0", imm = 2048
	successes: 0
	local rejects: 0
	global rejects: 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/encode_float_store_pbt.rs
