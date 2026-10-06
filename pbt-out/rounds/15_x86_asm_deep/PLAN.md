# PBT Campaign: src/backend/x86/assembler deep encoder sweep (round 15)

> Deep pass over the 0-bug round-09 surface: the teammate's encode_* findings
> (#577-755) show the assembler VALIDATION surface is bug-rich; round 09's
> C-level E2E never emitted SIMD/system instructions, so the x86 encoder
> surface is effectively untested.

## Scan findings

- **Probe results (hand, 6+3 forms):** the x86 assembler HAS some validation
  (3-op addq, 1-op vmulps, xmm→gpr movapd all rejected) but three validation
  gaps found immediately, all NEW after dedup (no tracker hits for
  x86-assembler operand/imm/width classes):
  1. `addq $4294967296, %rax` — imm64 out of imm32 range silently truncated
     to `add $0x0, %rax` (llvm-mc rejects).
  2. `movl %eax, %r8w` — GPR width mismatch silently coerced to r8d.
  3. `vaddps %ymm0, %ymm1, %xmm2` — SIMD width mismatch silently promoted.
- **Oracle:** llvm-mc (same reference the teammate used; -triple=x86_64).
- **Harness placement:** inline #[cfg(test)] mod pbt_asm in
  src/backend/x86/mod.rs — in-process Driver -c + llvm-mc + readelf .text
  extraction (same machinery as rounds 10/11). Scratch:
  pbt-out/rounds/15_x86_asm_deep/run/.
- **Candidate modules:** x86/assembler/parser.rs + encoder/* via template
  families: GPR ALU (all suffixes), mov/movabs, shifts (incl. invalid counts),
  SSE (movaps/addps/paddb/pshufd...), AVX 3-op (xmm/ymm), AVX2 broadcast,
  BMI2, bswap/lzcnt/tzcnt/popcnt, x87 (flds/fstpl/faddp), memory forms
  (disp(base,index,scale)).
- **Properties:** p1 valid-form byte equality vs llvm-mc; p2 invalid-form
  rejection (llvm-mc rejects ⇒ ccc must reject; mutations: wrong-width reg,
  out-of-range imm, reg-class swap, extra operand).
- **Explicitly out of scope:** ARM NEON encoders — the teammate's campaign
  owns that surface (263 issues #474-#773, enumerated 2026-10-06); this
  round does not generate NEON forms at all. RISC-V deep forms deferred
  (separate surface, budget).

## Module: x86_asm_deep

- [x] Scan: identify targets (9 hand probes; 3 new validation gaps confirmed)
- [x] Plan: formalize properties
- [x] Test: write and run
- [x] Review: triage — 5 classes (sweep added packed min/max), issues #774-#777 + #782 filed, 5 red witnesses committed; 1 coverage-driven sweep round consumed
