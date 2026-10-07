# Bug: encode_vmv_v_v ignores trailing v0.t
**Law:** ∀ vd, vs1 ∈ 0..31. encode_vmv_v_v([Reg(v{vd}), Reg(v{vs1}), Symbol("v0.t")]) is Err
**Impact:** A trailing `v0.t` is not rejected. RISC-V V 1.0 defines vmv.v.v as the unmasked move (vm=1, vs2=0); llvm-mc `-triple=riscv64 -mattr=+v` reports `invalid operand for instruction` on `vmv.v.v v0, v0, v0.t`. Callers that pass a mask token get a valid-looking unmasked encoding instead of an assembler error.
**Function:** encode_vmv_v_v
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/vector.rs:154
**Detected by:** Negative/Error Contract — trailing v0.t
**Minimal input:** encode_vmv_v_v([Reg("v0"), Reg("v0"), Symbol("v0.t")])
**Expected:** Err (vmv.v.v is unmasked-only; llvm-mc rejects the mask token)
**Actual:** Ok(Word(0x5e000057)) — same encoding as `vmv.v.v v0, v0`
**Severity:** medium
**Root cause:** vector.rs:155-159 encode_vmv_v_v reads only operands 0 and 1 via get_vreg, hardcodes vm=1, and never inspects a third operand, so a trailing v0.t is ignored.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/vector.rs:155`
```rust
    let vd = get_vreg(operands, 0)?;
    let vs1 = get_vreg(operands, 1)?;
    // funct6=010111, vm=1, vs2=0, funct3=000 (OPIVV)
    let word = (0b010111u32 << 26) | (1u32 << 25) | (vs1 << 15) | (vd << 7) | OP_V;
    Ok(EncodeResult::Word(word))
```
**Suggested fix:** Reject any operand list that is not exactly two operands (vmv.v.v has no masked form).
```rust
    if operands.len() != 2 {
        return Err(format!("vmv.v.v expects 2 operands, got {}", operands.len()));
    }
    let vd = get_vreg(operands, 0)?;
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_vmv_v_v_regression_mask_v0t -- --test-threads=1
```
**Raw output:**
```text
thread 'backend::riscv::assembler::encoder::encode_vmv_v_v_pbt::encode_vmv_v_v_neg_mask_v0t' (2884228) panicked at src/backend/riscv/assembler/encoder/encode_vmv_v_v_pbt.rs:204:1:
Test failed: trailing v0.t must Err for vmv.v.v (llvm-mc rejects mask on vmv.v.v); got Ok(Word(1577058391)) at src/backend/riscv/assembler/encoder/encode_vmv_v_v_pbt.rs:314.
minimal failing input: vd = 0, vs1 = 0
	successes: 0
	local rejects: 0
	global rejects: 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/encode_vmv_v_v_pbt.rs
