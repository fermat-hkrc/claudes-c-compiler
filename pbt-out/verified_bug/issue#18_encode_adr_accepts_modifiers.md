# Verified Bug: Issue #18 — encode_adr accepts :lo12:/:got: modifiers as a bare ADR reloc

**Issue:** [fermat-hkrc/claudes-c-compiler#18](https://github.com/fermat-hkrc/claudes-c-compiler/issues/18)
**Verdict:** ✅ **TRUE BUG — reproduced at unit level (failing assert), user-confirmed; reference cross-check: clang AArch64 assembler rejects the input.**
**Verified on:** branch `explore/pbt-test`, commit `bc6f5170`
**Verified by:** manual scratch test + `clang --target=aarch64-linux-gnu`
**Date:** 2026-09-15

## Claim

GNU `adr` takes no `:lo12:` / `:got:` / `:got_lo12:` modifiers (llvm-mc: "unexpected adr
label"). `encode_adr` must Err on Modifier operands; instead it emits a bare-symbol
`AdrPrelLo21` relocation, which the linker then applies as S+A−P to an illegal modifier.

## Evidence

```console
$ cargo test --lib scratch::manual_adr_modifier -- --nocapture
adr x0, :lo12:foo -> WordWithReloc { word: 0x10000000, AdrPrelLo21 }  (should be Err)
:lo12: modifier must be Err for adr, got WordWithReloc (AdrPrelLo21 applied to illegal modifier)
test …manual_adr_modifier ... FAILED
```

Reference:

```console
$ clang --target=aarch64-linux-gnu -c i18.s        # adr x0, :lo12:foo
error: unexpected adr label                        # REJECTED
```

Run independently reproduced by the user (verbatim same output).

## Root Cause

`get_symbol` (`src/backend/arm/assembler/encoder/mod.rs:980-981`) accepts **any**
Modifier/ModifierOffset and discards the `kind` field:

```rust
Some(Operand::Modifier { symbol, .. }) => Ok((symbol.clone(), 0)),
Some(Operand::ModifierOffset { symbol, offset, .. }) => Ok((symbol.clone(), *offset)),
```

`encode_adr` then emits `WordWithReloc { word: 0x10000000, AdrPrelLo21, symbol, addend }`
exactly as the issue's Actual states.

## Suggested Fix

In `encode_adr` (and any other directive whose grammar excludes modifiers), reject
Modifier operands before calling `get_symbol`; or make `get_symbol` return the kind so
callers can validate. Scratch test asserts the Modifier and ModifierOffset forms.

## Severity

As claimed: **high** — illegal modifier silently becomes a bare-symbol reloc; the linker
computes a wrong address with no diagnostic.

## Artifacts

- Scratch regression test: `load_store.rs` → `mod scratch::manual_adr_modifier`
- Reference: `/tmp/ref/i18.s` + clang output above
- Tracker: `pbt-out/sampling/sample_100_tracker.md` (#18 in-sample)
