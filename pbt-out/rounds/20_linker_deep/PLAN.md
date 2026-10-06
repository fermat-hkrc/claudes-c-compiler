# PBT Campaign: linker deep + numeric labels + inline asm (round 20)

## Scan findings
- Linker batteries (all vs gcc, natively executed): multi-file linking w/
  cross-TU calls, weak-symbol override, common(tentative)-vs-strong,
  shared library + PIC (both lib and caller built by ccc), TLS
  (_Thread_local incl. address-identity), copy relocations (.so data
  accessed directly) — 6/6 identical to gcc at -O0/-O2.
- Numeric labels (GNU 1f/1b semantics incl. redefinition): x86-64 bytes ==
  as == llvm-mc; AArch64 bytes == gas; RISC-V resolves via relocations at
  LINK time — linked exe's jal offsets (+4/-4/+8/-4) decode identical to
  llvm-mc's inline resolution (a correct design difference, not a bug).
- Inline asm E2E: constraints "=r"/"r"/"0"/"+r"/"=&r", multi-instruction
  templates, statement expressions — identical to gcc at both opt levels.
- i686 assembler validation carried over from round 19's probe (#800).

## Module: linker_deep
- [x] Scan (6 link batteries + 3-ISA label sweep + inline-asm battery)
- [x] Plan / [x] Test / [x] Review — ZERO bugs: this surface is solid;
  the only finding of the round (#800) was the i686 assembler, not the linker.
