# Bug: encode_seqz silently ignores extra operands
**Law:** ∀ rd ∈ GPR, ∀ rs ∈ GPR, ∀ extra. encode_seqz([Reg(rd), Reg(rs), extra]) is Err
**Impact:** A third (or later) operand is dropped with no diagnostic, so `seqz a0, a1, a2` encodes as `sltiu a0, a1, 1`. Typos and extra commas assemble without error.
**Function:** encode_seqz
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/pseudo.rs:257
**Detected by:** Negative/Error Contract — extra operand
**Minimal input:** encode_seqz([Reg("zero"), Reg("zero"), Reg("zero")])
**Expected:** Err (llvm-mc: "invalid operand for instruction"; README documents `seqz rd, rs`)
**Actual:** Ok(Word(0x00103013)) — encodes as if the extra operand were absent (`sltiu x0, x0, 1`).
**Severity:** medium
**Root cause:** `pseudo.rs:258` reads operand 0 as rd and operand 1 as rs1 and never checks `operands.len()`, so extra operands are ignored.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/pseudo.rs:258`
```rust
    let rd = get_reg(operands, 0)?;
    let rs1 = get_reg(operands, 1)?;
```
**Suggested fix:** Reject any operand list whose length is not exactly 2.
```rust
    if operands.len() != 2 {
        return Err(format!("seqz: expected 2 operands, got {}", operands.len()));
    }
    let rd = get_reg(operands, 0)?;
    let rs1 = get_reg(operands, 1)?;
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_seqz_regression_extra_operand -- --test-threads=1
```
**Raw output:**
```text
thread 'backend::riscv::assembler::encoder::encode_seqz_pbt::encode_seqz_neg_extra' panicked at src/backend/riscv/assembler/encoder/encode_seqz_pbt.rs:266:1:
Test failed: extra operand must Err for seqz zero, zero (llvm-mc rejects: true) at src/backend/riscv/assembler/encoder/encode_seqz_pbt.rs:394.
minimal failing input: rd = "zero", rs = "zero", extra = Reg(
    "zero",
)
	successes: 0
	local rejects: 0
	global rejects: 0

thread 'backend::riscv::assembler::encoder::encode_seqz_pbt::test_encode_seqz_regression_extra_operand' panicked at src/backend/riscv/assembler/encoder/encode_seqz_pbt.rs:259:5:
seqz zero, zero with a third operand must be rejected (llvm-mc rejects; README documents `seqz rd, rs`); got Ok(Word(1060883))
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/encode_seqz_pbt.rs
