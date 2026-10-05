# PBT Campaign Report: src/frontend/sema (round 05)

## Summary

**Verdict:** 5 confirmed SUT bugs — worst is high severity: `usual_arithmetic_conversion` types `unsigned long + long long` as SIGNED `LongLong` (C11 6.3.1.8 rule 5), so mixed 64-bit unsigned arithmetic compares/divides/shifts with wrong signedness; plus a global enum constant permanently clobbered by a function-local `enum { E = ... }` (silent wrong constants), an i64::MAX enum-value PANIC, an undo-log same-scope re-insert resurrection, and unsigned unary-minus not wrapping in const-eval.
**Date:** 2026-10-05
**Repository:** /home/shuhao/fermat-users/leo/github/claudes-c-compiler
**Modules tested:** src/frontend/sema (analysis, type_checker, type_context, const_eval, builtins) + change-surface obligations in parser/preprocessor
**Tests:** 15 properties + 5 deterministic regression witnesses + 3 KAT gates
**Result:** 10 passing, 5 failing (all 5 triaged as SUT bugs b1–b5)
**Change surface:** 3 changed functions, 2 with a property (parse_src → P14 failure-path; sut_tokens → P15 success+failure-injection), 1 skipped with reason (p9_split_first_word_contract — itself a round-04 proptest, re-executed in the probe run); 1 error-handling change with a failure-path property (sut_tokens via malformed-directive injection; parse_src via guaranteed-malformed mutations)
**Coverage evidence:** file-level (symbol presence) — no line-level data: this machine has neither gcovr nor lcov, so no coverage instrumentation was active (see COVERAGE_STATUS.md). `coverage_gaps` reported the 3 change-surface symbols as NOT LINKED (its symbol probe cannot see `#[cfg(test)]`-nested Rust functions); all three were in fact executed: P14 calls parse_src directly, P15 calls sut_tokens directly, and round-04's p9 ran green in the probe (554-passing baseline).
**Tier:** standard (30-min budget, ≥1000 generator runs per property, 1 coverage-driven sweep round — done)

## Modules Tested

| Module | Tests | Bugs | Oracles Used |
|--------|-------|------|--------------|
| sema/type_checker.rs | 2 props + KAT + regression | b1 | differential (C11 model + gcc KAT), algebraic |
| sema/analysis.rs | 4 props + KAT + 2 regressions | b2, b3 | differential (scope/enum models, -Wreturn-type rule model), negative_error |
| sema/type_context.rs | 1 prop + regression | b4 | state_machine |
| sema/const_eval.rs | 3 props + KAT | b5 | differential (C-semantics evaluator + gcc KAT), reference (SysV ABI) |
| sema/builtins.rs | 2 props | — | differential (documented families), algebraic |
| parser/parse.rs (change surface) | 1 prop | — | crash_only failure-path |
| preprocessor/pbt_support.rs (change surface) | 1 prop | — | algebraic + failure-injection |

## Bugs Found

### B1: `unsigned long + long long` typed as signed LongLong (C11 6.3.1.8 rule 5)
**Formal:** ∀ t1,t2 ∈ CScalarTypes², op ∈ BinOps. expr_types[(t1 a; t2 b; int p = a op b;).init] = C11_uac(promote(t1), promote(t2), op)
**Contract evidence:** inferred (C11 6.3.1.8 rule 5: same-width higher-rank signed type cannot represent all unsigned values → unsigned counterpart; gcc 9.4 semantics check on this box agrees)
**Documentation conflict:** (none — the code comment at types.rs:1578 claims "The signed type has higher rank and can represent all values", which is exactly the unverified assumption; no spec excludes these operands)
**Severity:** high
**Counterexample:** program `unsigned long a; long long b; int p = a + b;` — the initializer's annotated type is read from `expr_types`
**Expected / Actual:** `ULongLong` / `LongLong`
**Impact:** mixed size_t/unsigned-long ↔ long-long arithmetic is computed with signed semantics in sema annotations (and the shared `usual_arithmetic_conversion` downstream): comparisons against 0, division, and shifts of wrapped unsigned values all diverge from gcc.
**Root cause:** src/common/types.rs:1576-1580 — final else branch returns the signed type on rank alone, without the size check.
**Bug report:** bug_reports/b1_uac_unsigned_long_plus_long_long_signed.md
**Repro seed:** `cc 48bbef9a3ec8b0f195fbbf057d5231a588f8505d53adc3f15dff5742b5eb6284` (proptest-regressions/frontend/sema/type_checker.txt)
**Raw output:** `left: LongLong, right: ULongLong: src: unsigned long a; long long b; int p = a + b;`

