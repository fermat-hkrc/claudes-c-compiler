# Bug: encode_neon_ld_st_multi accepts W/XZR/x31/FP as the memory base
**Law:** AdvSIMD multiple-structure addressing allows only Xn|SP as base; encode_neon_ld_st_multi(list, Mem{base∈{w0,w31,wsp,xzr,x31,s0,d0,v0,q0}}, load, n) = Err
**Impact:** `st1 {v0.8b}, [w0]` is encoded as `[x0]`; `[xzr]`/`[x31]` are encoded as `[sp]`. A width or kind typo silently assembles the wrong addressing mode.
**Function:** encode_neon_ld_st_multi
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:1008
**Detected by:** Negative/Error Contract
**Minimal input:** encode_neon_ld_st_multi([RegList({v0.8b}), Mem{base:"w0", offset:0}], is_load=false, num_structs=1)
**Expected:** Err (llvm-mc rejects `[w0]`, `[xzr]`, `[x31]`, `[s0]` as the base)
**Actual:** Ok(Word(0x0c007000)) — encoded as `st1 {v0.8b}, [x0]`
**Severity:** medium
**Root cause:** neon.rs:1031-1033 calls parse_reg_num which accepts w/d/s/q/v/h/b prefixes and maps xzr/x31/wsp to 31, with no check that the base is Xn or SP.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:1032`
```rust
            let r = parse_reg_num(base).ok_or_else(|| format!("invalid base register: {}", base))?;
```
**Suggested fix:** Reject a base that is not an X register or SP (treat xzr/x31/W/FP as invalid).
```rust
            if !is_64bit_reg(base) || base.eq_ignore_ascii_case("xzr") || base.eq_ignore_ascii_case("x31") {
                return Err(format!("ld{}/st{}: base must be Xn or SP, got {}", num_structs, num_structs, base));
            }
            let r = parse_reg_num(base).ok_or_else(|| format!("invalid base register: {}", base))?;
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_ld_st_multi_regression_w_base -- --test-threads=1
```
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_neon_ld_st_multi_pbt::test_encode_neon_ld_st_multi_regression_w_base' panicked at src/backend/arm/assembler/encoder/encode_neon_ld_st_multi_pbt.rs:690:5:
st1 {v0.8b}, [w0] must Err
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_neon_ld_st_multi_pbt.rs
