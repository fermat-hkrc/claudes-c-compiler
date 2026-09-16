# Verified Bug: Issue #15 — ADD/SUB with SP/WSP and LSL #N uses shifted-register form (XZR) instead of extended-register form

**Issue:** [fermat-hkrc/claudes-c-compiler#15](https://github.com/fermat-hkrc/claudes-c-compiler/issues/15)
**Verdict:** ✅ **TRUE BUG — reproduced at unit level; actual encoding byte-for-byte identical to the issue's claim; differential mismatch vs llvm-mc confirmed.**
**Verified on:** branch `explore/pbt-test`, commit `bc6f5170`
**Verified by:** manual reproduction — scratch unit test, user-confirmed run
**Date:** 2026-09-15

---

## 1. The Claim

When Rd or Rn is SP/WSP, AArch64 ADD/SUB register form must use the **extended-register**
encoding (bit 21 = 1) so register 31 means SP; `lsl #N` with N in 0..=4 is the documented
assembler alias for UXTX/UXTW #N. llvm-mc encodes `add w0, wsp, w0, lsl #1` as
`0x0b2047e0`. ccc's `encode_add_sub` produces `0x0b0007e0` (shifted-register form,
Rn=31 → **WZR**) because the SP special case only fires when `operands.len() <= 3`.

## 2. Evidence

Scratch test `src/backend/arm/assembler/encoder/data_processing.rs` →
`mod scratch::manual_add_sub_sp_lsl`:

```console
$ cargo test --lib scratch::manual_add_sub_sp_lsl -- --nocapture
add w0,wsp,w0,lsl #1 -> 0x0b0007e0 (expected 0x0b2047e0)
assertion `left == right` failed: SP operand requires extended-register form (llvm-mc reference)
  left: 184551392      (= 0x0b0007e0 — actual)
 right: 186664928      (= 0x0b2047e0 — llvm-mc reference)
test …manual_add_sub_sp_lsl ... FAILED
```

Run independently reproduced by the user (verbatim same output).

Encoding diff (the defect is visible in the word itself):

| | bit 21 | Rn=31 decodes as | word |
|---|---|---|---|
| actual (ccc) | 0 — shifted-register | **WZR (zero register)** | `0x0b0007e0` |
| expected (llvm-mc) | 1 — extended (UXTW #1) | **WSP (stack pointer)** | `0x0b2047e0` |

## 3. Root Cause

`src/backend/arm/assembler/encoder/data_processing.rs:291` (`encode_add_sub`): the
SP-handling branch (visible in the source around the `rn_is_sp` / `rd_is_sp` checks) is
guarded by an operand-count condition (`operands.len() <= 3`); a 4th operand
(`Shift { lsl, #N }`) skips the SP path entirely and falls through to the plain
shifted-register encoder, where register 31 encodes as XZR/WZR.

## 4. Suggested Fix

Route SP operands through the extended-register form regardless of operand count:
treat `Shift { "lsl", n }` with `n <= 4` as `Extend { "uxtx"/"uxtw", n }` when Rd/Rn
is SP/WSP (the assembler alias the ISA documents), then use the existing extended
encoding (bit 21 = 1). The scratch test's exact-word assert is the ready regression test.

## 5. Severity

As claimed: **high** — unlike the silent-validation issues (#5, #14), this is a
**wrong-register misencoding of valid assembly**: `add w0, wsp, w0, lsl #1` reads the
zero register instead of the stack pointer. Any hand-written asm (or generated
frame-pointer/stack-adjust code) using this form computes wrong addresses at runtime.

## 6. Reproduction Artifacts

- Scratch regression test: `src/backend/arm/assembler/encoder/data_processing.rs` → `mod scratch::manual_add_sub_sp_lsl` (uncommitted; passes iff fixed)
- Sample tracker: `pbt-out/sampling/sample_100_tracker.md` (#15 in-sample)
