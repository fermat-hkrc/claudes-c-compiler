# Bug: parse_integer_expr panics in debug on arithmetic overflow and out-of-range shifts instead of returning Err
> **Filed as [issue #765](https://github.com/fermat-hkrc/claudes-c-compiler/issues/765)**

**Law:** `parse_integer_expr(s: &str) -> Result<i64, String>` has an error channel: any input the evaluator cannot handle must produce `Err`, never a panic. Detected during round 08's P8 differential (an unbounded generated expression hit the panic before the domain was bounded).

**Impact:** Debug builds panic (compiler crash on a malformed/oversized asm constant expression — e.g. a user macro computing `1 << 64` or a multiplication overflow in `.byte`/`.align`-style directives); release builds wrap silently — `1 << 64` evaluates to `1` (matches llvm-mc's silent wrap, but GNU `as` warns "shift count out of range" and yields 0). The debug/release divergence is itself a hazard: a constant that crashes a debug compiler silently changes value in release.

**Function:** `parse_integer_expr` (eval_tokens' mul/shift levels)
**Source location:** /home/shuhao/fermat-users/leo/github/claudes-c-compiler/src/backend/asm_expr.rs:181 (`val *= eval_add(...)`, plain i64 `*=`); the shift level (`val <<= n`, `(val as u64) >> n` — plain shifts, no amount check)
**Detected by:** Differential (P8 p8_asmexpr_parse_matches_evaluator, round 08) — the initial failure "attempt to multiply with overflow" / "attempt to shift left with overflow"; extended into the deterministic red regression below.
**Minimal input:** `parse_integer_expr("9223372036854775807 * 9223372036854775807")` → debug panic; `parse_integer_expr("1 << 64")` → debug panic.
**Expected:** `Err(...)` for both.
**Actual:** debug panic (`attempt to multiply with overflow` / `attempt to shift left with overflow`); release wraps silently (1<<64 → 1).
**Severity:** low (debug-only panic; release matches llvm-mc's wrap but diverges from GAS's warn+0 — a warning or Err would be strictly better)
**Contract evidence:** inferred (the `Result` return type is the error contract: inputs the parser rejects must yield Err; every other invalid-input path — bad tokens, trailing junk, div-by-zero — already returns Err)
**Fix:** checked arithmetic in eval_tokens: `val.checked_mul(rhs).ok_or("multiplication overflow")?` and validate shift amounts `< 64` (or wrap with a GAS-style diagnostic); apply to both shift directions.
**Regression test:** src/backend/asm_expr.rs `pbt_regression::test_asmexpr_regression_arith_panic_not_err` (red witness)

**Reproduction:**
```bash
cd /home/shuhao/fermat-users/leo/github/claudes-c-compiler
PATH="$HOME/.cargo/bin:$PATH" RUST_TEST_THREADS=1 cargo test --lib backend::asm_expr::pbt_regression::test_asmexpr_regression_arith_panic_not_err --exact --nocapture
```
**Raw output:**
```
thread 'backend::asm_expr::pbt_regression::test_asmexpr_regression_arith_panic_not_err' panicked:
attempt to multiply with overflow
```
