# Bug: Stack overflow (process abort) on a run of unknown non-ASCII characters

**Issue Synopsis**
`Lexer::lex_punctuation`'s unknown-character branch (`src/frontend/lexer/scan.rs:1173`)
handles each unrecognized multibyte character by recursing: `return self.next_token();`.
One recursion level is consumed **per character**, so a run of ~4000+ unknown non-ASCII
bytes (e.g. a UTF-8 blob outside comments/strings, or mis-encoded source) exhausts a 2 MiB
thread stack and aborts the process (`fatal runtime error: stack overflow, SIGABRT`).
Measured threshold on this machine: 2000 chars OK, 4000 chars crash. The in-code comment
("skip any remaining bytes of a multi-byte UTF-8 sequence ... and continue tokenizing")
states the intent is to *continue*, not to recurse once per character.

**Detection and Validation Methodology**
Crash-only oracle (P10 justification: the API has no error channel; termination is the only
statable contract) plus a dedicated depth probe run in isolation because the crash kills
the test process. `probe_unknown_char_stack_crash` (ignored by default — it aborts) holds
the 200_000-char witness; `probe_unknown_char_stack_depth_safe` pins the sub-threshold
behavior (1..2000 chars → terminates, ends with Eof).

**Reproduction Protocol**
```bash
cd /home/shuhao/fermat-users/leo/github/claudes-c-compiler/pbt-out/rounds/02_lexer/run
PATH="$HOME/.cargo/bin:$PATH" cargo test --lib frontend::lexer::scan::pbt_regression::probe_unknown_char_stack_crash -- --ignored --exact
# thread '...' has overflowed its stack / fatal runtime error: stack overflow, aborting
```

**Remediation Strategy**
Replace the recursion with iteration: loop over `next_token`-equivalent skipping (advance
`self.pos` past the unknown char + continuation bytes, then `continue` an outer loop), or
convert the unknown-char handling to consume the whole run of unknown characters in a
`while` loop before returning to `tokenize`'s loop.

**Law:** tokenize(s) terminates for every input string (no stack exhaustion).
**Impact:** robustness/DoS — a ~3 KB run of non-ASCII garbage (invalid C, but silently accepted input today) crashes the compiler instead of being skipped.
**Function:** Lexer::lex_punctuation
**Source location:** /home/shuhao/fermat-users/leo/github/claudes-c-compiler/src/frontend/lexer/scan.rs:1173
**Detected by:** Crash-only (termination contract) + isolated depth probe
**Minimal input:** `"\u{00FF}".repeat(4000)` (2 MiB stack; crashes; 2000 passes)
**Expected:** tokenize terminates, ends with Eof
**Actual:** `fatal runtime error: stack overflow, aborting` (SIGABRT)
**Severity:** medium

**Reproduction:**
```bash
cd /home/shuhao/fermat-users/leo/github/claudes-c-compiler/pbt-out/rounds/02_lexer/run
PATH="$HOME/.cargo/bin:$PATH" cargo test --lib frontend::lexer::scan::pbt_regression::probe_unknown_char_stack_crash -- --ignored --exact
```
**Regression test:** src/frontend/lexer/scan.rs `pbt_regression::probe_unknown_char_stack_crash` (#[ignore]d — aborts the process; run isolated)
