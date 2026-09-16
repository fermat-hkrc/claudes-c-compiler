# Verified Bug: Issues #127, #137, #138 — SADDLV dest type / STUR extra operand / simm9 wrap

**Issues:** [#127](https://github.com/fermat-hkrc/claudes-c-compiler/issues/127), [#137](https://github.com/fermat-hkrc/claudes-c-compiler/issues/137), [#138](https://github.com/fermat-hkrc/claudes-c-compiler/issues/138)
**Verdict:** ✅ all three **TRUE BUG** — unit probes fail; gcc (aarch64-linux-gnu-gcc 13.3) rejects all three inputs.
**Verified on:** branch `explore/pbt-test`, commit `bc6f5170`; **Date:** 2026-09-15

## Evidence

```console
$ cargo test --lib scratch_saddlv -- --nocapture
saddlv b0, v0.8b -> Ok(Word(238041088))  [#127]     (dest type ignored — same encoding as saddlv h0, v0.8b)
$ cargo test --lib scratch_ldur -- --nocapture
stur w0,[x0,#-256],x2 -> Ok(Word(3088056320))  [#137]  (trailing x2 dropped)
stur w0,[x0,#-257]    -> Ok(Word(3088052224))  [#138]  (-257 wrapped: same encoding as #255)
```

gcc reference (all REJECTED):

```console
saddlv b0, v0.8b          Error: operand mismatch
stur w0, [x0, #-256], x2  Error: cannot combine pre- and post-indexing at operand 2
stur w0, [x0, #-257]      Error: immediate offset out of range -256 to 255 at operand 2
```

## Root Cause (established families + one new)

- **#127** new family: destination register *type* ignored — `encode_neon_across_long`
  (`neon.rs:1724+`) extracts only `parse_reg_num` for the dest and discards the
  prefix (b/h/s/d) that the ISA requires to match the source arrangement (8b→h).
- **#137** arity (trailing operand ignored) — family #5/#38/#42/#46/#53/#85/#109/#113.
- **#138** immediate masking (simm9 masked, no range check) — family #14/#17/#25/#59/#121.

## Suggested Fix

Validate dest register class/prefix against the source arrangement; check trailing
operands; range-check simm9 ∈ [-256, 255].

## Severity

Medium ×3 (as claimed).

## Artifacts

- `neon.rs` → `mod scratch_saddlv`; `load_store.rs` → `mod scratch_ldur`; tracker: #127, #137, #138 in-sample
