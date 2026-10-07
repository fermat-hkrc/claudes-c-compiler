# Bug: encode_bgt silently accepts a fourth (and further) operand
**Law:** `bgt` is a three-operand pseudoinstruction (`rs, rt, label`); any trailing operand must be rejected with `Err`, matching llvm-mc and the README form.
**Impact:** Assembler typos such as `bgt a0, a1, foo, a2` assemble as if the extra operand were absent, masking mistakes in hand-written or generated assembly and diverging from llvm-mc / gas.
**Function:** encode_bgt
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/pseudo.rs:336
**Detected by:** Negative error contract (arity / extra operand) + deterministic regression
**Minimal input:** `encode_bgt(&[Reg("a0"), Reg("a1"), Symbol("foo"), Reg("a2")])` (also: `rs="zero", rt="zero", tgt="foo", extra=Reg("zero")`)
**Expected:** `Err(...)` (llvm-mc: `error: invalid operand for instruction`)
**Actual:** `Ok(WordWithReloc { word: 0x00a5c063, reloc: Branch "foo" addend 0 })` — encodes `blt a1, a0, foo` and ignores the fourth operand
**Severity:** medium
**Root cause:** `pseudo.rs:336-344` reads only indices 0, 1, and 2 via `get_reg` / `get_branch_target` and never checks `operands.len() == 3`, so trailing operands are discarded.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/pseudo.rs:336`
```rust
pub(crate) fn encode_bgt(operands: &[Operand]) -> Result<EncodeResult, String> {
    let rs1 = get_reg(operands, 0)?;
    let rs2 = get_reg(operands, 1)?;
    let label = get_branch_target(operands, 2)?;
    Ok(EncodeResult::WordWithReloc {
        word: encode_b(OP_BRANCH, 0b100, rs2, rs1, 0), // blt rs2, rs1
        reloc: Relocation { reloc_type: RelocType::Branch, symbol: label, addend: 0 },
    })
}
```
**Suggested fix:** Reject non-exact arity before decoding operands.
```rust
pub(crate) fn encode_bgt(operands: &[Operand]) -> Result<EncodeResult, String> {
    if operands.len() != 3 {
        return Err(format!("bgt: expected 3 operands, got {}", operands.len()));
    }
    let rs1 = get_reg(operands, 0)?;
    let rs2 = get_reg(operands, 1)?;
    let label = get_branch_target(operands, 2)?;
    Ok(EncodeResult::WordWithReloc {
        word: encode_b(OP_BRANCH, 0b100, rs2, rs1, 0), // blt rs2, rs1
        reloc: Relocation { reloc_type: RelocType::Branch, symbol: label, addend: 0 },
    })
}
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_bgt_regression_extra_operand -- --test-threads=1
cargo test --lib encode_bgt_neg_extra -- --test-threads=1
```
**Raw output:**
```text
thread 'backend::riscv::assembler::encoder::encode_bgt_pbt::encode_bgt_neg_extra' panicked at src/backend/riscv/assembler/encoder/encode_bgt_pbt.rs:348:1:
Test failed: extra operand must Err for bgt zero, zero, foo (llvm-mc rejects: true) at src/backend/riscv/assembler/encoder/encode_bgt_pbt.rs:516.
minimal failing input: rs = "zero", rt = "zero", tgt = "foo", extra = Reg(
    "zero",
)
	successes: 0
	local rejects: 0
	global rejects: 0

thread 'backend::riscv::assembler::encoder::encode_bgt_pbt::test_encode_bgt_regression_extra_operand' panicked at src/backend/riscv/assembler/encoder/encode_bgt_pbt.rs:341:5:
bgt a0, a1, foo with a fourth operand must be rejected (llvm-mc rejects; README documents `bgt rs, rt, label`); got Ok(WordWithReloc { word: 10862691, reloc: Relocation { reloc_type: Branch, symbol: "foo", addend: 0 } })
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/encode_bgt_pbt.rs (`test_encode_bgt_regression_extra_operand`)
