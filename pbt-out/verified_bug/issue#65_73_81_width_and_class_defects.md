# Verified Bug: Issues #65, #73, #81 — mixed-width / FP-as-GPR operands accepted (cinc / cmn / cmp)

**Issues:** [#65](https://github.com/fermat-hkrc/claudes-c-compiler/issues/65), [#73](https://github.com/fermat-hkrc/claudes-c-compiler/issues/73), [#81](https://github.com/fermat-hkrc/claudes-c-compiler/issues/81)
**Verdict:** ✅ all three **TRUE BUG** — one scratch test covers all probes; clang rejects all three inputs.
**Verified on:** branch `explore/pbt-test`, commit `bc6f5170`; **Date:** 2026-09-15

## Evidence

```console
$ cargo test --lib scratch_width::manual_width_defects -- --nocapture
cinc x0,w0,eq -> Ok(Word(2592084992))  [#65]
cmn d0,#0     -> Ok(Word(2969567263))  [#73]
cmp x0,w0     -> Ok(Word(3942645791))  [#81]
#65: mixed x/w must be Err for cinc
test …manual_width_defects ... FAILED
```

- #65: 64-bit CINC emitted with Rn taken from `w0` (width discarded).
- #73: `d0` (FP register) recoded as GPR 0 → `cmn x0, #0` class encoding.
- #81: `0xEB00001F` = `SUBS XZR, x0, w0` — 64-bit CMP from the prepended XZR;
  the `w0` width is never checked.

Reference (all REJECTED):

```console
cinc x0, w0, eq   error: invalid operand for instruction
cmn d0, #0        error: invalid operand for instruction
cmp x0, w0        error: too few operands for instruction   (mixed width requires an extend)
```

## Root Cause (register-class/width family: #24, #49, #54, these)

- `encode_cinc` (`compare_branch.rs:293+`): `let (rn, _) = get_reg(operands, 1)?` — Rn width discarded.
- `encode_cmp`/`encode_cmn` (`compare_branch.rs:6-28`): prepend XZR/WZR and derive
  32/64-bit **only from the first source operand**; `encode_add_sub` then discards
  the other operands' width flags (#24 root cause). FP/SIMD names: `is_32bit_reg`
  is false for d/s/q/v/h/b → XZR variant → 64-bit GPR encoding with the register
  number parsed from the FP name.

## Suggested Fix

Uniform width/class validation helper applied in every GPR operand read:
all GPR operands of one instruction must agree on width; FP/SIMD names must be
rejected in GPR contexts (`get_reg` should Err on non-GPR prefixes).

## Severity

As claimed: medium ×3 (silently wrong-width / wrong-class instructions).

## Artifacts

- `compare_branch.rs` → `mod scratch_width::manual_width_defects`; tracker: #65, #73, #81 in-sample
