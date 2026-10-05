# Bug: encode_neon_ext encodes a GPR or bare-V dest as 8B EXT
**Law:** EXT dest/sources must be NEON registers with arrangement Vn.T; encode_neon_ext([Reg(xN|wN|dN|vN), …]) = Err
**Impact:** `ext x0, v1.16b, v2.16b, #3` is assembled as `ext v0.8b, v1.8b, v2.8b, #3` (empty arrangement ⇒ Q=0, parse_reg_num maps x0→0). A GPR/FP/bare-V typo silently becomes a different vector extract.
**Function:** encode_neon_ext
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:405
**Detected by:** Negative/Error Contract
**Minimal input:** encode_neon_ext([x0, v0.8b, v0.8b, #0])
**Expected:** Err (llvm-mc/gas reject GPR dest; gas: "operand 1 must be an SVE vector register" / invalid operand)
**Actual:** Ok(Word(0x2e000000)) — encoded as `ext v0.8b, v0.8b, v0.8b, #0`
**Severity:** medium
**Root cause:** get_neon_reg accepts Operand::Reg and returns an empty arrangement; encode_neon_ext then treats any non-"16b" arrangement as Q=0. parse_reg_num maps x/w/d/s/q/v/h/b prefixes onto 0–31.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:14`
```rust
        Some(Operand::Reg(name)) => {
            let num = parse_reg_num(name)
                .ok_or_else(|| format!("invalid register: {}", name))?;
            Ok((num, String::new()))
        }
```
**Suggested fix:** Reject Operand::Reg in EXT (require RegArrangement on Vd/Vn/Vm).
```rust
        Some(Operand::Reg(name)) => {
            Err(format!("expected NEON register with arrangement, got {}", name))
        }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_ext_regression_gpr_dest -- --test-threads=1
```
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_neon_ext_pbt::test_encode_neon_ext_regression_gpr_dest' panicked at src/backend/arm/assembler/encoder/encode_neon_ext_pbt.rs:597:5:
ext x0, v1.16b, v2.16b, #3 must Err (gas/llvm-mc reject GPR dest)
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_neon_ext_pbt.rs
