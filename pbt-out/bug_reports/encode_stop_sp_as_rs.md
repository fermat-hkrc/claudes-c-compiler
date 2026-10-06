# Bug: encode_stop accepts SP as Rs and XZR/W as base
**Law:** STADD Rs is ZR not SP; the base is Xn|SP not ZR or a W register
**Impact:** `stadd sp, [x1]` encodes as `stadd xzr, [x1]`, and `stadd w0, [xzr]` encodes as `stadd w0, [sp]`. llvm-mc/gas reject both. Silent aliasing of SP and XZR (both register number 31) and W-bases produces the wrong instruction
**Function:** encode_stop
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/load_store.rs:927
**Detected by:** Negative/Error Contract
**Minimal input:** encode_stop("stadd", [Reg("sp"), Mem{base:"x1", offset:0}])
**Expected:** Err (llvm-mc: "invalid operand for instruction")
**Actual:** Ok(Word) — SP is encoded as XZR (register 31)
**Severity:** medium
**Root cause:** load_store.rs:931-933 uses get_reg/parse_reg_num, which map both SP and XZR to 31 and accept W names as a base, with no SP-vs-ZR or X-vs-W check
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/load_store.rs:931`
```rust
    let (rs, is_64) = get_reg(operands, 0)?;
    let rn = match operands.get(1) {
        Some(Operand::Mem { base, .. }) => parse_reg_num(base).ok_or_else(|| format!("{}: invalid base", mnemonic))?,
        _ => return Err(format!("{} requires memory operand [Xn]", mnemonic)),
    };
```
**Suggested fix:** Reject SP/WSP as Rs; require the base to be Xn or SP (not XZR, x31, W, or WSP)
```rust
    let (rs, is_64) = get_reg(operands, 0)?;
    if matches_sp_name(operands, 0) {
        return Err(format!("{}: SP is not a valid Rs", mnemonic));
    }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_stop_regression_sp_as_rs -- --test-threads=1
```
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_stop_pbt::test_encode_stop_regression_sp_as_rs' (2610723) panicked at src/backend/arm/assembler/encoder/encode_stop_pbt.rs:672:5:
stadd sp, [x1] must Err; llvm-mc/gas reject SP as Rs
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_stop_pbt.rs
