# Bug: f64 subnormals are misencoded by f64_to_f128_bytes and f64_to_x87_bytes
**Law:** ∀ finite f64 v: decode(encode(v)) == v, where decode is the field-level IEEE 754 decoder built from the formats' own definitions (both target formats — binary128 and x87 80-bit — represent every f64 exactly, subnormals included: they become NORMAL numbers in the wider formats).
**Impact:** Any f64 subnormal literal that flows into a long double constant (`IrConst::LongDouble` path or long-double data emission) is encoded ≈2^51 times too large (decoded 2^-1023-family instead of 2^-1074-family). On ARM64/RISC-V the f128 bytes are emitted verbatim → wrong data in .rodata; on x86 the bytes convert to a wrong x87 constant. Subnormal long-double constants are rare but legal C (`long double x = 5e-324;` and any underflowed constant expression).
**Function:** f64_to_f128_bytes, f64_to_x87_bytes
**Source location:** /home/shuhao/fermat-users/leo/github/claudes-c-compiler/src/ir/constants.rs:50 and :82
**Detected by:** Reference — independent IEEE 754 field decoder, property P7
**Minimal input:** f64_to_f128_bytes(5e-324)
**Expected:** the f128 encoding of 2^-1074 (normal f128, exponent field 15361, mantissa 0x0010000000000000000000000000)
**Actual:** exponent field 15360 (as if the source were 2^-1023-normal) with mantissa 2^60 and NO implicit-1 normalization → decodes to ≈1.1125e-308 instead of 5e-324
**Severity:** medium (rare input class; silent wrong constant data when hit)
**Doc contract:** src/ir/constants.rs:50 "Convert an f64 value to IEEE 754 binary128 (quad-precision) encoding" (fingerprint a40c100e) — a faithful encoding preserves the value; the subnormal fall-through into the "Normal number" arm violates it.
**Fix:** In both encoders, handle exp11 == 0 && mantissa52 != 0: normalize the subnormal (find the leading 1 of mantissa52, adjust the exponent) before packing; subnormal f64 m·2^-1074 needs exponent e = -1023 + (52 − msb_index) … i.e. treat it as the normal f64 m·2^(msb−52)·2^(−1022−(msb)) and reuse the normal path.
**Regression test:** src/ir/constants.rs `pbt_regression::test_ir_const_regression_subnormal_encoding`

**Reproduction:**
```bash
cd /home/shuhao/fermat-users/leo/github/claudes-c-compiler
PATH="$HOME/.cargo/bin:$PATH" cargo test --lib ir::constants::pbt_regression::test_ir_const_regression_subnormal_encoding
```
**Raw output:**
```
Test failed: f128: v=0.0000...005 decoded=0.0000...011125369292536007
minimal failing input: v = 5e-324
```
