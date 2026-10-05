# Bug: encode_neon_ld_st_single accepts W/XZR/x31/FP as the memory base
**Law:** The memory base of LD/ST single-structure must be Xn or SP; W, XZR, x31, and FP/SIMD names must be rejected.
**Impact:** `st1 {v0.b}[0], [w0]` encodes as `[x0]`, and `[xzr]`/`[x31]` encode as SP, so invalid assembly becomes a different, silently wrong instruction.
**Function:** encode_neon_ld_st_single
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:904
**Detected by:** Negative/Error Contract
**Minimal input:** encode_neon_ld_st_single([RegListIndexed({v0.b}[0]), Mem{base:"w0", offset:0}], is_load=false, num_structs=1)
**Expected:** Err
**Actual:** Ok(Word) — parse_reg_num("w0") yields 0, encoded as X0
**Severity:** medium
**Root cause:** neon.rs:931 calls parse_reg_num on the base with no Xn|SP check; parse_reg_num accepts w/d/s/q/v/h/b and maps xzr/x31/sp to 31.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:931`
```rust
            let rn = parse_reg_num(base).ok_or_else(|| format!("invalid base register: {}", base))?;
```
**Suggested fix:** Require a 64-bit X register or SP, and reject xzr/x31.
```rust
            if !is_64bit_reg(base) || base.eq_ignore_ascii_case("xzr") || base.eq_ignore_ascii_case("x31") {
                return Err(format!("invalid base register: {}", base));
            }
            let rn = parse_reg_num(base).ok_or_else(|| format!("invalid base register: {}", base))?;
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_ld_st_single_regression_w_base -- --test-threads=1
```
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::neon::encode_neon_ld_st_single_pbt::test_encode_neon_ld_st_single_regression_w_base' panicked at src/backend/arm/assembler/encoder/neon.rs:13432:9:
st1 {v0.b}[0], [w0] must Err
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_neon_ld_st_single_pbt.rs
