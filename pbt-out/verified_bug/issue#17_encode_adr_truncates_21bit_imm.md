# Verified Bug: Issue #17 — encode_adr silently truncates immediates outside the 21-bit signed range

**Issue:** [fermat-hkrc/claudes-c-compiler#17](https://github.com/fermat-hkrc/claudes-c-compiler/issues/17)
**Verdict:** ✅ **TRUE BUG — reproduced at unit level (failing assert), independently by the user. One detail in the issue corrected: the wrap goes to +1048575, not −1.**
**Verified on:** branch `explore/pbt-test`, commit `bc6f5170`
**Verified by:** manual reproduction — scratch unit test, user-confirmed run
**Date:** 2026-09-15

---

## 1. The Claim

ARM ADR immediate is a 21-bit signed PC-relative byte offset in [−1048576, 1048575]
(±1 MB). llvm-mc rejects `#1048576` and `#−1048577`. `encode_adr` must return `Err`
outside that range; the source itself documents the gap (`load_store.rs:697`:
`// TODO: validate 21-bit signed immediate range`).

## 2. Evidence

Scratch test `src/backend/arm/assembler/encoder/load_store.rs` →
`mod scratch::manual_adr_imm_range`:

```console
$ cargo test --lib scratch::manual_adr_imm_range -- --nocapture
adr x0, #-1048577 -> 0x707fffe0  (should be Err; wraps to #-1)
out-of-range immediate must be Err, got Ok(0x707fffe0)
test …manual_adr_imm_range ... FAILED
```

Run independently reproduced by the user (verbatim same output).

## 3. Correction to the Issue's "Actual" (verified detail)

The issue claims the result is `0x70ffffe0`, "same encoding as `adr x0, #-1`".
The observed word is **`0x707fffe0`**, which decodes differently:

| word | immlo (30:29) | immhi (23:5) | imm21 | decodes as |
|---|---|---|---|---|
| `0x707fffe0` (observed) | 3 | `0x3FFFF` | `0xFFFFF` = 1048575 | **`adr x0, #+1048575`** |
| `0x70ffffe0` (issue's claim) | 3 | `0x7FFFF` | `0x1FFFFF` | `adr x0, #-1` |

−1048577 mod 2²¹ = 2097152 − 1048577 = **1048575** (positive, in-range) — the encoder
keeps the low 21 bits, so the wrap lands at the opposite end of the range, not at −1.
**The bug class and severity are unchanged** — an out-of-range offset silently becomes a
far-away, in-range wrong target — but the impact detail is "wraps to +1048575", not
"wraps to −1". (The `0x70ffffe0` word in the issue appears to be a hand-computation
slip, not what the code emits.)

## 4. Root Cause

`src/backend/arm/assembler/encoder/load_store.rs:693-706` (`encode_adr`): the immediate
path splits `imm` into immlo/immhi with masks (`& 3`, `& 0x7FFFF`) and never range-checks;
the missing validation is self-documented by the `TODO` at `:697`.

## 5. Suggested Fix

```rust
if !(-1048576..=1048575).contains(&imm) {
    return Err(format!("adr immediate {} out of 21-bit signed range", imm));
}
```

Boundary controls in the scratch test (`#-1048576` must stay Ok) are ready.

## 6. Severity

As claimed: **high** — silent wrong-target address computation. Any hand-written or
generated `adr` with an out-of-range offset (easy with large code/data distances) wraps
to an unrelated address with no diagnostic.

## 7. Reproduction Artifacts

- Scratch regression test: `src/backend/arm/assembler/encoder/load_store.rs` → `mod scratch::manual_adr_imm_range` (uncommitted; probes −1048577, +1048576, boundary −1048576)
- Sample tracker: `pbt-out/sampling/sample_100_tracker.md` (#17 in-sample)
