# Bug: encode_neon_tbx encodes a GPR/FP dest as Vd
**Law:** TBX dest must be Vd.Ta; llvm-mc rejects `tbx x0, {v0.16b}, v0.8b`, so encode_neon_tbx([Reg("x0"), ...]) = Err
**Impact:** `tbx x0, {v0.16b}, v0.8b` is assembled as `tbx v0.8b, {v0.16b}, v0.8b` because parse_reg_num maps x0→0, so a GPR dest silently becomes a NEON encoding.
**Function:** encode_neon_tbx
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:803
**Detected by:** Negative/Error Contract
**Minimal input:** encode_neon_tbx([Reg("x0"), {v0.16b}, v0.8b])
**Expected:** Err (llvm-mc rejects `tbx x0, {v0.16b}, v0.8b`)
**Actual:** Ok(Word(0x0e001000))
**Severity:** medium
**Root cause:** neon.rs:807 calls get_neon_reg, which accepts Operand::Reg and parse_reg_num maps x/w/d/s/q/h/b prefixes onto 0..31.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:807`
```rust
    let (rd, arr_d) = get_neon_reg(operands, 0)?;
```
**Suggested fix:** Require Operand::RegArrangement with a v-prefix dest and Ta in {8b,16b}.
```rust
    let (rd, arr_d) = match &operands[0] {
        Operand::RegArrangement { reg, arrangement } if reg.starts_with('v') || reg.starts_with('V') => {
            (parse_reg_num(reg).ok_or("invalid NEON register")?, arrangement.clone())
        }
        other => return Err(format!("tbx: dest must be Vd.Ta, got {:?}", other)),
    };
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_tbx_regression_gpr_dest -- --test-threads=1
```
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_neon_tbx_pbt::test_encode_neon_tbx_regression_gpr_dest' panicked at src/backend/arm/assembler/encoder/encode_neon_tbx_pbt.rs:653:5:
tbx x0, {v0.16b}, v0.8b must Err (GPR dest is not Vd.Ta)
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_neon_tbx_pbt.rs
