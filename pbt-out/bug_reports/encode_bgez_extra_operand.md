# Bug: encode_bgez silently ignores extra operands
**Law:** ∀ rs ∈ GPR, ∀ tgt ∈ LabelIdents, ∀ extra. encode_bgez([Reg(rs), Symbol(tgt), extra]) is Err
**Impact:** A third (or later) operand is dropped with no diagnostic, so `bgez a0, foo, a1` encodes as `bge a0, x0, foo`. Typos and extra commas assemble without error and emit a valid branch relocation.
**Function:** encode_bgez
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/pseudo.rs:309
**Detected by:** Negative/Error Contract — extra operand
**Minimal input:** encode_bgez([Reg("a0"), Symbol("foo"), Reg("a1")])
**Expected:** Err (llvm-mc: "invalid operand for instruction"; README documents `blez/bgez/...` → corresponding `bge`/`blt` with x0 — two operands)
**Actual:** Ok(WordWithReloc { word: 0x00055063, reloc: Relocation { reloc_type: Branch, symbol: "foo", addend: 0 } }) — encodes as if the extra operand were absent.
**Severity:** medium
**Root cause:** `pseudo.rs:310-311` reads operand 0 as rs1 and operand 1 as the branch target and never checks `operands.len()`, so extra operands are ignored.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/pseudo.rs:310`
```rust
    let rs1 = get_reg(operands, 0)?;
    let label = get_branch_target(operands, 1)?;
```
**Suggested fix:** Reject any operand list whose length is not exactly 2.
```rust
    if operands.len() != 2 {
        return Err(format!("bgez: expected 2 operands, got {}", operands.len()));
    }
    let rs1 = get_reg(operands, 0)?;
    let label = get_branch_target(operands, 1)?;
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_bgez_regression_extra_operand -- --test-threads=1
```
**Raw output:**
```text
thread 'backend::riscv::assembler::encoder::encode_bgez_pbt::encode_bgez_neg_extra' panicked at src/backend/riscv/assembler/encoder/encode_bgez_pbt.rs:336:1:
Test failed: extra operand must Err for bgez zero, foo (llvm-mc rejects: true) at src/backend/riscv/assembler/encoder/encode_bgez_pbt.rs:481.
minimal failing input: rs = "zero", tgt = "foo", extra = Reg(
    "zero",
)
	successes: 0
	local rejects: 0
	global rejects: 0

thread 'backend::riscv::assembler::encoder::encode_bgez_pbt::test_encode_bgez_regression_extra_operand' panicked at src/backend/riscv/assembler/encoder/encode_bgez_pbt.rs:329:5:
bgez a0, foo with a third operand must be rejected (llvm-mc rejects; README documents `bgez rs, label`); got Ok(WordWithReloc { word: 348259, reloc: Relocation { reloc_type: Branch, symbol: "foo", addend: 0 } })
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/encode_bgez_pbt.rs
