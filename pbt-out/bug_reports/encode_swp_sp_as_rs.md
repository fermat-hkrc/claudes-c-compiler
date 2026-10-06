# Bug: encode_swp accepts SP/WSP as Rs or Rt
**Law:** SWP Rs/Rt must be integer GPRs (including ZR), never SP/WSP
**Impact:** `swp sp, w1, [x2]` is assembled as `swp wzr, w1, [x2]` (SP encodes as register 31). llvm-mc/gas report "invalid operand for instruction". The stack pointer is silently rewritten as the zero register
**Function:** encode_swp
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/load_store.rs:849
**Detected by:** Negative/Error Contract
**Minimal input:** encode_swp("swp", [Reg("sp"), Reg("w1"), Mem{base:"x2", offset:0}])
**Expected:** Err (llvm-mc: "invalid operand for instruction")
**Actual:** Ok(Word) — SP is encoded as register 31 (ZR)
**Severity:** medium
**Root cause:** load_store.rs:853-854 call get_reg, and parse_reg_num maps "sp"/"wsp" to 31 with no SWP-specific rejection of SP
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/load_store.rs:853`
```rust
    let (rs, is_64) = get_reg(operands, 0)?;
    let (rt, _) = get_reg(operands, 1)?;
```
**Suggested fix:** Reject SP/WSP as Rs and Rt
```rust
    let (rs, is_64) = get_reg(operands, 0)?;
    let (rt, _) = get_reg(operands, 1)?;
    if is_sp_name(operands, 0) || is_sp_name(operands, 1) {
        return Err("swp: Rs/Rt cannot be SP".to_string());
    }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_swp_regression_sp_as_rs -- --test-threads=1
```
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_swp_pbt::test_encode_swp_regression_sp_as_rs' panicked at src/backend/arm/assembler/encoder/encode_swp_pbt.rs:767:5:
swp sp, w1, [x2] must Err; llvm-mc/gas reject SP as Rs
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_swp_pbt.rs
