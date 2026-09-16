# Verified Bug: Issue #14 — ADD/SUB silently encodes ROR (and other invalid shifts/extends)

**Issue:** [fermat-hkrc/claudes-c-compiler#14](https://github.com/fermat-hkrc/claudes-c-compiler/issues/14)
**Verdict:** ✅ **TRUE BUG — reproduced at unit level with four distinct probes (failing asserts), independently by the user. Strengthened by the sweep round from 2 to 4 probes.**
**Verified on:** branch `explore/pbt-test`, commit `bc6f5170`
**Verified by:** manual reproduction — scratch unit test, user-confirmed run
**Date:** 2026-09-15

---

## 1. The Claim

AArch64 ADD/SUB shifted-register form allows only LSL/LSR/ASR with amounts in range
(0..=31 for w regs); ROR is invalid. Extended-register `imm3 > 4` is UNALLOCATED.
llvm-mc and GNU as reject the malformed forms; `encode_add_sub` must return `Err`,
but instead it defaults unknown kinds to LSL/UXTX and masks amounts.

## 2. Evidence

Scratch test `src/backend/arm/assembler/encoder/data_processing.rs` →
`mod scratch::manual_add_sub_invalid_shift`:

```console
$ cargo test --lib scratch::manual_add_sub_invalid_shift -- --nocapture
add w0,w1,w2,ror #0  -> Ok(Word(184680480))  (should be Err)
add w0,w1,w2,lsl #64 -> Ok(Word(184680480))  (should be Err)
add w0,w1,w2,lsl #2  -> Ok(Word(184682528))  (control, Ok)
add x0,x1,w2,sxtw #8 -> Ok(Word(2334310432))  (imm3 = 8 & 0x7 = 0 — silent truncation)
add x0,x1,w2,foo #0  -> Ok(Word(2334285856))  (option = _ => 0b011 UXTX — silent default)
thread '…manual_add_sub_invalid_shift' panicked:
ROR is not a valid ADD/SUB shift; must be Err
test …manual_add_sub_invalid_shift ... FAILED
```

Four distinct manifestations, all returning `Ok`:

| Input | Defect | Evidence |
|---|---|---|
| `ror #0` | unknown shift kind → `_ => 0b00` (LSL) | `0x0B020000` |
| `lsl #64` (w regs, max 31) | amount masked `& 0x3F` | **identical word** `0x0B020000` — 64 & 63 = 0 |
| `sxtw #8` | extend amount masked `& 0x7` → imm3 0 | extend arm silently truncates |
| extend kind `foo` | unknown kind → `_ => 0b011` UXTX default | invented encoding accepted |

Run independently reproduced by the user (verbatim same output).

## 3. Root Cause

`src/backend/arm/assembler/encoder/data_processing.rs:291` (`encode_add_sub`):

- Shift-kind match (`:435-440`): `"lsl" => 0b00, "lsr" => 0b01, "asr" => 0b10, _ => 0b00` —
  any unknown kind (ROR, typos) silently becomes LSL
- Shift amount (`:446`): `(shift_amount & 0x3F) << 10` — masked, never range-checked
  against sf (32-bit regs cap at 31)
- Extend-kind match (`:408`): `_ => 0b011, // default UXTX/LSL` — unknown extends accepted
- Extend amount (`:410`): `let imm3 = *amount & 0x7;` — masked; `imm3 > 4` is UNALLOCATED
  per the architecture, and legitimate amounts ≥ 8 silently truncate

## 4. Suggested Fix

Replace the catch-all arms with rejection and validate ranges:

```rust
let shift_type = match kind.as_str() {
    "lsl" => 0b00, "lsr" => 0b01, "asr" => 0b10,
    _ => return Err(format!("invalid shift kind for add/sub: {}", kind)),
};
let max = if is_64 { 63 } else { 31 };
if *amount > max { return Err(format!("shift amount {} out of range", amount)); }
```

and equivalently for the extend arm (validate kind against the 8 known specifiers; require
`amount <= 4`). The scratch test's four asserts are the ready regression test.

## 5. Severity

As claimed: **medium** — malformed asm silently assembled into well-formed but unintended
instructions. No conforming-program impact; authoring errors become silent mis-encodings.

## 6. Reproduction Artifacts

- Scratch regression test: `src/backend/arm/assembler/encoder/data_processing.rs` → `mod scratch::manual_add_sub_invalid_shift` (uncommitted; 4 asserts, passes iff fixed)
- Sweep round note: `pbt-out/REPORT.md` ("Sweep round (this session)") — extend-arm probes added by the coverage-substitute sweep
- Sample tracker: `pbt-out/sampling/sample_100_tracker.md` (#14 in-sample)