### B2: function-local enum constant permanently clobbers the global one
**Formal:** ∀ (g,l). analyze("enum { E = g; }; void f(void){ enum { E = l; } } enum { F = E + 0 };") ⇒ enum_constants[F] = g
**Contract evidence:** documented src/frontend/sema/type_context.rs:376-378 — "Pop the top type-system scope frame and undo changes to enum_constants, struct_layouts, ctype_cache, and typedefs." (the pop fails to undo the shadowed enum constant)
**Documentation conflict:** the doc on pop_scope promises the undo; the code has no `enums_shadowed` restore. The comment states the behavior IS handled — it is the contract the code violates (not a limitation notice).
**Severity:** high
**Counterexample:** `enum { E = 1 }; void f(void) { enum { E = 2 }; } enum { F = E + 0 };` → `enum_constants["F"]`
**Expected / Actual:** `1` / `2`
**Impact:** silent wrong compile-time constants for every use of the global enumerator after any function that locally redefines the name; also the parser's own enum map is affected downstream.
**Root cause:** insert_enum_scoped (type_context.rs:422) tracks only first-insert keys; TypeScopeFrame has no enums_shadowed list (typedefs/layouts/alignments all restore).
**Bug report:** bug_reports/b2_enum_constant_scope_shadow_leak.md
**Repro seed:** `cc 23805cb69dbbf1c75582a5c6cc72fbba3333394307828674ae4687785976d13d` (shrinks to g = 1, l = 2)
**Raw output:** `left: Some(2), right: Some(1): enum constant F leaked inner E=2`

### B3: enum with explicit i64::MAX value panics with integer overflow
**Formal:** ∀ variants ∈ VariantList. analyze("enum { A₀[=e₀], … }") ⇒ ∀i. enum_constants[Aᵢ] = eᵢ if explicit else prev+1 — no panic on any legal token stream
**Contract evidence:** inferred (gcc 9.4 diagnoses "overflow in enumeration values" for the same input — a diagnostic, never a crash; README promises information gathering, not rejection-by-panic)
**Documentation conflict:** (none)
**Severity:** medium
**Counterexample:** `enum { A = 9223372036854775807LL };`
**Expected / Actual:** diagnostic like gcc / panic "attempt to add with overflow" at parser/types.rs:828 (`val + 1`); the sema-side counter (analysis.rs:717) has the same defect
**Impact:** debug-build panic (crash on validly tokenized input); release builds silently wrap the counter to i64::MIN.
**Root cause:** unchecked `+ 1` on the enum counter in both the parser's variant processing and sema's process_enum_variants.
**Bug report:** bug_reports/b3_enum_counter_i64_max_overflow_panic.md
**Repro seed:** deterministic — see regression test (panic reproducers have no shrunk seed)
**Raw output:** `panicked at src/frontend/parser/types.rs:828:35: attempt to add with overflow`

### B4: undo-log resurrects a same-scope re-inserted value after pop
**Formal:** Automaton — states: layered scope maps; ops: Push/Pop/Insert{Typedef,Enum,Align,Layout,CacheInvalidate}; invariant: after each op every map equals the layered model
**Contract evidence:** inferred (symmetry: pop_scope restores typedefs/alignments/layouts — the same guarantee class must hold when a key is re-inserted within one scope; README: undo-log exists so that "local struct definitions inside a function body do not overwrite global layouts")
**Documentation conflict:** (none)
**Severity:** medium
**Counterexample:** ops `[push_scope, insert_typedef_alignment("kB",0), insert_typedef_alignment("kB",0), pop_scope]` → `typedef_alignments` still contains `kB`; pipeline shape: `void f(void){ typedef int T; typedef long T; }` leaks `T = long`
**Expected / Actual:** `kB` absent after pop / `kB == Some(0)`
**Impact:** same-scope redeclarations (accepted by design) leak their inner value past the function/block scope, poisoning later typedef/layout resolution.
**Root cause:** insert records "shadowed" whenever the key exists in the flat map, even when the existing value came from THIS frame; pop then removes (added) and re-inserts (shadowed). Affects typedefs, alignments, layouts, ctype_cache.
**Bug report:** bug_reports/b4_undolog_double_insert_resurrection.md
**Repro seed:** `cc 43047567ca57c611113d8cbde5340dfea3f3c34c124872ba5bae7de054b959a1` (shrinks to [Push, Al(1, 0), Al(1, 0), Pop])
**Raw output:** `right: None: typedef_alignments kB diverged at op #3 (Pop): ops=[Push, Al(1, 0), Al(1, 0), Pop]`

