# Bug: encode_li silently ignores extra operands
**Law:** `li` is a two-operand pseudo-instruction (`li rd, imm`); a third operand must be rejected
**Impact:** An extra operand is dropped with no diagnostic, so `li a0, 1, a1` encodes as `li a0, 1`. Typos and extra commas assemble without error.
**Function:** encode_li
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/pseudo.rs:5
**Detected by:** Negative/Error Contract
**Minimal input:** encode_li([Reg("zero"), Imm(0), Reg("zero")]); also encode_li([Reg("a0"), Imm(1), Reg("a1")])
**Expected:** Err (llvm-mc: "invalid operand for instruction"; README documents `li rd, imm`)
**Actual:** Ok — encodes as if the extra operand were absent (`li zero, 0` / `li a0, 1`)
**Severity:** medium
**Root cause:** `pseudo.rs:6` reads operand 0 as rd and operand 1 as imm and never checks `operands.len()`, so extra operands are ignored.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/pseudo.rs:6`
```rust
    let rd = get_reg(operands, 0)?;
    let imm = get_imm(operands, 1)?;
```
**Suggested fix:** Reject any operand list whose length is not exactly 2.
```rust
    if operands.len() != 2 {
        return Err(format!("li: expected 2 operands, got {}", operands.len()));
    }
    let rd = get_reg(operands, 0)?;
    let imm = get_imm(operands, 1)?;
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_li_regression_extra_operand -- --test-threads=1
```
**Raw output:**
```text
thread 'backend::riscv::assembler::encoder::encode_li_pbt::encode_li_neg_extra' panicked at src/backend/riscv/assembler/encoder/encode_li_pbt.rs:395:1:
Test failed: extra operand must Err for li zero, 0 (README two-operand; llvm-mc rejects) at src/backend/riscv/assembler/encoder/encode_li_pbt.rs:514.
minimal failing input: rd = "zero", imm = 0, extra = Reg(
    "zero",
)
	successes: 0
	local rejects: 0
	global rejects: 0

thread 'backend::riscv::assembler::encoder::encode_li_pbt::test_encode_li_regression_extra_operand' panicked at src/backend/riscv/assembler/encoder/encode_li_pbt.rs:389:5:
li a0, 1 with a third operand must be rejected (llvm-mc rejects; README documents `li rd, imm`)
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/encode_li_pbt.rs
