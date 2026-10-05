# Bug: empty object-like macro expansion glues adjacent operator tokens (`-EMPTY-` → `--`)

**Law:** Expanding an empty-bodied macro must still occupy a token position: gcc emits a
separating space at the expansion site, so `-EMPTY-` preprocessed is `- -` (two `-` tokens).
The SUT's anti-paste guard `append_with_paste_guard` returns early when the expansion is
empty, so the surrounding `-` `-` are glued into a single `--` token.

**Impact:** Silent token mutation in output fed to the lexer/parser: decrement becomes
`--`, `+EMPTY+` becomes `++`, `/EMPTY/` becomes `//` (a comment start!), `<EMPTY<` becomes
`<<`. Since empty helper macros are a standard obfuscation/sequencing idiom, any such use
adjacent to operator tokens changes the program's meaning with no diagnostic.

**Function:** `MacroTable::append_with_paste_guard`
**Source location:** /home/shuhao/fermat-users/leo/github/claudes-c-compiler/src/frontend/preprocessor/macro_defs.rs:253 (`if expanded.is_empty() { return; }`)
**Detected by:** Differential — gcc 9.4 token-stream KAT (P8b); predicted by P8's grammar
**Minimal input:** `#define EMPTY` then the line `-EMPTY-`
**Expected:** tokens `[-, -]` (gcc -E -P)
**Actual:** tokens `[--]`
**Severity:** medium

Fix direction: when `expanded` is empty, still check whether gluing `result`'s last byte with
the next source byte would form a multi-char token (`would_paste_tokens`) and insert a space —
i.e. the early return must not skip the trailing-edge guard.

**Reproduction:**
```bash
cd /home/shuhao/fermat-users/leo/github/claudes-c-compiler/pbt-out/rounds/03_preprocessor/run
PATH="$HOME/.cargo/bin:$PATH" cargo test --lib frontend::preprocessor::pipeline::pbt_kat::p8b_empty_macro_paste_guard_kat
# oracle: printf '#define EMPTY\n-EMPTY-\n' > em.c && gcc -E -P em.c   ->  - -
```
**Regression test:** src/frontend/preprocessor/pipeline.rs `pbt_kat::p8b_empty_macro_paste_guard_kat` (currently failing — the witness)
**Repro seed:** (deterministic KAT)
