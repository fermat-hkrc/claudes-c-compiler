# Verified Bug: Issue #5 — encode_adc silently ignores a trailing shift operand

**Issue:** [fermat-hkrc/claudes-c-compiler#5](https://github.com/fermat-hkrc/claudes-c-compiler/issues/5)
**Verdict:** ✅ **TRUE BUG — reproduced at unit level (failing assert), independently by the user.**
**Verified on:** branch `explore/pbt-test`, commit `bc6f5170`
**Verified by:** manual reproduction — scratch unit test, user-confirmed run
**Date:** 2026-09-15

---

## 1. The Claim

ARM ADC has no shifted-register form (its imm6 shift field is architecturally fixed 0);
`encode_adc` given `[Reg, Reg, Reg, Shift]` must return `Err` (llvm-mc / GNU as reject
`adc Rd, Rn, Rm, lsl #N`), but it silently drops the 4th operand and encodes the
unshifted instruction.

## 2. Evidence

Scratch test `src/backend/arm/assembler/encoder/data_processing.rs` →
`mod scratch::manual_adc_extra_shift`:

```console
$ cargo test --lib scratch::manual_adc_extra_shift -- --nocapture
adc w0,w0,w0,lsl #0  -> Ok(Word(436207616))
thread '…manual_adc_extra_shift' panicked:
trailing shift must be Err: ADC has no shift field
test …manual_adc_extra_shift ... FAILED
```

- `436207616` = `0x1A000000` — the encoding of plain `adc w0, w0, w0` (shift silently dropped)
- Control (valid 3-reg form) asserts `Ok(Word(0x1a000000))` — same word, proving the
  trailing operand has zero effect
- Run independently reproduced by the user (verbatim same output)

## 3. Root Cause

`src/backend/arm/assembler/encoder/data_processing.rs:774-782`:

```rust
pub(crate) fn encode_adc(operands: &[Operand], set_flags: bool) -> Result<EncodeResult, String> {
    let (rd, is_64) = get_reg(operands, 0)?;
    let (rn, _) = get_reg(operands, 1)?;
    let (rm, _) = get_reg(operands, 2)?;   // ← reads exactly 3; operands.len() never checked
    ...
}
```

No arity check: `operands` beyond index 2 are never inspected or rejected.
(`encode_sbc` at :784-792 has the identical defect — same fix applies.)

## 4. Suggested Fix

After reading the three registers, reject surplus operands:

```rust
if operands.len() != 3 {
    return Err(format!("adc requires exactly 3 operands, got {}", operands.len()));
}
```

(plus the same check in `encode_sbc`). The scratch test is the ready regression test.

## 5. Severity

As claimed: **medium** — invalid assembly silently accepted; the emitted instruction
disagrees with the written source. No conforming-program impact (only affects erroneous
asm input), but it converts an authoring mistake into a silent mis-encoding.

## 6. Reproduction Artifacts

- Scratch regression test: `src/backend/arm/assembler/encoder/data_processing.rs` → `mod scratch::manual_adc_extra_shift` (uncommitted; passes iff fixed)
- Sample tracker: `pbt-out/sampling/sample_100_tracker.md` (#5 in-sample)
