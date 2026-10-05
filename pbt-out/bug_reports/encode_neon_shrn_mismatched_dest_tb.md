# Bug: encode_neon_shrn ignores destination arrangement Tb
**Law:** Vector SHRN requires Vd.Tb to match Ta and the `2` suffix: 8H→8B (Q=0) / 16B (Q=1); 4S→4H / 8H; 2D→2S / 4S. A mismatched dest Tb must be rejected.
**Impact:** `shrn v0.8b, v0.2d, #1` is invalid AArch64 (llvm-mc: invalid operand) but the helper encodes it as if dest were v0.2s, using only Rd's register number. Wrong-arrangement assembly is silently accepted.
**Function:** encode_neon_shrn
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:1436
**Detected by:** Negative/Error Contract
**Minimal input:** encode_neon_shrn([v0.8b, v0.2d, #1], opcode=0b100001, is_high=false)
**Expected:** Err (llvm-mc: "invalid operand for instruction"; ARM requires Tb=2s for Ta=2d, Q=0)
**Actual:** Ok(Word) — `let (rd, _) = get_neon_reg(operands, 0)` discards dest arrangement
**Severity:** medium
**Root cause:** neon.rs:1438 discards the destination arrangement, so Q comes only from is_high and esize only from the source
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:1438`
```rust
    let (rd, _) = get_neon_reg(operands, 0)?;
```
**Suggested fix:** Require Tb to match Ta and Q
```rust
    let (rd, arr_d) = get_neon_reg(operands, 0)?;
    let expected_tb = match (arr_n.as_str(), is_high) {
        ("8h", false) => "8b",
        ("8h", true) => "16b",
        ("4s", false) => "4h",
        ("4s", true) => "8h",
        ("2d", false) => "2s",
        ("2d", true) => "4s",
        _ => return Err(format!("shrn: unsupported source: {}", arr_n)),
    };
    if arr_d != expected_tb {
        return Err(format!("shrn: dest arrangement {} does not match source {}", arr_d, arr_n));
    }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_shrn_regression_mismatched_dest_tb -- --test-threads=1 --exact
```
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_neon_shrn_pbt::encode_neon_shrn_neg_invalid_arrangement' (2365167) panicked at src/backend/arm/assembler/encoder/encode_neon_shrn_pbt.rs:427:1:
Test failed: invalid/mismatched Ta/Tb must Err (ARM SHRN Ta in {8H,4S,2D} with matching Tb; llvm-mc rejects shrn v0.8b, v0.2d, #1) at src/backend/arm/assembler/encoder/encode_neon_shrn_pbt.rs:451.
minimal failing input: rd = 0, rn = 0, tb = "8b", ta = "2d", shift = 1, is_high = false, opcode = 33
	successes: 0
	local rejects: 0
	global rejects: 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_neon_shrn_pbt.rs
