# PBT Campaign: src/backend/arm (round 10)

> Whole-repo module loop, round 10. Rounds 01–09 done (issues through #765; x86 E2E green).

## Scan findings

- **Spec:** src/backend/arm/ (assembler/linker READMEs); the NEON/system encoder surface was already heavily covered by another campaign (#577–#755, encode_neon_* etc. — filed against llvm-mc comparisons).
- **Test layout:** inline #[cfg(test)] modules; cargo test --lib backend::arm (all green in the 201-test baseline).
- **Buildability probe:** manual pipeline validated — ccc -mlittle-endian -c t.s produces a valid ELF .o whose .text bytes MATCH llvm-mc -triple=aarch64 on a 12-instruction sample (ALU/shifts/imm/movz/ldr/str/sxtw).
- **Harness placement:** rung 1 — inline #[cfg(test)] mod pbt_asm in src/backend/arm/mod.rs: generated .s files assembled in-process via driver::Driver (-mlittle-endian -c), .text bytes extracted with readelf and compared byte-exact against llvm-mc over the same source. Scratch: pbt-out/rounds/10_arm/run/.
- **Candidate modules:** assembler encoder paths for the general (non-NEON) instruction surface: shift-shift ALU, immediate ALU, logical-immediate (curated valid masks), movz/movk/movn, loads/stores (unsigned/negative offsets, byte/half), extend ops, cmp aliases, shifted-register operands.
- **Skipped modules:** NEON/SVE encoders (covered by #577–#755's campaign — dedup-checked), branches/label-relative forms (relocation-spanning comparison needs label-identical output; deferred), linker (no qemu execution on this host; structural ELF checks land in round 13), codegen (no execution oracle available; the ARM E2E would need qemu — unavailable).

## Module: backend_arm

- [x] Scan: identify targets (pipeline validated; llvm-mc reference confirmed)
- [x] Plan: formalize properties
- [x] Test: write and run
- [x] Review: triage results
