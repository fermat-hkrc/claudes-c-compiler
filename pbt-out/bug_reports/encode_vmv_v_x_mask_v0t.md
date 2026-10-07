# Bug: encode_vmv_v_x ignores trailing v0.t
**Law:** ∀ vd, rs1 ∈ 0..31. encode_vmv_v_x([Reg(v{vd}), Reg(x{rs1}), Symbol("v0.t")]) is Err
**Impact:** A trailing `v0.t` is not rejected. RISC-V V 1.0 defines vmv.v.x as the unmasked move (vm=1, vs2=0); llvm-mc `-triple=riscv64 -mattr=+v` reports `invalid operand for instruction` on `vmv.v.x v0, x0, v0.t`. Callers that pass a mask token get a valid-looking unmasked encoding instead of an assembler error.
**Function:** encode_vmv_v_x
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/vector.rs:163
**Detected by:** Negative/Error Contract — trailing v0.t
**Minimal input:** encode_vmv_v_x([Reg("v0"), Reg("x0"), Symbol("v0.t")])
**Expected:** Err (vmv.v.x is unmasked-only; llvm-mc rejects the mask token)
**Actual:** Ok(Word(0x5e004057)) — same encoding as `vmv.v.x v0, x0`
**Severity:** medium
**Root cause:** vector.rs:164-169 encode_vmv_v_x reads only operands 0 and 1 via get_vreg/get_reg, hardcodes vm=1, and never inspects a third operand, so a trailing v0.t is ignored.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/vector.rs:164`
```rust
    let vd = get_vreg(operands, 0)?;
    let rs1 = get_reg(operands, 1)?;
    // funct6=010111, vm=1, vs2=0
    let word = (0b010111u32 << 26) | (1u32 << 25) | (rs1 << 15) | (0b100 << 12) | (vd << 7) | OP_V;
    Ok(EncodeResult::Word(word))
```
**Suggested fix:** Reject any operand list that is not exactly two operands (vmv.v.x has no masked form).
```rust
    if operands.len() != 2 {
        return Err(format!("vmv.v.x expects 2 operands, got {}", operands.len()));
    }
    let vd = get_vreg(operands, 0)?;
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_vmv_v_x_regression_mask_v0t -- --test-threads=1
```
**Raw output:**
```text
thread 'backend::riscv::assembler::encoder::encode_vmv_v_x_pbt::encode_vmv_v_x_neg_mask_v0t' (2886142) panicked at src/backend/riscv/assembler/encoder/encode_vmv_v_x_pbt.rs:240:1:
Test failed: trailing v0.t must Err for vmv.v.x (llvm-mc rejects mask on vmv.v.x); got Ok(Word(1577074775)) at src/backend/riscv/assembler/encoder/encode_vmv_v_x_pbt.rs:372.
minimal failing input: vd = 0, rs1 = 0
	successes: 0
	local rejects: 0
	global rejects: 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/encode_vmv_v_x_pbt.rs
