# FUNCTION INDEX — src/frontend/sema (round 05)

> Total files: 6 | Total functions: 116 | PBT candidates: 28 | Excluded: 88
> Extraction: rg '^\s*(?:pub(?:\(\w+\))?\s+)?fn\s+(\w+)' per file. Plain-text cells.

| Function | Source File | Line | Kind | PBT Candidate | Reason |
|----------|-------------|------|------|---------------|--------|
| analyze | analysis.rs | 159 | method | yes | entry point driving all pipeline properties (P3/P5/P6/P12/P13) |
| analyze_function_def | analysis.rs | 200 | method | yes | -Wreturn-type emission, function scope push/pop (P6) |
| analyze_declaration | analysis.rs | 286 | method | yes | typedef/var/prototype registration (P12) |
| count_initializer_elements | analysis.rs | 548 | method | yes | brace-elision counting; deferred — tier budget (noted in COVERAGE_STATUS) |
| flat_scalar_count_for_type | analysis.rs | 672 | method | yes | same as above (reached only via count_initializer_elements) |
| eval_designator_index | analysis.rs | 705 | method | no | negative literal → usize wrap noted as observation; edge of brace-elision scope |
| process_enum_variants | analysis.rs | 717 | method | yes | enum auto-increment/explicit model (P3, i64::MAX boundary) |
| collect_enum_constants_from_type_spec | analysis.rs | 752 | method | yes | reached via P3 generator (inline enums) |
| analyze_compound_stmt | analysis.rs | 796 | method | yes | scope push/pop integration (P5) |
| analyze_stmt | analysis.rs | 813 | method | yes | reached via P6 bodies |
| compound_can_fall_through | analysis.rs | 922 | method | yes | -Wreturn-type model (P6) |
| stmt_can_fall_through | analysis.rs | 941 | method | yes | -Wreturn-type model (P6) |
| switch_can_fall_through | analysis.rs | 1022 | method | yes | -Wreturn-type model (P6) |
| segment_outcome | analysis.rs | 1090 | method | yes | reached via P6 switch segments |
| stmt_switch_outcome | analysis.rs | 1112 | method | yes | reached via P6 |
| switch_has_default | analysis.rs | 1219 | method | yes | reached via P6 |
| stmt_contains_default | analysis.rs | 1231 | method | yes | reached via P6 |
| is_noreturn_call | analysis.rs | 1246 | method | yes | reached via P6 noreturn arms |
| is_constant_true_expr | analysis.rs | 1294 | method | yes | reached via P6 loop arms |
| try_eval_constant_bool | analysis.rs | 1300 | method | yes | reached via P6 |
| analyze_expr | analysis.rs | 1324 | method | yes | reached via every pipeline property |
| annotate_expr_type | analysis.rs | 1533 | method | yes | populates expr_types (P1) |
| annotate_const_value | analysis.rs | 1548 | method | yes | populates const_values (P7/P8) |
| check_member_exists | analysis.rs | 1564 | method | no | error-path diag; not in property set this tier |
| pointee_types_compatible | analysis.rs | 1693 | method | no | helper for pointer-subtract check; not in set this tier |
| analyze_initializer | analysis.rs | 1723 | method | no | thin wrapper into count_initializer_elements scope |
| check_sizeof_incomplete_type | analysis.rs | 1738 | method | no | error-path diag; deferred (COVERAGE_STATUS) |
| declare_implicit_functions | analysis.rs | 1849 | method | yes | implicit libc seeding (P13 context) |
| enum_constant_type | type_checker.rs | 39 | function | yes | pure; documented GCC promotion bands (P2) |
| infer_expr_ctype | type_checker.rs | 80 | method | yes | literal/binop typing via expr_types (P1) |
| infer_binop_ctype | type_checker.rs | 351 | method | yes | usual arithmetic conversions (P1) |
| infer_call_return_ctype | type_checker.rs | 431 | method | no | reached incidentally via P12 call sites |
| resolve_type_spec | type_checker.rs | 566 | method | no | reached via P1 casts; no dedicated property this tier |
| builtin_return_ctype | type_checker.rs | 783 | method | no | partial coverage via P12; table lookup |
| push_scope | type_context.rs | 370 | method | yes | undo-log state machine (P4) |
| pop_scope | type_context.rs | 376 | method | yes | undo-log state machine (P4) — "undo changes to enum_constants..." contract |
| insert_enum_scoped | type_context.rs | 422 | method | yes | P4/P5 — shadow tracking gap |
| insert_struct_layout_scoped | type_context.rs | 434 | method | yes | P4 |
| insert_typedef_scoped | type_context.rs | 448 | method | yes | P4 |
| insert_typedef_alignment_scoped | type_context.rs | 461 | method | yes | P4 |
| invalidate_ctype_cache_scoped | type_context.rs | 474 | method | yes | P4 |
| insert_struct_layout_scoped_from_ref | type_context.rs | 490 | method | yes | P4 (from-ref twin of 434) |
| invalidate_ctype_cache_scoped_from_ref | type_context.rs | 505 | method | yes | P4 |
| seed_builtin_typedefs | type_context.rs | 196 | method | no | KAT-pinned in P8 prelude; target-aware table |
| next_anon_struct_id | type_context.rs | 343 | method | no | trivial counter |
| new (TypeContext) | type_context.rs | 169 | method | no | constructor + seeding |
| eval_const_expr | const_eval.rs | 76 | method | yes | P7/P9 differential |
| eval_const_expr_as_bits | const_eval.rs | 388 | method | no | reached via P7 cast chains |
| eval_offsetof_pattern | const_eval.rs | 406 | method | no | offsetof KAT inside P8 prelude only |
| ctype_size | const_eval.rs | 545 | method | yes | reached via P7/P8 |
| is_expr_unsigned | const_eval.rs | 551 | method | yes | reached via P7 signedness arms |
| bits_to_irconst | const_eval.rs | 618 | method | yes | reached via P7 casts |
| sizeof_type_spec | const_eval.rs | 790 | method | yes | P8 reference table |
| sizeof_expr | const_eval.rs | 903 | method | yes | P8 |
| alignof_type_spec | const_eval.rs | 921 | method | yes | P8 reference table |
| preferred_alignof_type_spec | const_eval.rs | 1024 | method | no | LP64 twin of 921; P8 asserts std _Alignof only |
| resolve_builtin | builtins.rs | 647 | function | yes | P11 |
| is_builtin | builtins.rs | 657 | function | yes | P10 |
| is_atomic_builtin | builtins.rs | 672 | function | yes | reached via P10 |
| strip_sync_size_suffix | builtins.rs | 715 | function | yes | P11 algebraic contract |
| normalize_atomic_size_suffix | builtins.rs | 734 | function | yes | P10 |
| simple/identity/constant_f64/intrinsic | builtins.rs | 629 | fn | no | 4-line BuiltinInfo constructors (data) |
| (remaining 58 functions across the 6 files: private AST-walk arms, `unwrap_case_label`, `is_case_label`, `default` impls, `borrow_*`, `insert_struct_layout_from_ref`, `is_struct_key_shadowed`, converters `type_spec_to_ctype`/`convert_struct_fields`/`ctype_from_type_spec*`, cast helpers `cast_long_double_to_ctype`/`cast_float_to_ctype`/`cast_i128_to_ctype`, `resolve_packed_enum_type`, `expr_is_always_nonzero`, symbol resolution `resolve_typedef`/`resolve_struct_or_union`/`resolve_enum`/`resolve_typeof_expr`/`eval_const_expr_as_usize`, `extract_fptr_typedef_info`, etc.) | analysis.rs / type_context.rs / type_checker.rs / const_eval.rs / builtins.rs | — | method | no | private arms reached transitively by the pipeline properties above; no independent oracle beyond what P1-P13 already pin; per-function deferred to COVERAGE_STATUS |
