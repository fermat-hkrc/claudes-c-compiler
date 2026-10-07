# Bug: encode_bnez silently ignores extra operands

**Law:** `bnez` is a two-operand pseudo-instruction (`rs`, label). A third (or further) operand must return `Err`, matching llvm-mc and the README expansion form.
**Impact:** A typo or accidental extra operand is silently dropped; `bnez a0, foo, a1` assembles as `bnez a0, foo`, so invalid assembly is accepted and the intended third operand never affects the encoding.
**Function:** encode_bnez
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/pseudo.rs:291
**Detected by:** Negative/Error Contract (arity) — differential agreement with llvm-mc rejection
**Minimal input:** `encode_bnez([Reg("a0"), Symbol("foo"), Reg("a1")])`
**Expected:** `Err(...)`
**Actual:** `Ok(WordWithReloc { word: 0x00051063, reloc: Branch/"foo"/0 })`
**Severity:** medium
**Root cause:** `pseudo.rs:292-293` only reads operands 0 and 1 via `get_reg` / `get_branch_target` and never checks `operands.len() == 2`, so trailing operands are ignored.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/pseudo.rs:291`
```rust
pub(crate) fn encode_bnez(operands: &[Operand]) -> Result<EncodeResult, String> {
    let rs1 = get_reg(operands, 0)?;
    let label = get_branch_target(operands, 1)?;
    Ok(EncodeResult::WordWithReloc {
        word: encode_b(OP_BRANCH, 0b001, rs1, 0, 0),
        reloc: Relocation { reloc_type: RelocType::Branch, symbol: label, addend: 0 },
    })
}
```
**Suggested fix:** Reject any operand list whose length is not exactly 2 before encoding.
```rust
pub(crate) fn encode_bnez(operands: &[Operand]) -> Result<EncodeResult, String> {
    if operands.len() != 2 {
        return Err(format!("bnez: expected 2 operands, got {}", operands.len()));
    }
    let rs1 = get_reg(operands, 0)?;
    let label = get_branch_target(operands, 1)?;
    Ok(EncodeResult::WordWithReloc {
        word: encode_b(OP_BRANCH, 0b001, rs1, 0, 0),
        reloc: Relocation { reloc_type: RelocType::Branch, symbol: label, addend: 0 },
    })
}
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_bnez_regression -- --test-threads=1
```
**Raw output:**
```text
thread 'backend::riscv::assembler::encoder::encode_bnez_pbt::test_encode_bnez_regression_extra_operand' panicked at src/backend/riscv/assembler/encoder/encode_bnez_pbt.rs:319:5:
bnez a0, foo with a third operand must be rejected (llvm-mc rejects; README documents `bnez rs, label`); got Ok(WordWithReloc { word: 331875, reloc: Relocation { reloc_type: Branch, symbol: "foo", addend: 0 } })
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/encode_bnez_pbt.rs (test_encode_bnez_regression_extra_operand)
