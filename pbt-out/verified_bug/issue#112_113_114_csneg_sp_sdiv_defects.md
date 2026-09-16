# Verified Bug: Issues #112, #113, #114 — CSNEG SP-as-XZR / SDIV extra operand / SDIV FP-as-GPR

**Issues:** [#112](https://github.com/fermat-hkrc/claudes-c-compiler/issues/112), [#113](https://github.com/fermat-hkrc/claudes-c-compiler/issues/113), [#114](https://github.com/fermat-hkrc/claudes-c-compiler/issues/114)
**Verdict:** ✅ all three **TRUE BUG** — unit probes fail; gcc (aarch64-linux-gnu-gcc 13.3) rejects all three inputs.
**Verified on:** branch `explore/pbt-test`, commit `bc6f5170`; **Date:** 2026-09-15

## Evidence

```console
$ cargo test --lib scratch_csneg_sp -- --nocapture
csneg sp,x0,x0,eq -> Ok(Word(3665822751))  [#112]    (SP encoded as XZR)
$ cargo test --lib scratch_div -- --nocapture
sdiv w0,w0,w0,x0 -> Ok(Word(448793600))   [#113]     (= 0x1AC00C00, issue's Actual verbatim)
sdiv d0,x1,x2    -> Ok(Word(448924704))   [#114]     (d0 recoded as Rd=0)
```

gcc reference (all REJECTED):

```console
csneg sp, x0, x0, eq   Error: expected an integer or zero register at operand 1
sdiv w0, w0, w0, x0    Error: unexpected characters following instruction at operand 3
sdiv d0, x1, x2        Error: expected an integer register or SVE vector register at operand 1
```

## Root Cause (established families)

- **#112** SP/XZR aliasing — same as #100 (CSNEG reg 31 = XZR/WZR; `parse_reg_num("sp")`=31).
- **#113** arity (4th operand ignored; `data_processing.rs:618+` inspects 0..2 only).
- **#114** register class (`parse_reg_num` accepts d/s/q/v/h/b prefixes).

## Suggested Fix

Reject SP/WSP names for CSNEG Rd; `operands.len() != 3 → Err` and GPR-only validation for SDIV/UDIV.

## Severity

Medium ×3 (as claimed).

## Artifacts

- `compare_branch.rs` → `mod scratch_csneg_sp`; `data_processing.rs` → `mod scratch_div`;
  tracker: #112, #113, #114 in-sample
