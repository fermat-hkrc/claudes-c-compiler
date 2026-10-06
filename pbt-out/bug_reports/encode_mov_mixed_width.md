# Bug: encode_mov accepts mixed X/W register MOV
**Law:** `mov Xd, Wm` / `mov Wd, Xm` is invalid; GNU as reports operand mismatch
**Impact:** A width mismatch is encoded as a 64-bit ORR using only Rd's sf bit, so `mov x0, w0` becomes `mov x0, x0` and silently uses the wrong source width
**Function:** encode_mov
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/data_processing.rs:127
**Detected by:** Negative/error contract — gas mixed-width rejection
**Minimal input:** encode_mov([Reg("x0"), Reg("w0")])
**Expected:** Err
**Actual:** Ok(Word(0xaa0003e0)) — 64-bit `mov x0, x0`
**Severity:** high
**Root cause:** data_processing.rs:127 takes `is_64` only from Rd and never checks that Rm has the same width
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/data_processing.rs:127`
```rust
        let is_64 = is_64bit_reg(rd_name);
```
**Suggested fix:** Require matching integer widths (and reject FP names) before encoding ORR.
```rust
        if is_64bit_reg(rd_name) != is_64bit_reg(rm_name) {
            return Err("mov requires matching register widths".to_string());
        }
        let is_64 = is_64bit_reg(rd_name);
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_mov_regression_mixed_width -- --test-threads=1
```
**Raw output:**
```text
mixed width must Err, got Ok(Word(2852127712))
minimal failing input: rd = 0, rm = 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_mov_pbt.rs
