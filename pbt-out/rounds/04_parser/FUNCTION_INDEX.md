> Module: src/frontend/parser | Files: 8 | Functions: 102 | PBT candidates: 26 | Excluded: 76

| Function | Source File | Line | Kind | PBT Candidate | Reason |
|----------|-------------|------|------|---------------|--------|
| apply | parse.rs | 33 | fn | no | helper/plumbing, exercised via driver property |
| parsing_typedef | parse.rs | 150 | fn | no | helper/plumbing, exercised via driver property |
| parsing_static | parse.rs | 151 | fn | no | helper/plumbing, exercised via driver property |
| parsing_extern | parse.rs | 152 | fn | no | helper/plumbing, exercised via driver property |
| parsing_thread_local | parse.rs | 153 | fn | no | helper/plumbing, exercised via driver property |
| parsing_inline | parse.rs | 154 | fn | no | helper/plumbing, exercised via driver property |
| parsing_const | parse.rs | 155 | fn | no | helper/plumbing, exercised via driver property |
| parsing_volatile | parse.rs | 156 | fn | no | helper/plumbing, exercised via driver property |
| parsing_constructor | parse.rs | 157 | fn | no | helper/plumbing, exercised via driver property |
| parsing_destructor | parse.rs | 158 | fn | no | helper/plumbing, exercised via driver property |
| parsing_weak | parse.rs | 159 | fn | no | helper/plumbing, exercised via driver property |
| parsing_used | parse.rs | 160 | fn | no | helper/plumbing, exercised via driver property |
| parsing_gnu_inline | parse.rs | 161 | fn | no | helper/plumbing, exercised via driver property |
| parsing_always_inline | parse.rs | 162 | fn | no | helper/plumbing, exercised via driver property |
| parsing_noinline | parse.rs | 163 | fn | no | helper/plumbing, exercised via driver property |
| parsing_noreturn | parse.rs | 164 | fn | no | helper/plumbing, exercised via driver property |
| parsing_error_attr | parse.rs | 165 | fn | no | helper/plumbing, exercised via driver property |
| parsing_transparent_union | parse.rs | 166 | fn | no | helper/plumbing, exercised via driver property |
| parsing_fastcall | parse.rs | 167 | fn | no | helper/plumbing, exercised via driver property |
| parsing_naked | parse.rs | 168 | fn | no | helper/plumbing, exercised via driver property |
| set_typedef | parse.rs | 172 | fn | no | helper/plumbing, exercised via driver property |
| set_static | parse.rs | 173 | fn | no | helper/plumbing, exercised via driver property |
| set_extern | parse.rs | 174 | fn | no | helper/plumbing, exercised via driver property |
| set_thread_local | parse.rs | 175 | fn | no | helper/plumbing, exercised via driver property |
| set_inline | parse.rs | 176 | fn | no | helper/plumbing, exercised via driver property |
| set_const | parse.rs | 177 | fn | no | helper/plumbing, exercised via driver property |
| set_volatile | parse.rs | 178 | fn | no | helper/plumbing, exercised via driver property |
| set_constructor | parse.rs | 179 | fn | no | helper/plumbing, exercised via driver property |
| set_destructor | parse.rs | 180 | fn | no | helper/plumbing, exercised via driver property |
| set_weak | parse.rs | 181 | fn | no | helper/plumbing, exercised via driver property |
| set_used | parse.rs | 182 | fn | no | helper/plumbing, exercised via driver property |
| set_gnu_inline | parse.rs | 183 | fn | no | helper/plumbing, exercised via driver property |
| set_always_inline | parse.rs | 184 | fn | no | helper/plumbing, exercised via driver property |
| set_noinline | parse.rs | 185 | fn | no | helper/plumbing, exercised via driver property |
| set_noreturn | parse.rs | 186 | fn | no | helper/plumbing, exercised via driver property |
| set_error_attr | parse.rs | 187 | fn | no | helper/plumbing, exercised via driver property |
| set_transparent_union | parse.rs | 188 | fn | no | helper/plumbing, exercised via driver property |
| set_fastcall | parse.rs | 189 | fn | no | helper/plumbing, exercised via driver property |
| set_naked | parse.rs | 190 | fn | no | helper/plumbing, exercised via driver property |
| set_flag | parse.rs | 193 | fn | no | helper/plumbing, exercised via driver property |
| save_flags | parse.rs | 201 | fn | no | helper/plumbing, exercised via driver property |
| restore_flags | parse.rs | 207 | fn | no | helper/plumbing, exercised via driver property |
| fmt | parse.rs | 213 | fn | no | helper/plumbing, exercised via driver property |
| new | parse.rs | 289 | fn | no | helper/plumbing, exercised via driver property |
| set_diagnostics | parse.rs | 310 | fn | no | helper/plumbing, exercised via driver property |
| take_diagnostics | parse.rs | 316 | fn | no | helper/plumbing, exercised via driver property |
| emit_error | parse.rs | 322 | fn | no | helper/plumbing, exercised via driver property |
| builtin_typedefs | parse.rs | 329 | fn | no | helper/plumbing, exercised via driver property |
| parse | parse.rs | 383 | fn | yes | - |
| at_eof | parse.rs | 402 | fn | no | helper/plumbing, exercised via driver property |
| peek | parse.rs | 406 | fn | no | helper/plumbing, exercised via driver property |
| peek_span | parse.rs | 414 | fn | no | helper/plumbing, exercised via driver property |
| advance | parse.rs | 422 | fn | no | helper/plumbing, exercised via driver property |
| expect | parse.rs | 432 | fn | no | helper/plumbing, exercised via driver property |
| expect_after | parse.rs | 451 | fn | no | helper/plumbing, exercised via driver property |
| expect_closing | parse.rs | 476 | fn | no | helper/plumbing, exercised via driver property |
| expect_context | parse.rs | 508 | fn | no | helper/plumbing, exercised via driver property |
| consume_if | parse.rs | 526 | fn | no | helper/plumbing, exercised via driver property |
| is_typedef_label | parse.rs | 540 | fn | no | helper/plumbing, exercised via driver property |
| is_type_specifier | parse.rs | 552 | fn | no | helper/plumbing, exercised via driver property |
| skip_cv_qualifiers | parse.rs | 569 | fn | no | helper/plumbing, exercised via driver property |
| skip_array_qualifiers | parse.rs | 600 | fn | no | helper/plumbing, exercised via driver property |
| skip_array_dimensions | parse.rs | 608 | fn | no | helper/plumbing, exercised via driver property |
| compound_assign_op | parse.rs | 618 | fn | no | helper/plumbing, exercised via driver property |
| skip_gcc_extensions | parse.rs | 636 | fn | no | helper/plumbing, exercised via driver property |
| parse_gcc_attributes | parse.rs | 645 | fn | no | helper/plumbing, exercised via driver property |
| parse_gcc_attribute_list | parse.rs | 684 | fn | no | helper/plumbing, exercised via driver property |
| dispatch_gcc_attribute | parse.rs | 703 | fn | no | helper/plumbing, exercised via driver property |
| parse_string_attr_arg | parse.rs | 795 | fn | no | helper/plumbing, exercised via driver property |
| skip_optional_paren_arg | parse.rs | 808 | fn | no | helper/plumbing, exercised via driver property |
| parse_cleanup_attr | parse.rs | 818 | fn | no | helper/plumbing, exercised via driver property |
| parse_mode_attr | parse.rs | 829 | fn | no | helper/plumbing, exercised via driver property |
| parse_vector_size_attr | parse.rs | 858 | fn | no | helper/plumbing, exercised via driver property |
| parse_ext_vector_type_attr | parse.rs | 872 | fn | no | helper/plumbing, exercised via driver property |
| parse_address_space_attr | parse.rs | 885 | fn | no | helper/plumbing, exercised via driver property |
| skip_asm_and_attributes | parse.rs | 898 | fn | no | helper/plumbing, exercised via driver property |
| parse_asm_and_attributes | parse.rs | 906 | fn | no | helper/plumbing, exercised via driver property |
| try_parse_asm_register_name | parse.rs | 950 | fn | no | helper/plumbing, exercised via driver property |
| handle_pragma_pack_token | parse.rs | 986 | fn | no | helper/plumbing, exercised via driver property |
| handle_pragma_visibility_token | parse.rs | 1037 | fn | no | helper/plumbing, exercised via driver property |
| skip_balanced_parens | parse.rs | 1076 | fn | no | helper/plumbing, exercised via driver property |
| skip_label_attributes | parse.rs | 1098 | fn | no | helper/plumbing, exercised via driver property |
| parse_alignment_expr | parse.rs | 1112 | fn | no | helper/plumbing, exercised via driver property |
| parse_alignas_argument | parse.rs | 1138 | fn | no | helper/plumbing, exercised via driver property |
| try_sizeof_type_spec | parse.rs | 1176 | fn | no | helper/plumbing, exercised via driver property |
| is_unsigned_type_spec | parse.rs | 1209 | fn | no | helper/plumbing, exercised via driver property |
| alignof_type_spec | parse.rs | 1228 | fn | no | helper/plumbing, exercised via driver property |
| preferred_alignof_type_spec | parse.rs | 1289 | fn | no | helper/plumbing, exercised via driver property |
| apply_pending_vector_attr | expressions.rs | 40 | fn | no | helper/plumbing, exercised via driver property |
| estimate_type_size | expressions.rs | 55 | fn | no | helper/plumbing, exercised via driver property |
| parse_expr | expressions.rs | 70 | fn | yes | - |
| parse_assignment_expr | expressions.rs | 82 | fn | yes | - |
| parse_conditional_expr | expressions.rs | 105 | fn | yes | - |
| token_to_binop | expressions.rs | 127 | fn | no | helper/plumbing, exercised via driver property |
| parse_binary_expr | expressions.rs | 153 | fn | yes | - |
| parse_next_tighter | expressions.rs | 165 | fn | no | helper/plumbing, exercised via driver property |
| parse_cast_expr | expressions.rs | 182 | fn | yes | - |
| parse_unary_expr | expressions.rs | 226 | fn | yes | - |
| parse_sizeof_expr | expressions.rs | 344 | fn | no | helper/plumbing, exercised via driver property |
| parse_postfix_expr | expressions.rs | 379 | fn | yes | - |
| parse_postfix_ops | expressions.rs | 385 | fn | yes | - |
| parse_primary_expr | expressions.rs | 450 | fn | yes | - |
| parse_generic_selection | expressions.rs | 690 | fn | yes | - |
| parse_type_specifier | types.rs | 45 | fn | yes | - |
| collect_trailing_specifiers | types.rs | 314 | fn | no | helper/plumbing, exercised via driver property |
| resolve_type_flags | types.rs | 390 | fn | yes | - |
| parse_struct_or_union | types.rs | 444 | fn | yes | - |
| parse_enum_specifier | types.rs | 492 | fn | yes | - |
| parse_typeof_specifier | types.rs | 518 | fn | yes | - |
| consume_trailing_qualifiers | types.rs | 550 | fn | no | helper/plumbing, exercised via driver property |
| parse_struct_fields | types.rs | 606 | fn | yes | - |
| parse_struct_field_declarators | types.rs | 643 | fn | no | helper/plumbing, exercised via driver property |
| consume_struct_field_qualifiers | types.rs | 717 | fn | no | helper/plumbing, exercised via driver property |
| fold_simple_derived | types.rs | 747 | fn | yes | - |
| parse_enum_variants | types.rs | 788 | fn | yes | - |
| register_enum_constants | types.rs | 814 | fn | no | helper/plumbing, exercised via driver property |
| parse_va_arg_type | types.rs | 840 | fn | no | helper/plumbing, exercised via driver property |
| parse_abstract_declarator_suffix | types.rs | 891 | fn | yes | - |
| parse_external_decl | declarations.rs | 37 | fn | yes | - |
| parse_function_def | declarations.rs | 214 | fn | yes | - |
| build_return_type | declarations.rs | 296 | fn | no | helper/plumbing, exercised via driver property |
| parse_kr_params | declarations.rs | 352 | fn | no | helper/plumbing, exercised via driver property |
| apply_kr_derivations | declarations.rs | 407 | fn | no | helper/plumbing, exercised via driver property |
| parse_declaration_rest | declarations.rs | 478 | fn | yes | - |
| parse_local_declaration | declarations.rs | 647 | fn | yes | - |
| parse_initializer | declarations.rs | 800 | fn | yes | - |
| expand_range_designators | declarations.rs | 870 | fn | no | helper/plumbing, exercised via driver property |
| eval_const_int_expr | declarations.rs | 909 | fn | no | helper/plumbing, exercised via driver property |
| eval_const_int_expr_with_enums | declarations.rs | 915 | fn | no | helper/plumbing, exercised via driver property |
| is_unsigned_int_expr | declarations.rs | 1083 | fn | no | helper/plumbing, exercised via driver property |
| type_spec_has_typedef | declarations.rs | 1108 | fn | no | helper/plumbing, exercised via driver property |
| consume_post_type_qualifiers | declarations.rs | 1120 | fn | no | helper/plumbing, exercised via driver property |
| register_typedefs | declarations.rs | 1151 | fn | no | helper/plumbing, exercised via driver property |
| expr_has_non_const_identifier | declarations.rs | 1165 | fn | no | helper/plumbing, exercised via driver property |
| parse_static_assert | declarations.rs | 1216 | fn | yes | - |
| parse_declarator | declarators.rs | 36 | fn | yes | - |
| parse_declarator_with_attrs | declarators.rs | 43 | fn | yes | - |
| is_paren_declarator | declarators.rs | 129 | fn | yes | - |
| combine_declarator_parts | declarators.rs | 161 | fn | yes | - |
| parse_param_list | declarators.rs | 312 | fn | yes | - |
| parse_kr_identifier_list | declarators.rs | 416 | fn | no | helper/plumbing, exercised via driver property |
| parse_param_declarator_full | declarators.rs | 439 | fn | yes | - |
| parse_paren_param_declarator | declarators.rs | 500 | fn | no | helper/plumbing, exercised via driver property |
| extract_paren_name | declarators.rs | 687 | fn | no | helper/plumbing, exercised via driver property |
| try_parse_paren_abstract_declarator | declarators.rs | 718 | fn | yes | - |
| parse_compound_stmt | statements.rs | 12 | fn | no | helper/plumbing, exercised via driver property |
| parse_stmt | statements.rs | 93 | fn | no | helper/plumbing, exercised via driver property |
| parse_for_stmt | statements.rs | 264 | fn | no | helper/plumbing, exercised via driver property |
| parse_inline_asm | statements.rs | 302 | fn | no | helper/plumbing, exercised via driver property |
| parse_asm_string | statements.rs | 351 | fn | no | helper/plumbing, exercised via driver property |
| parse_asm_operands | statements.rs | 360 | fn | no | helper/plumbing, exercised via driver property |
| parse_one_asm_operand | statements.rs | 375 | fn | no | helper/plumbing, exercised via driver property |
| parse_asm_clobbers | statements.rs | 415 | fn | no | helper/plumbing, exercised via driver property |
| parse_asm_goto_labels | statements.rs | 432 | fn | no | helper/plumbing, exercised via driver property |
