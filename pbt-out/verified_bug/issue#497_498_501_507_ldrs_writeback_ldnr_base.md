# Verified Bug: Issues #497, #498, #501, #507 — Rt==Rn writeback warning-class / XZR base / ldNr bases

**Issues:** [#497](https://github.com/fermat-hkrc/claudes-c-compiler/issues/497), [#498](https://github.com/fermat-hkrc/claudes-c-compiler/issues/498), [#501](https://github.com/fermat-hkrc/claudes-c-compiler/issues/501), [#507](https://github.com/fermat-hkrc/claudes-c-compiler/issues/507)
**Verdict:** ✅ all four **TRUE BUG** — #498/#501/#507: gcc rejects, ccc accepts. **#497: warning-class** (gcc accepts with "Warning: unpredictable transfer with writeback"; ccc silent — same class as #150).
**Verified on:** branch `explore/pbt-test`, commit `bc6f5170`; **Date:** 2026-09-15

## Evidence

```console
$ cargo test --lib scratch_ldrs3 scratch_ldnr -- --nocapture
ldr w0,[x0,#4]!           -> Ok(Word(952126464))  [#497]   (encoded, no diagnostic)
strb w0,[xzr]             -> Ok(Word(968885216))  [#498]   (= 0x39C003E0, issue's Actual)
ld2r {v0.8b,v1.8b},[d0]   -> Ok(Word(222351360))  [#501]
ld2r {v0.8b,v1.8b},[xzr]  -> Ok(Word(222352352))  [#507]
```

gcc reference:

```console
ldr w0, [x0, #4]!           Warning: unpredictable transfer with writeback    ← ACCEPTED + warn (#497)
strb w0, [xzr]              Error: invalid base register at operand 2
ld2r {v0.8b, v1.8b}, [d0]   Error: invalid base register at operand 2
ld2r {v0.8b, v1.8b}, [xzr]  Error: invalid base register at operand 2
```

## Root Cause

- **#497** missing CU diagnostic (Rt==Rn with writeback is CONSTRAINED UNPREDICTABLE;
  gas warns, ccc silent) — F9/#150 class.
- **#498/#507** XZR-as-base → 31=SP (P4 family; #152/#323/#333/#340 siblings).
- **#501** register class in base (P2).

## Severity

#497 low (warning-class) · #498/#501/#507 medium.

## Artifacts

- `load_store.rs` → `mod scratch_ldrs3`; `neon.rs` → `mod scratch_ldnr`; tracker: all four in-sample
