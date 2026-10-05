

---

# Round 05 (src/frontend/sema) — confirmed invariants & quirks

Confirmed invariants (1024-case runs unless noted):
- sizeof/_Alignof over the x86-64 SysV primitive/pointer/array grid, struct/union
  layout classics, and enum size match the ABI table exactly (P8, enumerated).
- -Wreturn-type fall-through matches the README-documented rule model over
  generated bodies incl. if/else, loops (const-true vs not), for(;;), switch
  segments, noreturn calls, goto/break/continue (P6, 1024; observable:
  warning_count, programs crafted so it is the only warning).
- FunctionInfo contract: params/variadic/is_defined/is_noreturn stickiness
  (P12, 1024); implicit declarations default Int/variadic/undefined with a
  warning, and builtin names produce NO implicit entries (P13, 1024).
- enum_constant_type GCC band model Int/UInt/LongLong/ULongLong holds
  boundary-exactly (P2, 1024).
- SemaConstEval: C11-defined integer arithmetic (casts, truncating division, C
  remainder, unsigned wrap at 32/64, arithmetic >>) matches an independent
  i128/u128 evaluator on all defined inputs (P7, 1024, UB excluded); ternary and
  elvis short-circuit semantics exact (P9, 1024).
- is_builtin / __atomic_* / __sync_* family recognition and size-suffix stripping
  match the documented families exactly, incl. _3 rejection and idempotence
  (P10, P11, 1024 each).
- parse_src failure branch: guaranteed-malformed mutations always report >=1
  error and never panic (P14, 1024); sut_tokens strips line markers and is
  token-idempotent, and leniently recovers on malformed directives (P15, 1024).

Confirmed BUGS (b1-b5, all serial-reconfirmed, deterministic red regressions in
pbt_regression mods): see rounds/05_sema/REPORT.md — ul+ll signedness (b1,
types.rs:1576), enum shadow leak (b2, type_context.rs:422), enum counter overflow
panic (b3, parser/types.rs:828 + analysis.rs:717), undo-log double-insert
resurrection (b4, insert_*_scoped family), unsigned negation no-wrap (b5,
const_eval.rs Neg arm).

Environment quirks (round 05):
- Driving sema from tests: Lexer::new(src,0).tokenize() -> Parser::new(toks).parse()
  -> SemanticAnalyzer::new() + analyze(&tu) (diagnostics self-initialized);
  observables: into_result().{functions,type_context,expr_types,const_values}
  keyed by Expr::id(), and take_diagnostics().warning_count().
- proptest quirks (round 03/04 ones still apply — prop_assert_eq! rejects inline
  format captures; use "{}", args). assert_eq! message strings must not carry
  program text with unescaped braces.
- Parser surface gaps observed while generating P6 bodies: `switch (x) { }` and a
  dangling `case n:` immediately before `}` are REJECTED (3 parse errors) although
  C11 6.8.4 permits them — parser-round follow-up, not filed under sema (no sema
  property fails on it; generator shaped around it).
- eval_designator_index maps negative literals to usize via `as usize`
  (analysis.rs:705) — designator edge for the deferred brace-elision scope.
- gcc 9.4 available on this box and used as KAT oracle: constant-folds UB
  expressions (1<<31, (int)65536*65536) WITHOUT gcc-runtime wrap semantics — UB
  cases must be excluded from const-eval differentials, not pinned to gcc's fold
  values.
