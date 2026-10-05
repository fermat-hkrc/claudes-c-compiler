# Bug: __builtin_bswap32 folds unsigned results ≥ 2^31 into signed I32

**Issue Synopsis**
`eval_builtin_call("__builtin_bswap32", …)` stores its result as
`IrConst::I32(v.swap_bytes() as i32)`. `__builtin_bswap32` takes and returns
`unsigned int`, so for results ≥ 2^31 the folded constant reads back negative.
This violates the codebase's own documented representation rule
(const_arith.rs:88-96): unsigned 32-bit results must be `I64` zero-extended
because `IrConst::I32` is signed — the exact rationale the arithmetic path
implements and cites ("(2147483647 * 2U + 1U) = 4294967295U would be stored as
I32(-1), which sign-extends to -1 … Using I64(4294967295) preserves the correct
unsigned value").

**Detection and Validation Methodology**
Property P12b (reference differential vs GCC semantics — Rust `swap_bytes`, zero-extended — plus involution), 1024 cases, shrunk to v = -5495501120125551105; reproduced serially; deterministic regression witness below.

**Reproduction Protocol**
```bash
cd /home/shuhao/fermat-users/leo/github/claudes-c-compiler
PATH="$HOME/.cargo/bin:$PATH" cargo test --lib -- --ignored common::const_eval::pbt_regression::test_eval_builtin_bswap32_regression_unsigned_repr
# assertion failed: left: Some(-4380928), right: Some(4290586368)
```

**Remediation Strategy**
Store the result zero-extended exactly as the arithmetic path does:
`Some(IrConst::I64(v.swap_bytes() as u64 as i64))` (or the documented
representation for unsigned 32-bit constants in this codebase), then un-ignore
the regression test.

---

**Law:** `__builtin_bswap32(x)` is an unsigned-int function; its folded constant must carry the unsigned value 0..2^32-1 (zero-extended), per the module's own representation rule for unsigned 32-bit results.
**Impact:** Constant folding of `__builtin_bswap32(x) | 1LL << 40`-style expressions reads the constant as -1-style sign-extended i64. Concretely, the i128 widening path (`eval_const_binop_i128`: `rhs.to_i64()? as u64 as u128` with `rhs_unsigned = true`) sign-extends the I32(-n) through `to_i64` and produces a wildly wrong `__int128` value (0xFFFFFFFFFFBBD010 instead of 0xFFBBD010 for the witness).
**Function:** eval_builtin_call (bswap32 arm)
**Source location:** /home/shuhao/fermat-users/leo/github/claudes-c-compiler/src/common/const_eval.rs:95
**Detected by:** Reference differential (GCC semantics via Rust swap_bytes; P12b)
**Minimal input:** `__builtin_bswap32(v)` with v = -5495501120125551105 (low u32 after bswap = 0xFFBBD010 ≥ 2^31)
**Expected:** folded constant 4290586368 (0xFFBBD010, zero-extended)
**Actual:** folded constant -4380928 (I32 sign-extended)
**Severity:** medium (documented representation rule violated; basis: documented-and-violated)
**Repro seed:** deterministic
**Regression test:** src/common/const_eval.rs `pbt_regression::test_eval_builtin_bswap32_regression_unsigned_repr` (#[ignore] witness)
**Raw output:**
```
Test failed: assertion failed: `(left == right)`
  left: `Some(-4380928)`,
 right: `Some(4290586368)`: bswap32 v=-5495501120125551105
minimal failing input: v = -5495501120125551105
```
