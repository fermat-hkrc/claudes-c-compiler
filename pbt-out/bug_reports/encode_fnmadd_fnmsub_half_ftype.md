# Bug: encode_fnmadd_fnmsub encodes H registers as ftype=00 (single) instead of ftype=11 (half)
**Law:** FNMADD/FNMSUB with matching Hd, Hn, Hm, Ha must encode ARM ftype=11 (half-precision), agreeing with llvm-mc -mattr=+fullfp16.
**Impact:** `fnmadd h0, h1, h2, h3` is assembled as the single-precision encoding 0x1f220c20 instead of 0x1fe20c20. A half-precision fused multiply-add is silently emitted as an S-form instruction, so the object file executes the wrong FP operation.
**Function:** encode_fnmadd_fnmsub
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/fp_scalar.rs:146
**Detected by:** Differential — llvm-mc AArch64 assembler
**Minimal input:** encode_fnmadd_fnmsub([Reg("h0"), Reg("h0"), Reg("h0"), Reg("h0")], false)
**Expected:** 0x1fe00000 (llvm-mc fnmadd h0, h0, h0, h0 with +fullfp16)
**Actual:** 0x1f200000 (ftype=00, same as fnmadd s0, s0, s0, s0 with o1=1)
**Severity:** high
**Root cause:** fp_scalar.rs:152-153 sets ftype from `rd_name.starts_with('d')` only, so H (and every non-D prefix) collapses to ftype=00.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/fp_scalar.rs:152`
```rust
    let is_double = rd_name.starts_with('d');
    let ftype = if is_double { 0b01u32 } else { 0b00 };
```
**Suggested fix:** Map H to ftype=11.
```rust
    let ftype = if rd_name.starts_with('d') {
        0b01u32
    } else if rd_name.starts_with('h') {
        0b11u32
    } else {
        0b00
    };
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_fnmadd_fnmsub_regression_half_ftype -- --test-threads=1
```
**Raw output:**
```text
---- backend::arm::assembler::encoder::fp_scalar::encode_fnmadd_fnmsub_pbt::encode_fnmadd_fnmsub_diff_half stdout ----
Test failed: assertion failed: `(left == right)`
  left: `522190848`,
 right: `534773760`: FNMADD/FNMSUB half-precision mismatch for fnmadd h0, h0, h0, h0
minimal failing input: rd = 0, rn = 0, rm = 0, ra = 0, is_sub = false
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_fnmadd_fnmsub_pbt.rs
