# Bug: encode_float_store ignores extra operands
**Law:** A third (or later) operand on FSW/FSD must return Err. llvm-mc rejects extra operands ("invalid operand for instruction"); encode_instruction passes the full operand slice through.
**Impact:** Malformed `fsw fa0, 0(x1), 0` still assembles as a valid store. A typo or extra token is silently dropped.
**Function:** encode_float_store
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/float.rs:33
**Detected by:** Negative/Error Contract
**Minimal input:** encode_float_store([Reg("f0"), Mem { base: "x0", offset: 0 }, Imm(0)], 0b010)
**Expected:** Err
**Actual:** Ok(Word(8231)) which is 0x00002027, the encoding of fsw f0, 0(x0)
**Severity:** medium
**Root cause:** float.rs:35 matches only operands.get(1) and never checks operands.len(), so extra operands are ignored.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/float.rs:35`
```rust
    match &operands.get(1) {
```
**Suggested fix:** Require exactly two operands before encoding.
```rust
    if operands.len() != 2 {
        return Err("float store: unexpected extra operand".to_string());
    }
    match &operands.get(1) {
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_float_store_neg_extra -- --test-threads=1
cargo test --lib test_encode_float_store_regression_extra_operand -- --test-threads=1
```
**Raw output:**
```text
Test failed: extra operand must Err for fsw f0, 0(x0) (llvm-mc rejects extra operands); got Ok(Word(8231)) at src/backend/riscv/assembler/encoder/encode_float_store_pbt.rs:531.
minimal failing input: (mn, f3) = (
    "fsw",
    2,
), rs2 = "f0", rs1 = "x0", off = 0, extra = Imm(
    0,
)
	successes: 0
	local rejects: 0
	global rejects: 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/encode_float_store_pbt.rs
