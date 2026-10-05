# PBT Campaign Report: src/frontend/lexer (ccc C compiler) — round 02

## Summary

**Verdict:** 5 confirmed lexer bugs, worst is **high** — hex floating constants wider than 16 hex digits (e.g. `0x10000000000000000p0`, exactly 2^64) silently evaluate to **0.0** (`u64::from_str_radix(...).unwrap_or(0)` in `lex_hex_float`); plus silent-0 for every integer literal over u64::MAX, a hex-float exponent that truncates to i32 (`0x1p4294967296` → 1.0 instead of inf), a stack-overflow abort on ~4000 consecutive unknown non-ASCII bytes, and an unterminated block comment leaking its last byte as a phantom token.
**Date:** 2026-10-05
**Repository:** /home/shuhao/fermat-users/leo/github/claudes-c-compiler
**Modules tested:** frontend::lexer (scan.rs, token.rs)
**Tests:** 29 (15 ledger properties + KAT + deterministic documented-behavior tests + regression witnesses)
**Result:** 20 passing, 8 failing (all 8 are the confirmed bugs' witnesses), 1 ignored (B4 process-abort witness)
**Change surface:** (no change source given — whole-module campaign per scope `src/frontend/lexer`)
**Coverage evidence:** file-level (symbol presence) — no line-level coverage on this machine (no gcovr/lcov, no profraw instrumentation; `coverage_gaps` reported every lexer symbol NOT LINKED, a false negative for Rust inline tests where the tests compile into the same binary — execution evidence is the test results themselves). Sweep round performed: README documented-behavior cross-check added P11/P12 (imaginary suffixes, surrogate fallback, string families), both passing.
**Tier:** standard (≈30-min budget, ≥1000 generator runs — 1024 used, 1 sweep round, ≥1 metamorphic/differential — P9 metamorphic + reference oracles P2/P4/P5/P6/P7/P8).

## Modules Tested

| Module | Tests | Bugs | Oracles Used |
|--------|-------|------|--------------|
| frontend::lexer (scan.rs) | 27 | 5 | reference (5), algebraic.round_trip (2), metamorphic (1), crash_only (2) |
| frontend::lexer (token.rs) | 2 (P2) | 0 | reference |

## Bugs Found

### B1: Hex float literals wider than 64 bits silently evaluate to 0.0
**Formal:** ∀ lit = 0x<hex 1..20 digits, ≥1 nonzero>p<−1000..1000>: value(lit) ≠ 0.0 ∧ ¬NaN
**Contract evidence:** documented README:181 — "Hex floats follow the C99 format 0x<int>.<frac>p<exp> and are converted via: value = (int_part + frac_part) * 2^exp" — the shipped formula has no overflow carve-out, and `0x10000000000000000p0` (2^64) is exactly representable and warning-free in GCC.
**Documentation conflict:** (none — no comment claims the collapse; `unwrap_or(0)` at scan.rs:242/244 is the producing statement with no purpose comment)
**Severity:** high
**Counterexample:** lex `0x10000000000000000p0` → `FloatLiteral(0.0)`
**Expected / Actual:** FloatLiteral(18446744073709551616.0) / FloatLiteral(0.0)
**Impact:** any program using a large hex-float constant silently compiles with 0.0 in its place — silent wrong-code.
**Root cause:** scan.rs:242 (`u64::from_str_radix(int_hex, 16).unwrap_or(0)`), same for frac at :244.
**Bug report:** bug_reports/lex_hex_float_wide_mantissa_zero.md
**Repro seed:** (deterministic witness)
**Raw output:** `Test failed: assertion failed: (left != right) left: 0.0, right: 0.0: nonzero literal collapsed to 0.0: lit="0x10000000000000000p0"`

### B2: Hex float exponent truncated to i32 / parse-overflow aliases to exponent 0
**Formal:** ∀ e ∈ {2^31, 2^32, 2^64, 10^20, negations}: value("0x1p"+e) = inf (e>0) / 0.0 (e<0)
**Contract evidence:** inferred (IEEE-754 overflow/underflow semantics of the README:181 formula; GCC accepts these literals with a range warning and yields inf/0)
**Documentation conflict:** (none)
**Severity:** medium
**Counterexample:** lex `0x1p4294967296` → `FloatLiteral(1.0)`; also `0x1p-4294967296` → 1.0 (expected 0.0)
**Expected / Actual:** FloatLiteral(inf) / FloatLiteral(1.0)
**Impact:** pathological-but-valid constants silently mis-evaluate.
**Root cause:** scan.rs:249 `(2.0_f64).powi(exp as i32)` (i64→i32 wrap) and scan.rs:235 `exp_str.parse().unwrap_or(0)`.
**Bug report:** bug_reports/lex_hex_float_exponent_truncation.md
**Repro seed:** (deterministic witness)
**Raw output:** `assertion failed: v.is_infinite() — got 1.0` (`0x1p4294967296`)

### B3: Integer literals larger than u64::MAX silently evaluate to 0 (all bases)
**Formal:** ∀ base ∈ {hex,bin,oct}, digits 1..20: u64_payload(tokenize(render(digits,base))) = u128(digits) mod 2^64
**Contract evidence:** inferred (GCC/Clang wraparound accumulation keeps the value mod 2^64 with a diagnostic; ccc's lexer has no diagnostic channel, so `.unwrap_or(0)` is silent data loss — and 0 is wrong under every candidate convention: mod gives 1, saturation gives u64::MAX for `0x10000000000000001`)
**Documentation conflict:** (none — README:191 only describes the happy path "parses via u64::from_str_radix(s, 2)")
**Severity:** medium
**Counterexample:** lex `0x10000000000000001` → `IntLiteral(0)`; decimal probes: `18446744073709551616` → 0, `99999999999999999999` → 0
**Expected / Actual:** IntLiteral(1) / IntLiteral(0)
**Impact:** oversized constants (hash/mask tables) silently become 0.
**Root cause:** scan.rs:199 (hex), :286 (bin), :314 (oct), :371 (dec) — `.unwrap_or(0)`.
**Bug report:** bug_reports/lex_int_u64_overflow_zero.md
**Repro seed:** (deterministic witness)
**Raw output:** `Test failed: left: 0, right: 1: text="0x10000000000000001"`

### B4: Stack overflow (process abort) on a run of unknown non-ASCII characters
**Formal:** ∀ n ≥ 1: tokenize("ÿ"×n) terminates and ends with Eof
**Contract evidence:** inferred (the in-code comment at scan.rs:1167 says "skip … and continue tokenizing" — continuation, not per-character recursion; `tokenize` must terminate for any `&str` it accepts)
**Documentation conflict:** scan.rs:1167 "Non-ASCII or unknown character: skip any remaining bytes of a multi-byte UTF-8 sequence (including PUA-encoded bytes from non-UTF-8 source files) and continue tokenizing." — states the intent (skip and continue); the recursion at :1173 violates the termination half of that intent. `(not independently verified: the code recurses once per character rather than continuing — verified by the crash itself)`
**Severity:** medium
**Counterexample:** `"\u{00FF}".repeat(4000)` → `fatal runtime error: stack overflow, aborting` (SIGABRT); 2000 passes (2 MiB thread stack)
**Expected / Actual:** tokenize terminates, ends with Eof / process aborts
**Impact:** robustness/DoS — ~3 KB of non-ASCII garbage outside comments/strings crashes the compiler.
**Root cause:** scan.rs:1173 `return self.next_token();` in `lex_punctuation`'s unknown-char branch — one stack frame pair per character.
**Bug report:** bug_reports/lex_punctuation_stack_overflow.md
**Repro seed:** (deterministic witness; test kept `#[ignore]`d because it aborts the process)
**Raw output:** `thread 'frontend::lexer::scan::pbt_regression::probe_unknown_char_stack_crash' has overflowed its stack / fatal runtime error: stack overflow, aborting`

### B5: Unterminated block comment leaks its last byte as a token
**Formal:** ∀ s ending in an unterminated `/*`: every byte after the comment start is consumed (tokenize(s) = tokens-before-comment ++ [Eof])
**Contract evidence:** documented README:158-160 — "Ignored by the lexer: … Block comments (`/*` to `*/`)" — a comment's bytes are ignored input, none may become a token
**Documentation conflict:** (none — no comment admits the leak; the loop bound at scan.rs:110 is the producing statement)
**Severity:** medium
**Counterexample:** lex `int /* gone` → `[Int, Identifier("e"), Eof]`
**Expected / Actual:** [Int, Eof] / [Int, Identifier("e"), Eof]
**Impact:** a file ending in `/* TODO` injects a phantom identifier `O` into the token stream — confusing downstream parser errors or silently-altered parses.
**Root cause:** scan.rs:110 block-comment loop `while self.pos + 1 < self.input.len()` stops one byte short when no `*/` appears.
**Bug report:** bug_reports/lex_comment_unterminated_leaks_last_byte.md
**Repro seed:** (deterministic witness)
**Raw output:** `left: [Int, Identifier("e"), Eof]  right: [Int, Eof]`

## Design Caveats (if any)

(none)

## Test Files Created

| File | Tests |
|------|-------|
| src/frontend/lexer/scan.rs — `mod pbt_tests` | 16 (P1..P12 + KAT gate) |
| src/frontend/lexer/scan.rs — `mod pbt_regression` | 13 (T1–T4, safe/crash depth probes, B1/B2/B3/B5 regression witnesses) |

## Reproduction

Whole suite (from scratch CWD so runners write nothing into the workspace):
```bash
cd /home/shuhao/fermat-users/leo/github/claudes-c-compiler/pbt-out/rounds/02_lexer/run
PATH="$HOME/.cargo/bin:$PATH" cargo test --lib frontend::lexer
```
Per bug (each deterministic, no seed needed):
```bash
cd /home/shuhao/fermat-users/leo/github/claudes-c-compiler/pbt-out/rounds/02_lexer/run
PATH="$HOME/.cargo/bin:$PATH" cargo test --lib frontend::lexer::scan::pbt_regression::test_lex_hex_float_regression_wide_mantissa_zero        # B1
PATH="$HOME/.cargo/bin:$PATH" cargo test --lib frontend::lexer::scan::pbt_regression::test_lex_hex_float_regression_exponent_truncation      # B2
PATH="$HOME/.cargo/bin:$PATH" cargo test --lib frontend::lexer::scan::pbt_regression::test_lex_int_regression_u64_overflow_zero             # B3
PATH="$HOME/.cargo/bin:$PATH" cargo test --lib frontend::lexer::scan::pbt_regression::probe_unknown_char_stack_crash -- --ignored --exact   # B4 (aborts — isolated)
PATH="$HOME/.cargo/bin:$PATH" cargo test --lib frontend::lexer::scan::pbt_regression::test_lex_comment_regression_unterminated_leaks_last_byte # B5
```

## Output Directories

- `pbt-out/` — canonical campaign set: this `REPORT.md`, auto-rendered `REPORT.html` (from `report.json`), `PROPERTIES.md`, `PLAN.md`, `COVERAGE.md`, `COVERAGE_STATUS.md`, `report.json`, `bug_reports/<slug>.md` + `<slug>.html` (one per confirmed bug).
- `pbt-out/rounds/02_lexer/` — round archive: `PLAN.md`, `PROPERTIES.md`, `COVERAGE.md`, `COVERAGE_STATUS.md`, `INVARIANTS.md`, `FUNCTION_INDEX.md`, `CHANGE_SURFACE.md`, `dependencies.json`, `guards.jsonl`, `build.log`, `bug_reports/*.md`, `run/` (scratch CWD).
- `proptest-regressions/` — framework-managed seed files at repo root (only pre-existing round-01 entries; this round's counterexamples all shrank to deterministic witnesses, no persisted seeds).
