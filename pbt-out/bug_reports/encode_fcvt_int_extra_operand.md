# Bug: encode_fcvt_int ignores a 4th operand
**Law:** A fourth operand after rd, rs1, and an optional rounding mode must return Err. llvm-mc rejects extra operands ("invalid operand for instruction"); encode_instruction passes the full operand slice through to encode_fcvt_int.
**Impact:** Malformed `fcvt.w.s x0, f0, rne, 0` still assembles as `fcvt.w.s x0, f0, rne`. A typo or extra token is silently dropped, so the assembler emits a valid OP-FP FCVT.W.S word instead of diagnosing the extra operand.
**Function:** encode_fcvt_int
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/float.rs:116
**Detected by:** Negative/Error Contract
**Minimal input:** encode_fcvt_int([Reg("x0"), Reg("f0"), RoundingMode("rne"), Imm(0)], 0b1100000, 0)
**Expected:** Err
**Actual:** Ok(Word(3221225555)) which is 0xc0000053, the encoding of fcvt.w.s x0, f0, rne
**Severity:** medium
**Root cause:** float.rs:120 only tests `operands.len() > 2` to read an optional rm and never rejects `operands.len() > 3`, so a 4th operand is ignored and line 128 still returns Ok.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/float.rs:120`
```rust
    let rm = if operands.len() > 2 {
```
**Suggested fix:** Reject more than three operands before packing the R-type word.
```rust
    if operands.len() > 3 {
        return Err("fcvt int: unexpected extra operand".to_string());
    }
    let rm = if operands.len() > 2 {
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_fcvt_int -- --test-threads=1
cargo test --lib test_encode_fcvt_int_regression_extra_operand -- --test-threads=1
```
**Raw output:**
```text
Test failed: 4th operand must Err for fcvt.w.s x0, f0, rne (llvm-mc rejects extra operands); got Ok(Word(3221225555)) at src/backend/riscv/assembler/encoder/encode_fcvt_int_pbt.rs:519.
minimal failing input: (mn, f7, rs2) = (
    "fcvt.w.s",
    96,
    0,
), rd = "x0", rs1 = "f0", extra = Imm(
    0,
)
	successes: 0
	local rejects: 0
	global rejects: 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/encode_fcvt_int_pbt.rs
