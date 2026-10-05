# Bug: encode_neon_add_sub encodes bare V and GPR operands as NEON registers
**Law:** Vector ADD/SUB require Vd.T, Vn.T, Vm.T; a bare V register, GPR, or xN.T dest must be rejected
**Impact:** The assembler silently encodes `add v0.8b, v0, v0.8b` as `add v0.8b, v0.8b, v0.8b` and `add x0.8b, v0.8b, v0.8b` as `add v0.8b, v0.8b, v0.8b`, so invalid assembly becomes the wrong NEON instruction
**Function:** encode_neon_add_sub
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:1164
**Detected by:** Negative/Error Contract
**Minimal input:** encode_neon_add_sub([v0.8b, Reg("v0"), v0.8b], is_sub=false)
**Expected:** Err (llvm-mc/gas require Vn.T)
**Actual:** Ok(Word(0x0e208400)) — same as `add v0.8b, v0.8b, v0.8b`
**Severity:** medium
**Root cause:** get_neon_reg accepts Operand::Reg (empty arrangement, discarded for sources) and parse_reg_num maps x/w/d/s/q/v/h/b to 0–31, so a bare V source or x0.8b dest encodes as v0
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:14`
```rust
        Some(Operand::Reg(name)) => {
            let num = parse_reg_num(name)
                .ok_or_else(|| format!("invalid register: {}", name))?;
            Ok((num, String::new()))
        }
```
**Suggested fix:** Require RegArrangement with a V register on every operand
```rust
        Some(Operand::RegArrangement { reg, arrangement }) => {
            if !reg.to_lowercase().starts_with('v') {
                return Err(format!("expected NEON V register, got {reg}"));
            }
            let num = parse_reg_num(reg)
                .ok_or_else(|| format!("invalid NEON register: {}", reg))?;
            Ok((num, arrangement.clone()))
        }
        other => Err(format!("expected NEON register at operand {}, got {:?}", idx, other)),
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_add_sub_regression_bare_src -- --test-threads=1 --exact
```
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_neon_add_sub_pbt::encode_neon_add_sub_neg_gpr_or_bare' (2332187) panicked at src/backend/arm/assembler/encoder/encode_neon_add_sub_pbt.rs:221:1:
Test failed: GPR/bare/non-arrangement kind=1 must Err (llvm-mc rejects add v0.8b, v0, v0.8b) at src/backend/arm/assembler/encoder/encode_neon_add_sub_pbt.rs:434.
minimal failing input: rd = 0, rn = 0, rm = 0, is_sub = false, kind = 1, fp_prefix = "x"
	successes: 0
	local rejects: 0
	global rejects: 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_neon_add_sub_pbt.rs
