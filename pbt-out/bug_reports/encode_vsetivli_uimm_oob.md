# Bug: encode_vsetivli silently truncates AVL outside 0..=31
**Law:** ∀ rd ∈ GPR names, uimm ∉ 0..=31, sew, lmul, ta, ma. encode_vsetivli([Reg(rd), Imm(uimm), Symbol(sew), Symbol(lmul), Symbol(ta), Symbol(ma)]) = Err(_)
**Impact:** An AVL immediate outside the 5-bit uimm field is wrapped with `& 0x1F` and encoded. `vsetivli x0, -1, e8, m1, tu, mu` becomes the encoding of AVL=31 (0xc00ff057); `vsetivli x0, 32, ...` becomes AVL=0. llvm-mc rejects both (`immediate must be an integer in the range [0, 31]`). Wrong VL is programmed with no assembler error.
**Function:** encode_vsetivli
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/vector.rs:59
**Detected by:** Negative/Error Contract — AVL out of range
**Minimal input:** encode_vsetivli([Reg("x0"), Imm(-1), Symbol("e8"), Symbol("m1"), Symbol("tu"), Symbol("mu")])
**Expected:** Err (llvm-mc: immediate must be an integer in the range [0, 31])
**Actual:** Ok(Word(0xc00ff057)) — AVL -1 truncated to 31
**Severity:** medium
**Root cause:** vector.rs:61 masks the immediate instead of range-checking: `get_imm(operands, 1)? as u32 & 0x1F`. Negative values become 31; 32 becomes 0.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/vector.rs:61`
```rust
    let uimm = get_imm(operands, 1)? as u32 & 0x1F;
```
**Suggested fix:** Reject AVL outside the documented 5-bit field.
```rust
    let avl = get_imm(operands, 1)?;
    if avl < 0 || avl > 31 {
        return Err(format!("vsetivli: AVL {avl} out of range [0, 31]"));
    }
    let uimm = avl as u32;
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_vsetivli_regression_uimm_oob -- --test-threads=1
```
**Raw output:**
```text
thread 'backend::riscv::assembler::encoder::encode_vsetivli_pbt::encode_vsetivli_neg_uimm_oob' panicked at src/backend/riscv/assembler/encoder/encode_vsetivli_pbt.rs:298:1:
Test failed: AVL -1 outside 0..=31 must Err (llvm-mc: immediate must be in [0, 31]); got Ok(Word(3222270039)) at src/backend/riscv/assembler/encoder/encode_vsetivli_pbt.rs:480.
minimal failing input: rd = "x0", uimm = -1, sew = "e8", lmul = "m1", ta = "tu", ma = "mu"
	successes: 0
	local rejects: 0
	global rejects: 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/encode_vsetivli_pbt.rs
