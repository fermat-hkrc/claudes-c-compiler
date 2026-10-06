# Bug: encode_vsetivli accepts two operands and defaults vtypei to 0
**Law:** ∀ ops with |ops| ∈ {0,1,2} (missing vtypei). encode_vsetivli(ops) = Err(_)
**Impact:** `vsetivli rd, uimm` with no vtypei is assembled as `vsetivli rd, uimm, e8, m1, tu, mu` (vtypei=0) instead of being rejected. llvm-mc reports "too few operands for instruction". A truncated operand list silently produces a valid-looking 32-bit word with the wrong vector type.
**Function:** encode_vsetivli
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/vector.rs:59
**Detected by:** Negative/Error Contract — arity
**Minimal input:** encode_vsetivli([Reg("x0"), Imm(0)])
**Expected:** Err (llvm-mc: too few operands for instruction)
**Actual:** Ok(Word(0xc0007057)) — same encoding as `vsetivli x0, 0, e8, m1, tu, mu`
**Severity:** medium
**Root cause:** encode_vsetivli calls parse_vtypei(operands, 2) with no arity check. When operands.len()==2 the parse_vtypei loop does not run and returns the default packed vtypei 0 (sew=e8, lmul=m1, tu, mu).
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/vector.rs:62`
```rust
    let vtypei = parse_vtypei(operands, 2)?;
```
**Suggested fix:** Require a vtypei operand (named fields or a raw immediate) before encoding.
```rust
    if operands.len() < 3 {
        return Err("vsetivli: missing vtypei".into());
    }
    let vtypei = parse_vtypei(operands, 2)?;
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_vsetivli_regression_arity_two -- --test-threads=1
```
**Raw output:**
```text
thread 'backend::riscv::assembler::encoder::encode_vsetivli_pbt::encode_vsetivli_neg_arity_fp' panicked at src/backend/riscv/assembler/encoder/encode_vsetivli_pbt.rs:298:1:
Test failed: arity 2 must Err (llvm-mc too few operands); got Ok(Word(3221254231)) at src/backend/riscv/assembler/encoder/encode_vsetivli_pbt.rs:408.
minimal failing input: ops = [
    Reg(
        "x0",
    ),
    Imm(
        0,
    ),
], fp = "f0"
	successes: 0
	local rejects: 0
	global rejects: 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/encode_vsetivli_pbt.rs
