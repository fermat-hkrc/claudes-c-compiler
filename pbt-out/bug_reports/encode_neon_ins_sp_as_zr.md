# Bug: encode_neon_ins encodes SP/WSP and FP names as the INS GPR source
**Law:** ARM INS (general) GPR source is Wn/Xn/WZR/XZR, not SP/WSP and not a SIMD/FP register; llvm-mc rejects those names, so encode_neon_ins([Vd.ts[i], SP|WSP|dN|sN|qN|vN]) = Err. Element-form Ts must also match.
**Impact:** `ins v0.b[0], sp` is assembled as `ins v0.b[0], wzr` because parse_reg_num maps SP to 31. The same path accepts `d1` as w1, and `ins v0.b[0], v1.h[0]` as a matching-B insert because `_src_size` is ignored.
**Function:** encode_neon_ins
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:549
**Detected by:** Negative/Error Contract
**Minimal input:** encode_neon_ins([v0.b[0], sp])
**Expected:** Err (llvm-mc: invalid operand for instruction)
**Actual:** Ok(Word(0x4e011fe0)) — encoded as `ins v0.b[0], wzr`
**Severity:** medium
**Root cause:** neon.rs:557 parse_reg_num("sp") yields 31 (XZR/WZR) and parse_reg_num accepts d/s/q/v/h/b prefixes; neon.rs:574 binds `_src_size` and never compares it to the destination Ts.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:555`
```rust
        (Operand::RegLane { reg, elem_size, index }, Operand::Reg(rn_name)) => {
            let rd = parse_reg_num(reg).ok_or("invalid NEON register")?;
            let rn = parse_reg_num(rn_name).ok_or("invalid register")?;
```
**Suggested fix:** Restrict the general-form source to W/X/WZR/XZR, and require matching Ts on the element form.
```rust
            let n = rn_name.to_ascii_lowercase();
            if n == "sp" || n == "wsp" || !n.starts_with('w') && !n.starts_with('x') && n != "xzr" && n != "wzr" && n != "lr" {
                return Err(format!("ins: GPR source must be Wn/Xn, got {}", rn_name));
            }
```
```rust
         (Operand::RegLane { reg: rd_name, elem_size: dst_size, index: dst_idx },
          Operand::RegLane { reg: rn_name, elem_size: src_size, index: src_idx }) => {
            if dst_size != src_size {
                return Err(format!("ins: element size mismatch {} vs {}", dst_size, src_size));
            }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_ins_regression_sp_as_zr -- --test-threads=1
```
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_neon_ins_pbt::test_encode_neon_ins_regression_sp_as_zr' panicked at src/backend/arm/assembler/encoder/encode_neon_ins_pbt.rs:632:5:
ins v0.b[0], sp must Err (llvm-mc rejects SP as INS GPR source)
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_neon_ins_pbt.rs
