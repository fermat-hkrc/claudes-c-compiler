# Bug: encode_neon_mul accepts a bare Vn / GPR dest as a NEON register
**Law:** Vector MUL operands must be Vd.T, Vn.T, Vm.T; a bare register or GPR-prefixed arrangement must be rejected
**Impact:** `mul v0.8b, v0, v0.8b` and `mul x0.8b, v0.8b, v0.8b` assemble as `mul v0.8b, v0.8b, v0.8b`, so missing arrangement or a GPR name is silently treated as a vector register
**Function:** encode_neon_mul
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:323
**Detected by:** Negative/Error Contract
**Minimal input:** encode_neon_mul([v0.8b, Reg("v0"), v0.8b])
**Expected:** Err (llvm-mc / gas require Vn.T)
**Actual:** Ok(Word(0x0e209c00)); kind=4 `mul x0.8b, v0.8b, v0.8b` also encodes as v0.8b
**Severity:** medium
**Root cause:** get_neon_reg accepts Operand::Reg (empty arrangement) and parse_reg_num accepts x/w prefixes; encode_neon_mul discards source arrangements, so a bare Vn or xN.T dest still produces a Word
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:14`
```rust
        Some(Operand::Reg(name)) => {
            let num = parse_reg_num(name)
                .ok_or_else(|| format!("invalid register: {}", name))?;
            Ok((num, String::new()))
        }
```
**Suggested fix:** Require a non-empty V-prefixed arrangement on every operand
```rust
        Some(Operand::Reg(_)) => {
            Err(format!("expected NEON register with arrangement at operand {idx}"))
        }
```
and reject non-`v` prefixes on RegArrangement.

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_mul_regression_bare_src -- --test-threads=1
```
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_neon_mul_pbt::encode_neon_mul_neg_gpr_or_bare' panicked at src/backend/arm/assembler/encoder/encode_neon_mul_pbt.rs:355:1:
Test failed: GPR/bare/non-arrangement kind=1 must Err (llvm-mc rejects mul v0.8b, v0, v0.8b) at src/backend/arm/assembler/encoder/encode_neon_mul_pbt.rs:416.
minimal failing input: rd = 0, rn = 0, rm = 0, kind = 1, fp_prefix = "x"
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_neon_mul_pbt.rs
