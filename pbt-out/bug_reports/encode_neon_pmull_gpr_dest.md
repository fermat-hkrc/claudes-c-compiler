# Bug: encode_neon_pmull encodes a GPR destination as a NEON register
**Law:** PMULL/PMULL2 destinations must be V registers with an arrangement (Vd.1Q or Vd.8H); a GPR dest must be rejected
**Impact:** `pmull x0, v0.1d, v0.1d` is encoded as `pmull v0.1q, v0.1d, v0.1d` (Rd taken from parse_reg_num on `x0`), so invalid assembly becomes a well-formed NEON instruction
**Function:** encode_neon_pmull
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:1128
**Detected by:** Negative/Error Contract
**Minimal input:** encode_neon_pmull([Reg("x0"), v0.1d, v0.1d], is_pmull2=false)
**Expected:** Err (llvm-mc/gas require Vd.1Q or Vd.8H)
**Actual:** Ok(Word(0x0ee0e000)) — same as `pmull v0.1q, v0.1d, v0.1d`
**Severity:** medium
**Root cause:** neon.rs:1132 calls get_neon_reg, which accepts Operand::Reg and maps x/w/d/s/q/v/h/b via parse_reg_num; encode_neon_pmull never checks that the dest is a V register with a PMULL arrangement
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:1132`
```rust
    let (rd, _) = get_neon_reg(operands, 0)?;
```
**Suggested fix:** Require Operand::RegArrangement with a V prefix and a legal Ta
```rust
    let (rd, arr_d) = get_neon_reg(operands, 0)?;
    if arr_d.is_empty() || !matches!(operands[0], Operand::RegArrangement { ref reg, .. } if reg.starts_with('v') || reg.starts_with('V')) {
        return Err("pmull dest must be a V register with arrangement".to_string());
    }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_pmull_regression_gpr_dest -- --test-threads=1 --exact
```
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_neon_pmull_pbt::encode_neon_pmull_neg_gpr_or_bare' (2327741) panicked at src/backend/arm/assembler/encoder/encode_neon_pmull_pbt.rs:210:1:
Test failed: GPR/bare/non-arrangement kind=0 must Err (llvm-mc rejects pmull x0, v0.1d, v0.1d) at src/backend/arm/assembler/encoder/encode_neon_pmull_pbt.rs:461.
minimal failing input: rd = 0, rn = 0, rm = 0, is_pmull2 = false, kind = 0, fp_prefix = "x"
	successes: 0
	local rejects: 0
	global rejects: 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_neon_pmull_pbt.rs
