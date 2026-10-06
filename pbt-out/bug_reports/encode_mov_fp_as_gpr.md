# Bug: encode_mov encodes FP scalar registers as integer ORR
**Law:** `mov d0, d1` is not integer MOV (GNU as: operand 2 must be a SIMD vector element); scalar FP uses `fmov`
**Impact:** `mov d0, d1` is assembled as `mov w0, w1`, so FP register names are silently reinterpreted as GPR numbers
**Function:** encode_mov
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/data_processing.rs:123
**Detected by:** Negative/error contract — gas FP-scalar rejection
**Minimal input:** encode_mov([Reg("d0"), Reg("d1")])
**Expected:** Err
**Actual:** Ok(Word(0x2a0103e0)) — ORR W0, WZR, W1
**Severity:** high
**Root cause:** parse_reg_num accepts `d`/`s`/`q`/`v` prefixes and the integer MOV path does not call is_fp_reg, so d0/d1 become GPR 0/1 with sf=0
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/data_processing.rs:123`
```rust
    if let (Some(Operand::Reg(rd_name)), Some(Operand::Reg(rm_name))) = (operands.first(), operands.get(1)) {
        let rd = parse_reg_num(rd_name).ok_or("invalid rd")?;
        let rm = parse_reg_num(rm_name).ok_or("invalid rm")?;
        let is_64 = is_64bit_reg(rd_name);
```
**Suggested fix:** Reject FP/SIMD register names on the integer MOV path.
```rust
        if is_fp_reg(rd_name) || is_fp_reg(rm_name) {
            return Err("integer mov does not accept FP/SIMD registers".to_string());
        }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_mov_regression_fp_scalar -- --test-threads=1
```
**Raw output:**
```text
FP scalar mov must Err, got Ok(Word(704709600))
minimal failing input: fp_n = 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_mov_pbt.rs
