# Bug: encode_fp_arith ignores a 5th operand
**Law:** A fifth operand after rd, rs1, rs2, and an optional rounding mode must return Err. llvm-mc rejects extra operands ("invalid operand for instruction"); encode_instruction passes the full operand slice through.
**Impact:** Malformed `fadd.s f0, f0, f0, rne, 0` still assembles as `fadd.s f0, f0, f0, rne`. A typo or extra token is silently dropped, so the assembler emits a valid OP-FP word instead of diagnosing the extra operand.
**Function:** encode_fp_arith
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/float.rs:61
**Detected by:** Negative/Error Contract
**Minimal input:** encode_fp_arith([Reg("f0"), Reg("f0"), Reg("f0"), RoundingMode("rne"), Imm(0)], 0)
**Expected:** Err
**Actual:** Ok(Word(83)) which is 0x00000053, the encoding of fadd.s f0, f0, f0, rne
**Severity:** medium
**Root cause:** float.rs:66 only tests `operands.len() > 3` to read an optional rm and never rejects `operands.len() > 4`, so a 5th operand is ignored and line 74 still returns Ok.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/float.rs:66`
```rust
    let rm = if operands.len() > 3 {
```
**Suggested fix:** Reject more than four operands before packing the R-type word.
```rust
    if operands.len() > 4 {
        return Err("fp arith: unexpected extra operand".to_string());
    }
    let rm = if operands.len() > 3 {
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_fp_arith_neg_extra -- --test-threads=1
cargo test --lib test_encode_fp_arith_regression_extra_operand -- --test-threads=1
```
**Raw output:**
```text
Test failed: 5th operand must Err for fadd.s f0, f0, f0, rne (llvm-mc rejects extra operands); got Ok(Word(83)) at src/backend/riscv/assembler/encoder/encode_fp_arith_pbt.rs:482.
minimal failing input: (mn, f7) = (
    "fadd.s",
    0,
), rd = "f0", rs1 = "f0", rs2 = "f0", extra = Imm(
    0,
)
	successes: 0
	local rejects: 0
	global rejects: 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/encode_fp_arith_pbt.rs
