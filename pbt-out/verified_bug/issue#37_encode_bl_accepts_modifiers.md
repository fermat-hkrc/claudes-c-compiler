# Verified Bug: Issue #37 — encode_bl accepts :lo12: / modifier operands as Call26 symbols

**Issue:** [fermat-hkrc/claudes-c-compiler#37](https://github.com/fermat-hkrc/claudes-c-compiler/issues/37)
**Verdict:** ✅ **TRUE BUG — reproduced at unit level (failing assert); reference cross-check: clang AArch64 assembler rejects the input.**
**Verified on:** branch `explore/pbt-test`, commit `bc6f5170`
**Verified by:** scratch test + `clang --target=aarch64-linux-gnu` (user independently confirmed the reference rejection)
**Date:** 2026-09-15

## Claim

BL's operand is a label or encodable PC offset; `bl :lo12:foo` (and `:lo12:foo+N`) must
be `Err`. Instead `encode_bl` emits an ordinary Call26 relocation to `foo`, dropping the
modifier — wrong relocation class relative to the source text.

## Evidence

```console
$ cargo test --lib scratch::manual_bl_modifier -- --nocapture
bl :lo12:foo -> WordWithReloc { 0x94000000, Call26 }  (should be Err)
modifier must be Err for bl, got WordWithReloc (Call26 applied to illegal modifier)
test …manual_bl_modifier ... FAILED
```

Reference (user-run):

```console
$ clang --target=aarch64-linux-gnu -x assembler -c - -o /dev/null <<< 'bl :lo12:foo'
error: unrecognized instruction mnemonic
```

## Root Cause

`src/backend/arm/assembler/encoder/compare_branch.rs:184-195`: `encode_bl` calls
`get_symbol(operands, 0)`, whose Modifier/ModifierOffset arms
(`src/backend/arm/assembler/encoder/mod.rs:980-981`) accept any kind and discard it —
the same root cause as issue #18 (`encode_adr`), different consumer.

## Suggested Fix

Reject `Operand::Modifier`/`ModifierOffset` in `encode_bl` before `get_symbol`; longer
term, make `get_symbol` return the modifier kind so each encoder can validate it against
its grammar (fixes #18 and #37 together and prevents the family from recurring).

## Severity

As claimed: **medium** — illegal modifier silently becomes a bare Call26 reloc.

## Artifacts

- Scratch regression test: `compare_branch.rs` → `mod scratch::manual_bl_modifier`
- Tracker: `pbt-out/sampling/sample_100_tracker.md` (#37 in-sample)
