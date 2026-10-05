# PBT Coverage Status (round 05 — src/frontend/sema + change surface)

| Function | Source file | Test file | Test target | Notes |
|----------|-------------|-----------|-------------|-------|
| infer_binop_ctype / infer_expr_ctype | type_checker.rs | src/frontend/sema/type_checker.rs | cargo test --lib frontend::sema::type_checker | P1 failing -> b1 (ul+ll signedness); P2 passing |
| enum_constant_type | type_checker.rs | src/frontend/sema/type_checker.rs | cargo test --lib frontend::sema::type_checker | P2 passing (1024, boundary-exact) |
| process_enum_variants / collect_enum_constants_from_type_spec | analysis.rs | src/frontend/sema/analysis.rs | cargo test --lib frontend::sema::analysis | P3 failing -> b3 (i64::MAX panic) |
| analyze / analyze_function_def / analyze_compound_stmt / analyze_stmt | analysis.rs | src/frontend/sema/analysis.rs | cargo test --lib frontend::sema::analysis | P5 failing -> b2; P6/P12/P13 passing |
| stmt_can_fall_through / compound_can_fall_through / switch_can_fall_through / segment_outcome / stmt_switch_outcome / switch_has_default / stmt_contains_default / is_noreturn_call / is_constant_true_expr / try_eval_constant_bool | analysis.rs | src/frontend/sema/analysis.rs | cargo test --lib frontend::sema::analysis | P6 passing (1024, documented-rule model) |
| declare_implicit_functions | analysis.rs | src/frontend/sema/analysis.rs | cargo test --lib frontend::sema::analysis | P13 passing (negative contract) |
| pop_scope / push_scope / insert_enum_scoped / insert_typedef_scoped / insert_typedef_alignment_scoped / insert_struct_layout_scoped / insert_struct_layout_scoped_from_ref / invalidate_ctype_cache_scoped(_from_ref) | type_context.rs | src/frontend/sema/type_context.rs | cargo test --lib frontend::sema::type_context | P4 failing -> b4 (state machine) |
| eval_const_expr (+cast/binop arms) | const_eval.rs | src/frontend/sema/const_eval.rs | cargo test --lib frontend::sema::const_eval | P7 failing -> b5; P9 passing |
| sizeof_type_spec / sizeof_expr / alignof_type_spec / ctype_size / is_expr_unsigned | const_eval.rs | src/frontend/sema/const_eval.rs | cargo test --lib frontend::sema::const_eval | P8 passing (SysV ABI reference grid) |
| is_builtin / is_atomic_builtin / normalize_atomic_size_suffix | builtins.rs | src/frontend/sema/builtins.rs | cargo test --lib frontend::sema::builtins | P10 passing (1024) |
| resolve_builtin / strip_sync_size_suffix | builtins.rs | src/frontend/sema/builtins.rs | cargo test --lib frontend::sema::builtins | P11 passing |
| pbt_tests::parse_src (change surface) | parse.rs | src/frontend/parser/parse.rs | cargo test --lib frontend::parser::parse::pbt_tests::p_r05 | P14 passing — FAILURE branch driven (errs>=1, no panic) |
| sut_tokens (change surface) | pbt_support.rs | src/frontend/preprocessor/pbt_support.rs | cargo test --lib frontend::preprocessor::pbt_support::round05_tests | P15 passing — success + failure-injection arms |
| p9_split_first_word_contract (change surface) | text_processing.rs | src/frontend/preprocessor/text_processing.rs | cargo test --lib frontend::preprocessor::text_processing | skipped-as-test-code; re-executed green in probe (round-04 property) |
