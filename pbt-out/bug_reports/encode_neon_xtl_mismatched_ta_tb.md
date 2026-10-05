# Bug: encode_neon_xtl encodes mismatched UXTL/SXTL arrangements
**Law:** UXTL/SXTL dest Ta must be 8H/4S/2D paired with source Tb 8B/4H/2S (Q=0) or 16B/8H/4S (Q=1)
**Impact:** Invalid GNU-style arrangement pairs (wrong dest Ta, or UXTL with a UXTL2 source) are encoded as if the dest and Q/Tb pairing were legal, producing the wrong widening instruction
**Function:** encode_neon_xtl
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:163
**Detected by:** Negative/Error Contract
**Minimal input:** encode_neon_xtl([v0.8b, v0.8b], u_bit=0, is_high=false)  (asm `sxtl v0.8b, v0.8b`)
**Expected:** Err
**Actual:** Ok(Word) — dest `.8b` is discarded; source `.8b` is treated as a legal SXTL source
**Severity:** medium
**Root cause:** neon.rs:167 binds dest arrangement as `_arr_d` and never checks it; Q comes only from `is_high`, and 8b|16b (4h|8h, 2s|4s) share one immh, so UXTL with a UXTL2 source still encodes
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:167`
```rust
    let (rd, _arr_d) = get_neon_reg(operands, 0)?;
```
**Suggested fix:** Require dest Ta in {8h,4s,2d} and Tb matching (Ta, Q)
```rust
    let (rd, arr_d) = get_neon_reg(operands, 0)?;
    let (rn, arr_n) = get_neon_reg(operands, 1)?;
    let expected_tb = match (arr_d.as_str(), is_high) {
        ("8h", false) => "8b",
        ("8h", true) => "16b",
        ("4s", false) => "4h",
        ("4s", true) => "8h",
        ("2d", false) => "2s",
        ("2d", true) => "4s",
        _ => return Err(format!("uxtl/sxtl: unsupported dest arrangement: {}", arr_d)),
    };
    if arr_n != expected_tb {
        return Err(format!("uxtl/sxtl: source arrangement {} does not match dest {}", arr_n, arr_d));
    }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_neon_xtl -- --test-threads=1
cargo test --lib test_encode_neon_xtl_regression_mismatched_dest_ta -- --test-threads=1
```
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_neon_xtl_pbt::encode_neon_xtl_neg_mismatched_ta_tb' panicked at src/backend/arm/assembler/encoder/encode_neon_xtl_pbt.rs:394:1:
Test failed: invalid/mismatched Ta/Tb must Err (ARM UXTL Ta in {8H,4S,2D} with matching Tb; llvm-mc rejects sxtl v0.8b, v0.8b) at src/backend/arm/assembler/encoder/encode_neon_xtl_pbt.rs:414.
minimal failing input: rd = 0, rn = 0, ta = "8b", tb = "8b", u_bit = 0, is_high = false
	successes: 0
	local rejects: 0
	global rejects: 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_neon_xtl_pbt.rs
