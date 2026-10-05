# Bug: coerce_to_with_src early-return keeps the forbidden sign-extending variant for U8/U16 targets
**Law:** Same storage convention as b1: coercing a constant to an unsigned sub-64-bit type must yield the zero-extended representation (`to_i64()` reads back the unsigned value). `from_i64(v, I8)` followed by `.coerce_to(U8)` and `from_i64(v, U8)` are two writers of the same "U8 constant with value v" and must agree.
**Impact:** Coercion is the universal "match the constant to the instruction type" step in the optimizer; for values above the signed range (U8 128–255, U16 32768–65535) the coerced constant reads back negative through `to_i64()`, corrupting constant folding exactly like b1 but through the coercion path (e.g. signed→unsigned assignment/conversion of constants).
**Function:** IrConst::coerce_to_with_src (and its wrapper coerce_to)
**Source location:** /home/shuhao/fermat-users/leo/github/claudes-c-compiler/src/ir/constants.rs:384
**Detected by:** Differential — same-job writers (from_i64 vs coerce_to), property P6
**Minimal input:** IrConst::from_i64(200, IrType::I8).coerce_to(IrType::U8)
**Expected:** to_i64() == Some(200) (I64(200) per the convention)
**Actual:** IrConst::I8(-56), to_i64() == Some(-56) — the `(IrConst::I8(_), IrType::I8 | IrType::U8) => return *self` early-return arm keeps the forbidden variant
**Severity:** high
**Doc contract:** src/ir/constants.rs:437 "Store unsigned sub-64-bit types (U8, U16, U32) as I64 with zero-extended values to preserve unsigned semantics." (fingerprint 86a284c8)
**Fix:** Split the early-return arms: `(I8(_), I8)` may return *self, but `(I8(_), U8)` must fall through to `from_i64` normalization (same for I16/U16). Note `narrowed_to` (constants.rs:579) already implements the correct behavior — the two same-job functions disagree.
**Regression test:** src/ir/constants.rs `pbt_regression::test_ir_const_regression_coerce_u8_early_return`

**Reproduction:**
```bash
cd /home/shuhao/fermat-users/leo/github/claudes-c-compiler
PATH="$HOME/.cargo/bin:$PATH" cargo test --lib ir::constants::pbt_regression::test_ir_const_regression_coerce_u8_early_return
```
**Raw output:**
```
Test failed: assertion failed: `(left == right)`
  left: `Some(-128)`, right: `Some(128)`: I8(-128).coerce_to(U8) = I8(-128) reads back -128
minimal failing input: kind = 0, raw = 0
```
