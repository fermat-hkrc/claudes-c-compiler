# Verified Bug: Issues #333, #334, #340, #345 — XZR-as-base ×2 / STTRB extra / PRFM W-base

**Issues:** [#333](https://github.com/fermat-hkrc/claudes-c-compiler/issues/333), [#334](https://github.com/fermat-hkrc/claudes-c-compiler/issues/334), [#340](https://github.com/fermat-hkrc/claudes-c-compiler/issues/340), [#345](https://github.com/fermat-hkrc/claudes-c-compiler/issues/345)
**Verdict:** ✅ all four **TRUE BUG** — unit probes fail; gcc rejects all inputs.
**Verified on:** branch `explore/pbt-test`, commit `bc6f5170`; **Date:** 2026-09-15

## Evidence

```console
$ cargo test --lib scratch_exclusive2 -- --nocapture
ldrsw x0,[xzr]      -> Ok(Word(3112174560))  [#333]   (XZR→Rn=31=SP)
sttrb w0,[x0,#-256],x2 -> Ok(Word(940574720))  [#334]  (trailing dropped)
ldtrb w0,[xzr]      -> Ok(Word(943721440))    [#340]   (XZR→SP)
prfm #0,[w0]        -> Ok(Word(4185915392))   [#345]   (W base accepted)
```

gcc reference (all REJECTED):

```console
ldrsw x0, [xzr]            Error: invalid base register at operand 2
sttrb w0, [x0, #-256], x2  Error: cannot combine pre- and post-indexing at operand 2
ldtrb w0, [xzr]            Error: invalid base register at operand 2
prfm #0, [w0]              Error: expected a 64-bit base register at operand 2
```

## Root Cause

- **#333/#340** XZR-as-base → silently SP (P4 identity family; #152/#323 siblings).
- **#334** arity (P1).
- **#345** base class/width (P2; #329 sibling).

## Severity

Medium ×4.

## Artifacts

- `load_store.rs` → `mod scratch_exclusive2` (batch-2 probes); tracker: all four in-sample
