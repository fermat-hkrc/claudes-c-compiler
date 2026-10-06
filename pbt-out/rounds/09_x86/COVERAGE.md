# PBT Coverage Status — round 09 (src/backend/x86)

| Function | Source file | Test file | Test target | Notes |
|----------|-------------|-----------|-------------|-------|
| full pipeline (driver::Driver Full mode, x86) | x86/{codegen,peephole,assembler,linker} | src/backend/x86/mod.rs (pbt_e2e) | cargo test --lib backend::x86::pbt_e2e | pass — 480 programs × {-O0,-O2} == gcc natively (u64 int paths + double/SSE paths + loop kernel + 1..10-arg calls) |
| codegen/* emitters (unit) | x86/codegen/*.rs | — | — | integration-bound (accumulator-model state); covered by the E2E composition at both opt levels |
| codegen/peephole/passes (unit) | x86/codegen/peephole/passes/* | — | — | internal API not pub; covered at -O2 E2E |
| assembler parser/encoders (unit) | x86/assembler/* | — | — | exercised on every compiled program; encoder KATs belong to the ARM round methodology |
| linker/* (unit) | x86/linker/* | — | — | exercised on every linked executable; dedicated ELF round 13 |
