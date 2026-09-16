# Verified Bug: Issues #368, #375, #379, #381 — FCVT extra / AES prefix / BFI panic / BFI SP

**Issues:** [#368](https://github.com/fermat-hkrc/claudes-c-compiler/issues/368), [#375](https://github.com/fermat-hkrc/claudes-c-compiler/issues/375), [#379](https://github.com/fermat-hkrc/claudes-c-compiler/issues/379), [#381](https://github.com/fermat-hkrc/claudes-c-compiler/issues/381)
**Verdict:** ✅ all four **TRUE BUG** — unit probes fail; gcc rejects all inputs. **#379 is panic-class** (third in sample).
**Verified on:** branch `explore/pbt-test`, commit `bc6f5170`; **Date:** 2026-09-15

## Evidence

```console
$ cargo test --lib scratch_fcvt scratch_aes scratch_bfi -- --nocapture
fcvt s0,d0,s0       -> Ok(Word(509755392))   [#368]  (= 0x1E624000, issue's Actual — 3rd dropped)
aese x0.16b,x0.16b  -> Ok(Word(1311262720))  [#375]  (= 0x4E284800, issue's Actual — GPR prefix)
bfi w0,w0,#0,#0     -> caught_panic=true     [#379]  (debug overflow at bitfield.rs:113)
bfi wsp,w0,#0,#1    -> Ok(Word(855638047))   [#381]  (SP→31=ZR)
```

gcc reference (all REJECTED):

```console
fcvt s0, d0, s0       Error: unexpected characters following instruction at operand 2
aese x0.16b, x0.16b   Error: expected a vector register at operand 1
bfi w0, w0, #0, #0    Error: immediate value out of range 1 to 32 at operand 4
bfi wsp, w0, #0, #1   Error: expected an integer or zero register at operand 1
```

## Root Cause

- **#368** arity (P1) · **#375** register class (P2) · **#381** SP/ZR aliasing (P4).
- **#379** no validation of `#lsb/#width` before arithmetic at `bitfield.rs:112-113` —
  **panic** (`attempt to subtract/add with overflow`) on `width=0` / `lsb > reg_width`;
  other out-of-range values silently encode. Crash + CWE-190 mix (#234 class).

## Severity

#368/#375/#381 medium · **#379 high** (crash).

## Artifacts

- `fp_scalar.rs` → `mod scratch_fcvt`; `neon.rs` → `mod scratch_aes`; `bitfield.rs` → `mod scratch_bfi`; tracker: all four in-sample
