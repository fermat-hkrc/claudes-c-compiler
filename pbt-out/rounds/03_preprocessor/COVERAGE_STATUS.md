# Coverage Status — round 03 (src/frontend/preprocessor)

- **Coverage evidence level:** file-level (symbol presence). No line-level data exists:
  this machine has neither `gcovr` nor `lcov`, and the harness note in effect for this
  campaign states C++/Rust coverage flags were not injected (build tree configured before
  the campaign). The `coverage_gaps` fallback probe reported every preprocessor function
  "NOT LINKED" in the single test binary it inspected — that probe result is inconsistent
  with direct observation (the properties call `eval_const_expr`, `preprocess`,
  `strip_block_comments`, `join_continued_lines` directly, and earlier iterations panicked
  inside those exact functions), so execution is certain even though symbol presence was
  not detected.
- **Scanned:** 171 functions across 11 files (see `rounds/03_preprocessor/FUNCTION_INDEX.md`).
- **Tested (property-driven):** 30+ functions listed in `COVERAGE.md` — the whole expansion
  engine (expand_text / expand_function_macro / handle_stringify_and_paste /
  substitute_params / parse_macro_args / append_with_paste_guard / get_va_args), the
  conditional evaluator (eval_const_expr + ExprParser + ConditionalStack), phase-2/3 text
  transforms, the #if pipeline (resolve_defined_in_expr, replace_remaining_idents_with_zero,
  evaluate_condition), pragma dispatch, #line, and the diagnostics side channels.
- **Sweep round 1 (standard tier):** executed — `coverage_gaps` returned file-level
  evidence only; the two documented surfaces it flagged as untested that had no property
  (`handle_pragma` table, `handle_line_directive`/`__LINE__` interplay) got new properties
  (P11, P9b), both passing.
- **Not driven this round (recorded, not silently skipped):** includes.rs filesystem
  resolvers (`handle_include`, `handle_include_next`, `resolve_include_path*`,
  `read_c_source_file`, `inject_*`) — live-filesystem integration, no hermetic oracle in a
  PBT harness; their pure helpers (`clean_path`, `normalize_include_path`,
  `detect_include_guard`, `make_absolute`) are candidates for a future round.
- **Bugs found in covered code:** B1 (handle_stringify_and_paste / get_va_args),
  B2 (current_file / diagnostics), B3 (append_with_paste_guard).
