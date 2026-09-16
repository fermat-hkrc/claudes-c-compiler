# Verified Bug: Issue #24 — encode_bic accepts mixed-width GPR operands

**Issue:** [fermat-hkrc/claudes-c-compiler#24](https://github.com/fermat-hkrc/claudes-c-compiler/issues/24)
**Verdict:** ✅ **TRUE BUG — reproduced at unit level (failing assert), user-confirmed; reference cross-check: clang AArch64 assembler rejects the input.**
**Verified on:** branch `explore/pbt-test`, commit `bc6f5170`
**Verified by:** manual scratch test + `clang --target=aarch64-linux-gnu`
**Date:** 2026-09-15

## Claim

All three GPR operands of scalar BIC must be the same width (all X or all W);
`bic w0, w0, x0` must be Err. ccc's `encode_bic` derives sf solely from Rd and
discards Rn/Rm width flags, silently assembling a 32-bit BIC.

## Evidence

```console
$ cargo test --lib scratch::manual_bic_mixed_width -- --nocapture
bic w0,w0,x0 -> 0x0a200000  (should be Err; sf taken from Rd only)
mixed-width operands must be Err, got Ok(0x0a200000)
test …manual_bic_mixed_width ... FAILED
```

Reference:

```console
$ clang --target=aarch64-linux-gnu -c i24.s        # bic w0, w0, x0
error: expected compatible register or logical immediate   # REJECTED
```

Run independently reproduced by the user (verbatim same output).

## Root Cause

`src/backend/arm/assembler/encoder/data_processing.rs:1023-1025` (and the sibling
`encode_bics` at `:985-987`):

```rust
let (rd, is_64) = get_reg(operands, 0)?;   // width used
let (rn, _) = get_reg(operands, 1)?;       // width discarded
let (rm, _) = get_reg(operands, 2)?;       // width discarded
let sf = sf_bit(is_64);
```

## Suggested Fix

Collect all three width flags and require equality:

```rust
let (rd, d64) = get_reg(operands, 0)?;
let (rn, n64) = get_reg(operands, 1)?;
let (rm, m64) = get_reg(operands, 2)?;
if d64 != n64 || d64 != m64 { return Err("bic operands must have matching width".into()); }
```

Same pattern applies to every encoder in the file with `let (r, _) = get_reg(...)` —
the issue family (mixed-width acceptance) likely affects all of them.

## Severity

As claimed: **medium** — width mismatch silently assembled as Rd's width.

## Artifacts

- Scratch regression test: `data_processing.rs` → `mod scratch::manual_bic_mixed_width`
- Reference: `/tmp/ref/i24.s` + clang output above
- Tracker: `pbt-out/sampling/sample_100_tracker.md` (#24 in-sample)
