# Bug: encode_neon_pmull encodes 8-bit PMULL as 64-bit PMULL
**Law:** Valid PMULL Vd.8H, Vn.8B, Vm.8B (and PMULL2 Vd.8H, Vn.16B, Vm.16B) must encode ARM three-different size=00, matching llvm-mc/gas
**Impact:** The assembler emits the crypto 64-bit polynomial-multiply-long encoding (size=11) for the baseline 8-bit form, so `pmull v0.8h, v0.8b, v0.8b` executes as `pmull v0.1q, v0.1d, v0.1d`
**Function:** encode_neon_pmull
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:1128
**Detected by:** Differential — llvm-mc AArch64 (+aes)
**Minimal input:** encode_neon_pmull([v0.8h, v0.8b, v0.8b], is_pmull2=false)
**Expected:** Ok(Word(0x0e20e000)) — llvm-mc `pmull v0.8h, v0.8b, v0.8b`
**Actual:** Ok(Word(0x0ee0e000)) — size bits[23:22]=11 instead of 00
**Severity:** high
**Root cause:** neon.rs:1141 hardcodes `(0b11 << 22)` (size=11) and discards arrangements, so the ARM 8H form cannot select size=00
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:1141`
```rust
    let word = ((q << 30) | (0b001110 << 24) | (0b11 << 22) | (1 << 21)
        | (rm << 16) | (0b11100 << 11)) | (rn << 5) | rd;
```
**Suggested fix:** Derive size from Ta/Tb (8H/8B/16B → size=00, 1Q/1D/2D → size=11), matching encode_neon_three_diff
```rust
    let size = match (arr_d.as_str(), arr_n.as_str()) {
        ("8h", "8b") | ("8h", "16b") => 0b00u32,
        ("1q", "1d") | ("1q", "2d") => 0b11u32,
        _ => return Err(format!("unsupported pmull arrangement: {}.{}", arr_d, arr_n)),
    };
    let word = ((q << 30) | (0b001110 << 24) | (size << 22) | (1 << 21)
        | (rm << 16) | (0b11100 << 11)) | (rn << 5) | rd;
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_pmull_regression_8h_as_64bit -- --test-threads=1 --exact
```
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_neon_pmull_pbt::encode_neon_pmull_diff_llvm_mc_8h' (2326713) panicked at src/backend/arm/assembler/encoder/encode_neon_pmull_pbt.rs:210:1:
Test failed: assertion failed: `(left == right)` 
  left: `249618432`, 
 right: `237035520`: mismatch for pmull v0.8h, v0.8b, v0.8b at src/backend/arm/assembler/encoder/encode_neon_pmull_pbt.rs:244.
minimal failing input: rd = 0, rn = 0, rm = 0, is_pmull2 = false
	successes: 0
	local rejects: 0
	global rejects: 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_neon_pmull_pbt.rs
