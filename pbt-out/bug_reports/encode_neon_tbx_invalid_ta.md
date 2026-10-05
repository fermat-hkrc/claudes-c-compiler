# Bug: encode_neon_tbx encodes Ta other than 8b/16b as Q=0
**Law:** ARM TBX Ta is only {8B,16B}; llvm-mc rejects other arrangements, so encode_neon_tbx(Vd.ta, ...) = Err for ta ∉ {8b,16b}
**Impact:** `tbx v0.4h, {v0.16b}, v0.4h` is assembled as Q=0 (`tbx v0.8b, ...`), so an illegal arrangement silently becomes a different instruction.
**Function:** encode_neon_tbx
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:803
**Detected by:** Negative/Error Contract
**Minimal input:** encode_neon_tbx([v0.4h, {v0.16b}, v0.4h])
**Expected:** Err (llvm-mc rejects `tbx v0.4h, {v0.16b}, v0.4h`)
**Actual:** Ok(Word(0x0e001000)) — encoded as Q=0 TBX
**Severity:** medium
**Root cause:** neon.rs:808 sets Q=1 only for the string "16b" and otherwise Q=0, with no arrangement whitelist.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:808`
```rust
    let q: u32 = if arr_d == "16b" { 1 } else { 0 };
```
**Suggested fix:** Reject any Ta other than 8b/16b.
```rust
    let q: u32 = match arr_d.as_str() {
        "16b" => 1,
        "8b" => 0,
        other => return Err(format!("tbx: Ta must be 8b or 16b, got {}", other)),
    };
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_tbx_regression_invalid_ta -- --test-threads=1
```
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_neon_tbx_pbt::test_encode_neon_tbx_regression_invalid_ta' panicked at src/backend/arm/assembler/encoder/encode_neon_tbx_pbt.rs:584:5:
tbx v0.4h, {v0.16b}, v0.4h must Err (Ta not 8b/16b)
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_neon_tbx_pbt.rs
