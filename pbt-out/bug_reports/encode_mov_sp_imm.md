# Bug: encode_mov encodes mov sp, #imm as MOVZ XZR
**Law:** `mov sp, #imm` is invalid; GNU as / llvm-mc reject it because MOV (wide immediate) Rd cannot be SP (register 31 is XZR)
**Impact:** `mov sp, #0` assembles as `mov xzr, #0`, so an attempt to materialize a constant into SP is dropped on the floor
**Function:** encode_mov
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/data_processing.rs:87
**Detected by:** Negative/error contract — gas/llvm-mc reject `mov sp, #0`
**Minimal input:** encode_mov([Reg("sp"), Imm(0)])
**Expected:** Err
**Actual:** Ok(Word(0xd28003ff)) — MOVZ XZR, #0
**Severity:** high
**Root cause:** the immediate path calls get_reg, which maps `sp` to 31, then encodes MOVZ; there is no check that Rd is not SP
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/data_processing.rs:87`
```rust
        let (rd, is_64) = get_reg(operands, 0)?;
        let imm = *imm;
```
**Suggested fix:** Reject SP/WSP as the destination of a wide-immediate MOV.
```rust
        let (rd, is_64) = get_reg(operands, 0)?;
        if matches!(operands.first(), Some(Operand::Reg(n)) if {
            let n = n.to_lowercase(); n == "sp" || n == "wsp"
        }) {
            return Err("mov immediate destination cannot be SP".to_string());
        }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_mov_regression_sp_imm -- --test-threads=1
```
**Raw output:**
```text
mov sp, #imm must Err, got Ok(Word(3531603999))
minimal failing input: imm = 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_mov_pbt.rs
