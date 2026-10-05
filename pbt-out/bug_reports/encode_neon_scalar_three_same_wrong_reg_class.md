# Bug: encode_neon_scalar_three_same encodes non-D sources as Dd
**Law:** Integer scalar ADD/SUB is ADD/SUB Dd, Dn, Dm; a non-D source (s/h/b/x/w/q/v/sp/xzr) must be rejected
**Impact:** Invalid assembly such as `add d0, s0, d0` is assembled into a scalar ADD word (Rn taken from the S register number) instead of an error. encode() routes here whenever dest is a d-register, so mixed-class sources are caller-reachable
**Function:** encode_neon_scalar_three_same
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:1791
**Detected by:** Negative/Error Contract
**Minimal input:** encode_neon_scalar_three_same([d0, s0, d0], u_bit=0, opcode=0b10000, size=0b11)
**Expected:** Err (non-D source)
**Actual:** Ok(Word(0x5ee08400)) encoding as if `add d0, d0, d0`
**Severity:** medium
**Root cause:** neon.rs:1794 extracts Rn via parse_reg_num, which accepts s/h/b/x/w/q/v/sp prefixes and never checks that the register is a D register required by size=11 ADD/SUB
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:1794`
```rust
    let rn = match &operands[1] { Operand::Reg(r) => parse_reg_num(r).ok_or("invalid reg")?, _ => return Err("expected register".to_string()) };
```
**Suggested fix:** Require a D-register prefix on every operand before packing
```rust
    if !r.to_lowercase().starts_with('d') {
        return Err(format!("scalar three-same requires Dd, Dn, Dm, got {r}"));
    }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_neon_scalar_three_same -- --test-threads=1
cargo test --lib encode_neon_scalar_three_same_neg_wrong_reg_class -- --test-threads=1
```
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_neon_scalar_three_same_pbt::encode_neon_scalar_three_same_neg_wrong_reg_class' (2481271) panicked at src/backend/arm/assembler/encoder/encode_neon_scalar_three_same_pbt.rs:182:1:
Test failed: non-D source slot=1 pfx=s must Err (llvm-mc rejects add d0, s0, d0) at src/backend/arm/assembler/encoder/encode_neon_scalar_three_same_pbt.rs:352.
minimal failing input: rd = 0, rn = 0, rm = 0, is_sub = false, slot = 1, pfx = "s"
	successes: 0
	local rejects: 0
	global rejects: 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_neon_scalar_three_same_pbt.rs
