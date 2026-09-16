# Verified Bug: Issues #281, #284, #289 — SXTH FP-as-GPR / SXTW extra operand / SQSHL extra operand

**Issues:** [#281](https://github.com/fermat-hkrc/claudes-c-compiler/issues/281), [#284](https://github.com/fermat-hkrc/claudes-c-compiler/issues/284), [#289](https://github.com/fermat-hkrc/claudes-c-compiler/issues/289)
**Verdict:** ✅ all three **TRUE BUG** — unit probes fail; gcc rejects all three inputs.
**Verified on:** branch `explore/pbt-test`, commit `bc6f5170`; **Date:** 2026-09-15

## Evidence

```console
$ cargo test --lib scratch_extend scratch_shl -- --nocapture
sxth d0,w1   -> Ok(Word(318782496))   [#281]   (FP name accepted as GPR)
sxtw x0,w0,x0 -> Ok(Word(2470476800))  [#284]   (3rd operand dropped)
sqshl v0.8b,v0.8b,#0,v0.8b -> Ok(Word(252212224))  [#289]   (= 0x0F087400, issue's Actual verbatim)
```

gcc reference (all REJECTED):

```console
sxth d0, w1                 Error: expected an integer register or SVE vector register at operand 1
sxtw x0, w0, x0             Error: unexpected characters following instruction at operand 2
sqshl v0.8b, v0.8b, #0, v0.8b Error: unexpected characters following instruction at operand 3
```

## Root Cause (established families)

- **#281** register class (P2) — `is_fp_reg` never consulted (#54/#73/#114/#191/#260).
- **#284** arity (P1) — 3rd operand ignored (#5/#176/#276).
- **#289** arity (P1) — only `len < 3` checked (#195).

## Severity

Medium ×3 (as claimed).

## Artifacts

- `data_processing.rs` → `mod scratch_extend`; `neon.rs` → `mod scratch_shl`; tracker: #281, #284, #289 in-sample
