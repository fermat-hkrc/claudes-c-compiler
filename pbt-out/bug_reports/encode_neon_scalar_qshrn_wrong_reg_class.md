# Bug: encode_neon_scalar_qshrn ignores dest/src scalar class pairing
**Law:** Scalar SQSHRN dest/src must be B<-H, H<-S, or S<-D; any other pair must be rejected
**Impact:** Invalid assembly such as `sqshrn b0, s0, #8` is assembled instead of an error. ARM/gas/llvm-mc require the narrowing pair; a mismatched source still yields a Word, so the GNU-style assembler emits machine code that gas/llvm-mc refuse. encode() passes Operand::Reg dest/src through unchanged.
**Function:** encode_neon_scalar_qshrn
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:1835
**Detected by:** Negative/Error Contract
**Minimal input:** encode_neon_scalar_qshrn([Reg("b0"), Reg("s0"), Imm(8)], u_bit=0, is_rounding=false)
**Expected:** Err (wrong source class for dest B; mandated source is H)
**Actual:** Ok(Word) using dest prefix B and Rn from s0
**Severity:** medium
**Root cause:** neon.rs:1838 accepts any Operand::Reg for the source; only dest prefix selects element size, so `sqshrn b0, s0, #8` is encoded as if the source were H
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:1838`
```rust
    let rn = match &operands[1] { Operand::Reg(r) => parse_reg_num(r).ok_or("invalid reg")?, _ => return Err("expected register".to_string()) };
```
**Suggested fix:** Require the source prefix to match the dest narrowing pair (B<-H, H<-S, S<-D)
```rust
    let (rn, rn_name) = match &operands[1] { Operand::Reg(r) => (parse_reg_num(r).ok_or("invalid reg")?, r.to_lowercase()), _ => return Err("expected register".to_string()) };
    let want_src = match rd_name.chars().next() {
        Some('b') => 'h',
        Some('h') => 's',
        Some('s') => 'd',
        _ => return Err(format!("scalar qshrn: unsupported dest: {}", rd_name)),
    };
    if !rn_name.starts_with(want_src) {
        return Err(format!("scalar qshrn: source {} incompatible with dest {}", rn_name, rd_name));
    }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_neon_scalar_qshrn_neg_wrong_reg_class -- --test-threads=1
cargo test --lib test_encode_neon_scalar_qshrn_regression_wrong_reg_class -- --test-threads=1
```
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_neon_scalar_qshrn_pbt::encode_neon_scalar_qshrn_neg_wrong_reg_class' (2508299) panicked at src/backend/arm/assembler/encoder/encode_neon_scalar_qshrn_pbt.rs:207:1:
Test failed: wrong class src=s must Err (llvm-mc rejects sqshrn b0, s0, #8) at src/backend/arm/assembler/encoder/encode_neon_scalar_qshrn_pbt.rs:394.
minimal failing input: rd = 0, rn = 0, vd = "b", src = "s", u = 0, round = false
	successes: 0
	local rejects: 0
	global rejects: 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_neon_scalar_qshrn_pbt.rs
