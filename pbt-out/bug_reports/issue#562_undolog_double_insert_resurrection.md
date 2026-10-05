# Bug: undo-log resurrects a same-scope re-inserted value after pop

**Law:** Re-inserting a key within ONE scope must not make that key survive the
scope's exit: on pop, the key's pre-scope value (or absence) must be restored.
**Impact:** `void f(void){ typedef int T; typedef long T; }` leaks `T = long` past
the function scope (should be absent/outer value). Same-scope redefinitions are
accepted by design ("information gathering, not strict checking", README), so the
undo-log must handle them; instead the second insert records the FIRST INSERT'S OWN
value as a "shadowed outer value" and pop re-inserts it. Affects typedefs,
typedef_alignments, struct_layouts, and ctype_cache paths (all four share the pattern).
**Function:** `TypeContext::insert_typedef_alignment_scoped` (+ same pattern in
`insert_typedef_scoped`, `insert_struct_layout_scoped`, `insert_struct_layout_scoped_from_ref`, `invalidate_ctype_cache_scoped`)
**Source location:** /home/shuhao/fermat-users/leo/github/claudes-c-compiler/src/frontend/sema/type_context.rs:461
**Detected by:** State machine — layered-scope model, invariant checked after every op (P4)
**Minimal input:** ops `[push_scope, insert_align("kB",0), insert_align("kB",0), pop_scope]`
→ `typedef_alignments` still contains `kB`
**Expected:** `kB` absent after pop (it did not exist before push)
**Actual:** `kB == Some(0)` after pop — the scope's own value resurrected
**Severity:** medium

Root cause: the insert records `shadowed` whenever the key exists in the flat map,
even when the existing value was inserted by THIS frame; pop then removes (added) and
re-inserts (shadowed) the same key. Correct discipline: record added/shadowed only if
the key was not already touched in this frame.

**Reproduction:**
```bash
cd /home/shuhao/fermat-users/leo/github/claudes-c-compiler/pbt-out/rounds/05_sema/run
PATH="$HOME/.cargo/bin:$PATH" cargo test --lib frontend::sema::type_context::pbt_regression::test_typecontext_regression_double_insert_resurrection
```
**Regression test:** src/frontend/sema/type_context.rs `pbt_regression::test_typecontext_regression_double_insert_resurrection` (intentionally red)
