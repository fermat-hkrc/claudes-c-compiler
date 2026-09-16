# Verified Bug: Issues #237, #260, #276 — TBL empty-list panic / SBC FP-as-GPR / SMULL extra operand

**Issues:** [#237](https://github.com/fermat-hkrc/claudes-c-compiler/issues/237), [#260](https://github.com/fermat-hkrc/claudes-c-compiler/issues/260), [#276](https://github.com/fermat-hkrc/claudes-c-compiler/issues/276)
**Verdict:** ✅ all three **TRUE BUG** — unit probes fail; gcc rejects the inputs. **#237 is panic-class** (second in sample after #234).
**Verified on:** branch `explore/pbt-test`, commit `bc6f5170`; **Date:** 2026-09-15

## Evidence

```console
$ cargo test --lib scratch_tbl scratch_sbc_smull -- --nocapture
tbl v0.8b, {}, v0.8b (empty list) -> caught_panic=true returned_Err=false  [#237]
sbc d0,x1,x2      -> Ok(Word(1510080544))   [#260]   (FP name accepted)
smull x0,w0,w0,x0 -> Ok(Word(2602597376))   [#276]   (= 0x9B207C00, issue's Actual verbatim)
```

gcc reference (all REJECTED):

```console
tbl v0.8b, {}, v0.8b    Error: syntax error in register list at operand 2
sbc d0, x1, x2          Error: expected an integer or zero register at operand 1
smull x0, w0, w0, x0    Error: unexpected characters following instruction at operand 3
```

## Root Cause

- **#237** `neon.rs:781` indexes `regs[0]` without checking the list is non-empty —
  **panic** (`index out of bounds`) on malformed input. Crash-class (CWE-128/754).
- **#260** register class — P2 family (#54/#73/#114/#118/#191).
- **#276** arity — P1 family (extra operand ignored).

## Suggested Fix

Check `regs.is_empty()` → `Err` before indexing; GPR-only validation for SBC;
`operands.len() != 3 → Err` for SMULL.

## Severity

#237 **high** (crash) · #260 medium · #276 medium.

## Artifacts

- `neon.rs` → `mod scratch_tbl`; `data_processing.rs` → `mod scratch_sbc_smull`; tracker: #237, #260, #276 in-sample
