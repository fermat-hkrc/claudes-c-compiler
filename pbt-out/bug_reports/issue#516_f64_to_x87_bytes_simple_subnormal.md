# Bug: f64_to_x87_bytes_simple mis-encodes subnormal f64 values

**Issue Synopsis**
`f64_to_x87_bytes_simple` handles zero and specials, then unconditionally encodes
the value as a normal: `exp15 = biased_exp - 1023 + 16383` with
`mantissa64 = (1<<63) | (mantissa << 11)` — i.e. it assumes an implicit leading 1.
A subnormal f64 (biased_exp = 0, mantissa != 0) has NO implicit bit; its value is
`mantissa × 2^-1074`. The function encodes it as `1.<mantissa> × 2^-1022` —
~2^752 times too large — and the value does not round-trip.

**Detection and Validation Methodology**
Property P11 (round-trip ∀ finite f64 x: x87_bytes_to_f64(f64_to_x87_bytes_simple(x)) = x bitwise), 1024 cases, subnormal-biased generator; shrunk to bits = 1; reproduced serially; pinned deterministic regression test (fails today, `#[ignore]`d).

**Reproduction Protocol**
```bash
cd /home/shuhao/fermat-users/leo/github/claudes-c-compiler
PATH="$HOME/.cargo/bin:$PATH" cargo test --lib -- --ignored common::long_double::pbt_regression::test_f64_to_x87_bytes_simple_regression_subnormal
# assertion failed: left: 0, right: 1  (decoded +0.0 vs original 5e-324 bits)
```

**Remediation Strategy**
Add a subnormal branch: normalize the mantissa (find MSB, shift left so the
integer bit is explicit, decrement the exponent accordingly, bias 16383), and add
subnormal cases to the existing unit tests.

---

**Law:** Widening f64 → x87 is exact ("zero-fills the extra mantissa bits" per its own doc), so narrowing back must be bitwise lossless for every finite f64.
**Impact:** Any subnormal `double` constant converted through this path on x86/i686 (long double folding, x87 FPU constant setup) becomes a completely different value (~2.22e-308 instead of 5e-324) — silent wrong-code.
**Function:** f64_to_x87_bytes_simple
**Source location:** /home/shuhao/fermat-users/leo/github/claudes-c-compiler/src/common/long_double.rs:1143
**Detected by:** Algebraic — Round-trip (P11)
**Minimal input:** `f64_to_x87_bytes_simple(f64::from_bits(1))` (5e-324)
**Expected:** decode back to 5e-324 (bits 0x1)
**Actual:** decodes to +0.0 (bits 0x0)
**Severity:** high
**Repro seed:** deterministic (bits = 1)
**Regression test:** src/common/long_double.rs `pbt_regression::test_f64_to_x87_bytes_simple_regression_subnormal` (#[ignore] witness)
**Raw output:**
```
Test failed: assertion failed: `(left == right)`
  left: `0`, right: `1`: x=5e-324 bits=0x1
minimal failing input: bits = 1
```
