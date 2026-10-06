# PBT Campaign: ARM assembler deep (round 16)

## Scan findings
- Probe matrix (27 valid + 19 invalid forms): 9 double-reference-confirmed validation gaps across 5 classes (width mix, imm ranges, shift ranges, addressing forms, SP slots).
- Oracle: llvm-mc + aarch64-linux-gnu-as (both present natively).
- Harness: inline pbt_asm_deep/pbt_witness in src/backend/arm/mod.rs; scratch pbt-out/rounds/16_arm_asm_deep/run/.
- Excluded: NEON/SVE (teammate's #474-#773), branches/labels (round 10 rationale).

## Module: arm_asm_deep
- [x] Scan — 5 classes identified, dedup-clean
- [x] Plan — p1/p2/p2b + regressions
- [x] Test — p1 passing; p2/p2b failing with shrunk witnesses
- [x] Review — issues #791-#795 filed; 5 red regressions committed
