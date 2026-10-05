# Bug: encode_neon_scalar_two_misc encodes mismatched dest/src SIMD class
**Law:** Scalar SQABS/SQNEG is `<V><d>, <V><n>` with the same B/H/S/D class on both operands; a mismatched or non-SIMD class must be rejected
**Impact:** Invalid assembly such as `sqabs h0, b0` is assembled as SQABS H0, H0 (source class ignored, size taken only from dest). `sqabs sp, s0` is assembled as SQABS S31, S0 because dest `sp` matches `starts_with('s')`. encode() does not inspect register class beyond dest not being RegArrangement, so both witnesses are caller-reachable.
**Function:** encode_neon_scalar_two_misc
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:1819
**Detected by:** Negative/Error Contract
**Minimal input:** encode_neon_scalar_two_misc([Reg("h0"), Reg("b0")], u_bit=0, opcode=0b00111); also encode_neon_scalar_two_misc([Reg("sp"), Reg("s0")], 0, 0b00111)
**Expected:** Err (mismatched / non-SIMD register class)
**Actual:** Ok(Word(0x5e607800)) encoding as if `sqabs h0, h0`
**Severity:** medium
**Root cause:** neon.rs:1822 extracts Rn via parse_reg_num with no class check; neon.rs:1823-1827 derives size from dest `starts_with('b'|'h'|'s'|'d')`, so `sp` is treated as Sd and a B source with an H dest is packed with size=H
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:1822`
```rust
    let rn = match &operands[1] { Operand::Reg(r) => parse_reg_num(r).ok_or("invalid reg")?, _ => return Err("expected register".to_string()) };
    let size = if rd_name.starts_with('b') { 0b00u32 }
        else if rd_name.starts_with('h') { 0b01 }
        else if rd_name.starts_with('s') { 0b10 }
        else if rd_name.starts_with('d') { 0b11 }
        else { return Err(format!("scalar two-misc: unsupported register type: {}", rd_name)); };
```
**Suggested fix:** Require matching B/H/S/D prefixes on dest and source; reject aliases such as sp
```rust
    let rn_name = match &operands[1] {
        Operand::Reg(r) => r.to_lowercase(),
        _ => return Err("expected register".to_string()),
    };
    let size = match &rd_name[..1] {
        "b" | "h" | "s" | "d" if rd_name.len() > 1 && rd_name[1..].parse::<u32>().ok().map_or(false, |n| n <= 31)
            && rn_name.starts_with(&rd_name[..1]) => match &rd_name[..1] {
            "b" => 0b00u32, "h" => 0b01, "s" => 0b10, _ => 0b11,
        },
        _ => return Err(format!("scalar two-misc: unsupported register type: {}", rd_name)),
    };
    let rn = parse_reg_num(&rn_name).ok_or("invalid reg")?;
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_neon_scalar_two_misc -- --test-threads=1
cargo test --lib encode_neon_scalar_two_misc_neg_wrong_reg_class -- --test-threads=1
```
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_neon_scalar_two_misc_pbt::encode_neon_scalar_two_misc_neg_wrong_reg_class' (2503869) panicked at src/backend/arm/assembler/encoder/encode_neon_scalar_two_misc_pbt.rs:199:1:
Test failed: wrong class slot=0 bad=h must Err (llvm-mc rejects sqabs h0, b0) at src/backend/arm/assembler/encoder/encode_neon_scalar_two_misc_pbt.rs:379.
minimal failing input: rd = 0, rn = 0, dest_pfx = "b", bad = "h", is_neg = false, slot = 0
	successes: 0
	local rejects: 0
	global rejects: 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_neon_scalar_two_misc_pbt.rs
