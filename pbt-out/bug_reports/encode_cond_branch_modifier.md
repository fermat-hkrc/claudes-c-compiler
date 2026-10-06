# Bug: encode_cond_branch accepts :lo12: modifiers as branch targets
**Law:** For every ARM condition name, `encode_cond_branch(cond, [Modifier{kind: "lo12", symbol: "foo"}])` and the `ModifierOffset` form must return Err. GNU as and llvm-mc reject `b.eq :lo12:foo`.
**Impact:** A `:lo12:` (or `:got:`) modifier is treated as an ordinary symbol named `foo`, emitting CondBr19 against the wrong symbol and silently dropping the modifier kind. That is not a valid B.cond addressing mode.
**Function:** encode_cond_branch
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/compare_branch.rs:197
**Detected by:** Negative/Error Contract — modifier operand
**Minimal input:** `encode_cond_branch("eq", [Operand::Modifier { kind: "lo12", symbol: "foo" }])`
**Expected:** `Err(...)` (llvm-mc/gas reject `:lo12:` on B.cond)
**Actual:** `Ok(WordWithReloc { word: 0x54000000, reloc: CondBr19 symbol foo addend 0 })`
**Severity:** medium
**Root cause:** compare_branch.rs:199 calls `get_symbol`, whose Modifier/ModifierOffset arms (encoder/mod.rs:1137-1138) return the inner symbol and drop the modifier kind, so `b.eq :lo12:foo` is encoded as `b.eq foo`.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/compare_branch.rs:199`
```rust
    let (sym, addend) = get_symbol(operands, 0)?;
```
**Suggested fix:** Reject `Operand::Modifier` and `Operand::ModifierOffset` in `encode_cond_branch` before calling `get_symbol`.
```rust
    match operands.get(0) {
        Some(Operand::Modifier { .. }) | Some(Operand::ModifierOffset { .. }) => {
            return Err(format!("b.{} does not take a relocation modifier", cond));
        }
        _ => {}
    }
    let (sym, addend) = get_symbol(operands, 0)?;
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_cond_branch_neg_modifier -- --test-threads=1
```
**Raw output:**
```text
Test failed: b.eq :lo12:foo must Err (which=0); llvm-mc/gas reject modifiers at src/backend/arm/assembler/encoder/encode_cond_branch_pbt.rs:500.
minimal failing input: cond = "eq", which = 0
	successes: 0
	local rejects: 0
	global rejects: 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_cond_branch_pbt.rs (test_encode_cond_branch_regression_modifier)
