# Verified Bug: Issues #187, #188, #191 — MOVN imm range / MOVN illegal shift / MOVZ FP-as-GPR

**Issues:** [#187](https://github.com/fermat-hkrc/claudes-c-compiler/issues/187), [#188](https://github.com/fermat-hkrc/claudes-c-compiler/issues/188), [#191](https://github.com/fermat-hkrc/claudes-c-compiler/issues/191)
**Verdict:** ✅ all three **TRUE BUG** — unit probes fail; gcc rejects all three inputs.
**Verified on:** branch `explore/pbt-test`, commit `bc6f5170`; **Date:** 2026-09-15

## Evidence

```console
$ cargo test --lib scratch_mov -- --nocapture
movn w0,#-1     -> Ok(Word(312475616))    [#187]   (imm masked: -1 → 0xFFFF)
movn w0,#0,lsr #0 -> Ok(Word(310378496))  [#188]   (non-lsl silently hw=0)
movz d0,#0      -> Ok(Word(1384120320))   [#191]   (FP name → 32-bit MOVZ Rd=0)
#187: imm -1 out of [0,65535] must be Err
```

gcc reference (all REJECTED):

```console
movn w0, #-1        Error: immediate out of range
movn w0, #0, lsr #0 Error: only 'LSL' shift is permitted at operand 2
movz d0, #0         Error: expected an integer or zero register at operand 1
```

## Root Cause (established families)

- **#187** immediate masking (`& 0xFFFF`) — CWE-190 family (#14/#17/#59/#121/#138)
- **#188** silent default to hw=0 — CWE-478 family (#183 identical for MOVK)
- **#191** register class — CWE-20 family (#54/#73/#114/#118)

## Severity

Medium ×3 (as claimed).

## Artifacts

- `data_processing.rs` → `mod scratch_mov`; tracker: #187, #188, #191 in-sample
