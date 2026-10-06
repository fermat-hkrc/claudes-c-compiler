# Bug: encode_store ignores extra operands
**Law:** A store with a third (or later) operand must return Err, matching llvm-mc which rejects extra operands on sb/sh/sw/sd.
**Impact:** A mistyped extra operand is silently dropped, so the assembler emits a well-formed store instead of diagnosing the line. Callers of encode_instruction that pass through a longer operand slice get wrong machine code with no error.
**Function:** encode_store
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/base.rs:194
**Detected by:** Negative/Error Contract
**Minimal input:** encode_store([Reg("x0"), Mem { base: "x0", offset: 0 }, Imm(0)], funct3=0)  // sb x0, 0(x0), 0
**Expected:** Err
**Actual:** Ok(Word(0x00000023))  // sb x0, 0(x0) — extra Imm ignored
**Severity:** high
**Root cause:** base.rs:196 matches only `operands.get(1)` and never checks `operands.len()`, so any trailing operands are ignored.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/base.rs:196`
```rust
    match &operands.get(1) {
```
**Suggested fix:** Reject a slice longer than two operands before matching.
```rust
    if operands.len() != 2 {
        return Err("store: expected rs2, offset(rs1)".to_string());
    }
    match &operands.get(1) {
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_store_neg_extra -- --test-threads=1
```
**Raw output:**
```text
Test failed: extra operand must Err for sb x0, 0(x0) (llvm-mc rejects extra operands); got Ok(Word(35)) at src/backend/riscv/assembler/encoder/encode_store_pbt.rs:535.
minimal failing input: (mn, f3) = (
    "sb",
    0,
), rs2 = "x0", rs1 = "x0", off = 0, extra = Imm(
    0,
)
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/encode_store_pbt.rs
