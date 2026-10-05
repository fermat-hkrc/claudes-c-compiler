# Bug: cast_float_to_target / cast_long_double_to_target break the U8/U16/U32 zero-extension storage convention
**Law:** A constant of unsigned sub-64-bit type (U8/U16/U32) must be stored zero-extended so that `to_i64()` reads back the unsigned value — the convention `from_i64` documents and implements ("Store unsigned sub-64-bit types (U8, U16, U32) as I64 with zero-extended values to preserve unsigned semantics. Storing them in their native IrConst variants (I8, I16, I32) would cause to_i64() to sign-extend, turning e.g. U8(255) into -1 instead of 255.").
**Impact:** Every optimization pass that reads constants through `to_i64()` (constant folding, GVN value numbering via `to_hash_key`, algebraic simplification, division strength reduction) sees −56 instead of 200 for a U8 constant 200 produced by a float→integer cast — silently wrong constant folding for unsigned types whenever the value exceeds the signed range of the storage variant (U8 > 127, U16 > 32767). Byte emission (`to_le_bytes`) happens to stay correct, so the wrong value only surfaces in folded results, not raw data — the worst kind of silent divergence.
**Function:** IrConst::cast_float_to_target, IrConst::cast_long_double_to_target
**Source location:** /home/shuhao/fermat-users/leo/github/claudes-c-compiler/src/ir/constants.rs:277 (cast_float_to_target), :317 (cast_long_double_to_target)
**Detected by:** Differential — same-job writers (from_i64 vs cast_float_to_target vs cast_long_double_to_target), property P5
**Minimal input:** cast_float_to_target(200.0, IrType::U8)
**Expected:** IrConst::I64(200) — to_i64() == Some(200)
**Actual:** IrConst::I8(-56) — to_i64() == Some(-56)
**Severity:** high
**Doc contract:** src/ir/constants.rs:437 "Store unsigned sub-64-bit types (U8, U16, U32) as I64 with zero-extended values to preserve unsigned semantics." (fingerprint 86a284c8); the function's own doc even claims the right outcome: "converts via the unsigned type first to get correct wrapping behavior (e.g., 200.0 as u8 = 200, not saturated to i8 max)." (fingerprint a21e7eb9) — the wrap is bit-correct but the storage variant violates the read-back convention.
**Fix:** In both cast functions, store U8/U16 as `IrConst::I64(v as u8 as i64)` / `I64(v as u16 as i64)` (mirroring from_i64), matching the existing U32/U64 arms.
**Regression test:** src/ir/constants.rs `pbt_regression::test_ir_const_regression_cast_float_u8_repr`

**Reproduction:**
```bash
cd /home/shuhao/fermat-users/leo/github/claudes-c-compiler
PATH="$HOME/.cargo/bin:$PATH" cargo test --lib ir::constants::pbt_regression::test_ir_const_regression_cast_float_u8_repr
```
**Raw output:**
```
Test failed: assertion failed: `(left == right)`
  left: `Some(-128)`, right: `Some(128)`: cast_float_to_target(U8, 128) = I8(-128) reads back Some(-128)
minimal failing input: ty_idx = 0, raw = 272865408
```
