# Bug: encode_neon_ext encodes invalid arrangements as 8B EXT
**Law:** EXT is defined only for T in {8B,16B}; encode_neon_ext([Vd.T, Vn.T, Vm.T, #i]) = Err when T ∉ {8b,16b}
**Impact:** `ext v0.8h, v1.8h, v2.8h, #1` (and .4s/.2d/.4h/…) is assembled as `ext v0.8b, …` (Q=0), so an illegal permute silently becomes a different instruction.
**Function:** encode_neon_ext
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:405
**Detected by:** Negative/Error Contract
**Minimal input:** encode_neon_ext([v0.8h, v0.8h, v0.8h, #0])
**Expected:** Err (llvm-mc/gas reject `ext v0.8h, v0.8h, v0.8h, #0`; gas suggests .8b/.16b)
**Actual:** Ok(Word(0x2e000000)) — encoded as `ext v0.8b, v0.8b, v0.8b, #0`
**Severity:** medium
**Root cause:** neon.rs:414 sets Q=1 only for arr_d=="16b" and otherwise Q=0 with no arrangement whitelist, so every non-16b T including 8h/4s/2d encodes as 8B EXT.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:414`
```rust
    let q: u32 = if arr_d == "16b" { 1 } else { 0 };
```
**Suggested fix:** Accept only 8b and 16b.
```rust
    let q: u32 = match arr_d.as_str() {
        "16b" => 1,
        "8b" => 0,
        _ => return Err(format!("unsupported EXT arrangement: {}", arr_d)),
    };
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_ext_regression_invalid_t -- --test-threads=1
```
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_neon_ext_pbt::test_encode_neon_ext_regression_invalid_t' panicked at src/backend/arm/assembler/encoder/encode_neon_ext_pbt.rs:542:5:
ext v0.8h, v1.8h, v2.8h, #1 must Err (gas/llvm-mc accept only .8b/.16b)
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_neon_ext_pbt.rs
