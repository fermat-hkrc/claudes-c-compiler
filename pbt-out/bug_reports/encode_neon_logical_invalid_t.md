# Bug: encode_neon_logical accepts arrangements other than .8b/.16b
**Law:** ARM Advanced SIMD AND/ORR/EOR allow only T in {8B,16B}; any other arrangement must be rejected
**Impact:** `and v0.4h, v0.4h, v0.4h` (and .8h/.4s/.2d/…) silently encodes as 64-bit AND (Q=0), so invalid SIMD shapes assemble as the wrong Q-width instruction
**Function:** encode_neon_logical
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:297
**Detected by:** Negative/Error Contract
**Minimal input:** encode_neon_logical([v0.4h, v0.4h, v0.4h], opc=0)
**Expected:** Err (llvm-mc: invalid operand; ARM T in {8B,16B})
**Actual:** Ok(EncodeResult::Word) of `and v0.8b, v0.8b, v0.8b` (Q=0 because arr_d != "16b")
**Severity:** medium
**Root cause:** neon.rs:302 sets Q=1 only when dest arrangement is exactly "16b", otherwise Q=0, with no allow-list for {8b,16b}
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:302`
```rust
    let q: u32 = if arr_d == "16b" { 1 } else { 0 };
```
**Suggested fix:** Accept only .8b and .16b before packing Q
```rust
    let q: u32 = match arr_d.as_str() {
        "8b" => 0,
        "16b" => 1,
        other => return Err(format!("NEON logical requires .8b or .16b, got .{other}")),
    };
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_logical_regression_invalid_t -- --test-threads=1
```
**Raw output:**
```text
Test failed: invalid T must Err (only .8b/.16b; llvm-mc rejects and v0.4h, v0.4h, v0.4h) at src/backend/arm/assembler/encoder/encode_neon_logical_pbt.rs:422.
minimal failing input: rd = 0, rn = 0, rm = 0, t = "4h", opc = 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_neon_logical_pbt.rs
