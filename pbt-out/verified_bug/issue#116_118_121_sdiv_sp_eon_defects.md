# Verified Bug: Issues #116, #118, #121 — SDIV SP-as-WZR / EON FP-as-GPR / EON shift range

**Issues:** [#116](https://github.com/fermat-hkrc/claudes-c-compiler/issues/116), [#118](https://github.com/fermat-hkrc/claudes-c-compiler/issues/118), [#121](https://github.com/fermat-hkrc/claudes-c-compiler/issues/121)
**Verdict:** ✅ all three **TRUE BUG** — unit probes fail; gcc (aarch64-linux-gnu-gcc 13.3) rejects all three inputs.
**Verified on:** branch `explore/pbt-test`, commit `bc6f5170`; **Date:** 2026-09-15

## Evidence

```console
$ cargo test --lib scratch_div -- --nocapture
sdiv wsp,w0,w0   -> Ok(Word(448793631))  [#116]   (= 0x1AC00C1F, issue's Actual verbatim — wsp → wzr)
$ cargo test --lib scratch_eon -- --nocapture
eon d0,x1,x2       -> Ok(Word(1243742240))  [#118]
eon w0,w0,w0,lsl #32 -> Ok(Word(1243643904))  [#121]
```

gcc reference (all REJECTED):

```console
sdiv wsp, w0, w0         Error: expected an integer register or SVE vector register at operand 1
eon d0, x1, x2           Error: expected an integer register or SVE vector register at operand 1
eon w0, w0, w0, lsl #32  Error: shift amount out of range 0 to 31 at operand 3
```

## Root Cause (established families)

- **#116** SP/XZR aliasing (reg 31 in SDIV = WZR/XZR; `parse_reg_num("wsp")`=31) — family #49/#100/#112.
- **#118** register class (`parse_reg_num` accepts FP prefixes) — family #54/#73/#114.
- **#121** shift-amount masking (`amount & 0x3F`, no width check) — family #14/#25.

## Suggested Fix

Reject SP/WSP in GPR-only slots; reject FP/SIMD names; range-check shift amounts per sf.

## Severity

Medium ×3 (as claimed).

## Artifacts

- `data_processing.rs` → `mod scratch_div` (extended), `mod scratch_eon`; tracker: #116, #118, #121 in-sample