### B5: unary minus on unsigned constants does not wrap (SemaConstEval)
**Formal:** ∀ e ∈ Expr. const_values[(int p = e;).init] == model_c_eval(e) under C11 6.5 semantics (unsigned ops wrap modulo 2^width)
**Contract evidence:** inferred (C11 6.5.3.3p4: unsigned negation is modulo 2^n; gcc 9.4 folds `-((unsigned)(0 + -1))` to `1u` — verified by static assert on this box)
**Documentation conflict:** the SIBLING BitNot arm documents the exact storage requirement and carries the fix-up ("For unsigned int operands (stored as I64 ...), the bitwise NOT must be truncated to 32 bits. Without this, ~0u produces I64(-1) ... instead of I64(0xFFFFFFFF)." const_eval.rs:110-125); the Neg arm lacks the analogous wrap — an internal-consistency contract the code itself asserts elsewhere. `(not independently verified for the Neg arm — no comment speaks to it; it is the absence of the sibling's documented discipline that is the defect)`
**Severity:** medium
**Counterexample:** `int p = -((unsigned)(0) + (-1));` → `const_values[p]`
**Expected / Actual:** `1` / `-4294967295`
**Impact:** enum values, array sizes, and static initializers built from negated unsigned constant expressions get negative values instead of the wrapped unsigned ones.
**Root cause:** const_eval.rs UnaryOp::Neg negates the promoted I64-stored value without re-wrapping to the operand's unsigned width.
**Bug report:** bug_reports/b5_unsigned_negation_const_eval_no_wrap.md
**Repro seed:** deterministic — see regression test
**Raw output:** `left: Some(-4294967295), right: Some(1): expr: (-((unsigned)(0) + (-1)))`

## Design Caveats (if any)

(none)

## Test Files Created

| File | Tests |
|------|-------|
| src/frontend/sema/type_checker.rs (mod pbt_tests + pbt_regression) | p1 (1024), p2 (1024), p1_kat_gcc, regression-b1 |
| src/frontend/sema/analysis.rs (mod pbt_tests + pbt_regression) | p3 (1024), p5 (1024), p6 (1024), p12 (1024), p13 (1024), p6_kat, regressions-b2/b3 |
| src/frontend/sema/type_context.rs (mod pbt_tests + pbt_regression) | p4 (1024), regression-b4 |
| src/frontend/sema/const_eval.rs (mod pbt_tests + pbt_regression) | p7 (1024), p9 (1024), p8 (enumeration), p7_kat_gcc, regression-b5 |
| src/frontend/sema/builtins.rs (mod pbt_tests) | p10 (1024), p11 (1024) |
| src/frontend/parser/parse.rs (mod pbt_tests, extended) | p_r05_parse_src_failure_path (1024) |
| src/frontend/preprocessor/pbt_support.rs (mod round05_tests) | p15_sut_tokens_markers_stripped (1024) |

## Reproduction

Whole suite (from scratch CWD):
```bash
cd /home/shuhao/fermat-users/leo/github/claudes-c-compiler/pbt-out/rounds/05_sema/run
PATH="$HOME/.cargo/bin:$PATH" RUST_TEST_THREADS=1 cargo test --lib frontend::sema
```
Bugs (one per line, copy-pasteable):
```bash
cd /home/shuhao/fermat-users/leo/github/claudes-c-compiler/pbt-out/rounds/05_sema/run
PATH="$HOME/.cargo/bin:$PATH" cargo test --lib frontend::sema::type_checker::pbt_regression::test_usual_arith_conversion_regression_ul_plus_ll
PATH="$HOME/.cargo/bin:$PATH" cargo test --lib frontend::sema::analysis::pbt_regression::test_enum_scope_regression_shadow_leak
PATH="$HOME/.cargo/bin:$PATH" cargo test --lib frontend::sema::analysis::pbt_regression::test_enum_regression_i64_max_counter_overflow
PATH="$HOME/.cargo/bin:$PATH" cargo test --lib frontend::sema::type_context::pbt_regression::test_typecontext_regression_double_insert_resurrection
PATH="$HOME/.cargo/bin:$PATH" cargo test --lib frontend::sema::const_eval::pbt_regression::test_const_eval_regression_unsigned_negation_no_wrap
```
Property-level replays (shrunk counterexamples, proptest persistence):
```bash
cd /home/shuhao/fermat-users/leo/github/claudes-c-compiler/pbt-out/rounds/05_sema/run
PATH="$HOME/.cargo/bin:$PATH" cargo test --lib frontend::sema::type_checker::pbt_tests::p1_uac_binop_ctype
PATH="$HOME/.cargo/bin:$PATH" cargo test --lib frontend::sema::analysis::pbt_tests::p3_enum_variant_values
PATH="$HOME/.cargo/bin:$PATH" cargo test --lib frontend::sema::type_context::pbt_tests::p4_scope_undo_state_machine
PATH="$HOME/.cargo/bin:$PATH" cargo test --lib frontend::sema::analysis::pbt_tests::p5_enum_scope_shadow_restore
PATH="$HOME/.cargo/bin:$PATH" cargo test --lib frontend::sema::const_eval::pbt_tests::p7_const_arith_c_semantics
```

