# Bug: encode_vmv_v_i truncates out-of-range immediates
**Law:** ∀ vd ∈ {0..31}, imm ∈ ℤ \ {-16..15}. encode_vmv_v_i([Reg("v{vd}"), Imm(imm)]) is Err
**Impact:** An immediate outside the RISC-V V 1.0 signed simm5 range is silently truncated with `as u32 & 0x1F` and encoded as a different in-range value. llvm-mc `-triple=riscv64 -mattr=+v` rejects `vmv.v.i v0, 16` ("immediate must be an integer in the range [-16, 15]"); the SUT emits 0x5e083057, which is `vmv.v.i v0, -16`. The same wrap turns `vmv.v.i v1, -17` into `vmv.v.i v1, 15`.
**Function:** encode_vmv_v_i
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/vector.rs:172
**Detected by:** Negative/Error Contract — immediate out of range
**Minimal input:** encode_vmv_v_i([Reg("v0"), Imm(16)])
**Expected:** Err (llvm-mc range [-16, 15] for vmv.v.i simm5)
**Actual:** Ok(Word(0x5e083057)) — same encoding as `vmv.v.i v0, -16`
**Severity:** medium
**Root cause:** vector.rs:174 packs `get_imm(operands, 1)? as u32 & 0x1F` with no range check, so values outside the signed 5-bit field wrap.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/vector.rs:174`
```rust
    let simm5 = get_imm(operands, 1)? as u32 & 0x1F;
```
**Suggested fix:** Reject immediates outside the signed 5-bit range [-16, 15] before packing.
```rust
    let imm = get_imm(operands, 1)?;
    if imm < -16 || imm > 15 {
        return Err(format!("simm5 out of range: {}", imm));
    }
    let simm5 = (imm as u32) & 0x1F;
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_vmv_v_i_regression_simm_oob_16 -- --test-threads=1
```
**Raw output:**
```text
thread 'backend::riscv::assembler::encoder::encode_vmv_v_i_pbt::encode_vmv_v_i_neg_imm_oob' (2888383) panicked at src/backend/riscv/assembler/encoder/encode_vmv_v_i_pbt.rs:266:1:
Test failed: vmv.v.i v0, 16 must Err (llvm-mc immediate out of range [-16, 15]); got Ok(Word(1577594967)) at src/backend/riscv/assembler/encoder/encode_vmv_v_i_pbt.rs:392.
minimal failing input: vd = 0, imm = 16
	successes: 0
	local rejects: 0
	global rejects: 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/encode_vmv_v_i_pbt.rs
