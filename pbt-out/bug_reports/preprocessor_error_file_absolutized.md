# Bug: `#error`/`#warning` diagnostics report an absolutized file path, not the input filename

**Law:** `PreprocessorDiagnostic::file` must be the filename the caller gave (`set_filename`),
matching `__FILE__` in the same run and gcc's `t.c:5:2: error:` diagnostic format. The SUT
reports the absolutized include-stack entry instead.

**Impact:** Every user-facing diagnostic rendered by the driver
(`src/driver/pipeline.rs:388`: `eprintln!("{}:{}:{}: error: {}", err.file, ...)`) prints a
long absolute host path where gcc prints the input path, and the value contradicts `__FILE__`
("t.c") produced by the same preprocessor instance for the same line.

**Function:** `Preprocessor::current_file` (fed by `set_filename`'s absolutized include-stack push)
**Source location:** /home/shuhao/fermat-users/leo/github/claudes-c-compiler/src/frontend/preprocessor/pipeline.rs:649 (`current_file`), pipeline.rs:632 (`make_absolute` push)
**Detected by:** Negative/error contract (P10)
**Minimal input:** `Preprocessor::new(); set_filename("t.c"); preprocess("#error boom\n")`
**Expected:** `errors()[0].file == "t.c"` (like `__FILE__` and gcc)
**Actual:** `errors()[0].file == "/home/shuhao/fermat-users/leo/github/claudes-c-compiler/t.c"`
**Severity:** low

Fix direction: `current_file` should return `self.filename` when the include stack's top is the
main file (or `set_filename` should remember the raw name alongside the absolutized stack entry).

**Reproduction:**
```bash
cd /home/shuhao/fermat-users/leo/github/claudes-c-compiler/pbt-out/rounds/03_preprocessor/run
PATH="$HOME/.cargo/bin:$PATH" cargo test --lib frontend::preprocessor::pipeline::pbt_regression::test_preprocess_regression_error_file_field
```
**Regression test:** src/frontend/preprocessor/pipeline.rs `pbt_regression::test_preprocess_regression_error_file_field` (currently failing — the witness)
**Repro seed:** 91fb0a610d8dda313385f440f9d9cc9053400d990e8cb6a6e687579b1df43b6 (proptest, P10)
