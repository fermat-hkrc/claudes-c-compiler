# Bug: encode_neon_bitwise_insert encodes T outside {8B,16B}
**Law:** BIT/BIF are defined only for T in {8B,16B}; every other arrangement must be Err
**Impact:** Invalid assembly such as `bit v0.4h, v0.4h, v0.4h` encodes as 8B BIT (Q=0), producing wrong machine code instead of an assemble error
**Function:** encode_neon_bitwise_insert
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:1667
**Detected by:** Negative/Error Contract
**Minimal input:** encode_neon_bitwise_insert([v0.4h, v0.4h, v0.4h], size=0b10)
**Expected:** Err (only .8b/.16b; llvm-mc: invalid operand)
**Actual:** Ok(Word) — Q=0 as if T were 8B
**Severity:** medium
**Root cause:** neon.rs:1674 sets Q=1 iff dest arrangement is exactly "16b", else 0, with no check that T is 8b or 16b
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:1674`
```rust
    let q: u32 = if arr_d == "16b" { 1 } else { 0 };
```
**Suggested fix:** Accept only 8b/16b
```rust
    let q: u32 = match arr_d.as_str() {
        "8b" => 0,
        "16b" => 1,
        _ => return Err(format!("bit/bif: unsupported arrangement: {}", arr_d)),
    };
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_bitwise_insert_regression_invalid_t -- --test-threads=1 --exact
```
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_neon_bitwise_insert_pbt::encode_neon_bitwise_insert_neg_invalid_t' panicked at src/backend/arm/assembler/encoder/encode_neon_bitwise_insert_pbt.rs:185:1:
Test failed: invalid T must Err (only .8b/.16b; llvm-mc rejects bit v0.4h, v0.4h, v0.4h) at src/backend/arm/assembler/encoder/encode_neon_bitwise_insert_pbt.rs:327.
minimal failing input: rd = 0, rn = 0, rm = 0, t = "4h", size = 2
	successes: 0
	local rejects: 0
	global rejects: 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_neon_bitwise_insert_pbt.rs
