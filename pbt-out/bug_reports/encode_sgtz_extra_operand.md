# Bug: encode_sgtz silently ignores extra operands
**Law:** ∀ rd ∈ GPR, ∀ rs ∈ GPR, ∀ extra. encode_sgtz([Reg(rd), Reg(rs), extra]) is Err
**Impact:** A third (or later) operand is dropped with no diagnostic, so `sgtz a0, a1, a2` encodes as `slt a0, x0, a1`. Typos and extra commas assemble without error.
**Function:** encode_sgtz
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/pseudo.rs:275
**Detected by:** Negative/Error Contract — extra operand
**Minimal input:** encode_sgtz([Reg("zero"), Reg("zero"), Reg("zero")])
**Expected:** Err (llvm-mc: "invalid operand for instruction"; README documents `sgtz rd, rs`)
**Actual:** Ok(Word(0x00002033)) — encodes as if the extra operand were absent (`slt x0, x0, x0`).
**Severity:** medium
**Root cause:** `pseudo.rs:276-277` reads operand 0 as rd and operand 1 as rs2 and never checks `operands.len()`, so extra operands are ignored.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/pseudo.rs:276`
```rust
    let rd = get_reg(operands, 0)?;
    let rs2 = get_reg(operands, 1)?;
```
**Suggested fix:** Reject any operand list whose length is not exactly 2.
```rust
    if operands.len() != 2 {
        return Err(format!("sgtz: expected 2 operands, got {}", operands.len()));
    }
    let rd = get_reg(operands, 0)?;
    let rs2 = get_reg(operands, 1)?;
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_sgtz_regression_extra_operand -- --test-threads=1
```
**Raw output:**
```text
thread 'backend::riscv::assembler::encoder::encode_sgtz_pbt::encode_sgtz_neg_extra' panicked at src/backend/riscv/assembler/encoder/encode_sgtz_pbt.rs:269:1:
Test failed: extra operand must Err for sgtz zero, zero (llvm-mc rejects: true) at src/backend/riscv/assembler/encoder/encode_sgtz_pbt.rs:402.
minimal failing input: rd = "zero", rs = "zero", extra = Reg(
    "zero",
)
	successes: 0
	local rejects: 0
	global rejects: 0

thread 'backend::riscv::assembler::encoder::encode_sgtz_pbt::test_encode_sgtz_regression_extra_operand' panicked at src/backend/riscv/assembler/encoder/encode_sgtz_pbt.rs:262:5:
sgtz zero, zero with a third operand must be rejected (llvm-mc rejects; README documents `sgtz rd, rs`); got Ok(Word(8243))
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/encode_sgtz_pbt.rs
