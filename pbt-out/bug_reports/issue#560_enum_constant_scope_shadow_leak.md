# Bug: function-local enum constant permanently clobbers the global one (undo-log gap)

**Law:** A scope's enum-constant definitions are undone when the scope closes
(`pop_scope` doc: "undo changes to enum_constants, struct_layouts, ctype_cache, and
typedefs").
**Impact:** Any later function/constants in the TU silently see the leaked inner value:
`enum { E = 1 }; void f(void){ enum { E = 2 }; } enum { F = E + 0 };` yields F == 2
instead of 1 — a silent wrong compile-time constant for the rest of the file.
**Function:** `TypeContext::insert_enum_scoped` / `TypeContext::pop_scope` (via `SemanticAnalyzer::analyze`)
**Source location:** /home/shuhao/fermat-users/leo/github/claudes-c-compiler/src/frontend/sema/type_context.rs:422
**Detected by:** Differential — layered-scope model through the real pipeline (P5)
**Minimal input:** `enum { E = 1 }; void f(void) { enum { E = 2 }; } enum { F = E + 0 };`
**Expected:** `enum_constants["F"] == 1`
**Actual:** `enum_constants["F"] == 2`
**Severity:** high

Root cause: `insert_enum_scoped` records only FIRST-insert keys in `frame.enums_added`
(`!contains_key` guard); a key shadowing an outer value is never recorded, and
`TypeScopeFrame` has no `enums_shadowed` restore list — unlike typedefs, layouts, and
alignments, which all restore shadowed values on pop.

**Reproduction:**
```bash
cd /home/shuhao/fermat-users/leo/github/claudes-c-compiler/pbt-out/rounds/05_sema/run
PATH="$HOME/.cargo/bin:$PATH" cargo test --lib frontend::sema::analysis::pbt_regression::test_enum_scope_regression_shadow_leak
```
**Regression test:** src/frontend/sema/analysis.rs `pbt_regression::test_enum_scope_regression_shadow_leak` (intentionally red)
