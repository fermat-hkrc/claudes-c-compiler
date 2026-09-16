# Verified Bug: Issues #174, #176 — logical SP-as-ZR / MADD extra operand

**Issues:** [#174](https://github.com/fermat-hkrc/claudes-c-compiler/issues/174), [#176](https://github.com/fermat-hkrc/claudes-c-compiler/issues/176)
**Verdict:** ✅ both **TRUE BUG** — unit probes fail; gcc rejects both inputs.
**Verified on:** branch `explore/pbt-test`, commit `bc6f5170`; **Date:** 2026-09-15

## Evidence

```console
$ cargo test --lib scratch_more -- --nocapture
and wsp,w0,w0       -> Ok(Word(167772191))   [#174]   (SP in logical shifted-register → reg 31 = ZR)
madd w0,w0,w0,w0,x0 -> Ok(Word(452984832))   [#176]   (5th operand dropped)
#174: SP must be Err in logical shifted-register form
```

gcc reference (both REJECTED):

```console
and wsp, w0, w0          Error: unexpected register in the immediate operand at operand 3
madd w0, w0, w0, w0, x0  Error: unexpected characters following instruction at operand 4
```

## Root Cause (established families)

- **#174** SP/XZR aliasing (logical shifted-register Rd/Rn slot: 31 = ZR; SP invalid) — family #49/#100/#112/#116/#152.
- **#176** arity (5th operand ignored) — family #5/#38/#42/#46/#53/#85/#109/#113/#137/#144.

## Severity

Medium ×2 (as claimed).

## Artifacts

- `data_processing.rs` → `mod scratch_more`; tracker: #174, #176 in-sample
