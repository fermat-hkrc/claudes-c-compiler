# Bug: encode_neon_faddp scalar form does not check dest size against source T
**Law:** Scalar FADDP is only FADDP Sd, Vn.2S or FADDP Dd, Vn.2D
**Impact:** `faddp d0, v0.2s` (and `faddp s0, v0.2d`, `faddp x0, v0.2s`) encodes as SISD FADDP using sz from the source arrangement and the dest register number, producing a different instruction than the text; gas/llvm-mc reject the same text
**Function:** encode_neon_faddp
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:1686
**Detected by:** Negative/Error Contract
**Minimal input:** encode_neon_faddp([Reg("d0"), v0.2s])
**Expected:** Err (dest must be S with .2S or D with .2D); llvm-mc rejects the same text
**Actual:** Ok(Word) encoding SISD FADDP with sz=0 (from .2s) and Rd=0, i.e. the encoding of `faddp s0, v0.2s`
**Severity:** medium
**Root cause:** neon.rs:1703-1712 accepts any Operand::Reg whose parse_reg_num succeeds (s/d/x/w/v/q/h/b/sp) and sets sz only from the source arrangement, never matching dest prefix to T
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:1704`
```rust
            Operand::Reg(r) => parse_reg_num(r).ok_or("invalid dest reg")?,
```
**Suggested fix:** Require dest prefix s with source .2s, or dest prefix d with source .2d
```rust
        let dest = match &operands[0] {
            Operand::Reg(r) => r,
            _ => return Err("faddp scalar: expected register".to_string()),
        };
        let rd = parse_reg_num(dest).ok_or("invalid dest reg")?;
        let prefix = dest.chars().next().unwrap_or(' ').to_ascii_lowercase();
        let (rn, arr_n) = get_neon_reg(operands, 1)?;
        let sz = match (prefix, arr_n.as_str()) {
            ('s', "2s") => 0u32,
            ('d', "2d") => 1,
            _ => return Err(format!("faddp scalar: dest {dest} incompatible with source {arr_n}")),
        };
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_neon_faddp_neg_gpr_bare_scalar_dest -- --test-threads=1
```
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_neon_faddp_pbt::encode_neon_faddp_neg_gpr_bare_scalar_dest' (2471482) panicked at src/backend/arm/assembler/encoder/encode_neon_faddp_pbt.rs:217:1:
Test failed: non-arranged NEON / GPR / SP / dest-size mismatch must Err (llvm-mc rejects faddp d0, v0.2s) at src/backend/arm/assembler/encoder/encode_neon_faddp_pbt.rs:453.
minimal failing input: rd = 0, rn = 0, rm = 0, kind = 4
	successes: 0
	local rejects: 0
	global rejects: 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_neon_faddp_pbt.rs
