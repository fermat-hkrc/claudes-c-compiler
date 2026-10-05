# Bug: encode_neon_float_elem leaves ARM size bit 23 clear
**Law:** Every valid FMUL/FMLA/FMLS/FMULX by-element encoding must match llvm-mc / ARM Advanced SIMD vector x indexed element, including size=10 for S and size=11 for D (bit 23 set)
**Impact:** Every NEON FP by-element instruction the assembler emits is the wrong 32-bit word (reserved size 00/01). Linked objects execute a different instruction than the assembly text
**Function:** encode_neon_float_elem
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:1613
**Detected by:** Differential vs llvm-mc; Algebraic — Invariant (ARM size field)
**Minimal input:** encode_neon_float_elem([v0.2s, v0.2s, v0.s[0]], u_bit=0, opcode=0b1001)
**Expected:** 0x0f809000 (llvm-mc `fmul v0.2s, v0.2s, v0.s[0]`)
**Actual:** 0x0f009000 (bit 23 clear)
**Severity:** high
**Root cause:** neon.rs:1633 shifts `sz` (0 for S, 1 for D) to bit 22 only, leaving bit 23 = 0. ARM/llvm-mc encode size[1:0] as 10 (S) / 11 (D)
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:1633`
```rust
    let word = (q << 30) | (u_bit << 29) | (0b01111 << 24) | (sz << 22)
```
**Suggested fix:** OR the high size bit so S=0b10 and D=0b11, matching encode_neon_elem
```rust
    let word = (q << 30) | (u_bit << 29) | (0b01111 << 24) | ((0b10 | sz) << 22)
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_neon_float_elem_diff_llvm_mc -- --test-threads=1
```
**Raw output:**
```text
Test failed: assertion failed: `(left == right)`
  left: `251695104`,
 right: `260083712`: SUT vs llvm-mc for fmul v0.2s, v0.2s, v0.s[0]
minimal failing input: rd = 0, rn = 0, rm = 0, idx_raw = 0, shape = ("2s", "s", 3), insn = (0, 9, "fmul")
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_neon_float_elem_pbt.rs::test_encode_neon_float_elem_regression_size_bit23
