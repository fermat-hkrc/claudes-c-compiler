# Verified Bug: Issues #390, #392, #397, #414 — CAS width/SP / CLS width / FP-arith H-ftype differential

**Issues:** [#390](https://github.com/fermat-hkrc/claudes-c-compiler/issues/390), [#392](https://github.com/fermat-hkrc/claudes-c-compiler/issues/392), [#397](https://github.com/fermat-hkrc/claudes-c-compiler/issues/397), [#414](https://github.com/fermat-hkrc/claudes-c-compiler/issues/414)
**Verdict:** ✅ all four **TRUE BUG** — gcc rejects three inputs; **#414 differentially confirmed** (gcc `0x1ee00800` vs ccc `0x1e200800`).
**Verified on:** branch `explore/pbt-test`, commit `bc6f5170`; **Date:** 2026-09-15

## Evidence

```console
$ cargo test --lib scratch_cas scratch_cls scratch_farith -- --nocapture
cas x0,w0,[x1]  -> Ok(Word(3365960736))    [#390]   (mixed W/X accepted)
cas sp,w1,[x2]  -> Ok(Word(3367992385))    [#392]   (SP→31=ZR)
cls x0,w0       -> Ok(Word(3670021120))    [#397]   (= 0xDAC01400, issue's Actual — Rn width ignored)
fmul h0,h0,h0   -> 0x1e200800              [#414]   (expected 0x1ee00800 — ftype=00)
```

gcc reference:

```console
cas x0, w0, [x1]   Error: operand mismatch          (-march=armv8.1-a)
cas sp, w1, [x2]   Error: expected an integer or zero register at operand 1
cls x0, w0         Error: operand mismatch
# differential (with -march=armv8.2-a+fp16):
$ gcc fmul h0, h0, h0 → objdump: 1ee00800          ← ccc emits 1e200800 (ftype=00 single instead of 11 half)
```

## Root Cause

- **#390/#397** width discard (P2) — `encode_cas`/`encode_cls` take sf from one operand.
- **#392** SP→ZR aliasing (P4).
- **#414** H-register ftype selection (`fp_scalar.rs:68+`: only `starts_with('d')` checked →
  H falls to ftype=00) — **same root cause as #361**, now in the 3-operand arithmetic path;
  valid FP16 code silently mis-encoded as single-precision ops (wrong numerics at runtime).

## Severity

#390/#392/#397 medium · **#414 high** (valid `__fp16` arithmetic mis-encoded).

## Artifacts

- `load_store.rs` → `mod scratch_cas`; `bitfield.rs` → `mod scratch_cls`; `fp_scalar.rs` → `mod scratch_farith`; tracker: all four in-sample
