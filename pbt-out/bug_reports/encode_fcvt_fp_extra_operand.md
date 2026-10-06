# Bug: encode_fcvt_fp ignores a 4th operand
**Law:** A fourth operand after rd, rs1, and an optional rounding mode must return Err. llvm-mc rejects extra operands ("invalid operand for instruction"); encode_instruction passes the full operand slice through to encode_fcvt_fp.
**Impact:** Malformed `fcvt.s.d f0, f0, rne, 0` still assembles as `fcvt.s.d f0, f0, rne`. A typo or extra token is silently dropped, so the assembler emits a valid OP-FP FCVT.S.D word instead of diagnosing the extra operand.
**Function:** encode_fcvt_fp
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/float.rs:146
**Detected by:** Negative/Error Contract
**Minimal input:** encode_fcvt_fp([Reg("f0"), Reg("f0"), RoundingMode("rne"), Imm(0)], 0b0100000, 1)
**Expected:** Err
**Actual:** Ok(Word(1074790483)) which is 0x40100053, the encoding of fcvt.s.d f0, f0, rne
**Severity:** medium
**Root cause:** float.rs:150 only tests `operands.len() > 2` to read an optional rm and never rejects `operands.len() > 3`, so a 4th operand is ignored and line 158 still returns Ok.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/float.rs:150`
```rust
    let rm = if operands.len() > 2 {
```
**Suggested fix:** Reject more than three operands before packing the R-type word.
```rust
    if operands.len() > 3 {
        return Err("fcvt fp: unexpected extra operand".to_string());
    }
    let rm = if operands.len() > 2 {
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_fcvt_fp -- --test-threads=1
cargo test --lib test_encode_fcvt_fp_regression_extra_operand -- --test-threads=1
```
**Raw output:**
```text
Test failed: 4th operand must Err for fcvt.s.d f0, f0, rne (llvm-mc rejects extra operands); got Ok(Word(1074790483)) at src/backend/riscv/assembler/encoder/encode_fcvt_fp_pbt.rs:497.
minimal failing input: (mn, f7, rs2) = (
    "fcvt.s.d",
    32,
    1,
), rd = "f0", rs1 = "f0", extra = Imm(
    0,
)
	successes: 0
	local rejects: 0
	global rejects: 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/encode_fcvt_fp_pbt.rs
