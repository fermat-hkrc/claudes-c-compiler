# Coverage status — round 05 (src/frontend/sema)

- Scanned: 6 files under src/frontend/sema (116 functions enumerated in
  rounds/05_sema/FUNCTION_INDEX.md; 28 PBT candidates, 88 excluded with reasons) +
  3 change-surface functions from commit aa13cf0d.
- Tested: 15 properties (all at >=1024 generator runs except enumerated/KAT tests) +
  5 deterministic regression witnesses + 3 gcc KAT gates. 10 passing, 5 failing
  (bugs b1-b5).
- Coverage evidence level: **file-level (symbol presence)** — no line-level data:
  this machine has neither gcovr nor lcov, so no coverage instrumentation was
  active (system note; installing a reporter re-enables it for later builds).
  `coverage_gaps` fell back to symbol presence and reported the 3 change-surface
  symbols NOT LINKED — a false negative for #[cfg(test)]-nested Rust fns; each is
  directly called by this round's tests (P14/P15) or re-executed in the probe run
  (p9).
- Module breakdown: analysis.rs 5 props (P3,P5,P6,P12,P13), type_checker.rs 2
  (P1,P2), type_context.rs 1 (P4), const_eval.rs 3 (P7,P8,P9), builtins.rs 2
  (P10,P11), change surface 2 (P14,P15).
- Untested function list (excluded, with reasons): rounds/05_sema/FUNCTION_INDEX.md —
  chiefly private AST-walk arms reached transitively, diagnostic emitters deferred
  (check_member_exists, check_sizeof_incomplete_type), brace-elision counters
  (count_initializer_elements / flat_scalar_count_for_type — deferred, noted for a
  later round), and data-only table construction.
