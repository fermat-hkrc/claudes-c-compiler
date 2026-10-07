# Bug: encode_vmv_v_i ignores trailing v0.t
**Law:** ∀ vd ∈ {0..31}, simm ∈ {-16..15}. encode_vmv_v_i([Reg("v{vd}"), Imm(simm), Symbol("v0.t")]) is Err
**Impact:** RISC-V V 1.0 vmv.v.i is unmasked-only (vm=1). llvm-mc `-triple=riscv64 -mattr=+v` reports `invalid operand for instruction` on `vmv.v.i v0, 0, v0.t`. The SUT encodes the two-operand unmasked form, so a mask token is silently dropped.
**Function:** encode_vmv_v_i
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/vector.rs:172
**Detected by:** Negative/Error Contract — trailing v0.t
**Minimal input:** encode_vmv_v_i([Reg("v0"), Imm(-16), Symbol("v0.t")])
**Expected:** Err (llvm-mc rejects mask on vmv.v.i; ISA has no masked form)
**Actual:** Ok(Word(0x5e083057)) — same encoding as `vmv.v.i v0, -16` (vm=1)
**Severity:** medium
**Root cause:** vector.rs:173-177 reads only operands 0 and 1 and hardcodes vm=1. Trailing Symbol("v0.t") is never inspected.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/vector.rs:173`
```rust
    let vd = get_vreg(operands, 0)?;
    let simm5 = get_imm(operands, 1)? as u32 & 0x1F;
    // funct6=010111, vm=1, vs2=0
    let word = (0b010111u32 << 26) | (1u32 << 25) | (simm5 << 15) | (0b011 << 12) | (vd << 7) | OP_V;
    Ok(EncodeResult::Word(word))
```
**Suggested fix:** Reject a trailing v0.t (and any third operand). vmv.v.i has no masked encoding.
```rust
    if operands.len() != 2 {
        return Err(format!("vmv.v.i expects 2 operands, got {}", operands.len()));
    }
    let vd = get_vreg(operands, 0)?;
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_vmv_v_i_regression_mask_v0t -- --test-threads=1
```
**Raw output:**
```text
thread 'backend::riscv::assembler::encoder::encode_vmv_v_i_pbt::encode_vmv_v_i_neg_mask_v0t' (2888384) panicked at src/backend/riscv/assembler/encoder/encode_vmv_v_i_pbt.rs:266:1:
Test failed: trailing v0.t must Err for vmv.v.i (llvm-mc rejects mask on vmv.v.i); got Ok(Word(1577594967)) at src/backend/riscv/assembler/encoder/encode_vmv_v_i_pbt.rs:382.
minimal failing input: vd = 0, simm = -16
	successes: 0
	local rejects: 0
	global rejects: 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/encode_vmv_v_i_pbt.rs
