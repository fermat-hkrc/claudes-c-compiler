# Bug: encode_neon_ld1r accepts W, XZR, x31, and FP bases
**Law:** LD1R base must be Xn or SP; W/WZR/WSP, XZR/x31, and FP/SIMD names must make encode_neon_ld1r return Err.
**Impact:** `ld1r {v0.8b}, [w0]` encodes as `ld1r {v0.8b}, [x0]`; `[xzr]`/`[x31]` encode as `[sp]`; `[d0]` encodes as `[x0]`. The assembler emits a different instruction than the text, so a mistyped base silently loads from the wrong register.
**Function:** encode_neon_ld1r
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:832
**Detected by:** Negative/Error Contract
**Minimal input:** encode_neon_ld1r([RegList({v0.8b}), Mem{base:"w0", offset:0}])
**Expected:** Err
**Actual:** Ok(Word(0x0d40c000)) — encoding of `ld1r {v0.8b}, [x0]`
**Severity:** medium
**Root cause:** neon.rs:868 calls parse_reg_num on the base with no Xn|SP check. parse_reg_num maps w/d/s/q/v/h/b prefixes and xzr/wzr/wsp/x31 to a 5-bit number.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:868`
```rust
            let rn = parse_reg_num(base).ok_or("invalid base reg")?;
```
**Suggested fix:** Accept only `sp` or `x0`–`x30` (not `xzr`, `x31`, W, or FP).
```rust
            let rn = parse_ld1r_base(base)?;
            // parse_ld1r_base: sp -> 31; x0..=x30 -> n; otherwise Err
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_ld1r_regression_w_base -- --test-threads=1
```
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::neon::encode_neon_ld1r_pbt::encode_neon_ld1r_neg_invalid_base' panicked at src/backend/arm/assembler/encoder/neon.rs:12939:5:
Test failed: ld1r base [w0] must Err (llvm-mc requires Xn|SP) at src/backend/arm/assembler/encoder/neon.rs:13106.
minimal failing input: t = "8b", rt = 0, base = "w0"
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_neon_ld1r_pbt.rs
