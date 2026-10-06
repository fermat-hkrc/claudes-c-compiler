# Bug: encode_vsetvli ignores or last-wins extra operands
**Law:** ∀ rd, rs1, sew, lmul, ta, ma, extra. encode_vsetvli([Reg(rd), Reg(rs1), Symbol(sew), Symbol(lmul), Symbol(ta), Symbol(ma), extra]) = Err(_)
**Impact:** A seventh (or later) operand is not rejected. A repeated named vtype field last-wins (can silently change SEW/LMUL/ta/ma); an extra Imm is treated as a raw vtypei and discards previously parsed fields; Mem/Csr/Label/etc. are skipped. llvm-mc `-triple=riscv64 -mattr=+v` rejects extra tokens (`operand must be e[...],m[...],[ta|tu],[ma|mu]`). Callers that pass a stray token can get a valid-looking 32-bit word instead of an assembler error.
**Function:** encode_vsetvli
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/vector.rs:48
**Detected by:** Negative/Error Contract — extra operand
**Minimal input:** encode_vsetvli([Reg("x0"), Reg("x0"), Symbol("e8"), Symbol("m1"), Symbol("tu"), Symbol("mu"), Symbol("e8")])
**Expected:** Err (complete named vtype already present; llvm-mc rejects the extra field)
**Actual:** Ok(Word(0x7057)) — same encoding as `vsetvli x0, x0, e8, m1, tu, mu` (last-wins of e8)
**Severity:** medium
**Root cause:** encode_vsetvli does not check operand arity. parse_vtypei (vector.rs:14-39) walks every remaining operand: known names overwrite sew/lmul/ta/ma, Imm returns immediately as a raw 11-bit vtypei, and other kinds `continue` (skipped).
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/vector.rs:14`
```rust
    for i in start_idx..operands.len() {
        let name = match &operands[i] {
            Operand::Symbol(s) => s.to_lowercase(),
            Operand::Reg(s) => s.to_lowercase(),
            // Raw immediate: treat as pre-encoded vtypei value
            Operand::Imm(v) => return Ok(*v as u32 & 0x7FF),
            _ => continue,
        };
```
**Suggested fix:** After a complete named vtype or a raw immediate, reject further operands; do not skip unknown kinds.
```rust
    if operands.len() > 6 {
        return Err(format!("vsetvli: extra operand, got {}", operands.len()));
    }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_vsetvli_regression_extra_operand -- --test-threads=1
```
**Raw output:**
```text
thread 'backend::riscv::assembler::encoder::encode_vsetvli_pbt::encode_vsetvli_neg_extra' panicked at src/backend/riscv/assembler/encoder/encode_vsetvli_pbt.rs:287:1:
Test failed: extra operand Symbol("e8") must Err for vsetvli (llvm-mc rejects extra); got Ok(Word(28759)) at src/backend/riscv/assembler/encoder/encode_vsetvli_pbt.rs:436.
minimal failing input: rd = "x0", rs1 = "x0", sew = "e8", lmul = "m1", ta = "tu", ma = "mu", extra = Symbol(
    "e8",
)
	successes: 0
	local rejects: 0
	global rejects: 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/encode_vsetvli_pbt.rs
