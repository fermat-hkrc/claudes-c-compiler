# Bug: enum with explicit i64::MAX value panics with integer overflow

**Law:** A legal token stream must never crash the compiler; gcc diagnoses
`enum { A = 9223372036854775807LL, B };` with "error: overflow in enumeration values".
**Impact:** Panic in debug builds (`attempt to add with overflow`) — a crash on validly
tokenized input; in release builds the counter silently wraps to i64::MIN. Either way
the documented information-gathering contract ("collect as much information as
possible", README) is broken by a hard crash.
**Function:** `Parser` enum-variant processing (`next_value = val + 1`) and
`SemanticAnalyzer::process_enum_variants` (`enum_counter += 1`)
**Source location:** /home/shuhao/fermat-users/leo/github/claudes-c-compiler/src/frontend/parser/types.rs:828
**Detected by:** Differential — C11 6.7.2.2 model through the pipeline (P3); gcc 9.4 ground truth
**Minimal input:** `enum { A = 9223372036854775807LL };`
**Expected:** a diagnostic (gcc: "overflow in enumeration values"), no crash
**Actual:** panic `attempt to add with overflow` at parser/types.rs:828 (`val + 1`);
the sema-side counter (analysis.rs:717 `enum_counter += 1`) has the same defect
**Severity:** medium

**Reproduction:**
```bash
cd /home/shuhao/fermat-users/leo/github/claudes-c-compiler/pbt-out/rounds/05_sema/run
PATH="$HOME/.cargo/bin:$PATH" cargo test --lib frontend::sema::analysis::pbt_regression::test_enum_regression_i64_max_counter_overflow
```
**Regression test:** src/frontend/sema/analysis.rs `pbt_regression::test_enum_regression_i64_max_counter_overflow` (intentionally red)
