# PBT Campaign: elf + linker_common + stack_layout (round 13)

> Whole-repo module loop, round 13. Rounds 01–12 done.

## Scan findings

- **Spec:** module docs in src/backend/elf/*.rs (canonical escape parser shared by all four backends; ELF string table; numeric local labels 1f/1b per GNU semantics), stack_layout README (three-tier slots), linker_common (archive/resolve/check helpers).
- **Test layout:** inline #[cfg(test)]; these modules are exercised transitively by every E2E round (09/12) and every -c assembly (10/11).
- **Buildability probe:** covered by rounds 09-12 baselines.
- **Harness placement:** rung 1 — pbt tests inline in src/backend/elf/mod.rs (p1-p3) and src/backend/mod.rs (p4 structural via readelf).
- **Candidate modules:** StringTable (interning invariants), parse_string_literal (canonical escape parser vs an independent renderer), numeric-label helpers (table laws), multi-target ELF structural KAT (e_machine per target via the four Driver targets).
- **Skipped modules:** stack_layout tiers (integration-bound: consumed inside calculate_stack_space_common during codegen — exercised by rounds 09/12 E2E at both opt levels), linker_common archive/resolve (exercised on every E2E link; unit harness would need .a fixtures — deferred with reason), object_writer/symbol_table internals (exercised by every .o produced in rounds 10-12 and byte-compared against llvm-mc's).

## Module: elf_linker

- [x] Scan: identify targets
- [x] Plan: formalize properties
- [x] Test: write and run
- [x] Review: triage results
