# Bug: encode_snez silently ignores extra operands
**Law:** ∀ rd ∈ GPR, ∀ rs ∈ GPR, ∀ extra. encode_snez([Reg(rd), Reg(rs), extra]) is Err
**Impact:** A third (or later) operand is dropped with no diagnostic, so `snez a0, a1, a2` encodes as `sltu a0, x0, a1`. Typos and extra commas assemble without error.
**Function:** encode_snez
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/pseudo.rs:263
**Detected by:** Negative/Error Contract — extra operand
**Minimal input:** encode_snez([Reg("zero"), Reg("zero"), Reg("zero")])
**Expected:** Err (llvm-mc: "invalid operand for instruction"; README documents `snez rd, rs`)
**Actual:** Ok(Word(0x00003033)) — encodes as if the extra operand were absent (`sltu x0, x0, x0`).
**Severity:** medium
**Root cause:** `pseudo.rs:264-265` reads operand 0 as rd and operand 1 as rs2 and never checks `operands.len()`, so extra operands are ignored.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/pseudo.rs:264`
```rust
    let rd = get_reg(operands, 0)?;
    let rs2 = get_reg(operands, 1)?;
```
**Suggested fix:** Reject any operand list whose length is not exactly 2.
```rust
    if operands.len() != 2 {
        return Err(format!("snez: expected 2 operands, got {}", operands.len()));
    }
    let rd = get_reg(operands, 0)?;
    let rs2 = get_reg(operands, 1)?;
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_snez_regression_extra_operand -- --test-threads=1
```
**Raw output:**
```text
thread 'backend::riscv::assembler::encoder::encode_snez_pbt::encode_snez_neg_extra' panicked at src/backend/riscv/assembler/encoder/encode_snez_pbt.rs:269:1:
Test failed: extra operand must Err for snez zero, zero (llvm-mc rejects: true) at src/backend/riscv/assembler/encoder/encode_snez_pbt.rs:398.
minimal failing input: rd = "zero", rs = "zero", extra = Reg(
    "zero",
)
	successes: 0
	local rejects: 0
	global rejects: 0

thread 'backend::riscv::assembler::encoder::encode_snez_pbt::test_encode_snez_regression_extra_operand' panicked at src/backend/riscv/assembler/encoder/encode_snez_pbt.rs:262:5:
snez zero, zero with a third operand must be rejected (llvm-mc rejects; README documents `snez rd, rs`); got Ok(Word(12339))
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/encode_snez_pbt.rs
