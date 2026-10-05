# PBT Coverage Status

| Function | Source file | Test file | Test target | Notes |
|----------|-------------|-----------|-------------|-------|
| bytes_to_string | encoding.rs | src/common/encoding.rs | cargo test --lib common::encoding | pass (P1: PUA round-trip, 1024 cases) |
| decode_pua_byte | encoding.rs | src/common/encoding.rs | cargo test --lib common::encoding | pass (P1) |
| truncate_and_extend_bits | const_arith.rs | src/common/const_arith.rs | cargo test --lib common::const_arith | pass (P2: mask/sign-extend reference + idempotence, 1024 cases) |
| eval_const_binop | const_arith.rs | src/common/const_arith.rs | cargo test --lib common::const_arith | pass (P3 C99 div/rem identity, P4 div-by-zero None, P5 comparison totality — 1024 cases each; int+float paths) |
| eval_const_binop_int | const_arith.rs | src/common/const_arith.rs | cargo test --lib common::const_arith | pass (P3/P4/P5 via eval_const_binop) |
| eval_const_binop_float | const_arith.rs | src/common/const_arith.rs | cargo test --lib common::const_arith | pass (P6 differential vs native f64, 1024 cases; LongDouble f128 path exercised by P9) |
| negate_const | const_arith.rs | src/common/const_arith.rs | cargo test --lib common::const_arith | pass (P14 involution, 1024 cases) |
| bitnot_const | const_arith.rs | src/common/const_arith.rs | cargo test --lib common::const_arith | pass (P14 involution, 1024 cases) |
| eval_builtin_call | const_eval.rs | src/common/const_eval.rs | cargo test --lib common::const_eval | fail (P12 clz/ctz/popcount/parity/ffs differential PASS 1024 cases; P12b bswap32 → BUG B4) |
| eval_literal | const_eval.rs | src/common/const_eval.rs | cargo test --lib common::const_eval | pass (P12/P12b via builtin64 eval closure) |
| f64_to_f128_bytes_lossless | long_double.rs | src/common/long_double.rs | cargo test --lib common::long_double | fail (P7/P9/boundary → BUG B1 critical: u128 underflow for all \|x\| < 1.0) |
| f128_bytes_to_f64 | long_double.rs | src/common/long_double.rs | cargo test --lib common::long_double | fail (P10 → BUG B3: truncates instead of round-to-nearest; also boundary constants) |
| x87_bytes_to_f64 | long_double.rs | src/common/long_double.rs | cargo test --lib common::long_double | fail (P10/P11 witnesses reach it; the function itself is the CORRECT sibling in B3; P11 failing side is B2) |
| x87_bytes_to_f128_bytes | long_double.rs | src/common/long_double.rs | cargo test --lib common::long_double | pass (exact bridge verified by P10's agreement on truncated-away-zero inputs and existing unit tests) |
| f64_to_x87_bytes_simple | long_double.rs | src/common/long_double.rs | cargo test --lib common::long_double | fail (P11 → BUG B2: subnormal mis-encode) |
| i64_to_f128_bytes | long_double.rs | src/common/long_double.rs | cargo test --lib common::long_double | pass (P8 exact round-trip, 1024 cases) |
| f128_bytes_to_i64 | long_double.rs | src/common/long_double.rs | cargo test --lib common::long_double | pass (P8 exact round-trip) |
| f128_add | long_double.rs | src/common/long_double.rs | cargo test --lib common::long_double | exercised by P9 (blocked by B1 panic before assertions; existing unit tests green) |
| f128_sub | long_double.rs | src/common/long_double.rs | cargo test --lib common::long_double | exercised by P9 (see above) |
| f128_mul | long_double.rs | src/common/long_double.rs | cargo test --lib common::long_double | exercised by P9 (see above) |
| f128_div | long_double.rs | src/common/long_double.rs | cargo test --lib common::long_double | exercised by P9 (see above) |
| align_up | types.rs | src/common/types.rs | cargo test --lib common::types | pass (P13 laws, 1024 cases) |
| FxHasher::write / write_uN / finish | fx_hash.rs | src/common/fx_hash.rs | cargo test --lib common::fx_hash | pass (P15 consistency, 1024 cases) |
| SymbolTable::new/push_scope/pop_scope/declare/lookup | symbol_table.rs | src/common/symbol_table.rs | cargo test --lib common::symbol_table | pass (P16 scope-shadowing state machine, 1024 cases) |

Untested this round (documented skips, see PLAN.md): error.rs (diagnostics rendering/UI), source.rs (file I/O + line maps), temp_files.rs (filesystem side effects), asm_constraints.rs (table lookup), type_builder.rs (builder over types.rs), const_arith AST predicates (is_zero_expr/is_null_pointer_constant — parser-bound, next round), const_eval promote_sub_int / irconst_to_bits (flagged as Design Caveat), eval_const_binop_i128 (reached via eval_const_binop dispatch only when operands are I128 — next round).
