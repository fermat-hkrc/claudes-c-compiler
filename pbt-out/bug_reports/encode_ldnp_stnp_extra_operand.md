# Bug: encode_ldnp_stnp ignores a fourth operand
**Law:** LDNP/STNP must reject more than three operands, matching llvm-mc/gas (`invalid operand for instruction`)
**Impact:** An extra operand is dropped and a 32-bit word is still emitted, so a mistyped instruction assembles to a silent pair load/store
**Function:** encode_ldnp_stnp
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/load_store.rs:518
**Detected by:** Negative/Error Contract
**Minimal input:** encode_ldnp_stnp([Reg("w0"), Reg("w0"), Mem{base:"x0", offset:0}, Reg("x0")], is_load=false)
**Expected:** Err (llvm-mc rejects `stnp w0, w0, [x0], x0`)
**Actual:** Ok(Word(0x28000000)) — STNP W0, W0, [X0]
**Severity:** medium
**Root cause:** load_store.rs:519 checks only `operands.len() < 3` and then reads operands[0..2], so any trailing operands are ignored
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/load_store.rs:519`
```rust
    if operands.len() < 3 {
        return Err("ldnp/stnp requires 3 operands".to_string());
    }
```
**Suggested fix:** Reject `operands.len() != 3` (or `> 3`) before encoding
```rust
    if operands.len() != 3 {
        return Err("ldnp/stnp requires 3 operands".to_string());
    }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_ldnp_stnp_regression_extra_operand -- --test-threads=1
```
**Raw output:**
```text
Test failed: extra operand must Err (llvm-mc rejects a fourth operand); got Ok(Word(671088640))
minimal failing input: is_load = false, is_64 = false, rt1 = 0, rt2 = 0, rn = 0, extra = Reg("x0")
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_ldnp_stnp_pbt.rs
