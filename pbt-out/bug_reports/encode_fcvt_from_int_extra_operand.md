# Bug: encode_fcvt_from_int ignores a 4th operand
**Law:** A fourth operand after rd, rs1, and an optional rounding mode must return Err. llvm-mc rejects extra operands ("invalid operand for instruction"); encode_instruction passes the full operand slice through to encode_fcvt_from_int.
**Impact:** Malformed `fcvt.s.w f0, x0, rne, 0` still assembles as `fcvt.s.w f0, x0, rne`. A typo or extra token is silently dropped, so the assembler emits a valid OP-FP FCVT.S.W word instead of diagnosing the extra operand.
**Function:** encode_fcvt_from_int
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/float.rs:131
**Detected by:** Negative/Error Contract
**Minimal input:** encode_fcvt_from_int([Reg("f0"), Reg("x0"), RoundingMode("rne"), Imm(0)], 0b1101000, 0)
**Expected:** Err
**Actual:** Ok(Word(3489661011)) which is 0xd0000053, the encoding of fcvt.s.w f0, x0, rne
**Severity:** medium
**Root cause:** float.rs:135 only tests `operands.len() > 2` to read an optional rm and never rejects `operands.len() > 3`, so a 4th operand is ignored and line 142 still returns Ok.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/float.rs:135`
```rust
    let rm = if operands.len() > 2 {
```
**Suggested fix:** Reject more than three operands before packing the R-type word.
```rust
    if operands.len() > 3 {
        return Err("fcvt from int: unexpected extra operand".to_string());
    }
    let rm = if operands.len() > 2 {
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_fcvt_from_int -- --test-threads=1
cargo test --lib test_encode_fcvt_from_int_regression_extra_operand -- --test-threads=1
```
**Raw output:**
```text
Test failed: 4th operand must Err for fcvt.s.w f0, x0, rne (llvm-mc rejects extra operands); got Ok(Word(3489661011)) at src/backend/riscv/assembler/encoder/encode_fcvt_from_int_pbt.rs:541.
minimal failing input: (mn, f7, rs2) = (
    "fcvt.s.w",
    104,
    0,
), rd = "f0", rs1 = "x0", extra = Imm(
    0,
)
	successes: 0
	local rejects: 0
	global rejects: 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/encode_fcvt_from_int_pbt.rs
