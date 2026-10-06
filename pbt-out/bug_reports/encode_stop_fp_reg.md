# Bug: encode_stop accepts FP/SIMD Rs and X registers on STADDB/STADDH
**Law:** STADD Rs is a GPR (W or X); STADDB/STADDH require W registers
**Impact:** `stadd b0, [x1]` encodes as `stadd w0, [x1]`, and `staddb x0, [x1]` encodes as STADDB with Rs=0. llvm-mc/gas reject both. A SIMD or 64-bit source is silently recoded as a 32-bit GPR
**Function:** encode_stop
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/load_store.rs:927
**Detected by:** Negative/Error Contract
**Minimal input:** encode_stop("stadd", [Reg("b0"), Mem{base:"x1", offset:0}])
**Expected:** Err (llvm-mc: "invalid operand for instruction")
**Actual:** Ok(Word) — b0 is parsed as register 0, 32-bit
**Severity:** medium
**Root cause:** load_store.rs:931 get_reg/parse_reg_num accept b/h/s/d/q/v prefixes; load_store.rs:950 takes size from a 'b'/'h' suffix so X registers still encode for STADDB/STADDH
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/load_store.rs:931`
```rust
    let (rs, is_64) = get_reg(operands, 0)?;
```
**Suggested fix:** Require a GPR Rs; for byte/half variants require a W register
```rust
    let (rs, is_64) = get_gpr_rs(operands, 0)?;
    if is_byte_or_half && is_64 {
        return Err(format!("{} requires a W register", mnemonic));
    }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_stop_regression_fp_reg -- --test-threads=1
```
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_stop_pbt::test_encode_stop_regression_fp_reg' (2610721) panicked at src/backend/arm/assembler/encoder/encode_stop_pbt.rs:687:5:
stadd b0, [x1] must Err; llvm-mc/gas require integer registers
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_stop_pbt.rs
