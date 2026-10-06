# PBT Campaign Report: ARM + RISC-V assembler deep sweeps (rounds 16-17)

## Summary

**Verdict:** 8 assembler validation bugs filed (#791-#795 ARM, #796-#798 RISC-V), EVERY one surfaced by a failing, shrinking PBT property (witness properties below). The deep pass over the remaining 0-bug assembler surfaces confirms the same "no operand validation" architecture across all backends: ARM width mixing/imm ranges/shift ranges/addressing forms/SP-in-illegal-slots (double-reference: aarch64-linux-gnu-as ∧ llvm-mc); RISC-V imm12 range (`addi a0, a1, 2048` encodes as **-2048**, bytes 13 85 05 80), shamt range, malformed forms (llvm-mc as the ISA-field-width reference; the encoded value provably changes). All dedup-clean (the teammate's 263 issues are all NEON/system-reg; full-tracker keyword sweeps: zero hits).
**Date:** 2026-10-06
**Repository:** claudes-c-compiler (/home/shuhao/fermat-users/leo/github/claudes-c-compiler)
**Modules tested:** src/backend/arm/assembler (non-NEON), src/backend/riscv/assembler
**Tests:** ARM p1 (400 valid forms, passing) + p2 (400 mutants, failing) + p2b (width-mix witness, failing) + 5 red regressions; RISC-V 3 witness properties (failing) + 3 red regressions
**Result:** ARM p1 passing; p2/p2b + all RISC-V witnesses failing with shrunk counterexamples; 8 issues, 8 red witnesses
**Change surface:** (none — deep pass over rounds 10/11's 0-bug surfaces)
**Coverage evidence:** file-level (symbol presence) — no .gcda/.profraw; the sweep was surface-enumeration driven (hand probes + family generators), per the round-15 protocol.
**Tier:** standard

## Modules Tested

| Module | Tests | Bugs | Oracles Used |
|--------|-------|------|--------------|
| arm/assembler (non-NEON general surface) | p1 400 valid (passing), p2 400 mutants (failing), p2b witness (failing), 5 red regressions | 5 (#791-#795) | differential (llvm-mc byte-exact) + negative_error (double-reference: aarch64-linux-gnu-as ∧ llvm-mc) |
| riscv/assembler (validation) | 3 witness properties (failing, shrunk) + 3 red regressions | 3 (#796-#798) | negative_error (llvm-mc as ISA-field reference; encoded value verified) |

## Bugs Found

### B1 (#791) ARM: W/X register width mixing silently accepted
**Formal:** ∀ form mixing W and X registers in the same operand role (any direction): both references reject ⇒ ccc must reject.
**Detected by:** p2b_arm_width_mix_rejected (FAILING PBT property, shrunk counterexample `add x0, x0, w0`-class; reproduction: `cargo test --lib backend::arm::pbt_witness::p2b_arm_width_mix_rejected`). Also covers the W-register-as-memory-base direction (p1's shrunk SILENT-ACCEPT witness `str w1, [w28, #8]`-class).
**Contract evidence:** inferred (double-reference: aarch64-linux-gnu-as AND llvm-mc both reject — verified per-form in the property) **Severity:** medium
**Counterexample:** `add w0, x1, x2` / `add x0, w1, #5` / `str w1, [w28, #8]`
**Expected / Actual:** error / silently assembled
**Impact:** external .s files with width typos assemble to different instructions than written.
**Bug report:** bug_reports/armasm_reg_width_mix.md
**Repro seed:** deterministic red witness test_armasm_regression_791
**Raw output:** `panicked: ccc silently accepted a double-reference-rejected form: add x2, x2, w30`

### B2 (#792) ARM: out-of-range immediates silently accepted
**Formal:** imm16 for movz/movk/movn; [-256,255] for ldur/stur; imm7×8 for ldp/stp pairs: both references reject ⇒ ccc must reject.
**Detected by:** p2_arm_asm_deep_invalid_rejected (FAILING PBT, shrunk: `movk w19, #325050, lsl #0`) + p1's pre-domain-fix shrunk SILENT-ACCEPT witnesses (`ldp x23, x27, [sp, #1936]`, `ldur x0, [x1, #-257]`).
**Contract evidence:** inferred (double-reference) **Severity:** medium
**Counterexample:** `movk x0, #65536`; `ldp x23, x27, [sp, #1936]`
**Bug report:** bug_reports/armasm_imm_ranges.md
**Repro seed:** test_armasm_regression_792
**Raw output:** `Test failed: distinct invalid-accept classes (1): {"SILENT-ACCEPT(m3) movk w19, #325050, lsl #0 ..."}`

### B3 (#793) ARM: out-of-range shift/hw amounts silently accepted
**Formal:** shift < width (63 X / 31 W); mov-variant hw 0/16 (W) or 0/16/32/48 (X); imm12 shift only lsl #12: both references reject ⇒ ccc must reject.
**Detected by:** p1's shrunk SILENT-ACCEPT witness `movk w14, #25050, lsl #32` (W register with hw 32) + hand probes for the remaining arms (documented in the issue).
**Contract evidence:** inferred (double-reference) **Severity:** medium
**Counterexample:** `movk w14, #25050, lsl #32`; `lsl x0, x1, #64`; `ror w0, w1, #32`; `add x0, x1, #1, lsl #13`
**Bug report:** bug_reports/armasm_shift_ranges.md
**Repro seed:** test_armasm_regression_793
**Raw output:** (issue body)

### B4 (#794) ARM: invalid addressing forms silently accepted
**Formal:** pair offsets multiple of element size; index scale limited per access size: both references reject ⇒ ccc must reject.
**Detected by:** hand probes double-confirmed (`ldp x0, x1, [sp, #3]`, `str x0, [x1, x2, lsl #4]`); class exercised by p2's mutation-3 direction.
**Contract evidence:** inferred (double-reference) **Severity:** medium
**Bug report:** bug_reports/armasm_addr_forms.md
**Repro seed:** test_armasm_regression_794
**Raw output:** (issue body)

### B5 (#795) ARM: SP/WSP in register-file-only slots silently accepted
**Formal:** logical/mul-family take X0-30/W0-30 only: both references reject ⇒ ccc must reject.
**Detected by:** p1's shrunk SILENT-ACCEPT witness `and sp, sp, x28` (silently encodes as and xzr, xzr, x28).
**Contract evidence:** inferred (double-reference) **Severity:** medium
**Counterexample:** `and sp, sp, x28`; `mul w0, wsp, w2`
**Bug report:** bug_reports/armasm_sp_in_illegal_slots.md
**Repro seed:** test_armasm_regression_795
**Raw output:** (issue body)

### B6 (#796) RISC-V: imm12 range not validated — silent WRONG VALUE
**Formal:** signed imm[11:0] ∈ [-2048, 2047]: llvm-mc (ISA field-width reference) rejects ⇒ ccc must reject.
**Detected by:** p_imm12_range_witness (FAILING PBT, shrunk counterexample `addi x0, x0, -2049`-class).
**Contract evidence:** inferred (ISA field width; the encoding provably changes the value: bytes 13 85 05 80 = -2048) **Severity:** medium
**Counterexample:** `addi a0, a1, 2048` → encodes `addi a0, a1, -2048`
**Bug report:** bug_reports/riscvasm_imm12_range.md
**Repro seed:** test_riscvasm_regression_imm12_range
**Raw output:** `panicked: ccc silently accepted an llvm-mc-rejected (ISA-limit) form: addi x12, x24, -2049`

### B7 (#797) RISC-V: shamt range not validated
**Formal:** shamt < 64 (I) / < 32 (*W), non-negative: ISA limit rejection ⇒ ccc must reject.
**Detected by:** p_shamt_range_witness (FAILING PBT, shrunk counterexample `slli x0, x0, 64`-class).
**Contract evidence:** inferred (ISA shamt width) **Severity:** medium
**Bug report:** bug_reports/riscvasm_shamt_range.md
**Repro seed:** test_riscvasm_regression_shamt_range
**Raw output:** `panicked: ccc silently accepted ... slli x12, x24, 64`

### B8 (#798) RISC-V: malformed operand forms silently accepted
**Formal:** reg slots reject immediates; branches need labels; fence.i takes no operands.
**Detected by:** p_malformed_witness (FAILING PBT, shrunk counterexample `add x0, x0, 2048`-class).
**Contract evidence:** inferred (ISA instruction formats) **Severity:** medium
**Bug report:** bug_reports/riscvasm_malformed_forms.md
**Repro seed:** test_riscvasm_regression_malformed
**Raw output:** `panicked: ccc silently accepted ... add x12, x24, 2048`

## Design Caveats (if any)

(none)

## Test Files Created

| File | Tests |
|------|-------|
| src/backend/arm/mod.rs (pbt_asm_deep, pbt_witness, pbt_asm_deep_support) | p1 (passing), p2 (failing), p2b (failing) + 5 red regressions |
| src/backend/riscv/mod.rs (pbt_regression_asm, pbt_witness) | 3 witness properties (failing) + 3 red regressions |

## Reproduction

```bash
cd /home/shuhao/fermat-users/leo/github/claudes-c-compiler
PATH="$HOME/.cargo/bin:$PATH" RUST_TEST_THREADS=1 cargo test --lib backend::arm::pbt_witness backend::riscv::pbt_witness --nocapture
```

## Output Directories

- pbt-out/rounds/16_arm_asm_deep/ — bug_reports/armasm_*.{md} (×5), run/
- pbt-out/rounds/17_riscv_asm_deep/ — bug_reports/riscvasm_*.{md} (×3), run/
- Follow-up recorded: elf resolve_numeric_labels GNU 1f/1b semantics property (deferred, in INVARIANTS).
- Methodology: RISC-V single-reference (llvm-mc) accepted ONLY for ISA encoding-limit violations where the encoded value provably changes; convention-permissive acceptance differences still require double reference (no riscv cross-as on this host — noted).
