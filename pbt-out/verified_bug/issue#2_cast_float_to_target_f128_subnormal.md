# Verified Bug: Issue #2 — `cast_float_to_target` panics on f64 subnormals when targeting F128

**Issue:** [fermat-hkrc/claudes-c-compiler#2](https://github.com/fermat-hkrc/claudes-c-compiler/issues/2)
**Verdict:** ✅ **TRUE BUG — manually verified** (compiler crash in debug, silent miscompilation in release)
**Verified on:** branch `explore/pbt-test`, commit `bc6f5170` (SUT code identical to `7db46890`; only test files differ)
**Verified by:** manual reproduction, three independent levels (Rust unit, debug E2E, release E2E + gcc differential)
**Date:** 2026-09-15

---

## 1. The Claim

`IrConst::cast_float_to_target(fv, IrType::F128)` panics on **subnormal f64** inputs
(`biased_exp == 0 && mantissa != 0`) instead of returning `Some(LongDouble(fv, correct_bytes))`:

- **Debug builds:** panic `attempt to subtract with overflow` → any compile-time
  subnormal→`long double` cast crashes the compiler.
- **Release builds:** `u128` subtraction wraps → garbage binary128 exponent bytes
  emitted silently.

---

## 2. Evidence Summary

| # | Level | Build | Input | Result |
|---|-------|-------|-------|--------|
| 1 | Rust unit | debug (test) | `f64::from_bits(1)`, `from_bits(0x8000000000000001)` → `F128` | ❌ panic at `src/common/long_double.rs:1040:18` |
| 2 | C end-to-end | `target/debug/ccc` | `/tmp/sub.c` (see §3.2) | ❌ `ccc: internal error: attempt to subtract with overflow` — valid C crashes the compiler |
| 3 | C end-to-end | `target/release/ccc` | `/tmp/sub2.c` (see §3.3) | ❌ **silent miscompilation**: `1.112537e-308` instead of `4.940656e-324` (factor 2⁵¹), no diagnostic |
| — | Reference | gcc | same files | ✅ exact: `4.940656e-324`, equality checks pass |

**Note on oracle strength:** a weak oracle (`ld > 0.0L`) cannot distinguish the bug — the
miscompiled value is positive, so it prints `1` and *appears* correct. The sharp oracle
(`ld == 0x1p-1074L`, exact equality against a correctly-parsed hex literal) is required (§3.3).

---

## 3. Reproduction

### 3.1 Unit level (direct, definitive)

```rust
// src/ir/constants.rs (bottom, temporary scratch module)
#[cfg(test)]
mod scratch {
    use super::*;

    #[test]
    fn manual_f128_subnormal() {
        // subnormal: biased_exp == 0, mantissa != 0
        let fv = f64::from_bits(1);                       // smallest positive subnormal
        let neg = f64::from_bits(1u64 << 63 | 1);         // 0x8000000000000001 (issue's minimal input)
        println!("pos = {:?}", IrConst::cast_float_to_target(fv, IrType::F128));
        println!("neg = {:?}", IrConst::cast_float_to_target(neg, IrType::F128));
    }
}
```

```console
$ cargo test --lib scratch::manual_f128_subnormal -- --nocapture

thread 'ir::constants::scratch::manual_f128_subnormal' (386334) panicked at src/common/long_double.rs:1040:18:
attempt to subtract with overflow
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
test ir::constants::scratch::manual_f128_subnormal ... FAILED

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 504 filtered out; finished in 0.00s
```

### 3.2 End-to-end, debug build — compiler crash on valid C

`/tmp/sub.c`:

```c
#include <stdio.h>

/* global initializer: must be constant-folded at compile time */
long double g = (long double)4.9406564584124654e-324; /* smallest positive subnormal double */

int main(void) {
    long double ld = (long double)4.9406564584124654e-324; /* subnormal double -> long double */
    printf("g  > 0 : %d\n", g  > 0.0L);
    printf("ld > 0 : %d\n", ld > 0.0L);
    return 0;
}
```

```console
$ ./target/debug/ccc /tmp/sub.c -o /tmp/sub_ccc

thread '<unnamed>' (388711) panicked at src/common/long_double.rs:1040:18:
attempt to subtract with overflow
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
ccc: internal error: attempt to subtract with overflow
```

### 3.3 End-to-end, release build — silent miscompilation (sharp oracle)

`/tmp/sub2.c`:

```c
#include <stdio.h>

/* smallest positive subnormal double = 2^-1074 = 4.9406564584124654e-324 */
long double g = (long double)4.9406564584124654e-324; /* global: folded at compile time */

int main(void) {
    long double ld  = (long double)4.9406564584124654e-324;
    long double ref = 0x1p-1074L;  /* exact 2^-1074, parsed directly as long double literal */

    printf("g  exact: %d\n", g  == ref);   /* gcc: 1 | buggy fold: 0 (value ~1.1e-308) */
    printf("ld exact: %d\n", ld == ref);   /* gcc: 1 | buggy fold: 0 */
    printf("g  = %Le\n", g);
    printf("ld = %Le\n", ld);
    printf("ref= %Le\n", ref);
    return 0;
}
```

```console
$ gcc /tmp/sub2.c -o /tmp/sub2_gcc && /tmp/sub2_gcc
g  exact: 1
ld exact: 1
g  = 4.940656e-324
ld = 4.940656e-324
ref= 4.940656e-324

$ ./target/release/ccc /tmp/sub2.c -o /tmp/sub2_ccc_rel && /tmp/sub2_ccc_rel
g  exact: 0
ld exact: 0
g  = 1.112537e-308
ld = 1.112537e-308
ref= 4.940656e-324
```

Both the global initializer and the local constant fold to the wrong value.

---

## 4. Root Cause

Call path:

```
cast_float_to_target(fv, IrType::F128)      src/ir/constants.rs:279
  └─ IrConst::long_double(fv)               src/ir/constants.rs:168
       └─ f64_to_f128_bytes_lossless(fv)    src/common/long_double.rs:1028
```

In `f64_to_f128_bytes_lossless`, the guards are:

- `F64Decomposed::is_zero` = `biased_exp == 0 && mantissa == 0` (`src/common/long_double.rs:590`)
- `is_special` = `biased_exp == 0x7FF` (`src/common/long_double.rs:591`)

A **subnormal** (`biased_exp == 0 && mantissa != 0`) passes both guards and falls
through to the normal-path exponent computation (`src/common/long_double.rs:1040`):

```rust
let exp15 = (d.biased_exp as u128 - 1023 + 16383) as u128;  // 0u128 - 1023 → underflow
```

- **Debug:** `0u128 - 1023` panics (`attempt to subtract with overflow`).
- **Release:** wraps — `(0 − 1023 + 16383) mod 2¹²⁸ = 15360`, `mantissa112 = 1 << 60`,
  producing `(1 + 2⁻⁵²) × 2⁻¹⁰²³ ≈ 1.1125369292536e-308`.

### Prediction vs observation (release miscompiled value)

| | Value |
|---|---|
| Predicted from wrapped arithmetic | `(1+2⁻⁵²) × 2⁻¹⁰²³ ≈ 1.1125369292536e-308` |
| Observed (`%Le` output) | `1.112537e-308` |

Exact match at printed precision — the miscompiled constant is fully explained by the
wrapped exponent, confirming the mechanism (not an unrelated codegen issue).

### Correct expected encoding

`f64::from_bits(1)` = 2⁻¹⁰⁷⁴ → binary128: sign 0, biased exponent `16383 − 1074 = 15309`
(`0x3BCD`), mantissa 0 → `0x3BCD_0000_0000_0000_0000_0000_0000_0000`. Negative input adds
the sign bit (`0x8000…`).

---

## 5. Suggested Fix

Add a subnormal branch in `f64_to_f128_bytes_lossless` before `src/common/long_double.rs:1040`:
the value is `mantissa × 2⁻¹⁰⁷⁴`, so renormalize — find the mantissa's leading bit, shift the
mantissa, and compute the exponent accordingly. The renormalization pattern already exists in
this file for integer→f128 conversion (`src/common/long_double.rs:1015-1025`,
`bl = 128 - leading_zeros()`).

---

## 6. Notes

- The scratch unit test in `src/ir/constants.rs` (`mod scratch`) is still present at
  verification time — keep it as a regression test or delete it. The issue text cites
  `cast_float_to_target_pbt::test_cast_float_to_target_regression_f128_neg_subnormal`,
  which does **not** exist in the repo; the scratch test is the only current repro test.
- Draft confirmation comment for the issue thread:
  `pbt-out/bug_reports/issue2_confirmation_comment.md` (not yet posted).
- Severity assessment in the issue (**high**) is reasonable: release builds miscompile
  a C11-conforming construct with no diagnostic; debug builds crash the compiler.
