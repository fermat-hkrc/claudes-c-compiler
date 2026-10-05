# Bug: `unsigned long + long long` typed as signed LongLong (C11 6.3.1.8 rule 5 broken)

**Law:** In usual arithmetic conversions, when the higher-ranked signed type cannot
represent all values of the unsigned operand (same width), the result must be the
UNSIGNED counterpart of that signed type.
**Impact:** Every mixed `unsigned long`/`size_t` ↔ `long long` expression — comparisons,
division, shifts — is computed in SIGNED semantics by sema's ExprTypeChecker (and the
same `usual_arithmetic_conversion` is used lower in the pipeline), so constants like
`(size_t)0 - 1 + ll` compare `>= 0` and divide/shift differently than gcc. Common in
kernel-style size arithmetic.
**Function:** `CType::usual_arithmetic_conversion` (reached via `ExprTypeChecker::infer_binop_ctype`)
**Source location:** /home/shuhao/fermat-users/leo/github/claudes-c-compiler/src/common/types.rs:1576
**Detected by:** Differential — independent C11 6.3.1.8 model + gcc 9.4 semantics check (P1)
**Minimal input:** `unsigned long a; long long b; int p = a + b;` → `expr_types[p] == LongLong`
**Expected:** `ULongLong` (unsigned long long)
**Actual:** `LongLong`
**Severity:** high

gcc verification (this box, gcc 9.4): program testing `(ul_all - 1 + ll_zero) < 0`
returns "unsigned" semantics (exit 0), i.e. the sum type is unsigned per gcc.

Root cause: the final `else` branch of `usual_arithmetic_conversion` assumes "The signed
type has higher rank and can represent all values" without comparing SIZES; for
same-width higher-rank pairs (unsigned long vs long long on LP64) it must fall through
to the unsigned counterpart.

**Reproduction:**
```bash
cd /home/shuhao/fermat-users/leo/github/claudes-c-compiler/pbt-out/rounds/05_sema/run
PATH="$HOME/.cargo/bin:$PATH" cargo test --lib frontend::sema::type_checker::pbt_regression::test_usual_arith_conversion_regression_ul_plus_ll
```
**Regression test:** src/frontend/sema/type_checker.rs `pbt_regression::test_usual_arith_conversion_regression_ul_plus_ll` (intentionally red)
