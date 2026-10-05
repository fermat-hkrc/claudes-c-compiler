# Bug: unary minus on unsigned constants does not wrap (SemaConstEval)

**Law:** `-x` for unsigned x computes the unsigned negation, modulo 2^width
(gcc 9.4 folds `-((unsigned)(0 + -1))` to `1u`).
**Impact:** Any compile-time constant built by negating an unsigned expression — enum
values, array sizes, static initializers — gets a negative value instead of the
wrapped unsigned one: `enum { X = -((unsigned)(0 + -1)) }` becomes -4294967295 instead
of 1.
**Function:** `SemaConstEval::eval_const_expr` (UnaryOp::Neg path)
**Source location:** /home/shuhao/fermat-users/leo/github/claudes-c-compiler/src/frontend/sema/const_eval.rs:103
**Detected by:** Differential — independent C-semantics evaluator (P7); gcc 9.4 static-assert ground truth
**Minimal input:** `int p = -((unsigned)(0) + (-1));`
**Expected:** `const_values[p] == 1` (-(4294967295u) wraps to 1u)
**Actual:** `const_values[p] == -4294967295`
**Severity:** medium

Root cause: the Neg arm negates the promoted I64-stored value without re-wrapping to
the operand's unsigned width. The sibling BitNot arm carries an explicit 32-bit
truncation fix-up ("~0u produces I64(-1) ... instead of I64(0xFFFFFFFF)",
const_eval.rs:110-125) — Neg lacks the analogous wrap.

**Reproduction:**
```bash
cd /home/shuhao/fermat-users/leo/github/claudes-c-compiler/pbt-out/rounds/05_sema/run
PATH="$HOME/.cargo/bin:$PATH" cargo test --lib frontend::sema::const_eval::pbt_regression::test_const_eval_regression_unsigned_negation_no_wrap
```
**Regression test:** src/frontend/sema/const_eval.rs `pbt_regression::test_const_eval_regression_unsigned_negation_no_wrap` (intentionally red)
