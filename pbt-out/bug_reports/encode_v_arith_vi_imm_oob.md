# Bug: encode_v_arith_vi truncates out-of-range immediates
**Law:** ∀ vd, vs2 ∈ 0..31, (mnem, funct6) ∈ signed_family, imm ∈ ℤ \ [-16,15]. encode_v_arith_vi([v{vd}, v{vs2}, Imm(imm)], funct6) is Err. ∀ (mnem, funct6) ∈ slide_family, imm ∈ ℤ \ [0,31]. encode_v_arith_vi(...) is Err.
**Impact:** An immediate outside the RISC-V V 1.0 simm5 / uimm5 range is silently truncated with `as u32 & 0x1F` and encoded as a different in-range value. llvm-mc `-triple=riscv64 -mattr=+v` rejects `vadd.vi v0, v0, -17` ("immediate must be an integer in the range [-16, 15]"); the SUT emits 0x0207b057, which is `vadd.vi v0, v0, 15`. The same wrap turns `vadd.vi ..., 16` into `-16` and `vslideup.vi ..., 32` into `0`.
**Function:** encode_v_arith_vi
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/vector.rs:144
**Detected by:** Negative/Error Contract — immediate out of range
**Minimal input:** encode_v_arith_vi([Reg("v0"), Reg("v0"), Imm(-17)], funct6=0b000000)
**Expected:** Err (llvm-mc range [-16, 15] for vadd.vi)
**Actual:** Ok(Word(0x0207b057)) — same encoding as `vadd.vi v0, v0, 15`
**Severity:** medium
**Root cause:** vector.rs:147 packs `get_imm(operands, 2)? as u32 & 0x1F` with no range check, so values outside the 5-bit field wrap.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/vector.rs:147`
```rust
    let simm5 = get_imm(operands, 2)? as u32 & 0x1F;
```
**Suggested fix:** Reject immediates outside the mnemonic's 5-bit range before packing. Signed OPIVI uses [-16, 15]; vslideup.vi / vslidedown.vi use [0, 31].
```rust
    let imm = get_imm(operands, 2)?;
    if imm < -16 || imm > 15 {
        return Err(format!("simm5 out of range: {}", imm));
    }
    let simm5 = (imm as u32) & 0x1F;
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_v_arith_vi_regression_simm_oob_m17 -- --test-threads=1
```
**Raw output:**
```text
thread 'backend::riscv::assembler::encoder::encode_v_arith_vi_pbt::encode_v_arith_vi_neg_imm_oob' (2882353) panicked at src/backend/riscv/assembler/encoder/encode_v_arith_vi_pbt.rs:356:1:
Test failed: vadd.vi v0, v0, -17 must Err (llvm-mc immediate out of range); got Ok(Word(34058327)) at src/backend/riscv/assembler/encoder/encode_v_arith_vi_pbt.rs:542.
minimal failing input: vd = 0, vs2 = 0, s_oob = -17, u_oob = -1, kind = 0
	successes: 0
	local rejects: 0
	global rejects: 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/encode_v_arith_vi_pbt.rs
