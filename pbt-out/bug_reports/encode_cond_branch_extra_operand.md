# Bug: encode_cond_branch ignores extra operands
**Law:** For every ARM condition name and every extra operand kind, `encode_cond_branch(cond, [Symbol(s), extra])` must return Err. GNU as and llvm-mc reject a second operand on `b.eq`.
**Impact:** A typo such as `b.eq foo, x0` is assembled as `b.eq foo` instead of being rejected, so invalid assembly is silently accepted and the extra operand is dropped.
**Function:** encode_cond_branch
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/compare_branch.rs:197
**Detected by:** Negative/Error Contract — extra operand
**Minimal input:** `encode_cond_branch("eq", [Operand::Symbol("labl0"), Operand::Reg("x0")])`
**Expected:** `Err(...)` (llvm-mc: invalid operand)
**Actual:** `Ok(WordWithReloc { word: 0x54000000, reloc: CondBr19 symbol labl0 addend 0 })`
**Severity:** medium
**Root cause:** compare_branch.rs:199-209 calls `get_symbol(operands, 0)` and returns success without checking `operands.len()`, so any operands after index 0 are ignored.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/compare_branch.rs:199`
```rust
    let (sym, addend) = get_symbol(operands, 0)?;
```
**Suggested fix:** Reject any operand list whose length is not exactly 1 before encoding.
```rust
    if operands.len() != 1 {
        return Err(format!("b.{}: expected 1 operand, got {}", cond, operands.len()));
    }
    let (sym, addend) = get_symbol(operands, 0)?;
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_cond_branch_neg_extra_operand -- --test-threads=1
```
**Raw output:**
```text
Test failed: b.eq label, extra (which=0) must Err (llvm-mc: invalid operand) at src/backend/arm/assembler/encoder/encode_cond_branch_pbt.rs:447.
minimal failing input: cond = "eq", suffix = 0, which = 0
	successes: 0
	local rejects: 0
	global rejects: 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_cond_branch_pbt.rs (test_encode_cond_branch_regression_extra_operand)
