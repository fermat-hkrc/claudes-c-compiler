# Verified Bug: Issues #178, #183, #185 — MADD mixed width / MOVK illegal shift / MOVN extra operand

**Issues:** [#178](https://github.com/fermat-hkrc/claudes-c-compiler/issues/178), [#183](https://github.com/fermat-hkrc/claudes-c-compiler/issues/183), [#185](https://github.com/fermat-hkrc/claudes-c-compiler/issues/185)
**Verdict:** ✅ all three **TRUE BUG** — unit probes fail; gcc (aarch64-linux-gnu-gcc 13.3) rejects all three inputs.
**Verified on:** branch `explore/pbt-test`, commit `bc6f5170`; **Date:** 2026-09-15

## Evidence

```console
$ cargo test --lib scratch_more2 -- --nocapture
madd w0,w0,w0,x0 -> Ok(Word(452984832))    [#178]   (sf from Rd only — Ra width discarded)
movk w0,#0,lsr #0 -> Ok(Word(1920991232))  [#183]   (non-lsl silently defaults hw=0)
movn x0,#0,x0     -> Ok(Word(2457862144))  [#185]   (non-Shift at index 2 silently sets hw=0)
#178: mixed x/w must be Err for madd
```

gcc reference (all REJECTED):

```console
madd w0, w0, w0, x0   Error: operand mismatch
movk w0, #0, lsr #0   Error: only 'LSL' shift is permitted at operand 2
movn x0, #0, x0       Error: shift operator expected at operand 2
```

## Root Cause (established families)

- **#178** width discarded (family #24/#65/#81/#103)
- **#183** silent default: MOVK's hw field only encodes lsl 0/16; any other kind falls
  through to hw=0 (family #14's `_ =>` default pattern, CWE-478)
- **#185** arity/operand-type at index 2 (non-Shift accepted, hw silently 0) (family #5/#176)

## Severity

Medium ×3 (as claimed).

## Artifacts

- `data_processing.rs` → `mod scratch_more2`; tracker: #178, #183, #185 in-sample
