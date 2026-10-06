# Bug: encode_ldr_str_auto accepts SP as LDR/STR Rt and encodes it as D31
**Law:** SP is not a valid LDR/STR data register; llvm-mc/gas reject `ldr sp, [Xn]` (ARM Rt=31 is ZR)
**Impact:** `ldr sp, [x0]` / `str sp, [x0]` assemble instead of erroring. Because `is_fp_reg("sp")` is true (prefix `s`), the word is 64-bit FP D31, not even integer XZR — a silent wrong opcode.
**Function:** encode_ldr_str_auto
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/load_store.rs:7
**Detected by:** Negative/error contract vs llvm-mc
**Minimal input:** encode_ldr_str_auto([Reg("sp"), Mem{base:"x0", offset:0}], is_load=false)
**Expected:** Err (llvm-mc: "error: invalid operand for instruction")
**Actual:** Ok(Word(0xfd00001f)) — same bits as `str d31, [x0]`
**Severity:** high
**Root cause:** encode_ldr_str_auto maps `sp` to size=11 then calls encode_ldr_str, which never rejects SP as Rt. `is_fp_reg("sp")` matches prefix `s`, so V=1 and the encoding is D-form Rt=31.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/load_store.rs:17`
```rust
    } else if reg_name.starts_with('x') || reg_name == "sp" || reg_name == "xzr" || reg_name == "lr" {
        0b11 // 64-bit
```
**Suggested fix:** Reject SP (and WSP) as the data register before delegating.
```rust
    if reg_name == "sp" || reg_name == "wsp" {
        return Err("ldr/str: SP is not a valid Rt".to_string());
    }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_ldr_str_auto_regression_sp_dest -- --test-threads=1
cargo test --lib encode_ldr_str_auto_neg_sp_dest -- --test-threads=1
```
**Raw output:**
```text
Test failed: SP dest must Err (llvm-mc: invalid operand); got Ok(Word(4244635679))
minimal failing input: is_load = false, rn = 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_ldr_str_auto_pbt.rs
