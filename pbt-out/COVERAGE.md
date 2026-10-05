# PBT Coverage Status — round 03 (src/frontend/preprocessor)

Round 03 campaign set (canonical): this file plus `REPORT.md`, `PROPERTIES.md`, `PLAN.md`,
`report.json` at `pbt-out/`; round archive with FUNCTION_INDEX / INVARIANTS / build.log /
bug_reports under `pbt-out/rounds/03_preprocessor/`. (Round 01 = `src/common`, round 02 =
`src/frontend/lexer`, archives under `rounds/`.)

| Function | Source file | Test file | Test target | Notes |
|----------|-------------|-----------|-------------|-------|
| Preprocessor::preprocess | pipeline.rs | src/frontend/preprocessor/pipeline.rs | cargo test --lib frontend::preprocessor | FAIL→B1 via P8, B3 via P8b; passes P2/P9/P11 |
| Preprocessor::preprocess_source | pipeline.rs | src/frontend/preprocessor/pipeline.rs | cargo test --lib frontend::preprocessor | pass (via P2/P3/P9/P11) |
| Preprocessor::process_directive | pipeline.rs | src/frontend/preprocessor/pipeline.rs | cargo test --lib frontend::preprocessor | pass (P3/P9b/P10/P11) |
| Preprocessor::handle_if / handle_elif | pipeline.rs | src/frontend/preprocessor/pipeline.rs | cargo test --lib frontend::preprocessor | pass (P2, P3) |
| Preprocessor::handle_define / handle_undef / handle_ifdef | pipeline.rs | src/frontend/preprocessor/pipeline.rs | cargo test --lib frontend::preprocessor | pass (P2/P3/P11) |
| Preprocessor::handle_line_directive | pipeline.rs | src/frontend/preprocessor/pipeline.rs | cargo test --lib frontend::preprocessor | pass (P9b, gcc-verified C11 6.10.4) |
| Preprocessor::errors / warnings | pipeline.rs | src/frontend/preprocessor/pipeline.rs | cargo test --lib frontend::preprocessor | FAIL→B2 via P10 (file field absolutized) |
| Preprocessor::current_file | pipeline.rs | src/frontend/preprocessor/pipeline.rs | cargo test --lib frontend::preprocessor | FAIL→B2 (root cause site) |
| Preprocessor::handle_pragma (dispatch) | pragmas.rs | src/frontend/preprocessor/pipeline.rs | cargo test --lib frontend::preprocessor | pass (P11 pack/weak/redefine_extname/push_macro/pop_macro) |
| ConditionalStack::{push_if,handle_elif,handle_else,handle_endif,is_active} | conditionals.rs | src/frontend/preprocessor/conditionals.rs | cargo test --lib frontend::preprocessor | pass (P3 state machine, 1024 cases) |
| evaluate_condition | conditionals.rs | src/frontend/preprocessor/conditionals.rs | cargo test --lib frontend::preprocessor | pass (via P2) |
| eval_const_expr / tokenize_expr / ExprParser::* | conditionals.rs | src/frontend/preprocessor/conditionals.rs | cargo test --lib frontend::preprocessor | pass (P1 1024 cases vs independent C99 6.10.1 reference + P1b matrix) |
| expand_condition_macros | conditionals.rs | src/frontend/preprocessor/conditionals.rs | cargo test --lib frontend::preprocessor | pass (via P2) |
| resolve_defined_in_expr | expr_eval.rs | src/frontend/preprocessor/pipeline.rs | cargo test --lib frontend::preprocessor | pass (P2 defined()/defined X forms) |
| replace_remaining_idents_with_zero | expr_eval.rs | src/frontend/preprocessor/pipeline.rs | cargo test --lib frontend::preprocessor | pass (P2 undefined idents → 0) |
| Preprocessor::join_continued_lines / find_continuation_backslash | text_processing.rs | src/frontend/preprocessor/text_processing.rs | cargo test --lib frontend::preprocessor | pass (P4, 1024 cases + idempotence) |
| Preprocessor::strip_block_comments | text_processing.rs | src/frontend/preprocessor/text_processing.rs | cargo test --lib frontend::preprocessor | pass (P5 differential + idempotence, P5b literals) |
| MacroTable::expand_line / expand_line_reuse / expand_text | macro_defs.rs | src/frontend/preprocessor/pipeline.rs | cargo test --lib frontend::preprocessor | pass via P6/P7/P8 grammar; B3 witness on empty-body path |
| MacroTable::expand_identifier / expand_macro_invocation | macro_defs.rs | src/frontend/preprocessor/pipeline.rs | cargo test --lib frontend::preprocessor | pass (via P6/P7/P8) |
| MacroTable::expand_function_macro | macro_defs.rs | src/frontend/preprocessor/pipeline.rs | cargo test --lib frontend::preprocessor | pass (via P7/P8) incl. `body_ended_with_func_ident` |
| MacroTable::handle_stringify_and_paste | macro_defs.rs | src/frontend/preprocessor/macro_defs.rs | cargo test --lib frontend::preprocessor | FAIL→B1 (empty-variadic comma); stringify/paste paths pass (P6/P7) |
| MacroTable::append_with_paste_guard | macro_defs.rs | src/frontend/preprocessor/pipeline.rs | cargo test --lib frontend::preprocessor | FAIL→B3 via P8b (empty expansion skips guard) |
| MacroTable::substitute_params | macro_defs.rs | src/frontend/preprocessor/pipeline.rs | cargo test --lib frontend::preprocessor | pass (via P6/P7/P8) |
| MacroTable::parse_macro_args | macro_defs.rs | src/frontend/preprocessor/pipeline.rs | cargo test --lib frontend::preprocessor | pass (via P7/P8 incl. empty args, literals with commas) |
| stringify_arg | macro_defs.rs | src/frontend/preprocessor/macro_defs.rs | cargo test --lib frontend::preprocessor | pass (P6 vs C11 6.10.3.2 reference, 1024 cases) |
| parse_define | macro_defs.rs | src/frontend/preprocessor/pipeline.rs | cargo test --lib frontend::preprocessor | pass (all P8/P11 programs define macros) |
| MacroTable::get_va_args / get_named_va_args | macro_defs.rs | src/frontend/preprocessor/pipeline.rs | cargo test --lib frontend::preprocessor | FAIL→B1 root-cause site (VAF/VAC invocations in P8) |
| would_paste_tokens / extract_trailing_ident / contains_standalone_ident | macro_defs.rs | src/frontend/preprocessor/pipeline.rs | cargo test --lib frontend::preprocessor | pass (via P7/P8 paste paths) |
| MacroTable CRUD (define/undefine/is_defined/get/set_line/set_file) | macro_defs.rs | src/frontend/preprocessor/pipeline.rs | cargo test --lib frontend::preprocessor | pass (fixtures of every property) |
| has_unbalanced_parens / split_first_word / strip_line_comment | text_processing.rs | src/frontend/preprocessor/pipeline.rs | cargo test --lib frontend::preprocessor | pass (via preprocess paths) |

Excluded (see rounds/03_preprocessor/FUNCTION_INDEX.md): includes.rs filesystem resolvers
(handle_include, handle_include_next, resolve_include_path*, read_c_source_file, inject_* —
live-filesystem integration, no hermetic oracle; pure helpers clean_path /
normalize_include_path / detect_include_guard / make_absolute not directly driven this round),
builtin_macros.rs / predefined_macros.rs static loaders and config setters (fixtures only),
utils.rs char predicates and literal-copy helpers (exercised via the expansion properties).
