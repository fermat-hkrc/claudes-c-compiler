# Verified Bug: Issues #297, #303, #304, #310, #311 — arity / arrangement / register-class family

**Issues:** [#297](https://github.com/fermat-hkrc/claudes-c-compiler/issues/297), [#303](https://github.com/fermat-hkrc/claudes-c-compiler/issues/303), [#304](https://github.com/fermat-hkrc/claudes-c-compiler/issues/304), [#310](https://github.com/fermat-hkrc/claudes-c-compiler/issues/310), [#311](https://github.com/fermat-hkrc/claudes-c-compiler/issues/311)
**Verdict:** ✅ all five **TRUE BUG** — unit probes fail; gcc rejects all inputs.
**Verified on:** branch `explore/pbt-test`, commit `bc6f5170`; **Date:** 2026-09-15

## Evidence

```console
$ cargo test --lib scratch_batch5 scratch_rbit -- --nocapture
umulh x0,x0,x0,x0 -> Ok(Word(2613083136))   [#297]   (4th dropped — P1)
uxtw x0,w0,x0     -> Ok(Word(704644064))    [#310]   (3rd dropped — P1)
uxtw d0,w1        -> Ok(Word(704709600))    [#311]   (FP name — P2)
rbit v0.8b, v0.16b -> Ok(Word(778065920))   [#303]   (source T discarded — P3)
rbit x0.8b, x0.8b  -> Ok(Word(778065920))   [#304]   (GPR prefix as V — P2)
```

gcc reference (all REJECTED):

```console
umulh x0, x0, x0, x0  Error: unexpected characters following instruction at operand 3
uxtw x0, w0, x0       Error: unexpected characters following instruction at operand 2
uxtw d0, w1           Error: expected an integer register or SVE vector register at operand 1
rbit v0.8b, v0.16b    Error: operand mismatch
rbit x0.8b, x0.8b     Error: comma expected between operands at operand 2
```

## Root Cause

Established families only: P1 arity (#297, #310), P2 register class (#311, #304),
P3 source arrangement (#303 — same `let (rn, _) = get_neon_reg` discard as #231).

## Severity

Medium ×5.

## Artifacts

- `data_processing.rs` → `mod scratch_batch5`; `neon.rs` → `mod scratch_rbit`; tracker: all five in-sample
