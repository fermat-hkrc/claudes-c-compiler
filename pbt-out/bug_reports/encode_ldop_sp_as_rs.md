# Bug: encode_ldop accepts SP/WSP as Rs or Rt and W/XZR as base
**Law:** LDADD Rs/Rt must be integer GPRs (including ZR), never SP/WSP; Rn must be Xn|SP, never ZR/W
**Impact:** `ldadd sp, w1, [x2]` is assembled as `ldadd wzr, w1, [x2]` (SP encodes as register 31). llvm-mc/gas report "invalid operand for instruction". The stack pointer is silently rewritten as the zero register. `[xzr]` and `[w2]` are likewise accepted
**Function:** encode_ldop
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/load_store.rs:882
**Detected by:** Negative/Error Contract
**Minimal input:** encode_ldop("ldadd", [Reg("sp"), Reg("w1"), Mem{base:"x2", offset:0}])
**Expected:** Err (llvm-mc: "invalid operand for instruction")
**Actual:** Ok(Word) — SP is encoded as register 31 (ZR)
**Severity:** medium
**Root cause:** load_store.rs:886-887 call get_reg, and parse_reg_num maps "sp"/"wsp" to 31 with no LDADD-specific rejection of SP; Mem base uses parse_reg_num which also accepts xzr/wN
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/load_store.rs:886`
```rust
    let (rs, is_64) = get_reg(operands, 0)?;
    let (rt, _) = get_reg(operands, 1)?;
```
**Suggested fix:** Reject SP/WSP as Rs and Rt; require Xn|SP as base
```rust
    let (rs, is_64) = get_reg(operands, 0)?;
    let (rt, _) = get_reg(operands, 1)?;
    if is_sp_name(operands, 0) || is_sp_name(operands, 1) {
        return Err("ldop: Rs/Rt cannot be SP".to_string());
    }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_ldop_regression_sp_as_rs -- --test-threads=1
```
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_ldop_pbt::test_encode_ldop_regression_sp_as_rs' panicked at src/backend/arm/assembler/encoder/encode_ldop_pbt.rs:846:5:
ldadd sp, w1, [x2] must Err; llvm-mc/gas reject SP as Rs
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_ldop_pbt.rs
