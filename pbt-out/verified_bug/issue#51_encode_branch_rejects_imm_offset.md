# Verified Bug: Issue #51 — encode_branch rejects immediate PC-offset form `b #imm`

**Issue:** [fermat-hkrc/claudes-c-compiler#51](https://github.com/fermat-hkrc/claudes-c-compiler/issues/51)
**Verdict:** ✅ **TRUE BUG — reproduced at unit level (failing assert); reference cross-check: clang assembles the input and emits exactly the llvm-mc-expected word. Reverse-direction defect: ccc is too strict (missing encoding path), not too lax.**
**Verified on:** branch `explore/pbt-test`, commit `bc6f5170`; **Date:** 2026-09-15

## Claim

`b #imm` (explicit PC offset, 4-byte-aligned, range ±2^27) is valid gas syntax;
`encode_branch([Imm(imm)])` must produce the llvm-mc word (`b #0` → 0x14000000,
`b #4` → 0x14000001, `b #-134217728` → 0x16000000). ccc's encoder only handles the
symbol form and rejects immediates.

## Evidence

```console
$ cargo test --lib scratch_br::manual_branch_imm -- --nocapture
b #0 -> Err("expected symbol at operand 0, got Some(Imm(0))")  (llvm-mc assembles it as 0x14000000)
immediate PC-offset form must be accepted (llvm-mc differential)
test …manual_branch_imm ... FAILED
```

Reference:

```console
$ echo 'b #0' > b0.s && clang --target=aarch64-linux-gnu -c b0.s -o b0.o
$ objdump -s -j .text b0.o
 0000 00000014        # little-endian → 0x14000000 — exactly the llvm-mc word
```

## Root Cause

`src/backend/arm/assembler/encoder/compare_branch.rs:171-181`: `encode_branch` calls
`get_symbol(operands, 0)` unconditionally; an `Imm` operand falls into `get_symbol`'s
error arm. No immediate-offset branch exists.

## Suggested Fix

Add the immediate path before `get_symbol`:

```rust
if let Some(Operand::Imm(imm)) = operands.first() {
    if imm % 4 != 0 || !(-(1 << 27)..(1 << 27)).contains(imm) { return Err(...); }
    let imm26 = ((imm / 4) as u32) & 0x03FF_FFFF;
    return Ok(EncodeResult::Word(0b000101 << 26 | imm26));
}
```

## Severity

As claimed: latent for compiler-generated code (emits `b <label>`), but hand-written
gas-style `.s` with explicit offsets is rejected — a gas-compatibility gap.

## Artifacts

- `compare_branch.rs` → `mod scratch_br::manual_branch_imm`; tracker: #51 in-sample
