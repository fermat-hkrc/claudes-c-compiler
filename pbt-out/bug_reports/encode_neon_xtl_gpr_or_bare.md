# Bug: encode_neon_xtl encodes GPR or bare-V destinations as SIMD V registers
**Law:** UXTL/SXTL operands must be arranged SIMD registers Vd.Ta, Vn.Tb; GPR, scalar FP, SP, and bare V without arrangement must be rejected
**Impact:** `uxtl x0, v0.8b` and `uxtl v0, v0.8b` assemble to the same word as `uxtl v0.8h, v0.8b`, so a wrong register class in the .s file becomes silent wrong SIMD code
**Function:** encode_neon_xtl
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:163
**Detected by:** Negative/Error Contract
**Minimal input:** encode_neon_xtl([Reg("x0"), v0.8b], u_bit=1, is_high=false)  (asm `uxtl x0, v0.8b`)
**Expected:** Err
**Actual:** Ok(Word) — x0 is parsed as register number 0 and dest arrangement is discarded
**Severity:** medium
**Root cause:** neon.rs:167 calls get_neon_reg, whose Operand::Reg arm (neon.rs:14) accepts x/w/d/s/q/v/h/b prefixes via parse_reg_num, and encode_neon_xtl then ignores dest arrangement
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:14`
```rust
        Some(Operand::Reg(name)) => {
            let num = parse_reg_num(name)
                .ok_or_else(|| format!("invalid register: {}", name))?;
            Ok((num, String::new()))
        }
```
**Suggested fix:** Require RegArrangement with a V-prefixed register at both dest and source
```rust
        Some(Operand::RegArrangement { reg, arrangement }) if reg.to_lowercase().starts_with('v') => {
            let num = parse_reg_num(reg)
                .ok_or_else(|| format!("invalid NEON register: {}", reg))?;
            Ok((num, arrangement.clone()))
        }
        other => Err(format!("expected NEON register at operand {}, got {:?}", idx, other)),
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_neon_xtl -- --test-threads=1
cargo test --lib test_encode_neon_xtl_regression_gpr_dest -- --test-threads=1
```
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_neon_xtl_pbt::encode_neon_xtl_neg_gpr_or_bare' panicked at src/backend/arm/assembler/encoder/encode_neon_xtl_pbt.rs:422:1:
Test failed: GPR/bare/non-arrangement kind=0 must Err (llvm-mc rejects uxtl x0, v0.8b) at src/backend/arm/assembler/encoder/encode_neon_xtl_pbt.rs:477.
minimal failing input: rd = 0, rn = 0, kind = 0, fp_prefix = "x"
	successes: 0
	local rejects: 0
	global rejects: 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_neon_xtl_pbt.rs
