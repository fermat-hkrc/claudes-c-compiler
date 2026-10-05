# Bug: encode_neon_scalar_addp encodes non-D dest and non-V source as scalar ADDP
**Law:** Integer scalar ADDP is ADDP Dd, Vn.2D; a non-D destination or a non-V source prefix must be rejected
**Impact:** Invalid assembly such as `addp s0, v0.2d` or `addp d0, x0.2d` is assembled into a scalar ADDP word (Rd/Rn taken from parse_reg_num) instead of an error. encode() requires dest to start with d/D, so a non-D dest is dispatcher-sanitized; a non-V source with .2d (`addp d0, x0.2d`) is caller-reachable because the dispatcher does not inspect the source prefix.
**Function:** encode_neon_scalar_addp
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:1802
**Detected by:** Negative/Error Contract
**Minimal input:** encode_neon_scalar_addp([Reg("s0"), RegArrangement { reg: "v0", arrangement: "2d" }]); also encode_neon_scalar_addp([Reg("d0"), RegArrangement { reg: "x0", arrangement: "2d" }])
**Expected:** Err (non-D dest / non-V source)
**Actual:** Ok(Word(0x5ef1b800)) encoding as if `addp d0, v0.2d`
**Severity:** medium
**Root cause:** neon.rs:1805 extracts Rd via parse_reg_num without requiring a D prefix (error text says "expected d register" only for non-Reg); neon.rs:1808 extracts Rn via parse_reg_num on the arrangement register without requiring a V prefix
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:1805`
```rust
    let rd = match &operands[0] { Operand::Reg(r) => parse_reg_num(r).ok_or("invalid reg")?, _ => return Err("expected d register".to_string()) };
```
**Suggested fix:** Require a D dest prefix and a V source prefix before packing
```rust
    let rd_name = r.to_lowercase();
    if !rd_name.starts_with('d') {
        return Err(format!("scalar addp requires Dd dest, got {r}"));
    }
    let rd = parse_reg_num(r).ok_or("invalid reg")?;
    // and for source:
    if !reg.to_lowercase().starts_with('v') {
        return Err(format!("scalar addp requires Vn.2d source, got {reg}"));
    }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_neon_scalar_addp -- --test-threads=1
cargo test --lib encode_neon_scalar_addp_neg_wrong_reg_class -- --test-threads=1
```
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_neon_scalar_addp_pbt::encode_neon_scalar_addp_neg_wrong_reg_class' (2494598) panicked at src/backend/arm/assembler/encoder/encode_neon_scalar_addp_pbt.rs:170:1:
Test failed: wrong register class which=0 must Err (llvm-mc rejects addp s0, v0.2d) at src/backend/arm/assembler/encoder/encode_neon_scalar_addp_pbt.rs:324.
minimal failing input: rd = 0, rn = 0, which = 0, dest_pfx = "s", src_pfx = "s"
	successes: 0
	local rejects: 0
	global rejects: 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_neon_scalar_addp_pbt.rs
