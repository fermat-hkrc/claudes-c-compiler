# Bug: encode_mv silently ignores extra operands
**Law:** ∀ rd ∈ GPR, ∀ rs ∈ GPR, ∀ extra. encode_mv([Reg(rd), Reg(rs), extra]) is Err
**Impact:** A third (or later) operand is dropped with no diagnostic, so `mv a0, a1, a2` encodes as `add a0, x0, a1`. Typos and extra commas assemble without error.
**Function:** encode_mv
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/pseudo.rs:225
**Detected by:** Negative/Error Contract — extra operand
**Minimal input:** encode_mv([Reg("zero"), Reg("zero"), Reg("zero")]); also encode_mv([Reg("a0"), Reg("a1"), Imm(0)])
**Expected:** Err (llvm-mc: "invalid operand for instruction"; README documents `mv rd, rs`)
**Actual:** Ok(Word(0x00000033)) for zero,zero,zero — encodes as if the extra operand were absent (`add x0, x0, x0`). Ok(Word(0x00b00533)) for a0,a1,Imm(0).
**Severity:** medium
**Root cause:** `pseudo.rs:226` reads operand 0 as rd and operand 1 as rs and never checks `operands.len()`, so extra operands are ignored.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/pseudo.rs:226`
```rust
    let rd = get_reg(operands, 0)?;
    let rs = get_reg(operands, 1)?;
```
**Suggested fix:** Reject any operand list whose length is not exactly 2.
```rust
    if operands.len() != 2 {
        return Err(format!("mv: expected 2 operands, got {}", operands.len()));
    }
    let rd = get_reg(operands, 0)?;
    let rs = get_reg(operands, 1)?;
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_mv_regression_extra_operand -- --test-threads=1
```
**Raw output:**
```text
thread 'backend::riscv::assembler::encoder::encode_mv_pbt::encode_mv_neg_extra' panicked at src/backend/riscv/assembler/encoder/encode_mv_pbt.rs:327:1:
Test failed: extra operand must Err for mv zero, zero (llvm-mc rejects: true); got Ok(Word(51)) at src/backend/riscv/assembler/encoder/encode_mv_pbt.rs:455.
minimal failing input: rd = "zero", rs = "zero", extra = Reg(
    "zero",
)
	successes: 0
	local rejects: 0
	global rejects: 0

thread 'backend::riscv::assembler::encoder::encode_mv_pbt::test_encode_mv_regression_extra_operand' panicked at src/backend/riscv/assembler/encoder/encode_mv_pbt.rs:320:5:
mv a0, a1 with a third operand must be rejected (llvm-mc rejects; README documents `mv rd, rs`); got Ok(Word(11535667))
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/encode_mv_pbt.rs
