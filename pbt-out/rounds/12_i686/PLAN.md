# PBT Campaign: src/backend/i686 (round 12)

> Whole-repo module loop, round 12. Rounds 01–11 done.

## Scan findings

- **Spec:** src/backend/i686/ (READMEs); ILP32 SysV ABI (stack args by default, -mregparm up to 3), i686 shares x86_common/peephole_common with x86-64.
- **Test layout:** inline #[cfg(test)]; cargo test --lib backend::i686 (green in baseline).
- **Buildability probe:** native i386 ELF execution works on this host (gcc -m32 binary runs); ccc selects i686 via -m32.
- **Harness placement:** rung 1 — inline #[cfg(test)] mod pbt_e2e in src/backend/i686/mod.rs, same shape as round 09's x86 harness (driver in-process, gcc -m32 reference, native execution). Scratch: pbt-out/rounds/12_i686/run/.
- **Candidate modules:** i686 codegen+peephole+assembler+linker composition (E2E), incl. the ILP32 stack-argument paths (1..10 args), 64-bit arithmetic on 32-bit target (u64 ops via register pairs — a classic i686 hazard), double/SSE paths.
- **Skipped modules:** unit emitters (integration-bound, covered by the E2E composition), x87 F128 paths (i686 long double = 80-bit x87; printf %.17g round-trips through double — kept out of v1 to avoid oracle noise), -mregparm variants (flag exists; default cdecl covered).

## Module: backend_i686

- [x] Scan: identify targets
- [x] Plan: formalize properties
- [x] Test: write and run
- [x] Review: triage results
