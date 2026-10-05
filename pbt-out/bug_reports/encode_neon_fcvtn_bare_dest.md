# Bug: encode_neon_fcvtn encodes a bare V dest as FCVTN Vd.4H
**Law:** FCVTN dest must be a NEON V register with arrangement (Vd.Tb); gas/llvm-mc reject `fcvtn v0, v0.4s`
**Impact:** A dest without arrangement (or with a GPR prefix) is encoded as the corresponding V register, so a mistyped `fcvtn v0, v0.4s` silently becomes `fcvtn v0.4h, v0.4s`
**Function:** encode_neon_fcvtn
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:1652
**Detected by:** Negative/Error Contract
**Minimal input:** encode_neon_fcvtn([Reg("v0"), v0.4s], is_high=false)
**Expected:** Err (llvm-mc: invalid operand)
**Actual:** Ok(Word(0x0e216800)) — get_neon_reg accepts Operand::Reg; dest arrangement is unused so a bare v0 is encoded as FCVTN Q=0 sz=0
**Severity:** medium
**Root cause:** neon.rs:1653 calls get_neon_reg which accepts Operand::Reg, then discards dest arrangement; encode_neon_fcvtn never requires a V-prefixed arrangement dest
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:1653`
```rust
    let (rd, _) = get_neon_reg(operands, 0)?;
```
**Suggested fix:** Reject a destination that is not a V-prefixed RegArrangement
```rust
    let (rd, arr_d) = get_neon_reg(operands, 0)?;
    match &operands[0] {
        Operand::RegArrangement { reg, .. } if reg.to_ascii_lowercase().starts_with('v') => {}
        _ => return Err("fcvtn: destination must be a V register with arrangement".to_string()),
    }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_neon_fcvtn_neg_gpr_or_bare -- --test-threads=1
```
**Raw output:**
```text
Test failed: GPR/bare/non-arrangement kind=2 must Err (llvm-mc rejects fcvtn v0, v0.4s) at src/backend/arm/assembler/encoder/encode_neon_fcvtn_pbt.rs:438.
minimal failing input: rd = 0, rn = 0, kind = 2, fp_prefix = "x"
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_neon_fcvtn_pbt.rs
