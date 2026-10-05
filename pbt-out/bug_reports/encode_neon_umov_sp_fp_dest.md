# Bug: encode_neon_umov encodes SP/WSP and FP names as the UMOV GPR dest
**Law:** ARM UMOV dest is Wd/Xd/WZR/XZR; encode_neon_umov([sp|wsp|dN|sN|qN|vN|hN|bN, Vn.Ts[i]]) = Err
**Impact:** `umov sp, v0.b[0]` is encoded as UMOV XZR with Q=1 (invalid pairing); `umov wsp, v0.b[0]` as UMOV WZR; `umov d0, v0.b[0]` as UMOV W0. A mistyped dest silently becomes a different register.
**Function:** encode_neon_umov
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:461
**Detected by:** Negative/Error Contract
**Minimal input:** encode_neon_umov([sp, v0.b[0]])
**Expected:** Err (llvm-mc: invalid operand for instruction)
**Actual:** Ok(Word(0x4e013c1f)) for SP; Ok(Word(0x0e013c1f)) for WSP; Ok(Word(0x0e013c00)) for d0
**Severity:** medium
**Root cause:** neon.rs:465 calls get_reg, which uses parse_reg_num (sp/wsp → 31; d/s/q/v/h/b prefixes accepted) and is_64bit_reg (SP is 64-bit). encode_neon_umov never restricts dest to W/X/WZR/XZR.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:465`
```rust
    let (rd, is_64) = get_reg(operands, 0)?;
```
**Suggested fix:** After get_reg, reject SP/WSP and FP/SIMD dest names.
```rust
    let (rd, is_64) = get_reg(operands, 0)?;
    let dest = match operands.get(0) {
        Some(Operand::Reg(name)) => name,
        _ => return Err("umov: expected GPR dest".into()),
    };
    let lower = dest.to_lowercase();
    if lower == "sp" || lower == "wsp" || matches!(lower.chars().next(), Some('d' | 's' | 'q' | 'v' | 'h' | 'b')) {
        return Err(format!("umov dest must be a W/X register, got {}", dest));
    }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_umov_regression_sp_as_zr -- --test-threads=1
```
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_neon_umov_pbt::test_encode_neon_umov_regression_sp_as_zr' panicked at src/backend/arm/assembler/encoder/encode_neon_umov_pbt.rs:615:9:
umov sp, v0.b[0] must Err (llvm-mc rejects SP as UMOV dest)
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_neon_umov_pbt.rs
