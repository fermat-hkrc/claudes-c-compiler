# PBT Coverage Status

| Function | Source file | Test file | Test target | Notes |
|----------|-------------|-----------|-------------|-------|
| bytes_to_string | encoding.rs | src/common/pbt_tests/encoding_pbt.rs | cargo test --lib pbt_tests | pass (P6 round-trip, P7 idempotence, 1000 cases each) |
| decode_pua_byte | encoding.rs | src/common/pbt_tests/encoding_pbt.rs | cargo test --lib pbt_tests | pass (via P6 decode_all) |
| eval_const_binop | const_arith.rs | src/common/pbt_tests/const_arith_pbt.rs | cargo test --lib pbt_tests | pass (P8 vs C11 reference model, 1000 cases, boundary-skewed) |
| align_up | types.rs | src/common/pbt_tests/types_pbt.rs | cargo test --lib pbt_tests | pass (P9 minimality laws incl. overflow branch, 1000 cases) |
| compiler_main (E2E pipeline: lexer→parser→sema→IR→opt→codegen→asm→link) | lib.rs | tests/e2e_diff.rs | cargo test --test e2e_diff | P1 arith / P2 ctrl-flow / P3 layout / P4 calls-ABI / P5 globals — see REPORT.md for final verdicts |
