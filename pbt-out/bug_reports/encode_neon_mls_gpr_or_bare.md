# Bug: encode_neon_mls accepts bare V / GPR as NEON MLS operands
**Law:** GNU-style vector MLS requires Vd.T, Vn.T, Vm.T; a bare `vN` or GPR `xN`/`xN.8b` must be rejected
**Impact:** `mls v0.8b, v0, v0.8b` and `mls x0.8b, v0.8b, v0.8b` assemble as vector MLS instead of failing, so a missing arrangement or wrong register class is silently encoded
**Function:** encode_neon_mls
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:361
**Detected by:** Negative/Error Contract
**Minimal input:** encode_neon_mls([v0.8b, Operand::Reg("v0"), v0.8b])
**Expected:** Err (gas/llvm-mc require Vn.T)
**Actual:** Ok(Word(0x2e209400)) — encoded as `mls v0.8b, v0.8b, v0.8b`
**Severity:** medium
**Root cause:** neon.rs:362-364 call get_neon_reg, which accepts Operand::Reg (empty arrangement) and parse_reg_num maps x/w/d/s/q/v/h/b prefixes to the same 0..31 index; dest with a real arrangement still encodes
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:363`
```rust
    let (rn, _) = get_neon_reg(operands, 1)?;
```
**Suggested fix:** Require RegArrangement with a V-prefixed register on every operand
```rust
    match operands.get(idx) {
        Some(Operand::RegArrangement { reg, arrangement }) if reg.to_lowercase().starts_with('v') => { /* parse */ }
        other => return Err(format!("expected NEON Vn.T at operand {idx}, got {:?}", other)),
    }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_mls_regression_bare_src -- --test-threads=1
```
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_neon_mls_pbt::encode_neon_mls_neg_gpr_or_bare' panicked at src/backend/arm/assembler/encoder/encode_neon_mls_pbt.rs:358:1:
Test failed: GPR/bare/non-arrangement kind=1 must Err (llvm-mc rejects mls v0.8b, v0, v0.8b) at src/backend/arm/assembler/encoder/encode_neon_mls_pbt.rs:419.
minimal failing input: rd = 0, rn = 0, rm = 0, kind = 1, fp_prefix = "x"
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_neon_mls_pbt.rs
