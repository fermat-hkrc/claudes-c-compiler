# Bug: encode_neon_pmul encodes GPR and bare-V operands as NEON registers
**Law:** PMUL operands must be NEON arrangement registers Vd.T, Vn.T, Vm.T; GPR, scalar, and bare V names must be rejected
**Impact:** `pmul x0, v0.8b, v0.8b` and `pmul v0.8b, v0, v0.8b` assemble as if x0/v0 were v0.8b, so a wrong register class or missing arrangement is silently encoded
**Function:** encode_neon_pmul
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:336
**Detected by:** Negative/Error Contract
**Minimal input:** encode_neon_pmul([Reg("x0"), v0.8b, v0.8b])
**Expected:** Err (gas/llvm-mc require Vd.T / Vn.T / Vm.T)
**Actual:** Ok(Word(0x2e209c00)) — parse_reg_num("x0")=0, encoded as v0.8b
**Severity:** medium
**Root cause:** get_neon_reg (neon.rs:14-17) accepts Operand::Reg and parse_reg_num (mod.rs:194) accepts x/w/d/s/q/v/h/b prefixes, so GPR dest, bare V, and xN.8b arrangement all encode as V-register numbers
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:337`
```rust
    let (rd, arr_d) = get_neon_reg(operands, 0)?;
    let (rn, _) = get_neon_reg(operands, 1)?;
    let (rm, _) = get_neon_reg(operands, 2)?;
```
**Suggested fix:** Require Operand::RegArrangement whose register name starts with v/V, and a non-empty arrangement
```rust
    let (rd, arr_d) = get_neon_reg(operands, 0)?;
    if !operands[0].is_v_arrangement() {
        return Err("pmul destination must be Vd.T".to_string());
    }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_pmul_regression_gpr_dest -- --test-threads=1
```
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_neon_pmul_pbt::encode_neon_pmul_neg_gpr_or_bare' panicked at src/backend/arm/assembler/encoder/encode_neon_pmul_pbt.rs:337:1:
Test failed: GPR/bare/non-arrangement kind=0 must Err (llvm-mc rejects pmul x0, v0.8b, v0.8b) at src/backend/arm/assembler/encoder/encode_neon_pmul_pbt.rs:398.
minimal failing input: rd = 0, rn = 0, rm = 0, kind = 0, fp_prefix = "x"
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_neon_pmul_pbt.rs
