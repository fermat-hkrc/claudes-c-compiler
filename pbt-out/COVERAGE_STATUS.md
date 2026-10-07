# Coverage Status

**Campaign:** encode_bgtz (standard tier)
**Coverage evidence:** file-level (symbol presence) — `coverage_gaps` reported no .gcda/.profraw for this Rust target and listed unrelated C++ pbt_test binaries; encode_bgtz was NOT LINKED into those binaries. Rust execution evidence is cargo test output (14 tests: 12 pass + 2 fail on extra-operand) plus llvm-mc differential KATs.

## Summary
| Metric | Value |
|--------|-------|
| Target function | encode_bgtz |
| Properties | 12 (11 passing, 1 failing) |
| KAT + regression | 1 KAT pass, 1 regression fail (witness) |
| Bugs | 1 (extra operand ignored) |
| Sweep rounds spent | 1 (manual audit of documented surface) |

## Documented behaviors vs properties
| Behavior | Property | Result |
|----------|----------|--------|
| llvm-mc bgtz rs, 0 word match | encode_bgtz_diff_llvm_mc | pass |
| llvm-mc blt x0, rs, 0 match | encode_bgtz_diff_llvm_mc_blt | pass |
| ≡ encode_branch_instr BLT [x0,rs,label] | encode_bgtz_eq_blt_x0_rs | pass |
| B-type opcode/funct3/rs1=0/rs2/imm=0 | encode_bgtz_isa_b_type | pass |
| ABI/xN/fp aliases | encode_bgtz_abi_xn_alias | pass |
| Symbol/Label/Reg target forms | encode_bgtz_target_forms | pass |
| Imm target → reloc string | encode_bgtz_imm_target | pass |
| Imm(0..31) as rs | encode_bgtz_imm_as_rs | pass |
| arity < 2 → Err | encode_bgtz_neg_arity | pass |
| invalid rs → Err | encode_bgtz_neg_invalid_rs | pass |
| invalid target → Err | encode_bgtz_neg_invalid_target | pass |
| extra operand → Err | encode_bgtz_neg_extra | FAIL (bug) |

## Untested in this campaign
- Other pseudo.rs functions (HARD scope: encode_bgtz only)
