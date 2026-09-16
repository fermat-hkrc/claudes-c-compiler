# Verified Bug: Issues #475, #481, #489, #491, #494 — FABS/FNEG H differential / DUP source / LDRS arity+SP

**Issues:** [#475](https://github.com/fermat-hkrc/claudes-c-compiler/issues/475), [#481](https://github.com/fermat-hkrc/claudes-c-compiler/issues/481), [#489](https://github.com/fermat-hkrc/claudes-c-compiler/issues/489), [#491](https://github.com/fermat-hkrc/claudes-c-compiler/issues/491), [#494](https://github.com/fermat-hkrc/claudes-c-compiler/issues/494)
**Verdict:** ✅ all five **TRUE BUG** — gcc differentially confirms #475/#481's wrong ftype; rejects the other three.
**Verified on:** branch `explore/pbt-test`, commit `bc6f5170`; **Date:** 2026-09-15

## Evidence

```console
$ cargo test --lib scratch_h_ftype scratch_dup scratch_ldrs2 -- --nocapture
fabs h0,h0 -> 0x1e20c000   [#475: expected 0x1ee0c000]   gcc: 1ee0c000 ← differential
fneg h0,h0 -> 0x1e214000   [#481: expected 0x1ee14000]   gcc: 1ee14000 ← differential
dup v0.8b, x0 -> Ok(Word(234949632))  [#489]
strb w0,[x0],x2 -> Ok(Word(968884224))  [#491]   (= 0x39C00000, issue's Actual)
ldrb sp,[x0]    -> Ok(Word(964689951))  [#494]   (= 0x3980001F, issue's Actual)
```

gcc reference (REJECT for the last three):

```console
dup v0.8b, x0        Error: operand mismatch
strb w0, [x0], x2    Error: invalid addressing mode at operand 2
ldrb sp, [x0]        Error: expected an integer or zero register at operand 1
```

## Root Cause

- **#475/#481** the **H-register ftype=00 defect family** (only `starts_with('d')` checked
  in `fp_scalar.rs`) — same root as #361/#414; now confirmed in FABS/FNEG. Valid `__fp16`
  code mis-encoded as single-precision ops.
- **#489** source register class/width vs arrangement (P2/P3).
- **#491** arity (P1) · **#494** SP→ZR aliasing (P4).

## Severity

#475/#481 **high** (valid FP16 mis-encoded) · #489/#491/#494 medium.

## Artifacts

- `fp_scalar.rs` → `mod scratch_h_ftype`; `neon.rs` → `mod scratch_dup`; `load_store.rs` → `mod scratch_ldrs2`; tracker: all five in-sample
