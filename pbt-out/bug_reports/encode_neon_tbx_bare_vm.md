# Bug: encode_neon_tbx encodes a bare GPR as Vm
**Law:** TBX Vm must be Vm.Ta; llvm-mc rejects `tbx v0.8b, {v0.16b}, x0`, so encode_neon_tbx with Operand::Reg Vm = Err
**Impact:** `tbx v0.8b, {v0.16b}, x0` is assembled as `tbx v0.8b, {v0.16b}, v0.8b` because parse_reg_num maps x0→0.
**Function:** encode_neon_tbx
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:803
**Detected by:** Negative/Error Contract
**Minimal input:** encode_neon_tbx([v0.8b, {v0.16b}, Reg("x0")])
**Expected:** Err (llvm-mc rejects `tbx v0.8b, {v0.16b}, x0`)
**Actual:** Ok(Word(0x0e001000))
**Severity:** medium
**Root cause:** neon.rs:822 calls get_neon_reg, which accepts Operand::Reg for Vm.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:822`
```rust
    let (rm, _) = get_neon_reg(operands, 2)?;
```
**Suggested fix:** Require Operand::RegArrangement for Vm with Ta matching Vd.
```rust
    let (rm, arr_m) = match &operands[2] {
        Operand::RegArrangement { reg, arrangement } => {
            (parse_reg_num(reg).ok_or("invalid NEON register")?, arrangement.clone())
        }
        other => return Err(format!("tbx: Vm must be Vm.Ta, got {:?}", other)),
    };
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_tbx_regression_bare_vm -- --test-threads=1
```
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_neon_tbx_pbt::test_encode_neon_tbx_regression_bare_vm' panicked at src/backend/arm/assembler/encoder/encode_neon_tbx_pbt.rs:681:5:
tbx v0.8b, {v0.16b}, x0 must Err (GPR Vm is not Vm.Ta)
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_neon_tbx_pbt.rs
