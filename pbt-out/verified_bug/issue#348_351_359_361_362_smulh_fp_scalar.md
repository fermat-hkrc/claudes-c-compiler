# Verified Bug: Issues #348, #351, #359, #361, #362 — SMULH arity/width / FP mixed / H-dest differential / SP source

**Issues:** [#348](https://github.com/fermat-hkrc/claudes-c-compiler/issues/348), [#351](https://github.com/fermat-hkrc/claudes-c-compiler/issues/351), [#359](https://github.com/fermat-hkrc/claudes-c-compiler/issues/359), [#361](https://github.com/fermat-hkrc/claudes-c-compiler/issues/361), [#362](https://github.com/fermat-hkrc/claudes-c-compiler/issues/362)
**Verdict:** ✅ all five **TRUE BUG** — unit probes fail; gcc rejects four inputs and **differentially confirms #361's wrong encoding** (`0x1ee30000` vs ccc's `0x1e230000`).
**Verified on:** branch `explore/pbt-test`, commit `bc6f5170`; **Date:** 2026-09-15

## Evidence

```console
$ cargo test --lib scratch_smulh scratch_fp -- --nocapture
smulh x0,x0,x0,x0 -> Ok(Word(2604694528))   [#348]  (4th dropped)
smulh w0,w0,w0    -> Ok(Word(2604694528))   [#351]  (identical word — W widths discarded; SMULH is X-only)
fp1src s0,d0      -> Ok(Word(505692160))    [#359]  (= 0x1E244000, issue's Actual — mixed S/D accepted)
ucvtf h0,w0       -> 0x1e230000             [#361]  (expected 0x1ee30000 — ftype wrong)
scvtf s0,wsp      -> Ok(Word(505545696))    [#362]  (SP → 31=ZR)
```

gcc reference:

```console
smulh x0, x0, x0, x0  Error: unexpected characters following instruction at operand 3
smulh w0, w0, w0      Error: operand mismatch
fabs s0, d0           Error: operand mismatch
scvtf s0, wsp         Error: unexpected register type at operand 2
# differential (with -march=armv8.2-a+fp16):
$ gcc -c ucvtf h0, w0 → objdump: 1ee30000      ← the correct ftype=11 encoding; ccc emits 1e230000 (ftype=00)
```

## Root Cause

- **#348/#351** arity + width discard (P1/P2) — SMULH is 64-bit-only; both defects in one
  probe pair (identical words).
- **#359** FP register type mix accepted (P2) — `encode_fp_1src` parses only the number.
- **#361** **wrong ftype selection for H destinations** (`fp_scalar.rs:210+`: only
  `starts_with('d')` is checked → H falls into ftype=00 single) — valid input, wrong
  encoding; same class as #15 (F5/CWE-681, differential).
- **#362** SP→ZR aliasing in the source slot (P4).

## Severity

#348/#351/#359/#362 medium · **#361 high** (valid FP16 code silently mis-encoded as
single-precision convert — wrong numeric results at runtime for `__fp16` code).

## Artifacts

- `data_processing.rs` → `mod scratch_smulh`; `fp_scalar.rs` → `mod scratch_fp`; tracker: all five in-sample
