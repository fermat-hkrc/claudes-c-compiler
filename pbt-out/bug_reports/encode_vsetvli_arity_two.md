# Bug: encode_vsetvli accepts two-operand form (missing vtypei)
**Law:** ∀ ops with len ≤ 2. encode_vsetvli(ops) = Err(_)
**Impact:** `vsetvli x0, x0` (no vtypei) encodes as `vsetvli x0, x0, e8, m1, tu, mu` (word 0x00007057) instead of being rejected. llvm-mc `-triple=riscv64 -mattr=+v` reports `too few operands for instruction`. Accidental omission of the vtype fields silently selects the all-zero vtype rather than failing the assemble.
**Function:** encode_vsetvli
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/vector.rs:48
**Detected by:** Negative/Error Contract — arity
**Minimal input:** encode_vsetvli([Reg("x0"), Reg("x0")])
**Expected:** Err (vsetvli requires rd, rs1, and a vtypei; llvm-mc: too few operands)
**Actual:** Ok(Word(0x7057)) — the encoding of `vsetvli x0, x0, e8, m1, tu, mu`
**Severity:** medium
**Root cause:** vector.rs:49-51 reads rd/rs1 via get_reg and then parse_vtypei(operands, 2). When operands.len()==2 the parse loop is empty and returns Ok(0) (sew=0, lmul=0, ta=0, ma=0). encode_vsetvli never checks that a vtypei was supplied.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/vector.rs:49`
```rust
    let rd = get_reg(operands, 0)?;
    let rs1 = get_reg(operands, 1)?;
    let vtypei = parse_vtypei(operands, 2)?;
```
**Suggested fix:** Require a vtypei operand (named fields or a raw immediate) before packing.
```rust
    let rd = get_reg(operands, 0)?;
    let rs1 = get_reg(operands, 1)?;
    if operands.len() < 3 {
        return Err(format!("vsetvli: expected rd, rs1, vtypei, got {} operands", operands.len()));
    }
    let vtypei = parse_vtypei(operands, 2)?;
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_vsetvli_regression_arity_two -- --test-threads=1
```
**Raw output:**
```text
thread 'backend::riscv::assembler::encoder::encode_vsetvli_pbt::encode_vsetvli_neg_arity_fp' panicked at src/backend/riscv/assembler/encoder/encode_vsetvli_pbt.rs:287:1:
Test failed: arity 2 must Err (llvm-mc too few operands); got Ok(Word(28759)) at src/backend/riscv/assembler/encoder/encode_vsetvli_pbt.rs:402.
minimal failing input: ops = [
    Reg(
        "x0",
    ),
    Reg(
        "x0",
    ),
], fp = "f0"
	successes: 6
	local rejects: 0
	global rejects: 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/encode_vsetvli_pbt.rs
