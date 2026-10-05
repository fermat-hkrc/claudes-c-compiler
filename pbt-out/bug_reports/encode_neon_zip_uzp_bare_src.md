# Bug: encode_neon_zip_uzp encodes a bare V or GPR source
**Law:** ZIP/UZP/TRN sources must be arranged NEON registers (Vn.T, Vm.T); bare V, GPR, and FP scalar sources must be rejected
**Impact:** The assembler encodes `zip1 v0.8b, v0, v0.8b` as `zip1 v0.8b, v0.8b, v0.8b` (and likewise `xN` as Rm), so invalid operand class is silently accepted
**Function:** encode_neon_zip_uzp
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:1094
**Detected by:** Negative/Error Contract
**Minimal input:** encode_neon_zip_uzp([v0.8b, Reg("v0"), v0.8b], 0b011, false)
**Expected:** Err (llvm-mc/gas require Vn.T)
**Actual:** Ok(Word(0x0e003800)) — same as `zip1 v0.8b, v0.8b, v0.8b`
**Severity:** medium
**Root cause:** get_neon_reg accepts Operand::Reg and returns an empty arrangement; encode_neon_zip_uzp discards source arrangements, so a bare V or x-register source still encodes
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:14`
```rust
        Some(Operand::Reg(name)) => {
            let num = parse_reg_num(name)
                .ok_or_else(|| format!("invalid register: {}", name))?;
            Ok((num, String::new()))
        }
```
**Suggested fix:** Require RegArrangement on every ZIP/UZP/TRN operand (or reject empty source arrangement)
```rust
        Some(Operand::Reg(_)) => {
            Err(format!("expected NEON register with arrangement at operand {}", idx))
        }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_zip_uzp_regression_bare_src -- --test-threads=1 --exact
```
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_neon_zip_uzp_pbt::encode_neon_zip_uzp_neg_gpr_or_bare' panicked at src/backend/arm/assembler/encoder/encode_neon_zip_uzp_pbt.rs:187:1:
Test failed: GPR/bare/non-arrangement kind=1 must Err (llvm-mc rejects zip1 v0.8b, v0, v0.8b) at src/backend/arm/assembler/encoder/encode_neon_zip_uzp_pbt.rs:429.
minimal failing input: rd = 0, rn = 0, rm = 0, t = "8b", m = "zip1", kind = 1, fp_prefix = "x"
	successes: 0
	local rejects: 0
	global rejects: 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_neon_zip_uzp_pbt.rs
