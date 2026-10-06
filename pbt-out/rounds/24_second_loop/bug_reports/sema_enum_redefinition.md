# Bug: sema accepts same-scope enum redefinition (enum E { A }; enum E { A };) — C11 6.7.2.3 constraint violation
> **Filed as [issue #812](https://github.com/fermat-hkrc/claudes-c-compiler/issues/812)**

**Law:** C11 6.7.2.3p3: a redeclaration of the same tag in the same scope without an intervening closing brace is a constraint violation — gcc: "error: nested redefinition of 'enum E'".

**Impact:** accepts-invalid: programs with accidental enum redefinitions (typically merge/hdr-include accidents) compile silently; if the two definitions DISAGREE (different values for the same name), later uses silently take one of the definitions — a silent wrong-constant risk.

**Function:** sema enum-declaration handling (same-scope redefinition check missing)
**Source location:** src/frontend/sema/analysis.rs (enum declaration path — the same area as #560/#561)
**Detected by:** round-24 second-loop sema battery (10 gcc -fsyntax-only differential cases; 9 agree, this one diverges)
**Minimal input:** `enum E { A };\nenum E { A };\nint main(void){return A;}` — gcc: error; ccc: compiles (exit 0)
**Expected:** error (nested redefinition)
**Actual:** accepted
**Severity:** low (accepts-invalid; distinct from #560 scope-shadow leak — that was cross-scope)
**Contract evidence:** inferred (C11 6.7.2.3; gcc rejects with the constraint diagnostic)
**Fix:** in the enum declaration path, reject a second enumerator-list definition of a tag already completed in the SAME scope (inner-scope shadowing remains legal, cf. #560's fix).
**Regression test:** deterministic: ccc -fsyntax-only on the minimal input must fail.
