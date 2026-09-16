# Verified Bug: Issues #318, #323, #327, #329 — exclusive Rt/base identity / LDRSW imm9 & base

**Issues:** [#318](https://github.com/fermat-hkrc/claudes-c-compiler/issues/318), [#323](https://github.com/fermat-hkrc/claudes-c-compiler/issues/323), [#327](https://github.com/fermat-hkrc/claudes-c-compiler/issues/327), [#329](https://github.com/fermat-hkrc/claudes-c-compiler/issues/329)
**Verdict:** ✅ all four **TRUE BUG** — unit probes fail; gcc rejects all inputs. #318's word `0xC85FFC1F` matches the issue's Actual verbatim.
**Verified on:** branch `explore/pbt-test`, commit `bc6f5170`; **Date:** 2026-09-15

## Evidence

```console
$ cargo test --lib scratch_exclusive2 -- --nocapture
ldaxr sp,[x0]       -> Ok(Word(3361733663))  [#318]  (= 0xC85FFC1F, issue's Actual — SP→Rt=31=ZR)
ldaxr x0,[xzr]      -> Ok(Word(3361734624))  [#323]  (XZR→Rn=31=SP)
ldrsw x0,[x1,#-257] -> Ok(Word(3096440864))  [#327]  (imm9 masked)
ldrsw x0,[w1]       -> Ok(Word(3112173600))  [#329]  (W base accepted)
```

gcc reference (all REJECTED):

```console
ldaxr sp, [x0]         Error: expected an integer or zero register at operand 1
ldaxr x0, [xzr]        Error: invalid base register at operand 2
ldrsw x0, [x1, #-257]  Error: immediate offset out of range
ldrsw x0, [w1]         Error: expected a 64-bit base register at operand 2
```

## Root Cause

- **#318/#323** SP/XZR aliasing (P4) — reg 31 means ZR in Rt, SP in Rn; both directions wrong (#100/#152 mirror pair).
- **#327** imm9 masking (P5, `& 0x1FF`) — CWE-190 family (#138).
- **#329** base register class/width (P2).

## Severity

Medium ×4.

## Artifacts

- `load_store.rs` → `mod scratch_exclusive2`; tracker: all four in-sample
