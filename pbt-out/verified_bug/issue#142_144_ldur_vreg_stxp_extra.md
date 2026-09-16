# Verified Bug: Issue #142 — encode_ldur_stur encodes a V-register Rt as a 64-bit SIMD (D) load/store
# Verified Bug: Issue #144 — encode_ldxp_stxp ignores extra operands

**Issues:** [#142](https://github.com/fermat-hkrc/claudes-c-compiler/issues/142), [#144](https://github.com/fermat-hkrc/claudes-c-compiler/issues/144)
**Verdict:** ✅ both **TRUE BUG** — unit probes fail; gcc (aarch64-linux-gnu-gcc 13.3) rejects both inputs.
**Verified on:** branch `explore/pbt-test`, commit `bc6f5170`; **Date:** 2026-09-15

## Evidence

```console
$ cargo test --lib scratch_ldxp -- --nocapture
ldur v0,[x0]          -> Ok(Word(4232052736))  [#142]   (bare V reg encoded as d0)
stxp w0,w0,w0,[x0],x2 -> Ok(Word(2283798528))  [#144]   (trailing operand dropped)
#142: bare V-register Rt must be Err for ldur
```

gcc reference (both REJECTED):

```console
ldur v0, [x0]             Error: unexpected register type at operand 1
stxp w0, w0, w0, [x0], x2 Error: invalid addressing mode at operand 4
```

## Root Cause

- **#142**: `encode_ldur_stur`'s FP-register classification treats a bare `v0` (no
  arrangement suffix) like `d0` — no validation that the Rt token carries a valid
  SIMD/FPSIMD size prefix for unscaled load/store.
- **#144**: arity (5th operand never inspected) — established family.

## Suggested Fix

Reject bare `v*` names in `encode_ldur_stur` (require b/h/s/d/q or GPR); arity check
in `encode_ldxp_stxp`.

## Severity

Medium ×2 (as claimed).

## Artifacts

- `load_store.rs` → `mod scratch_ldxp`; tracker: #142, #144 in-sample
