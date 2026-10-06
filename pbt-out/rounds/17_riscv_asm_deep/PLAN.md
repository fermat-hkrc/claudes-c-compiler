# PBT Campaign: RISC-V assembler deep (round 17)

## Scan findings
- Probe matrix (15 invalid forms): 11 silently accepted (imm12 range, shamt range, malformed forms). No riscv cross-as on host — llvm-mc used as the ISA-field-width reference (valid for encoding-limit violations where the value provably changes; verified bytes).
- Valid-form surface already green (round 11).

## Module: riscv_asm_deep
- [x] Scan — 3 classes, dedup-clean (keyword sweeps: zero hits)
- [x] Plan — 3 witness properties + 3 red regressions
- [x] Test — all 3 failing with shrunk witnesses
- [x] Review — issues #796-#798 filed
