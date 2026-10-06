# PBT Coverage Status — round 15 (x86 assembler deep)

| Function | Source file | Test file | Test target | Notes |
|----------|-------------|-----------|-------------|-------|
| x86 builtin assembler (parser + encoders) | assembler/parser.rs, assembler/encoder/* | src/backend/x86/mod.rs (pbt_asm_deep) | cargo test --lib backend::x86::pbt_asm_deep | p1/p2/p3 FAILING — 5 bug classes #774-#777, #782 (GPR width, SIMD width, imm32 truncation, bare popcnt/lzcnt/tzcnt, packed min/max); all double-reference confirmed; 5 red regressions committed |
| valid-form encodings (58 families: 18 + 20 sweep) | assembler/encoder/* | src/backend/x86/mod.rs (p1, p3) | cargo test --lib backend::x86::pbt_asm_deep | byte-identical to llvm-mc on every accepted form (GPR ALU/mov/movabs/shifts/SSE/AVX128/AVX256/broadcast/BMI2/x87/memory/lea/test/xchg/cmov/setcc/imul/idiv/mul/adc/scalar-SSE/cvt*/psrlq/vpermd/crc32/rdtsc/cpuid/cqo/cdq/min-max-scalar) |
| AVX-512 EVEX forms | assembler/encoder/* | — | — | excluded: llvm-mc default target lacks avx512 (needs matched -mattr both sides) — follow-up |
| ARM NEON encoders | arm/assembler | — | — | EXCLUDED — teammate's surface (#474-#773, 263 issues enumerated 2026-10-06); no NEON forms generated, no duplicate-detection risk |

Coverage evidence: file-level (symbol presence) — no .gcda/.profraw; coverage_gaps' passes NOT LINKED list is the known cfg(test) false negative (round-07 properties call those functions directly).
