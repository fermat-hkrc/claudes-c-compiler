# Bug: `, ## __VA_ARGS__` deletes the comma for a supplied-but-empty variadic argument

**Law:** The GNU comma-swallow extension `g(fmt, ## __VA_ARGS__)` removes the comma only when
the variadic arguments are entirely absent (`VAC(a)`); when an (empty) variadic argument is
explicitly supplied (`VAC(a, )`), the comma must survive — gcc 9.4 keeps it.

**Impact:** Any macro invocation that explicitly passes an empty trailing argument (common in
generated code and wrapper macros) silently changes its expansion arity: `g(a, )` becomes
`g(a)`. Downstream C parses differently (wrong argument count), with no diagnostic.

**Function:** `MacroTable::handle_stringify_and_paste` (with `MacroTable::get_va_args`)
**Source location:** /home/shuhao/fermat-users/leo/github/claudes-c-compiler/src/frontend/preprocessor/macro_defs.rs:856 (first `if va_args.is_empty()` comma-delete site; `get_va_args` at macro_defs.rs:1121)
**Detected by:** Differential — gcc 9.4 token-stream differential (P8)
**Minimal input:** `#define VAC(fmt, ...) g(fmt, ## __VA_ARGS__)` then `VAC(a, )`
**Expected:** `g(a, )` — tokens `[g ( a , )]` (gcc -E -P output)
**Actual:** `g(a)` — tokens `[g ( a )]`
**Severity:** medium

The root cause is representational: `get_va_args` collapses `args.len() > named_count` with a
single empty argument into the same `""` as "no variadic arguments", and the `##` handler then
deletes the comma on `va_args.is_empty()`. Distinguishing `args.len() > named_count` (argument
supplied, possibly empty) from `args.len() == named_count` (absent) restores gcc behavior.

**Reproduction:**
```bash
cd /home/shuhao/fermat-users/leo/github/claudes-c-compiler/pbt-out/rounds/03_preprocessor/run
PATH="$HOME/.cargo/bin:$PATH" cargo test --lib frontend::preprocessor::pipeline::pbt_regression::test_preprocess_regression_va_args_empty_arg_keeps_comma
# oracle: printf '#define VAC(fmt, ...) g(fmt, ## __VA_ARGS__)\nVAC(a, )\n' > vac.c && gcc -E -P vac.c
```
**Regression test:** src/frontend/preprocessor/pipeline.rs `pbt_regression::test_preprocess_regression_va_args_empty_arg_keeps_comma` (currently failing — the witness)
**Repro seed:** c7181e76815e7dc40cd26a6bb5266921f7b0666afb675d8c1aa910811cb95e93 (proptest, P8)
