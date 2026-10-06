# PBT Campaign: src/backend/x86 (round 09)

> Whole-repo module loop, round 09. Rounds 01–08 done (23 issues filed through #765).

## Scan findings

- **Spec:** `src/backend/x86/README.md` + submodule READMEs (codegen, peephole, assembler, linker): full pipeline IR → AT&T asm → builtin assembler → builtin linker (dynamically-linked executables with PLT/GOT); SysV AMD64 ABI (6 GP + 8 FP arg regs); accumulator model + linear-scan regalloc; peephole 15 passes in 7 phases; assembler REX/VEX/EVEX, SSE..AVX2/AVX-512, x87; native x86-64 execution available on this host.
- **Test layout:** inline `#[cfg(test)]` modules, `cargo test --lib backend::x86`; x86 suite is part of the 201-test backend baseline (all green).
- **Buildability probe:** round-08 final serial backend:: run: 201 passed / 1 failed (#765 red witness, asm_expr) / 6 ignored — x86's own tests all green. ccc -O2 end-to-end verified working in round 07's bug triage (narrow_e2e.c compiled and ran correctly).
- **Harness placement:** rung 1 — inline `#[cfg(test)] mod pbt_e2e` in `src/backend/x86/mod.rs` driving the REAL compiler in-process via the public `driver::Driver` (parse_cli_args + run), executing produced binaries natively; gcc as the reference compiler. Scratch dir: pbt-out/rounds/09_x86/run/ (absolute paths; the Driver writes outputs where -o points).
- **Candidate modules:** END-TO-END pipeline (codegen + peephole + assembler + linker together — the only honest unit for "the compiled program computes the C11 semantics"): expression functions over unsigned long long (defined wrap semantics), bitwise/compare/cast mixes, 1..10-argument calls (register + stack args), a loop kernel (LICM/loop codegen/regalloc), -O0 vs gcc and -O2 vs -O0.
- **Skipped modules:** per-file unit properties for `codegen/*` emitters (accumulator-model state machines — integration-bound; the E2E differential covers their composition at both opt levels), `assembler/parser.rs` text parser + `encoder/*` (exercised on every compiled program; the ARM NEON encoder lessons apply at the ARM round), `linker/*` (exercised on every linked executable; dedicated ELF round 13), `peephole/passes/*` in isolation (covered at -O2 E2E; a unit-level asm-text property would need the peephole internal API which is not pub).

## Module: backend_x86

- [x] Scan: identify targets (spec read; baseline green; E2E path verified)
- [x] Plan: formalize properties
- [x] Test: write and run
- [x] Review: triage results
