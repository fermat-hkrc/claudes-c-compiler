
---

# Round 04 (src/frontend/parser) — confirmed invariants & quirks

Confirmed invariants (1024-case runs unless noted):
- Binary precedence/associativity over flat chains of all 18 C binary operators
  matches an independent C11 6.5 table-driven climb (P1, 1024). Fully-parenthesized
  expressions round-trip through parse/render for the subset {lit, id, bin, un,
  post, call, subscript, member, assign, cond, comma} (P2, 1024).
- Type specifier multisets resolve identically under ALL permutations (P3, 1024),
  incl. the doc KATs unsigned==unsigned int, long==long int, signed==int,
  long unsigned int==unsigned long int.
- Declarator inside-out: `int *a[d0..dk]` folds to array-of-pointers and
  `int (*a)[d0..dk]` to pointer-to-array — gcc 9.4 ground truth
  (`int (*a)[5]` → sizeof(*a)=20, `int *b[5]` → sizeof(*b)=8). Function pointers:
  derived chain = [Pointer, FunctionPointer(params), Pointer×extra-indirection]
  per declarators.rs:196-199 (P4/P4b).
- Statement sub-grammar (blocks/if-else/while/do-while/for/return/break/continue/
  expr-stmts + local decls + for-init decls + nested struct/enum in blocks)
  never false-rejects (P5/P5b). Malformed token soup, truncated programs, and
  128-deep nesting never panic (P6/P6b/P6c).
- split_first_word honors its doc contract incl. '(' boundary and #if(x) KATs
  (P9, 1024). pbt_support is_ident_start/cont match C11 6.4.2.1 ASCII exactly;
  tokens() is whitespace-run-width invariant and join-idempotent (P10).

Environment quirks (round 04):
- AST nodes do NOT derive PartialEq and Debug output embeds Span values:
  compare ASTs via a span-stripped Shape enum (pattern match, ignore spans), not
  format!("{:?}") equality across different sources.
- DerivedDeclarator fold convention (types.rs fold_simple_derived): entries apply
  left-to-right as wraps; a RUN of consecutive Array dims applies with the LAST
  dim innermost (source order). Getting this backwards flips pointer-to-array
  into array-of-pointers in the MODEL (not the SUT).
- proptest 1.11 quirks (round-03 ones still apply): prop_oneof! cannot mix
  weighted/unweighted arms; prop_assert_eq! rejects inline format captures
  ({var}) — use positional "{}", args. prop_sampled_from does not exist —
  use proptest::sample::select. String-literal strategies are REGEXES: token
  spellings with regex metachars need classes ("[+]" not "+").
- Member-access renderings on numeric literals are lexically ambiguous
  ("876.g5" maximal-munches as float): render member objects as identifiers.

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
# Invariants confirmed — round 06 (src/ir)

- IrBinOp::eval_i64/eval_i128 satisfy the C11 6.5.5 division identity and the
  None ⇔ rhs==0 error contract (1024 cases each).
- eval_i128 agrees with eval_i64 on non-negative operands whenever the exact
  result fits i64 (shifts need k < 64); they legitimately diverge on negative
  operands for UDiv/URem/LShr (width-specific bit reinterpretation).
- IrCmpOp: signed/unsigned float variants agree on ALL f64 including NaN (IEEE);
  the Slt==!Sge duality laws hold only for non-NaN operands.
- IrConst::from_i64 is the authoritative writer of the U8/U16/U32 zero-extension
  convention; cast_float_to_target, cast_long_double_to_target and
  coerce_to_with_src currently violate it (bugs b1/b2 — red until fixed).
- f64_to_f128_bytes / f64_to_x87_bytes are exact for normals, ±0, ±inf, NaN;
  subnormals are misencoded (bug b3).
- Environment quirks: tests run from pbt-out/rounds/06_ir/run (scratch CWD);
  `use proptest::prelude::*` + `prop!` macro name clash — use `proptest!` (repo
  convention); `FlatAdj::from_vecs_usize` and `IrFunction::new` are #[cfg(test)]
  constructors usable from any inline test module.
- mem2reg: unused allocas are NOT promoted and their Alloca instructions
  legitimately survive; next_value_id stays 0 ("not yet computed") when nothing
  was promoted; unreachable blocks keep dead phis with empty incoming lists.
- No in-tree producer creates CFG edges INTO the entry block (lower_label_stmt
  always terminates + starts a fresh block) — relevant to bug b5's reachability.
