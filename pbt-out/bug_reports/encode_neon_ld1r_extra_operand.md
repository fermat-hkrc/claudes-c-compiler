# Bug: encode_neon_ld1r ignores a surplus operand
**Law:** A surplus operand after `ld1r {Vt.T}, [Xn]` that llvm-mc/gas reject must make encode_neon_ld1r return Err.
**Impact:** The assembler silently drops trailing junk and emits a no-offset LD1R, so `ld1r {v0.8b}, [x0], eq` assembles as `ld1r {v0.8b}, [x0]` instead of failing. Callers that pass an extra operand get a wrong instruction with no diagnostic.
**Function:** encode_neon_ld1r
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:832
**Detected by:** Negative/Error Contract
**Minimal input:** encode_neon_ld1r([RegList({v0.8b}), Mem{x0, 0}, Cond("eq")])
**Expected:** Err
**Actual:** Ok(Word(0x0d40c000)) — encoding of `ld1r {v0.8b}, [x0]`
**Severity:** medium
**Root cause:** neon.rs:834 checks `operands.len() < 2` only, so a third Cond/Shift/Label/arrangement is ignored and the no-offset arm still encodes.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:834`
```rust
    if operands.len() < 2 {
        return Err("ld1r requires 2 operands".to_string());
    }
```
**Suggested fix:** Reject a surplus operand that is not a register post-index Xm.
```rust
    if operands.len() < 2 {
        return Err("ld1r requires 2 operands".to_string());
    }
    if operands.len() > 2 && !matches!(operands.get(2), Some(Operand::Reg(_))) {
        return Err("ld1r: unexpected extra operand".to_string());
    }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_ld1r_regression_extra_operand -- --test-threads=1
```
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::neon::encode_neon_ld1r_pbt::encode_neon_ld1r_neg_extra' panicked at src/backend/arm/assembler/encoder/neon.rs:12939:5:
Test failed: ld1r extra operand must Err (llvm-mc/gas reject a surplus operand) at src/backend/arm/assembler/encoder/neon.rs:13085.
minimal failing input: t = "8b", rt = 0, rn = 0, extra_kind = 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_neon_ld1r_pbt.rs
