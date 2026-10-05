# Bug: encode_neon_fcvtn accepts dest 8B and source 2S
**Law:** FCVTN{2} is defined only for Ta in {4S,2D} with matching Tb {4H/8H, 2S/4S}; every other pair must be Err
**Impact:** Invalid assembly such as `fcvtn v0.8b, v0.2s` or `fcvtn v0.8b, v0.4s` encodes as a different valid FCVTN (sz from source only, Q from is_high), producing the wrong machine code instead of an assemble error
**Function:** encode_neon_fcvtn
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:1652
**Detected by:** Negative/Error Contract
**Minimal input:** encode_neon_fcvtn([v0.8b, v0.2s], is_high=false)
**Expected:** Err (llvm-mc: invalid operand; ARM source is 4S or 2D with matching dest)
**Actual:** Ok(Word(0x0e216800)) — dest "8b" is discarded; source "2s" is treated as sz=0 (same as 4S)
**Severity:** high
**Root cause:** neon.rs:1653 discards the dest arrangement; neon.rs:1655 matches source "2s" as sz=0, which is not an ARM FCVTN source
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:1653`
```rust
    let (rd, _) = get_neon_reg(operands, 0)?;
    let (rn, arr_n) = get_neon_reg(operands, 1)?;
    let sz = match arr_n.as_str() { "4s" | "2s" => 0u32, "2d" => 1,
        _ => return Err(format!("fcvtn: unsupported source: {}", arr_n)), };
```
**Suggested fix:** Accept only source 4s/2d and require the ARM-mandated dest arrangement
```rust
    let (rd, arr_d) = get_neon_reg(operands, 0)?;
    let (rn, arr_n) = get_neon_reg(operands, 1)?;
    let sz = match arr_n.as_str() {
        "4s" => 0u32,
        "2d" => 1,
        _ => return Err(format!("fcvtn: unsupported source: {}", arr_n)),
    };
    let expected_tb = match (arr_n.as_str(), is_high) {
        ("4s", false) => "4h",
        ("4s", true) => "8h",
        ("2d", false) => "2s",
        ("2d", true) => "4s",
        _ => return Err(format!("fcvtn: unsupported source: {}", arr_n)),
    };
    if arr_d != expected_tb {
        return Err(format!("fcvtn: source {} requires dest {}", arr_n, expected_tb));
    }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_neon_fcvtn_neg_mismatched_ta_tb -- --test-threads=1
```
**Raw output:**
```text
Test failed: invalid/mismatched Tb/Ta must Err (ARM FCVTN Ta in {4S,2D} with matching Tb; llvm-mc rejects fcvtn v0.8b, v0.2s) at src/backend/arm/assembler/encoder/encode_neon_fcvtn_pbt.rs:375.
minimal failing input: rd = 0, rn = 0, tb = "8b", ta = "2s", is_high = false
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_neon_fcvtn_pbt.rs
