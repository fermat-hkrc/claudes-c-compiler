# Bug: encode_neon_three_same encodes a GPR name as Vd
**Law:** Three-same destinations are NEON V registers (`v0.8b`, …). A GPR name such as `x0` is not a valid Vd; llvm-mc rejects `cmeq x0.8b, v0.8b, v0.8b`.
**Impact:** `cmeq x0.8b, v0.8b, v0.8b` encodes as `cmeq v0.8b, v0.8b, v0.8b` because parse_reg_num maps the `x` prefix onto register 0.
**Function:** encode_neon_three_same
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:65
**Detected by:** Negative/Error Contract (4e)
**Minimal input:** encode_neon_three_same([RegArrangement{reg:"x0", arrangement:"8b"}, v0.8b, v0.8b], u=1, opcode=0b10001)
**Expected:** Err (llvm-mc: "invalid operand for instruction")
**Actual:** Ok(Word) identical to `cmeq v0.8b, v0.8b, v0.8b`
**Severity:** medium
**Root cause:** neon.rs:69 uses get_neon_reg, which calls parse_reg_num; parse_reg_num accepts prefix `x`/`w`/`d`/`s`/`q`/`h`/`b` as well as `v`.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:69`
```rust
    let (rd, arr_d) = get_neon_reg(operands, 0)?;
```
**Suggested fix:** Restrict NEON register names to the `v` prefix (and reject `x`/`w`/scalar FP names) before encoding.
```rust
    let (rd, arr_d) = get_neon_reg(operands, 0)?;
    // get_neon_reg / parse_reg_num must require a 'v' prefix for three-same Vd/Vn/Vm
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_three_same_regression_gpr_dest -- --test-threads=1
```
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_neon_three_same_pbt::test_encode_neon_three_same_regression_gpr_dest' panicked at src/backend/arm/assembler/encoder/encode_neon_three_same_pbt.rs:536:5:
cmeq x0.8b, v0.8b, v0.8b must Err (gas/llvm-mc require Vd.T)
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_neon_three_same_pbt.rs
