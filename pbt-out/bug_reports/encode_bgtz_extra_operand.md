# Bug: encode_bgtz silently ignores extra operands
**Law:** ∀ rs ∈ GPR, ∀ tgt ∈ LabelIdents, ∀ extra. encode_bgtz([Reg(rs), Symbol(tgt), extra]) is Err
**Impact:** A third (or later) operand is dropped with no diagnostic, so `bgtz a0, foo, a1` encodes as `blt x0, a0, foo`. Typos and extra commas assemble without error and emit a valid branch relocation.
**Function:** encode_bgtz
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/pseudo.rs:327
**Detected by:** Negative/Error Contract — extra operand
**Minimal input:** encode_bgtz([Reg("a0"), Symbol("foo"), Reg("a1")])
**Expected:** Err (llvm-mc: invalid operand; README documents `blez/bgez/...` → corresponding `bge`/`blt` with x0 — two operands)
**Actual:** Ok(WordWithReloc { word: 0x00a04063, reloc: Relocation { reloc_type: Branch, symbol: "foo", addend: 0 } }) — encodes as if the extra operand were absent.
**Severity:** medium
**Root cause:** `pseudo.rs:328-329` reads operand 0 as rs2 and operand 1 as the branch target and never checks `operands.len()`, so extra operands are ignored.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/pseudo.rs:328`
```rust
    let rs2 = get_reg(operands, 0)?;
    let label = get_branch_target(operands, 1)?;
```
**Suggested fix:** Reject any operand list whose length is not exactly 2.
```rust
    if operands.len() != 2 {
        return Err(format!("bgtz: expected 2 operands, got {}", operands.len()));
    }
    let rs2 = get_reg(operands, 0)?;
    let label = get_branch_target(operands, 1)?;
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_bgtz_regression_extra_operand -- --test-threads=1
```
**Raw output:**
```text
thread 'backend::riscv::assembler::encoder::encode_bgtz_pbt::encode_bgtz_neg_extra' panicked at src/backend/riscv/assembler/encoder/encode_bgtz_pbt.rs:333:1:
Test failed: extra operand must Err for bgtz zero, foo (llvm-mc rejects: true) at src/backend/riscv/assembler/encoder/encode_bgtz_pbt.rs:478.
minimal failing input: rs = "zero", tgt = "foo", extra = Reg(
    "zero",
)
	successes: 0
	local rejects: 0
	global rejects: 0

thread 'backend::riscv::assembler::encoder::encode_bgtz_pbt::test_encode_bgtz_regression_extra_operand' panicked at src/backend/riscv/assembler/encoder/encode_bgtz_pbt.rs:326:5:
bgtz a0, foo with a third operand must be rejected (llvm-mc rejects; README documents `bgtz rs, label`); got Ok(WordWithReloc { word: 10502243, reloc: Relocation { reloc_type: Branch, symbol: "foo", addend: 0 } })
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/encode_bgtz_pbt.rs
