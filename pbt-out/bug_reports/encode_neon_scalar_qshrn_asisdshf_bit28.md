# Bug: encode_neon_scalar_qshrn packs scalar SQSHRN as vector SQSHRN2
**Law:** Valid scalar SQSHRN/SQRSHRN/UQSHRN/UQRSHRN must encode ARM asisdshf with bits[31:24]=01 U 11111, matching llvm-mc/gas
**Impact:** Every successful scalar SQSHRN is emitted as the vector Q=1 encoding (sqshrn2). Assembled objects disagree with gas/llvm-mc and execute the wrong instruction class. encode() routes non-RegArrangement `sqshrn` dest into this helper, so the witness is caller-reachable from the public assembler.
**Function:** encode_neon_scalar_qshrn
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:1835
**Detected by:** Differential — llvm-mc AArch64 assembler
**Minimal input:** encode_neon_scalar_qshrn([Reg("b0"), Reg("h0"), Imm(1)], u_bit=0, is_rounding=false)
**Expected:** Ok(Word(0x5f0f9400)) matching llvm-mc `sqshrn b0, h0, #1`
**Actual:** Ok(Word(0x4f0f9400)) which is the vector `sqshrn2` prefix (bit 28 clear)
**Severity:** high
**Root cause:** neon.rs:1849 ORs vector asimdshf fixed bits `(0b011110 << 23)` onto `(0b01 << 30)`, leaving bit 28 = 0; ARM scalar asisdshf needs bits[28:24]=11111
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:1849`
```rust
    let word = (0b01 << 30) | (u_bit << 29) | (0b011110 << 23) | ((immhb >> 3) << 19) | ((immhb & 7) << 16)
        | (opcode_bits << 10) | (rn << 5) | rd;
```
**Suggested fix:** Place scalar asisdshf fixed bits at [28:24]=11111
```rust
    let word = (0b01 << 30) | (u_bit << 29) | (0b11111 << 24) | ((immhb >> 3) << 19) | ((immhb & 7) << 16)
        | (opcode_bits << 10) | (rn << 5) | rd;
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_neon_scalar_qshrn_diff_llvm_mc -- --test-threads=1
cargo test --lib test_encode_neon_scalar_qshrn_regression_asisdshf_bit28 -- --test-threads=1
```
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_neon_scalar_qshrn_pbt::encode_neon_scalar_qshrn_diff_llvm_mc' (2506258) panicked at src/backend/arm/assembler/encoder/encode_neon_scalar_qshrn_pbt.rs:207:1:
Test failed: assertion failed: `(left == right)` 
  left: `1326420992`, 
 right: `1594856448`: mismatch for sqshrn b0, h0, #1 at src/backend/arm/assembler/encoder/encode_neon_scalar_qshrn_pbt.rs:226.
minimal failing input: rd = 0, rn = 0, case = (
    "b",
    "h",
    8,
    1,
), u = 0, round = false
	successes: 0
	local rejects: 0
	global rejects: 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_neon_scalar_qshrn_pbt.rs