## Output Directories

- pbt-out/rounds/05_sema/ — this round's artifacts: PLAN.md, PROPERTIES.md, report.json, COVERAGE.md, COVERAGE_STATUS.md, FUNCTION_INDEX.md, INVARIANTS.md, CHANGE_SURFACE.md, change-surface.json, build.log, dependencies.json, bug_reports/b1..b5 (.md), run/ (scratch CWD for all runs). The campaign summary REPORT.md itself lives at pbt-out/REPORT.md (close-out ledger location enforced by the harness).
- pbt-out/ — top-level ledger holds this round's state: REPORT.md, REPORT.html, report.json, PLAN.md, PROPERTIES.md, COVERAGE.md, COVERAGE_STATUS.md, INVARIANTS.md, FUNCTION_INDEX.md (sema entries appended), plan.md, bug_reports/ (b1–b5 .md + auto-rendered .html)
- Test code itself lives in the repository tree (inline `#[cfg(test)]` mods, rung 1)

## Coverage Report

# PBT Coverage Status

> Last updated: 2026-10-05 10:19 (campaign: coverage)
> Files: 6/6 scanned (100%) | Functions: 14/116 total | PBT candidates: 14 | Tested: 14 (100%) | 6 pass, 7 fail, 1 other

## Summary

| Metric | Value |
|--------|-------|
| Total source files | 6 |
| Files scanned | 6 / 6 (100%) |
| Total functions (all files) | 116 |
| PBT candidates (from FUNCTION_INDEX) | 14 |
| **Tested (of PBT candidates)** | **14 / 14 (100%)** |
| &nbsp;&nbsp;↳ Pass / Fail / Other | 6 / 7 / 1 |
| **Overall (tested / all functions)** | **14 / 116 (12%)** |
| Untested | 0 |
| Skipped | 0 |

## Module Breakdown

| Module | Scanned | Tested | Skipped | Coverage |
|--------|---------|--------|---------|----------|
|  | 14 | 14 | 0 | 100% |

## Oracle Type Distribution

| Oracle Type | Total | Covered | Skipped | Coverage |
|-------------|-------|---------|---------|----------|
| unknown | 14 | 14 | 0 | 100% |

## File Coverage

| Source File | Funcs | Candidates | Tested | Coverage | Status |
|-------------|-------|------------|--------|----------|--------|
| analysis.rs | 28 | 23 | 4 | 17% | partial |
| builtins.rs | 6 | 5 | 2 | 40% | partial |
| const_eval.rs | 10 | 7 | 2 | 29% | partial |
| type_checker.rs | 6 | 3 | 2 | 67% | partial |
| type_context.rs | 12 | 9 | 1 | 11% | partial |
| analysis.rs / type_context.rs / type_checker.rs / const_eval.rs / builtins.rs | 1 | 0 | 0 | - | excluded |

## Files Not Yet Scanned (1)

| Source File | Module |
|-------------|--------|
| analysis.rs / type_context.rs / type_checker.rs / const_eval.rs / builtins.rs | analysis.rs  |

## Recommended Focus

> **Priority 1 — Fix failing tests**
> These functions have failing PBT properties — fix before adding new tests.

| Function | Source |
|----------|--------|
| infer_binop_ctype / infer_expr_ctype | type_checker.rs |
| process_enum_variants / collect_enum_constants_from_type_spec | analysis.rs |
| analyze / analyze_function_def / analyze_compound_stmt / analyze_stmt | analysis.rs |
| pop_scope / push_scope / insert_enum_scoped / insert_typedef_scoped / insert_typedef_alignment_scoped / insert_struct_layout_scoped / insert_struct_layout_scoped_from_ref / invalidate_ctype_cache_scoped(_from_ref) | type_context.rs |
| eval_const_expr (+cast/binop arms) | const_eval.rs |
| pbt_tests::parse_src (change surface) | parse.rs |
| sut_tokens (change surface) | pbt_support.rs |

> **Priority 3 — Scan uncovered files**
> 1 file(s) not yet scanned: analysis.rs  (1 files)
> Run `pi-pbt scan <dir>` to add them to FUNCTION_INDEX.md.
