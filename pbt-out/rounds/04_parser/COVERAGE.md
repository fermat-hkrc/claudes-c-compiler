# PBT Coverage Status — round 04 (src/frontend/parser)

Round 04 campaign set (canonical): this file plus `REPORT.md`, `PROPERTIES.md`,
`PLAN.md`, `report.json` at `pbt-out/`; round archive under
`pbt-out/rounds/04_parser/` (FUNCTION_INDEX.md, CHANGE_SURFACE.md, build.log,
run/). (Round 01 = `src/common`, round 02 = `src/frontend/lexer`, round 03 =
`src/frontend/preprocessor`, archives under `rounds/`.)

| Function | Source file | Test file | Test target | Notes |
|----------|-------------|-----------|-------------|-------|
| Parser::parse_binary_expr + parse_expr/parse_assignment/parse_conditional/parse_cast/parse_unary/parse_postfix/parse_primary | expressions.rs | src/frontend/parser/parse.rs | cargo test --lib frontend::parser | pass (P1 precedence differential 1024 cases, P2 round-trip 1024 cases) |
| Parser::resolve_type_flags + parse_type_specifier/collect_trailing_specifiers/consume_trailing_qualifiers | types.rs | src/frontend/parser/parse.rs | cargo test --lib frontend::parser | pass (P3 permutations 1024 + documented KAT pairs) |
| Parser::combine_declarator_parts + parse_declarator_with_attrs/is_paren_declarator/fold_simple_derived | declarators.rs, types.rs | src/frontend/parser/parse.rs | cargo test --lib frontend::parser | pass (P4 bare/paren inside-out 1024 cases, P4b fptr layout 512; gcc-verified model) |
| Parser::parse_stmt (+ parse_compound/selection/iteration/jump paths) | statements.rs | src/frontend/parser/parse.rs | cargo test --lib frontend::parser | pass (P5 no-false-reject 1024; P5b local-decl/for-init 512) |
| Parser::parse_local_declaration / parse_initializer / parse_struct_fields / parse_enum_* | declarations.rs, types.rs | src/frontend/parser/parse.rs | cargo test --lib frontend::parser | pass (P5b, P8 struct field preservation 512) |
| Parser::parse (entry loop + error recovery) | parse.rs | src/frontend/parser/parse.rs | cargo test --lib frontend::parser | pass (P6 soup 512, P6b truncation, P6c deep nesting 256 — no panic) |
| Parser::builtin_typedefs + typedef table context (is_type_specifier/is_typedef_label) | parse.rs | src/frontend/parser/parse.rs | cargo test --lib frontend::parser | pass (P7 context flip 512) |
| split_first_word (change surface) | text_processing.rs | src/frontend/preprocessor/text_processing.rs | cargo test --lib pbt_round04 | pass (P9 1024 cases + doc KATs) |
| is_ident_start / is_ident_cont / tokens (change surface, test-support oracle) | pbt_support.rs | src/frontend/preprocessor/pbt_support.rs | cargo test --lib round04_tests | pass (P10: u8 classification 512 + ws-collapse invariance 512) |
| spacer / new_pp / pp_output (change surface) | conditionals.rs, pipeline.rs, macro_defs.rs | — (#[cfg(test)] fixtures) | cargo test --lib frontend::preprocessor | skipped: round-03 test fixtures from the archive commit, not SUT code (PLAN.md Skipped modules); they execute on every preprocessor test run |
| parse_gcc_attribute_list / dispatch_gcc_attribute / mode+vector attrs | parse.rs | src/frontend/parser/parse.rs | cargo test --lib frontend::parser | incidental only (P5/P5b/P6 programs); dedicated attribute property deferred — recorded, not silently skipped |
| eval_const_int_expr* family | declarations.rs | — | — | deferred to a future round (constant evaluator shared with enum/bitfield/alignas paths) |
