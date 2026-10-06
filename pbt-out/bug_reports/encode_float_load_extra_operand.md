# Bug: encode_float_load ignores extra operands
**Law:** For every FLW/FLD with more than two operands, encode_float_load must return Err, matching llvm-mc ("invalid operand for instruction").
**Impact:** Malformed `flw rd, offset(rs1), extra` is encoded as if the extra operand were absent. Downstream assembly that accidentally carries a third operand silently drops it instead of failing the build.
**Function:** encode_float_load
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/float.rs:5
**Detected by:** Negative/Error Contract
**Minimal input:** encode_float_load([Reg("f0"), Mem { base: "x0", offset: 0 }, Imm(0)], 0b010)
**Expected:** Err (llvm-mc rejects `flw f0, 0(x0), 0`)
**Actual:** Ok(Word(8199)) which is 0x00002007, the encoding of flw f0, 0(x0)
**Severity:** medium
**Root cause:** float.rs:7 matches only `operands.get(1)` and never checks `operands.len()`, so any trailing operands are ignored after a valid FP dest and Mem operand.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/float.rs:7`
```rust
    match &operands.get(1) {
```
**Suggested fix:** Require exactly two operands before encoding.
```rust
    if operands.len() != 2 {
        return Err("float load: unexpected extra operand".to_string());
    }
    match &operands.get(1) {
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_float_load_neg_extra -- --test-threads=1
cargo test --lib test_encode_float_load_regression_extra_operand -- --test-threads=1
```
**Raw output:**
```text
Test failed: extra operand must Err for flw f0, 0(x0) (llvm-mc rejects extra operands); got Ok(Word(8199)) at src/backend/riscv/assembler/encoder/encode_float_load_pbt.rs:499.
minimal failing input: (mn, f3) = (
    "flw",
    2,
), rd = "f0", rs1 = "x0", off = 0, extra = Imm(
    0,
)
	successes: 0
	local rejects: 0
	global rejects: 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/encode_float_load_pbt.rs
