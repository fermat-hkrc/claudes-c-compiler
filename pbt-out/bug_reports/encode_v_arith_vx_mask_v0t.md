# Bug: encode_v_arith_vx drops v0.t and encodes unmasked
**Law:** ∀ vd ∈ {1..31}, vs2, rs1 ∈ {0..31}, (mnem, funct6) ∈ opivx_family with not (slide mnemonic and vd = vs2). encode_v_arith_vx([Reg(v{vd}), Reg(v{vs2}), Reg(x{rs1}), Symbol("v0.t")], funct6) = llvm-mc(mnem v{vd}, v{vs2}, x{rs1}, v0.t)
**Impact:** A trailing `v0.t` mask token is ignored, so a masked OPIVX instruction is silently encoded as unmasked (vm=1). Inactive elements that should be left undisturbed are overwritten. llvm-mc `-triple=riscv64 -mattr=+v` encodes `vadd.vx v1, v0, x0, v0.t` as 0x000040d7 (vm=0); the SUT emits 0x020040d7 (vm=1).
**Function:** encode_v_arith_vx
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/vector.rs:134
**Detected by:** Differential — llvm-mc masked OPIVX
**Minimal input:** encode_v_arith_vx([Reg("v1"), Reg("v0"), Reg("x0"), Symbol("v0.t")], funct6=0b000000)
**Expected:** Ok(Word(0x000040d7)) — llvm-mc `vadd.vx v1, v0, x0, v0.t`
**Actual:** Ok(Word(0x020040d7)) — same as unmasked `vadd.vx v1, v0, x0`
**Severity:** low (documented by the author)
**Root cause:** vector.rs:138 hardcodes `vm = 1` and never inspects operand 3, so a v0.t mask token cannot clear bit 25.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/vector.rs:138`
```rust
    let vm: u32 = 1;
```
**Suggested fix:** Accept an optional fourth operand `v0.t` and set vm=0; reject any other extra operand.
```rust
    let vm: u32 = match operands.get(3) {
        None => 1,
        Some(Operand::Symbol(s)) | Some(Operand::Reg(s)) if s == "v0.t" => 0,
        Some(other) => return Err(format!("operand 3 must be v0.t, got {:?}", other)),
    };
    if operands.len() > 4 {
        return Err(format!("OPIVX expects 3 or 4 operands, got {}", operands.len()));
    }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_v_arith_vx_regression_mask_v0t -- --test-threads=1
```
**Raw output:**
```text
thread 'backend::riscv::assembler::encoder::encode_v_arith_vx_pbt::encode_v_arith_vx_mask_v0t' (2880287) panicked at src/backend/riscv/assembler/encoder/encode_v_arith_vx_pbt.rs:317:1:
Test failed: assertion failed: `(left == right)` 
  left: `33571031`, 
 right: `16599`: SUT 0x020040d7 != llvm-mc 0x000040d7 for masked vadd.vx v1, v0, x0, v0.t at src/backend/riscv/assembler/encoder/encode_v_arith_vx_pbt.rs:502.
minimal failing input: vd = 1, vs2 = 0, rs1 = 0, kind = 0
	successes: 0
	local rejects: 0
	global rejects: 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/encode_v_arith_vx_pbt.rs
