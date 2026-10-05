# Function Index — src/frontend/preprocessor (round 03)

> Total files: 11 | Total functions: 171 | PBT candidates: 68 | Excluded: 103

| Function | Source File | Line | Kind | PBT Candidate | Reason |
|----------|-------------|------|------|---------------|--------|
| def | builtin_macros.rs | 10 | function | no | helper, exercised via enclosing property |
| define_stddef_macros | builtin_macros.rs | 195 | function | no | helper, exercised via enclosing property |
| define_stdbool_macros | builtin_macros.rs | 213 | function | no | helper, exercised via enclosing property |
| define_stdbool_true_false | builtin_macros.rs | 218 | function | no | helper, exercised via enclosing property |
| define_stdatomic_macros | builtin_macros.rs | 226 | function | no | helper, exercised via enclosing property |
| define_builtin_macros | builtin_macros.rs | 22 | function | no | helper, exercised via enclosing property |
| define_float_macros | builtin_macros.rs | 244 | function | no | helper, exercised via enclosing property |
| define_inttypes_macros | builtin_macros.rs | 286 | function | no | helper, exercised via enclosing property |
| define_type_traits_macros | builtin_macros.rs | 359 | function | no | helper, exercised via enclosing property |
| define_limits_macros | builtin_macros.rs | 37 | function | no | helper, exercised via enclosing property |
| define_stdint_macros | builtin_macros.rs | 71 | function | no | helper, exercised via enclosing property |
| expand_condition_macros | conditionals.rs | 105 | function | no | helper, exercised via enclosing property |
| eval_const_expr | conditionals.rs | 250 | function | yes | - |
| tokenize_expr | conditionals.rs | 277 | function | yes | - |
| new | conditionals.rs | 33 | function | yes | - |
| is_active | conditionals.rs | 38 | function | yes | - |
| push_if | conditionals.rs | 43 | function | yes | - |
| new | conditionals.rs | 460 | function | yes | - |
| peek | conditionals.rs | 464 | function | no | 1-token lookahead helper |
| advance | conditionals.rs | 468 | function | yes | - |
| peek_op | conditionals.rs | 477 | function | no | 1-token lookahead helper |
| parse_ternary | conditionals.rs | 481 | function | yes | - |
| parse_or | conditionals.rs | 496 | function | yes | - |
| parse_and | conditionals.rs | 507 | function | yes | - |
| parse_bitor | conditionals.rs | 517 | function | yes | - |
| parse_bitxor | conditionals.rs | 528 | function | yes | - |
| parse_bitand | conditionals.rs | 539 | function | yes | - |
| handle_elif | conditionals.rs | 54 | function | yes | - |
| parse_equality | conditionals.rs | 550 | function | yes | - |
| parse_relational | conditionals.rs | 580 | function | yes | - |
| parse_shift | conditionals.rs | 630 | function | yes | - |
| parse_additive | conditionals.rs | 661 | function | yes | - |
| parse_multiplicative | conditionals.rs | 682 | function | yes | - |
| handle_else | conditionals.rs | 69 | function | yes | - |
| parse_unary | conditionals.rs | 730 | function | yes | - |
| parse_primary | conditionals.rs | 760 | function | yes | - |
| handle_endif | conditionals.rs | 81 | function | yes | - |
| default | conditionals.rs | 87 | function | no | helper, exercised via enclosing property |
| evaluate_condition | conditionals.rs | 94 | function | yes | - |
| resolve_defined_in_expr | expr_eval.rs | 101 | function | yes | - |
| replace_remaining_idents_with_zero | expr_eval.rs | 16 | function | yes | - |
| resolve_has_builtin_call_bytes | expr_eval.rs | 176 | function | no | helper, exercised via enclosing property |
| resolve_has_attribute_call_bytes | expr_eval.rs | 203 | function | no | helper, exercised via enclosing property |
| resolve_has_include_call_bytes | expr_eval.rs | 231 | function | no | helper, exercised via enclosing property |
| skip_paren_arg_bytes | expr_eval.rs | 297 | function | no | helper, exercised via enclosing property |
| is_supported_builtin | expr_eval.rs | 319 | function | no | helper, exercised via enclosing property |
| is_supported_attribute | expr_eval.rs | 336 | function | no | helper, exercised via enclosing property |
| extract_identifier | includes.rs | 161 | function | no | helper, exercised via enclosing property |
| strip_inline_comments<'a> | includes.rs | 180 | function | no | helper, exercised via enclosing property |
| read_c_source_file | includes.rs | 214 | function | no | helper, exercised via enclosing property |
| make_absolute | includes.rs | 229 | function | yes | - |
| format_path_for_line_directive | includes.rs | 249 | function | no | helper, exercised via enclosing property |
| clean_path | includes.rs | 259 | function | yes | - |
| normalize_include_path | includes.rs | 294 | function | yes | - |
| handle_include | includes.rs | 307 | function | no | helper, exercised via enclosing property |
| detect_include_guard | includes.rs | 31 | function | yes | - |
| handle_include_next | includes.rs | 451 | function | no | helper, exercised via enclosing property |
| resolve_include_next_path | includes.rs | 553 | function | no | helper, exercised via enclosing property |
| resolve_include_path | includes.rs | 623 | function | no | helper, exercised via enclosing property |
| resolve_include_path_uncached | includes.rs | 644 | function | no | helper, exercised via enclosing property |
| inject_builtin_macros_for_header | includes.rs | 727 | function | no | helper, exercised via enclosing property |
| inject_fallback_declarations_for_header | includes.rs | 797 | function | no | helper, exercised via enclosing property |
| substitute_params | macro_defs.rs | 1019 | function | yes | - |
| get_va_args | macro_defs.rs | 1121 | function | yes | - |
| get_named_va_args | macro_defs.rs | 1131 | function | yes | - |
| default | macro_defs.rs | 1141 | function | no | helper, exercised via enclosing property |
| is_ppnumber_context | macro_defs.rs | 1147 | function | no | helper, exercised via enclosing property |
| extract_trailing_ident | macro_defs.rs | 1175 | function | yes | - |
| contains_standalone_ident | macro_defs.rs | 1203 | function | yes | - |
| stringify_arg | macro_defs.rs | 1229 | function | yes | - |
| new | macro_defs.rs | 130 | function | yes | - |
| parse_define | macro_defs.rs | 1332 | function | yes | - |
| define | macro_defs.rs | 142 | function | no | helper, exercised via enclosing property |
| new | macro_defs.rs | 1461 | function | yes | - |
| format | macro_defs.rs | 1465 | function | no | helper, exercised via enclosing property |
| undefine | macro_defs.rs | 147 | function | no | helper, exercised via enclosing property |
| is_defined | macro_defs.rs | 152 | function | no | helper, exercised via enclosing property |
| iter | macro_defs.rs | 167 | function | no | helper, exercised via enclosing property |
| get | macro_defs.rs | 172 | function | no | helper, exercised via enclosing property |
| set_line | macro_defs.rs | 178 | function | no | helper, exercised via enclosing property |
| set_file | macro_defs.rs | 186 | function | no | helper, exercised via enclosing property |
| get_file_body | macro_defs.rs | 203 | function | no | helper, exercised via enclosing property |
| set_track_expansions | macro_defs.rs | 210 | function | no | helper, exercised via enclosing property |
| take_expanded_macros | macro_defs.rs | 216 | function | no | helper, exercised via enclosing property |
| expand_line | macro_defs.rs | 222 | function | yes | - |
| expand_line_reuse | macro_defs.rs | 231 | function | yes | - |
| append_with_paste_guard | macro_defs.rs | 252 | function | yes | - |
| would_paste_tokens | macro_defs.rs | 27 | function | yes | - |
| expand_trailing_func_macros | macro_defs.rs | 280 | function | yes | - |
| try_resolve_objlike_to_funclike | macro_defs.rs | 322 | function | yes | - |
| expand_text | macro_defs.rs | 362 | function | yes | - |
| copy_blue_painted | macro_defs.rs | 416 | function | no | helper, exercised via enclosing property |
| expand_identifier | macro_defs.rs | 429 | function | yes | - |
| is_ppnumber_suffix | macro_defs.rs | 493 | function | no | helper, exercised via enclosing property |
| skip_pragma | macro_defs.rs | 510 | function | no | helper, exercised via enclosing property |
| expand_macro_invocation | macro_defs.rs | 535 | function | yes | - |
| copy_block_comment | macro_defs.rs | 596 | function | no | helper, exercised via enclosing property |
| copy_line_comment | macro_defs.rs | 615 | function | no | helper, exercised via enclosing property |
| copy_ppnumber | macro_defs.rs | 628 | function | no | helper, exercised via enclosing property |
| parse_macro_args | macro_defs.rs | 656 | function | yes | - |
| expand_function_macro | macro_defs.rs | 727 | function | yes | - |
| handle_stringify_and_paste<'a> | macro_defs.rs | 823 | function | no | helper, exercised via enclosing property |
| strip_blue_paint | macro_defs.rs | 97 | function | no | helper, exercised via enclosing property |
| new | pipeline.rs | 125 | function | yes | - |
| preprocess | pipeline.rs | 159 | function | yes | - |
| preprocess_included | pipeline.rs | 188 | function | no | helper, exercised via enclosing property |
| preprocess_source | pipeline.rs | 198 | function | yes | - |
| dedup_macro_names | pipeline.rs | 20 | function | no | helper, exercised via enclosing property |
| accumulate_and_expand | pipeline.rs | 485 | function | yes | - |
| ends_with_funclike_macro | pipeline.rs | 577 | function | yes | - |
| set_asm_mode | pipeline.rs | 609 | function | no | helper, exercised via enclosing property |
| set_filename | pipeline.rs | 614 | function | no | helper, exercised via enclosing property |
| errors | pipeline.rs | 637 | function | no | helper, exercised via enclosing property |
| warnings | pipeline.rs | 642 | function | no | helper, exercised via enclosing property |
| current_file | pipeline.rs | 649 | function | no | helper, exercised via enclosing property |
| take_macro_expansion_info | pipeline.rs | 658 | function | no | helper, exercised via enclosing property |
| dump_defines | pipeline.rs | 668 | function | no | helper, exercised via enclosing property |
| define_macro | pipeline.rs | 689 | function | no | helper, exercised via enclosing property |
| undefine_macro | pipeline.rs | 701 | function | no | helper, exercised via enclosing property |
| add_include_path | pipeline.rs | 707 | function | no | helper, exercised via enclosing property |
| add_quote_include_path | pipeline.rs | 714 | function | no | helper, exercised via enclosing property |
| add_system_include_path | pipeline.rs | 720 | function | no | helper, exercised via enclosing property |
| add_after_include_path | pipeline.rs | 726 | function | no | helper, exercised via enclosing property |
| preprocess_force_include | pipeline.rs | 734 | function | no | helper, exercised via enclosing property |
| process_directive | pipeline.rs | 782 | function | yes | - |
| handle_define | pipeline.rs | 888 | function | yes | - |
| handle_undef | pipeline.rs | 894 | function | yes | - |
| handle_ifdef | pipeline.rs | 901 | function | yes | - |
| handle_if | pipeline.rs | 908 | function | yes | - |
| handle_elif | pipeline.rs | 922 | function | yes | - |
| handle_line_directive | pipeline.rs | 932 | function | yes | - |
| default | pipeline.rs | 963 | function | no | helper, exercised via enclosing property |
| handle_pragma_pop_macro | pragmas.rs | 101 | function | no | helper, exercised via enclosing property |
| extract_pragma_macro_name | pragmas.rs | 115 | function | no | helper, exercised via enclosing property |
| handle_pragma_weak | pragmas.rs | 133 | function | no | helper, exercised via enclosing property |
| handle_pragma_redefine_extname | pragmas.rs | 155 | function | no | helper, exercised via enclosing property |
| handle_pragma_pack | pragmas.rs | 172 | function | no | helper, exercised via enclosing property |
| handle_pragma_gcc_visibility | pragmas.rs | 69 | function | no | helper, exercised via enclosing property |
| handle_pragma_push_macro | pragmas.rs | 90 | function | no | helper, exercised via enclosing property |
| handle_pragma | pragmas.rs | 9 | function | yes | - |
| define_predefined_macros | predefined_macros.rs | 17 | function | no | helper, exercised via enclosing property |
| define_simple_macro | predefined_macros.rs | 215 | function | no | helper, exercised via enclosing property |
| bundled_include_dir | predefined_macros.rs | 232 | function | no | helper, exercised via enclosing property |
| default_system_include_paths | predefined_macros.rs | 261 | function | no | helper, exercised via enclosing property |
| set_strict_ansi | predefined_macros.rs | 293 | function | no | helper, exercised via enclosing property |
| set_gnu89_inline | predefined_macros.rs | 306 | function | no | helper, exercised via enclosing property |
| set_optimize | predefined_macros.rs | 322 | function | no | helper, exercised via enclosing property |
| set_pic | predefined_macros.rs | 340 | function | no | helper, exercised via enclosing property |
| set_sse_macros | predefined_macros.rs | 360 | function | no | helper, exercised via enclosing property |
| set_extended_simd_macros | predefined_macros.rs | 390 | function | no | helper, exercised via enclosing property |
| set_target | predefined_macros.rs | 425 | function | no | helper, exercised via enclosing property |
| insert_arch_paths_after_bundled | predefined_macros.rs | 644 | function | no | helper, exercised via enclosing property |
| override_ldbl_binary128 | predefined_macros.rs | 668 | function | no | helper, exercised via enclosing property |
| set_riscv_abi | predefined_macros.rs | 694 | function | no | helper, exercised via enclosing property |
| set_riscv_march | predefined_macros.rs | 726 | function | no | helper, exercised via enclosing property |
| strip_block_comments | text_processing.rs | 100 | function | yes | - |
| join_continued_lines<'a> | text_processing.rs | 212 | function | no | helper, exercised via enclosing property |
| find_continuation_backslash | text_processing.rs | 249 | function | yes | - |
| strip_line_comment | text_processing.rs | 263 | function | yes | - |
| split_first_word | text_processing.rs | 286 | function | yes | - |
| get | text_processing.rs | 29 | function | no | helper, exercised via enclosing property |
| contains_comment_marker | text_processing.rs | 41 | function | no | helper, exercised via enclosing property |
| has_unbalanced_parens | text_processing.rs | 67 | function | yes | - |
| is_ident_start | utils.rs | 10 | function | no | helper, exercised via enclosing property |
| is_ident_cont | utils.rs | 16 | function | no | helper, exercised via enclosing property |
| is_ident_start_byte | utils.rs | 23 | function | no | helper, exercised via enclosing property |
| is_ident_cont_byte | utils.rs | 30 | function | no | helper, exercised via enclosing property |
| bytes_to_str | utils.rs | 39 | function | no | helper, exercised via enclosing property |
| skip_literal_bytes | utils.rs | 46 | function | no | helper, exercised via enclosing property |
| copy_literal_bytes_raw | utils.rs | 64 | function | no | helper, exercised via enclosing property |
| copy_literal_bytes_to_string | utils.rs | 88 | function | no | helper, exercised via enclosing property |

Classification notes:
- builtin_macros.rs / predefined_macros.rs: static macro-table loaders and config setters (`set_target`, `set_pic`, ...) — population of static data, no independent oracle; exercised as fixtures.
- includes.rs `handle_include` / `handle_include_next` / `resolve_include_path(_uncached)` / `resolve_include_next_path` / `read_c_source_file` / `inject_*`: filesystem/integration-bound; the pure parts (`detect_include_guard`, `clean_path`, `normalize_include_path`, `make_absolute`, `strip_inline_comments`, `extract_identifier`) are candidates.
- MacroTable CRUD (`define`, `undefine`, `is_defined`, `get`, `iter`, `set_line`, `set_file`, `get_file_body`, `set_track_expansions`, `take_expanded_macros`): trivial map ops, exercised as fixtures by every expansion property.
- utils.rs char predicates and literal copy/skip helpers: exercised via the expansion/strip properties.
