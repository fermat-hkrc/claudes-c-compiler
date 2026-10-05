# Bug: encode_neon_fcvtl accepts dest 2S and ignores source arrangement
**Law:** FCVTL{2} is defined only for Ta in {4S,2D} with matching Tb {4H/8H, 2S/4S}; every other pair must be Err
**Impact:** Invalid assembly such as `fcvtl v0.2s, v0.8b` or `fcvtl v0.4s, v0.8b` encodes as a different valid FCVTL (sz from dest only, Q from is_high), producing the wrong machine code instead of an assemble error
**Function:** encode_neon_fcvtl
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:1640
**Detected by:** Negative/Error Contract
**Minimal input:** encode_neon_fcvtl([v0.2s, v0.8b], is_high=false)
**Expected:** Err (llvm-mc: invalid operand; ARM dest is 4S or 2D with matching source)
**Actual:** Ok(Word) — dest "2s" is treated as sz=0 (same as 4S); source arrangement is discarded
**Severity:** high
**Root cause:** neon.rs:1642 discards the source arrangement; neon.rs:1643 matches dest "2s" as sz=0, which is not an ARM FCVTL destination
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:1642`
```rust
    let (rn, _) = get_neon_reg(operands, 1)?;
    let sz = match arr_d.as_str() { "4s" | "2s" => 0u32, "2d" => 1,
        _ => return Err(format!("fcvtl: unsupported dest: {}", arr_d)), };
```
**Suggested fix:** Accept only dest 4s/2d and require the ARM-mandated source arrangement
```rust
    let (rn, arr_n) = get_neon_reg(operands, 1)?;
    let sz = match arr_d.as_str() {
        "4s" => 0u32,
        "2d" => 1,
        _ => return Err(format!("fcvtl: unsupported dest: {}", arr_d)),
    };
    let expected_tb = match (arr_d.as_str(), is_high) {
        ("4s", false) => "4h",
        ("4s", true) => "8h",
        ("2d", false) => "2s",
        ("2d", true) => "4s",
        _ => return Err(format!("fcvtl: unsupported dest: {}", arr_d)),
    };
    if arr_n != expected_tb {
        return Err(format!("fcvtl: dest {} requires source {}", arr_d, expected_tb));
    }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_neon_fcvtl_neg_mismatched_ta_tb -- --test-threads=1
```
**Raw output:**
```text
Test failed: invalid/mismatched Ta/Tb must Err (ARM FCVTL Ta in {4S,2D} with matching Tb; llvm-mc rejects fcvtl v0.2s, v0.8b) at src/backend/arm/assembler/encoder/encode_neon_fcvtl_pbt.rs:375.
minimal failing input: rd = 0, rn = 0, ta = "2s", tb = "8b", is_high = false
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_neon_fcvtl_pbt.rs
