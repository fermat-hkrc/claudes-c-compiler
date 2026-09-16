# Verified Bug: Issues #444, #447, #458, #465 — BFM arity/width / SBFX SP / UBFX FP

**Issues:** [#444](https://github.com/fermat-hkrc/claudes-c-compiler/issues/444), [#447](https://github.com/fermat-hkrc/claudes-c-compiler/issues/447), [#458](https://github.com/fermat-hkrc/claudes-c-compiler/issues/458), [#465](https://github.com/fermat-hkrc/claudes-c-compiler/issues/465)
**Verdict:** ✅ all four **TRUE BUG** — unit probes fail; gcc rejects all inputs.
**Verified on:** branch `explore/pbt-test`, commit `bc6f5170`; **Date:** 2026-09-15

## Evidence

```console
$ cargo test --lib scratch_bfm_family -- --nocapture
bfm w0,w0,#0,#0,x0 -> Ok(Word(855638016))   [#444]   (5th dropped)
bfm x0,w0,#0,#0    -> Ok(Word(3007315968))  [#447]   (Rn width discarded)
sbfx wsp,w0,#0,#1  -> Ok(Word(318767135))   [#458]   (SP→31=ZR)
ubfx d0,x1,#0,#1   -> Ok(Word(1392508960))  [#465]   (FP name as GPR)
```

gcc reference (all REJECTED):

```console
bfm w0, w0, #0, #0, x0   Error: unexpected characters following instruction at operand 4
bfm x0, w0, #0, #0       Error: operand mismatch
sbfx wsp, w0, #0, #1     Error: expected an integer or zero register at operand 1
ubfx d0, x1, #0, #1      Error: expected an integer or zero register at operand 1
```

## Root Cause (established families)

P1 arity (#444) · P2 width (#447) · P4 SP/ZR (#458) · P2 register class (#465) — the
bitfield family repeats the same four root causes seen throughout.

## Severity

Medium ×4.

## Artifacts

- `bitfield.rs` → `mod scratch_bfm_family`; tracker: all four in-sample
