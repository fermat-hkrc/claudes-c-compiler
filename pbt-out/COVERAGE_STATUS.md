# Coverage Status

**Campaign target:** encode_bltz (`src/backend/riscv/assembler/encoder/pseudo.rs:318`)
**Effort tier:** standard
**Coverage evidence:** file-level (symbol presence)

## Native coverage tool result

`coverage_gaps` reported no `.gcda`/`.profraw` and listed unrelated C++ `*_pbt_test` binaries. It marked `encode_bltz` NOT LINKED against those binaries. That is expected: this campaign's SUT is a Rust `cargo test --lib` target (`ccc-70d56e2a1978a8d3`), not those C++ tests. The function is linked and executed by:

```text
cargo test --lib encode_bltz -- --test-threads=1
→ 12 property/KAT/regression tests ran against encode_bltz (11 pass, 1 fail)
```

## Sweep round 1 (standard tier allowance)

Manual contract-surface audit of documented behaviors for `encode_bltz`:

| Documented behavior | Property | Result |
|---------------------|----------|--------|
| llvm-mc `bltz rs, 0` word match | encode_bltz_diff_llvm_mc | pass (1000) |
| llvm-mc `blt rs, x0, 0` expansion | encode_bltz_diff_llvm_mc_blt | pass (1000) |
| In-tree BLT rs,x0 metamorphic | encode_bltz_eq_blt_rs_x0 | pass (1000) |
| B-type BRANCH/funct3=100/rs2=x0/imm=0 | encode_bltz_isa_b_type | pass (1000) |
| ABI / xN / fp alias | encode_bltz_abi_xn_alias | pass (1000) |
| Symbol/Label/Reg target forms | encode_bltz_target_forms | pass (1000) |
| Imm target → reloc symbol | encode_bltz_imm_target | pass (1000) |
| Imm(0..31) as rs | encode_bltz_imm_as_rs | pass (1000) |
| Too few operands → Err | encode_bltz_neg_arity | pass (1000) |
| Invalid rs → Err | encode_bltz_neg_invalid_rs | pass (1000) |
| Invalid target → Err | encode_bltz_neg_invalid_target | pass (1000) |
| Extra operand → Err | encode_bltz_neg_extra | FAIL (bug filed) |

**Sweep close reason:** tier round spent; every documented behavior has a property; remaining gap is the extra-operand bug.

## Counts

- Scanned (change surface): 1 function
- Tested this campaign: 1 (encode_bltz)
- Passing properties: 11
- Failing properties / bugs: 1
