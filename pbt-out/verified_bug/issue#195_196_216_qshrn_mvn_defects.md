# Verified Bug: Issues #195, #196, #216 — SQSHRN extra operand / SQSHRN GPR dest / MVN arrangement

**Issues:** [#195](https://github.com/fermat-hkrc/claudes-c-compiler/issues/195), [#196](https://github.com/fermat-hkrc/claudes-c-compiler/issues/196), [#216](https://github.com/fermat-hkrc/claudes-c-compiler/issues/216)
**Verdict:** ✅ all three **TRUE BUG** — unit probes fail; gcc rejects all three inputs.
**Verified on:** branch `explore/pbt-test`, commit `bc6f5170`; **Date:** 2026-09-15

## Evidence

```console
$ cargo test --lib scratch_qshrn scratch_mvn -- --nocapture
sqshrn v0.8b,v0.8h,#1,v0.8b -> Ok(Word(252679168))  [#195]   (4th operand dropped)
sqshrn x0,v0.8h,#1          -> Ok(Word(252679168))  [#196]   (GPR dest accepted — same word as #195!)
mvn v0.4h, v0.4h            -> Ok(Word(773871616))  [#216]   (4h accepted, encoded as 8b form Q=0)
#195: fourth operand must be Err for sqshrn
```

The identical word for #195/#196 confirms both the surplus operand and the dest
register type are entirely ignored by `encode_neon_qshrn`.

gcc reference (all REJECTED):

```console
sqshrn v0.8b, v0.8h, #1, v0.8b  Error: unexpected characters following instruction at operand 3
sqshrn x0, v0.8h, #1            Error: unexpected register type at operand 1
mvn v0.4h, v0.4h                Error: operand mismatch
```

## Root Cause (established families)

- **#195** arity (only `len < 3` checked) — CWE-628 family.
- **#196** `get_neon_reg` accepts `Operand::Reg` via `parse_reg_num` — register-class family (CWE-20).
- **#216** arrangement allow-list missing (only 8b/16b valid for MVN; others silently take the 8b encoding) — CWE-20/478 family.

## Severity

Medium ×3 (as claimed).

## Artifacts

- `neon.rs` → `mod scratch_qshrn`; `data_processing.rs` → `mod scratch_mvn`; tracker: #195, #196, #216 in-sample
