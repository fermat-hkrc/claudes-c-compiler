# Bug: encode_neon_bitwise_insert accepts GPR, SP, bare V, and FP scalars
**Law:** BIT/BIF require arranged NEON operands Vd.T, Vn.T, Vm.T; GPR, SP, bare V, and scalar FP must be Err
**Impact:** `bit x0, x0, x0` (and `bit sp, …`, `bit v0, v1, v2`, `bit d0, d1, d2`) encodes as 8B BIT, so a wrong register class is not diagnosed
**Function:** encode_neon_bitwise_insert
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:1667
**Detected by:** Negative/Error Contract
**Minimal input:** encode_neon_bitwise_insert([x0, x0, x0], size=0b10)
**Expected:** Err (llvm-mc/gas reject GPR operands)
**Actual:** Ok(Word) — get_neon_reg accepts Operand::Reg and empty arrangement yields Q=0
**Severity:** medium
**Root cause:** neon.rs:1671 calls get_neon_reg, which accepts Operand::Reg (x/w/v/d/s/q/sp); empty arrangement then takes the Q=0 arm at neon.rs:1674
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:1671`
```rust
    let (rd, arr_d) = get_neon_reg(operands, 0)?;
    let (rn, _) = get_neon_reg(operands, 1)?;
    let (rm, _) = get_neon_reg(operands, 2)?;
    let q: u32 = if arr_d == "16b" { 1 } else { 0 };
```
**Suggested fix:** Require RegArrangement with T in {8b,16b} on every operand
```rust
    let (rd, arr_d) = get_neon_reg(operands, 0)?;
    if arr_d != "8b" && arr_d != "16b" {
        return Err(format!("bit/bif: expected Vd.8b or Vd.16b, got {}", arr_d));
    }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_bitwise_insert_regression_gpr_dest -- --test-threads=1 --exact
```
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_neon_bitwise_insert_pbt::encode_neon_bitwise_insert_neg_gpr_bare_sp' panicked at src/backend/arm/assembler/encoder/encode_neon_bitwise_insert_pbt.rs:185:1:
Test failed: non-arranged NEON / GPR / SP / FP must Err (llvm-mc rejects bit x0, x0, x0) at src/backend/arm/assembler/encoder/encode_neon_bitwise_insert_pbt.rs:411.
minimal failing input: rd = 0, rn = 0, rm = 0, t = "8b", size = 2, kind = 0
	successes: 0
	local rejects: 0
	global rejects: 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_neon_bitwise_insert_pbt.rs
