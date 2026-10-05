# Invariants confirmed — round 06 (src/ir)

- IrBinOp::eval_i64/eval_i128 satisfy the C11 6.5.5 division identity and the
  None ⇔ rhs==0 error contract (1024 cases each).
- eval_i128 agrees with eval_i64 on non-negative operands whenever the exact
  result fits i64 (shifts need k < 64); they legitimately diverge on negative
  operands for UDiv/URem/LShr (width-specific bit reinterpretation).
- IrCmpOp: signed/unsigned float variants agree on ALL f64 including NaN (IEEE);
  the Slt==!Sge duality laws hold only for non-NaN operands.
- IrConst::from_i64 is the authoritative writer of the U8/U16/U32 zero-extension
  convention; cast_float_to_target, cast_long_double_to_target and
  coerce_to_with_src currently violate it (bugs b1/b2 — red until fixed).
- f64_to_f128_bytes / f64_to_x87_bytes are exact for normals, ±0, ±inf, NaN;
  subnormals are misencoded (bug b3).
- Environment quirks: tests run from pbt-out/rounds/06_ir/run (scratch CWD);
  `use proptest::prelude::*` + `prop!` macro name clash — use `proptest!` (repo
  convention); `FlatAdj::from_vecs_usize` and `IrFunction::new` are #[cfg(test)]
  constructors usable from any inline test module.
- mem2reg: unused allocas are NOT promoted and their Alloca instructions
  legitimately survive; next_value_id stays 0 ("not yet computed") when nothing
  was promoted; unreachable blocks keep dead phis with empty incoming lists.
- No in-tree producer creates CFG edges INTO the entry block (lower_label_stmt
  always terminates + starts a fresh block) — relevant to bug b5's reachability.
