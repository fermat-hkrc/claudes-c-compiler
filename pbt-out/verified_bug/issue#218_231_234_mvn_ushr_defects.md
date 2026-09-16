# Verified Bug: Issues #218, #231, #234 — MVN SP-as-ZR / USHR arrangement / USHR shift panic

**Issues:** [#218](https://github.com/fermat-hkrc/claudes-c-compiler/issues/218), [#231](https://github.com/fermat-hkrc/claudes-c-compiler/issues/231), [#234](https://github.com/fermat-hkrc/claudes-c-compiler/issues/234)
**Verdict:** ✅ all three **TRUE BUG** — unit probes fail; gcc rejects all inputs. **#234 additionally panics** (`attempt to subtract with overflow`) — first assembler-crash-class defect in the sample.
**Verified on:** branch `explore/pbt-test`, commit `bc6f5170`; **Date:** 2026-09-15

## Evidence

```console
$ cargo test --lib scratch_mvn scratch_shiftimm -- --nocapture
mvn wsp, w0     -> Ok(Word(706741247))   [#218]   (SP → Rd=31 = WZR)
ushr v0.8b, v0.16b, #1  -> Ok(Word(789513216))  [#231]   (source arrangement discarded)
ushr v0.8b, v0.8b, #-1  -> caught_panic=true returned_Err=false  [#234]   (PANIC: subtract with overflow)
```

gcc reference (all REJECTED):

```console
mvn wsp, w0              Error: expected an integer register or Advanced SIMD vector register at operand 1
ushr v0.8b, v0.16b, #1   Error: operand mismatch
ushr v0.8b, v0.8b, #-1   Error: immediate value out of range 1 to 64 at operand 3
```

## Root Cause (established families + one new)

- **#218** SP/XZR aliasing — P4 family (#100/#112/#116/#152/#174).
- **#231** source arrangement dropped (`let (rn, _) = get_neon_reg(operands, 1)`, `neon.rs:379`) — P3 family (#127).
- **#234** NEW: no range check on the shift immediate, and `(16 - shift as u32)` **underflows and
  panics** for negative shifts (`neon.rs:389-394`); `#0`/`#9` silently mask via `& 0xF` —
  combines CWE-190 (wrap) with a crash-class failure (first in sample).

## Suggested Fix

Range-check the shift against the element size before computing the encoding
(`1..=elem_bits`); reject mismatched source/dest arrangements; reject SP/WSP in MVN.

## Severity

#218 medium · #231 medium · #234 **high** (panic = assembler crash on malformed input).

## Artifacts

- `data_processing.rs` → `mod scratch_mvn` (+#218); `neon.rs` → `mod scratch_shiftimm`; tracker: #218, #231, #234 in-sample
