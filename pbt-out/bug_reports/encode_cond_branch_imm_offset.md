# Bug: encode_cond_branch rejects a valid immediate PC offset
**Law:** For every ARM condition name and every 4-byte-aligned offset in ±1 MiB, `encode_cond_branch(cond, [Imm(imm)])` must return `Word(w)` equal to llvm-mc `b.{cond} #imm`.
**Impact:** The assembler cannot encode `b.eq #0` / `b.ne #4` (and the rest of the 16 conditions). GNU as and llvm-mc accept that form; compiler output or hand-written `.s` that uses an immediate displacement fails to assemble.
**Function:** encode_cond_branch
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/compare_branch.rs:197
**Detected by:** Differential — llvm-mc AArch64 assembler (immediate form)
**Minimal input:** `encode_cond_branch("eq", [Operand::Imm(-1048576)])`
**Expected:** `Ok(EncodeResult::Word(0x54800000))` matching llvm-mc `b.eq #-1048576`
**Actual:** `Err("expected symbol at operand 0, got Some(Imm(-1048576))")`
**Severity:** high
**Root cause:** compare_branch.rs:199 always calls `get_symbol`, which has no `Operand::Imm` arm, so every immediate form is rejected instead of being encoded as `01010100 imm19 0 cond` with `imm19 = imm/4`.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/compare_branch.rs:199`
```rust
    let (sym, addend) = get_symbol(operands, 0)?;
```
**Suggested fix:** Handle `Operand::Imm` before `get_symbol`: require a 4-byte-aligned offset in `[-1048576, 1048572]`, then emit `Word((0b01010100 << 24) | (((imm as i32 >> 2) as u32 & 0x7ffff) << 5) | cond_val)`.
```rust
    if let Some(Operand::Imm(imm)) = operands.get(0) {
        if operands.len() != 1 {
            return Err(format!("b.{}: extra operand", cond));
        }
        if imm % 4 != 0 || *imm < -1_048_576 || *imm > 1_048_572 {
            return Err(format!("b.{} offset {} unaligned or out of range", cond, imm));
        }
        let imm19 = ((*imm as i32) >> 2) as u32 & 0x7ffff;
        return Ok(EncodeResult::Word((0b01010100 << 24) | (imm19 << 5) | cond_val));
    }
    let (sym, addend) = get_symbol(operands, 0)?;
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_cond_branch_diff_imm_llvm_mc -- --test-threads=1
```
**Raw output:**
```text
Test failed: SUT rejected valid B.cond b.eq #-1048576: Err("expected symbol at operand 0, got Some(Imm(-1048576))").
minimal failing input: cond = "eq", imm = -1048576
	successes: 0
	local rejects: 0
	global rejects: 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_cond_branch_pbt.rs (test_encode_cond_branch_regression_imm_offset)
