# Bug: encode_neon_ins accepts the wrong GPR width for INS (general)
**Law:** ARM INS (general) takes Wn for Ts in {B,H,S} and Xn for Ts=D; llvm-mc rejects the swapped width, so encode_neon_ins([Vd.ts[i], wrong_width(R)]) = Err
**Impact:** `ins v0.b[0], x0` is assembled as `ins v0.b[0], w0` because parse_reg_num strips the prefix; a 64-bit source on a byte insert (or a 32-bit source on a doubleword insert) silently becomes the other width.
**Function:** encode_neon_ins
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:549
**Detected by:** Negative/Error Contract
**Minimal input:** encode_neon_ins([v0.b[0], x0])
**Expected:** Err (llvm-mc: invalid operand for instruction)
**Actual:** Ok(Word(0x4e011c00)) — encoded as `ins v0.b[0], w0`
**Severity:** medium
**Root cause:** neon.rs:555-557 takes Operand::Reg and calls parse_reg_num, which accepts both xN and wN as the same number; no width check against Ts.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:555`
```rust
        (Operand::RegLane { reg, elem_size, index }, Operand::Reg(rn_name)) => {
            let rd = parse_reg_num(reg).ok_or("invalid NEON register")?;
            let rn = parse_reg_num(rn_name).ok_or("invalid register")?;
```
**Suggested fix:** Require Wn for B/H/S and Xn for D (and reject SP).
```rust
        (Operand::RegLane { reg, elem_size, index }, Operand::Reg(rn_name)) => {
            let rd = parse_reg_num(reg).ok_or("invalid NEON register")?;
            let want_x = elem_size == "d";
            let is_x = rn_name.eq_ignore_ascii_case("xzr")
                || rn_name.eq_ignore_ascii_case("lr")
                || rn_name.to_ascii_lowercase().starts_with('x');
            let is_w = rn_name.eq_ignore_ascii_case("wzr")
                || rn_name.to_ascii_lowercase().starts_with('w');
            if want_x && !is_x || !want_x && !is_w {
                return Err(format!("ins: GPR source {} has wrong width for .{}", rn_name, elem_size));
            }
            let rn = parse_reg_num(rn_name).ok_or("invalid register")?;
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_ins_regression_wrong_width_gpr -- --test-threads=1
```
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_neon_ins_pbt::test_encode_neon_ins_regression_wrong_width_gpr' panicked at src/backend/arm/assembler/encoder/encode_neon_ins_pbt.rs:607:5:
ins v0.b[0], x1 must Err (llvm-mc requires Wn for Ts=B)
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_neon_ins_pbt.rs
