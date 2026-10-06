# PBT Campaign: src/backend/riscv (round 11)

> Whole-repo module loop, round 11. Rounds 01–10 done.

## Scan findings

- **Spec:** src/backend/riscv/ (assembler/linker READMEs); riscv-specific flags (-march=, -mno-relax) in the driver CLI.
- **Test layout:** inline #[cfg(test)]; cargo test --lib backend::riscv (green in baseline).
- **Buildability probe:** manual — ccc-riscv -c r.s produces .o whose .text bytes MATCH llvm-mc -triple=riscv64 on a 19-instruction sample (ALU/imm/lui/loads/stores/shifts/w-ops/sltu).
- **Harness placement:** rung 1 — inline #[cfg(test)] mod pbt_asm in src/backend/riscv/mod.rs; in-process Driver with argv[0]="ccc-riscv" (target selected by binary name); readelf .text extraction; llvm-mc reference. Scratch: pbt-out/rounds/11_riscv/run/.
- **Candidate modules:** builtin RISC-V assembler general surface: register-register ALU (add/sub/and/or/xor/sll/srl/sra/slt/sltu + w-forms), immediates (addi/andi/ori/xori ±2048, shifts 0..31, lui), loads/stores (ld/sd/lw/sw/lbu/lh with in-range offsets).
- **Skipped modules:** branches/label forms (same rationale as round 10), codegen+linker (no qemu on this host; structural ELF checks in round 13), compressed-instruction (RVC) surface (march-dependent; default march=gc emits compressed — probe confirmed uncompressed agreement; RVC differential deferred with reason: assembler emits what codegen produces and codegen output can't be executed here).

## Module: backend_riscv

- [x] Scan: identify targets
- [x] Plan: formalize properties
- [x] Test: write and run
- [x] Review: triage results
