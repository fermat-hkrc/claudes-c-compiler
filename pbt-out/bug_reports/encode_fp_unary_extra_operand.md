# Bug: encode_fp_unary ignores a 4th operand
**Law:** A fourth operand after rd, rs1, and an optional rounding mode must return Err. llvm-mc rejects extra operands ("invalid operand for instruction"); encode_instruction passes the full operand slice through to encode_fp_unary.
**Impact:** Malformed `fsqrt.s f0, f0, rne, 0` still assembles as `fsqrt.s f0, f0, rne`. A typo or extra token is silently dropped, so the assembler emits a valid OP-FP FSQRT word instead of diagnosing the extra operand.
**Function:** encode_fp_unary
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/float.rs:81
**Detected by:** Negative/Error Contract
**Minimal input:** encode_fp_unary([Reg("f0"), Reg("f0"), RoundingMode("rne"), Imm(0)], 0b0101100, 0)
**Expected:** Err
**Actual:** Ok(Word(1476395091)) which is 0x58000053, the encoding of fsqrt.s f0, f0, rne
**Severity:** medium
**Root cause:** float.rs:84 only tests `operands.len() > 2` to read an optional rm and never rejects `operands.len() > 3`, so a 4th operand is ignored and line 92 still returns Ok.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/float.rs:84`
```rust
    let rm = if operands.len() > 2 {
```
**Suggested fix:** Reject more than three operands before packing the R-type word.
```rust
    if operands.len() > 3 {
        return Err("fp unary: unexpected extra operand".to_string());
    }
    let rm = if operands.len() > 2 {
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_fp_unary_pbt -- --test-threads=1
cargo test --lib test_encode_fp_unary_regression_extra_operand -- --test-threads=1
```
**Raw output:**
```text
Test failed: 4th operand must Err for fsqrt.s f0, f0, rne (llvm-mc rejects extra operands); got Ok(Word(1476395091)) at src/backend/riscv/assembler/encoder/encode_fp_unary_pbt.rs:492.
minimal failing input: (mn, f7) = (
    "fsqrt.s",
    44,
), rd = "f0", rs1 = "f0", extra = Imm(
    0,
)
	successes: 0
	local rejects: 0
	global rejects: 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/encode_fp_unary_pbt.rs
