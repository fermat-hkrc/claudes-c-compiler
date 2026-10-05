# PROPERTIES — round 05, src/frontend/sema (+ change-surface obligations)

Statuses: proposed → approved → passing | failing | retired.

---

## P1 uac_binop_ctype
- Tier: 2
- Rationale: The core of expression typing. Independent oracle: C11 6.3.1.1p2 integer
  promotions + 6.3.1.8 usual arithmetic conversions, implemented from the clause as a
  Rust model (rank table + "unsigned wins unless signed can represent all values"
  via byte sizes; LP64 long==long long==8). Shifts: promoted left only (6.5.7p3);
  comparisons/logical → int. Stronger oracles rejected: no state; reference impl is
  the C standard itself. gcc KAT gate (one gcc compile of _Static_asserts) pins the
  model before the PBT runs.
- Doc contract: src/frontend/sema/README.md "Binary operators: comparison and logical operators produce `int`; shift operators produce the promoted type of the left operand; arithmetic operators apply the usual arithmetic conversions" — asserted fingerprint e65d1090
- Seed: (none — sema has no existing tests)
- Formal: ∀ (t1,t2) ∈ CScalarTypes², op ∈ {+,-,*,/,%,&,|,^,<<,>>,==,!=,<,<=,>,>=,&&,||}.
  expr_types[(t1 a; t2 b; int p = a op b;).init] = C11_uac(promote(t1), promote(t2), op)
- Test file: src/frontend/sema/type_checker.rs
- Status: failing
- Counterexample: t1="unsigned long", t2="long long", op="+" — expr_types[a+b] == LongLong, C11 6.3.1.8 requires ULongLong (gcc-verified)
- Bug report: bug_reports/b1_uac_unsigned_long_plus_long_long_signed.md

```property
function: frontend::sema::type_checker::ExprTypeChecker::infer_binop_ctype
oracle: differential
predicate:
  quantifier: forall
  vars: [t1, t2, op]
  domain: { t1: 14 scalar C type spellings, t2: same, op: 17 binary ops }
  body: expr_type_of("T1 a; T2 b; int p = a OP b;") == model_uac(t1, t2, op)
generators:
  t1: { gen: oneof, of: ["char","signed char","unsigned char","short","unsigned short","int","unsigned int","long","unsigned long","long long","unsigned long long","float","double","long double"] }
  t2: { gen: oneof, of: ["char","signed char","unsigned char","short","unsigned short","int","unsigned int","long","unsigned long","long long","unsigned long long","float","double","long double"] }
  op: { gen: oneof, of: ["+","-","*","/","%","&","|","^","<<",">>","==","!=","<","<=",">",">=","&&","||"] }
evidence: C11 6.3.1.8 + src/frontend/sema/README.md (Expression Type Inference section)
```

## P2 enum_constant_type_bands
- Tier: 3
- Rationale: Pure function with the contract written in its own doc comment — the
  GCC enum promotion progression. Boundary-exact sampling: i32::MIN-1, i32::MIN,
  i32::MAX, i32::MAX+1, u32::MAX, u32::MAX+1, i64::MIN, i64::MAX plus uniform i64.
- Doc contract: src/frontend/sema/type_checker.rs:38-39 "Determine the C type of an enum constant value, following GCC's promotion rules. GCC uses the progression: int -> unsigned int -> long long -> unsigned long long." — asserted fingerprint d2b381bc
- Seed: (none)
- Formal: ∀ v ∈ i64. enum_constant_type(v) = Int if v∈[i32::MIN,i32::MAX]; UInt if v∈[2³¹,2³²−1]; ULongLong if v>2³²−1; LongLong if v<i32::MIN
- Test file: src/frontend/sema/type_checker.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: frontend::sema::type_checker::enum_constant_type
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [v]
  domain: { v: boundary i64 set ∪ uniform i64 }
  relation:
    op: eq
    lhs: enum_constant_type(v)
    rhs: model_band(v)
generators:
  v: { gen: int, min: -9223372036854775808, max: 9223372036854775807, type: i64 }
evidence: src/frontend/sema/type_checker.rs:38-39 doc comment
```

## P3 enum_variant_values_model
- Tier: 2
- Rationale: process_enum_variants implements C11 6.7.2.2: explicit initializer
  sets the value, otherwise previous+1. Independent model over generated variant
  lists. Generator deliberately includes i64::MAX explicit values followed by an
  implicit variant (counter += 1 overflow) and negative values.
- Doc contract: src/frontend/sema/analysis.rs:717 (process_enum_variants has no doc comment) — (none)
- Seed: (none)
- Formal: ∀ variants ∈ VariantList. analyze("enum { A₀[=e₀], A₁[=e₁], … }") ⇒
  ∀i. enum_constants[Aᵢ] = eᵢ if explicit else enum_constants[Aᵢ₋₁] + 1 (A₀: 0)
- Test file: src/frontend/sema/analysis.rs
- Status: failing
- Counterexample: variants = [("A", Some(9223372036854775807))] — `enum { A = 9223372036854775807LL };` panics "attempt to add with overflow" at parser/types.rs:828 (gcc: "overflow in enumeration values" diagnostic instead)
- Bug report: bug_reports/b3_enum_counter_i64_max_overflow_panic.md

```property
function: frontend::sema::analysis::SemanticAnalyzer::process_enum_variants
oracle: differential
predicate:
  quantifier: forall
  vars: [variants]
  domain: { variants: 1..6 entries, each optional explicit i64 }
  body: enum_constants_of(variants) == model_enum_values(variants)
generators:
  variants: { gen: list, elem: { gen: tuple, of: [{ gen: string, regex: "[A-Z][A-Z0-9]{0,3}" }, { gen: optional, of: { gen: int, min: -9223372036854775808, max: 9223372036854775807, type: i64 } }] }, minLen: 1, maxLen: 6 }
evidence: C11 6.7.2.2p2 (enumerator value = preceding + 1 unless explicit initializer)
```

## P4 typecontext_scope_undo_state_machine
- Tier: 1
- Rationale: TypeContext is the module's mutable state core; the undo-log is the
  documented mechanism ("undo changes to enum_constants, struct_layouts,
  ctype_cache, and typedefs"). State-machine oracle: random op sequences vs a
  layered-scope model, maps compared after every op. Documented carve-out modeled:
  pop does not restore an EMPTY shadowed struct layout over a full definition.
  Suspected defect under test: insert_enum_scoped never records shadowed previous
  values (no enums_shadowed restore in pop_scope) — C scope semantics say the outer
  value must reappear after pop.
- Doc contract: src/frontend/sema/type_context.rs:376-378 "Pop the top type-system scope frame and undo changes to enum_constants, struct_layouts, ctype_cache, and typedefs." — asserted fingerprint be11a9e7
- Seed: (none)
- Formal: Automaton — states: layered scope maps (typedefs, enum_constants,
  typedef_alignments, ctype_cache; layouts kept non-empty to stay inside the
  documented carve-out); ops: Push, Pop, InsertTypedef(n,t), InsertEnum(n,v),
  InsertAlign(n,a), InvalidateCache(k), InsertLayout(k); invariant:
  after each op, every observable map == layered model (with the empty-layout
  carve-out), and Pop on empty stack is a no-op.
- Test file: src/frontend/sema/type_context.rs
- Status: failing
- Counterexample: ops = [Push, Al(1,0), Al(1,0), Pop] — typedef_alignments["kB"] survives the pop (same-scope re-insert resurrection)
- Bug report: bug_reports/b4_undolog_double_insert_resurrection.md

```property
function: frontend::sema::type_context::TypeContext::pop_scope
oracle: state_machine
predicate:
  quantifier: forall
  vars: [ops]
  domain: { ops: 1..24 random ops from {Push,Pop,InsertTypedef,InsertEnum,InsertAlign,InvalidateCache,InsertLayout} over 4 names }
  body: after each op, all tracked maps equal the layered-scope model (enum restore included)
generators:
  ops: { gen: list, elem: { gen: oneof, of: [tuple forms] }, minLen: 1, maxLen: 24 }
evidence: src/frontend/sema/type_context.rs:376-378 doc comment + README "Scope Management via Undo-Log"
```

## P5 scope_shadow_via_analyze
- Tier: 2
- Rationale: The integration witness for P4's suspected defect through the real
  SemanticAnalyzer pipeline: a function-local enum constant shadowing a global one
  must not leak its value after the function scope closes (README: undo-log
  "Ensures that local struct definitions inside a function body do not overwrite
  global layouts" — same guarantee class for enum constants).
- Doc contract: src/frontend/sema/type_context.rs:423-424 "Insert an enum constant, tracking the change in the current scope frame." — asserted fingerprint be11a9e7
- Seed: (none)
- Formal: ∀ (g, l). analyze("enum { E = g; }; void f(void){ enum { E = l; } return; } enum { F = E };") ⇒
  enum_constants[F] == g (outer E restored after f's scope pops)
- Test file: src/frontend/sema/analysis.rs
- Status: failing
- Counterexample: g=1, l=2 — `enum { E = 1 }; void f(void){ enum { E = 2 }; } enum { F = E + 0 };` gives F == 2 (inner value leaked past scope exit)
- Bug report: bug_reports/b2_enum_constant_scope_shadow_leak.md

```property
function: frontend::sema::analysis::SemanticAnalyzer::analyze
oracle: differential
predicate:
  quantifier: forall
  vars: [g, l]
  domain: { g: small i64, l: small i64, l != g }
  body: enum_constants["F"] == g after f's body scope pops
generators:
  g: { gen: int, min: 1, max: 4096, type: i64 }
  l: { gen: int, min: 1, max: 4096, type: i64 }
evidence: src/frontend/sema/type_context.rs:376-378 ("undo changes to enum_constants") + README "Scope Management via Undo-Log"
```

## P6 return_type_fallthrough_model
- Tier: 2
- Rationale: -Wreturn-type is pure control-flow logic with fully documented rules
  (README section). Differential against a model implementing the documented rules:
  return/goto/goto-indirect/break/continue diverge; if/else needs both branches;
  if-without-else diverges iff constant-true cond and then diverges;
  while(const-true) never falls through; do-while falls iff cond not const-true and
  body falls; for(;;) infinite, for(cond) falls unless const-true; switch falls
  unless default present, no breaking segment, last segment diverges; expr-stmts
  fall unless noreturn call. Observable: warning_count (programs crafted so this is
  the only possible warning). gcc KAT spot-check pins the model on 12 curated bodies.
- Doc contract: src/frontend/sema/analysis.rs:941-943 "Check whether a statement can fall through (i.e., control can reach the point immediately after this statement without a return/goto/diverge)." — asserted fingerprint e65d1090
- Seed: (none)
- Formal: ∀ body ∈ generated small function bodies. warning_emitted(int f(void){body}) ==
  model_falls_through(body) ∧ body ≠ trivially-returning
- Test file: src/frontend/sema/analysis.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: frontend::sema::analysis::SemanticAnalyzer::stmt_can_fall_through
oracle: differential
predicate:
  quantifier: forall
  vars: [body]
  domain: { body: 1..5 statements from {return, goto, break, continue, if/else, while, do-while, for, switch, noreturn call, plain expr} }
  body: warning_count("int f(void){BODY}") == model_falls_through(body) as u32
generators:
  body: { gen: recursive stmt tree, depth 0..3 }
evidence: src/frontend/sema/README.md "-Wreturn-type Analysis" section + analysis.rs:941-943 doc
```

## P7 const_arith_c_semantics
- Tier: 1
- Rationale: SemaConstEval's contract is to produce IrConst values that the lowerer
  trusts (README: "compile-time constant evaluation returning IrConst values").
  Differential vs an independent C-semantics evaluator over generated integer
  expressions with mixed signedness casts ((unsigned)/(long)/(int)/(char) casts),
  truncating division toward zero, C remainder sign, and width-aware wrap after
  casts. Division/modulo by zero excluded (invalid input). gcc KAT gate pins the
  model once (single gcc compile of static asserts).
- Doc contract: src/frontend/sema/const_eval.rs:70-76 (struct doc; eval_const_expr has none) — (none)
- Seed: (none)
- Formal: ∀ e ∈ Expr(int literals, +,-,*,/,%,<<,>>,&,|,^,unary -~, casts).
  const_values[(int p = e;).init] == model_c_eval(e)  (C11 6.5 semantics, width-aware)
- Test file: src/frontend/sema/const_eval.rs
- Status: failing
- Counterexample: e = Un("-", Bin("+", Cast("unsigned", Lit(0)), Lit(-1))) — `int p = -((unsigned)(0) + (-1));` const-folds to -4294967295; C (gcc 9.4) wraps to 1
- Bug report: bug_reports/b5_unsigned_negation_const_eval_no_wrap.md

```property
function: frontend::sema::const_eval::SemaConstEval::eval_const_expr
oracle: differential
predicate:
  quantifier: forall
  vars: [expr]
  domain: { expr: depth-0..4 integer expression trees, literal leaves in [-2^31, 2^31), cast nodes }
  body: const_value_of(expr) == model_c_eval(expr)
generators:
  expr: { gen: recursive expr tree, depth 0..4 }
evidence: C11 6.5.5/6.5.6 (mul/div/mod/add/sub/shift semantics) + README "What It Evaluates"
```

## P8 sizeof_alignof_abi_table
- Tier: 3
- Rationale: Reference oracle — the x86-64 System V ABI size/alignment table for
  all primitive types plus composed struct/union/array/pointer/enum types, read
  through the real pipeline's const_values for `sizeof(T)`/`_Alignof(T)`
  expressions. Enumeration over the full type grid (not sampling): every bound is
  exercised exactly.
- Doc contract: src/frontend/sema/README.md "`sizeof`/`_Alignof`: always `CType::ULong`" — asserted fingerprint 3d0a4412
- Seed: (none)
- Formal: ∀ T ∈ TypeGrid. const_value(sizeof(T)) == size(T) ∧ const_value(_Alignof(T)) == align(T)
  per x86-64 SysV; and typeof(sizeof(T)) == ULong.
- Test file: src/frontend/sema/const_eval.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: frontend::sema::const_eval::SemaConstEval::sizeof_type_spec
oracle: reference
predicate:
  quantifier: forall
  vars: [T]
  domain: { T: 11 integer spellings + float/double/long double/pointer/array/struct/union/enum grid }
  relation:
    op: eq
    lhs: const_value_of("sizeof(T)")
    rhs: abi_size(T)
generators:
  T: { gen: oneof, of: [enumerated type grid] }
evidence: x86-64 System V ABI (sizes/alignments) + README sizeof section
```

## P9 ternary_elvis_const_semantics
- Tier: 3
- Rationale: Documented const-eval table rows: "Ternary ?: Evaluates condition,
  returns selected branch"; "GnuConditional (Elvis a ?: b): Evaluates condition; if
  nonzero returns condition value, otherwise evaluates else branch". Differential
  vs an independent short-circuit model, incl. non-constant branches (result None).
- Doc contract: src/frontend/sema/README.md "Ternary `?:` | Evaluates condition, returns selected branch" and "GnuConditional (Elvis `a ?: b`) | Evaluates condition; if nonzero returns condition value" — asserted fingerprint e65d1090
- Seed: (none)
- Formal: ∀ (c,a,b) ∈ ConstInts³. const_value(c ? a : b) == (c≠0 ? a : b) ∧
  const_value(c ?: b) == (c≠0 ? c : b)
- Test file: src/frontend/sema/const_eval.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: frontend::sema::const_eval::SemaConstEval::eval_const_expr
oracle: differential
predicate:
  quantifier: forall
  vars: [c, a, b]
  domain: { c,a,b: small i64 }
  body: const_value_of("C ? A : B") == if C!=0 {A} else {B} && const_value_of("C ?: B") == if C!=0 {C} else {B}
generators:
  c: { gen: int, min: -4096, max: 4096, type: i64 }
  a: { gen: int, min: -4096, max: 4096, type: i64 }
  b: { gen: int, min: -4096, max: 4096, type: i64 }
evidence: src/frontend/sema/README.md "What It Evaluates" table rows for Ternary/GnuConditional
```

## P10 builtin_family_recognition
- Tier: 2
- Rationale: is_builtin's documented contract covers exactly the BUILTIN_MAP keys,
  3 special names, the __atomic_* family (with size suffixes normalizing to _n
  variants), and __sync_* (with size suffixes stripped before matching). Model
  differential over generated names: valid op+suffix combos → true; unknown ops,
  wrong suffixes (_3), non-prefixed names → false.
- Doc contract: src/frontend/sema/builtins.rs:648-653 "Check if a name is a known builtin function." — asserted fingerprint c9e48be6
- Seed: (none)
- Formal: ∀ name ∈ Generated{prefix, op, suffix}. is_builtin(name) ==
  (op ∈ documented_family(prefix) ∧ suffix ∈ {ε,1,2,4,8,16})
- Test file: src/frontend/sema/builtins.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: frontend::sema::builtins::is_builtin
oracle: differential
predicate:
  quantifier: forall
  vars: [prefix, op, suffix, known]
  domain: { prefix: {__atomic_, __sync_, __builtin_, ""}, op: known/unknown op stems, suffix: {"",_1,_2,_3,_4,_8,_16} }
  body: is_builtin(name) == model_builtin(name)
generators:
  prefix: { gen: oneof, of: ["__atomic_", "__sync_", "__builtin_", ""] }
  op: { gen: oneof, of: [valid op stems + "nope", "frobnicate"] }
  suffix: { gen: oneof, of: ["", "_1", "_2", "_3", "_4", "_8", "_16"] }
evidence: src/frontend/sema/builtins.rs:648-699 doc + family lists
```

## P11 libc_alias_and_sync_strip
- Tier: 3
- Rationale: Two algebraic contracts from the code's own docs: (a) LibcAlias
  builtins of the form __builtin_<libc> alias to <libc> for the documented libc
  family (memcpy/printf/strlen/abs/labs/fabs/sqrt/malloc/strcpy/…); (b)
  strip_sync_size_suffix strips exactly one of _1/_2/_4/_8/_16 else returns the
  name unchanged, and is idempotent.
- Doc contract: src/frontend/sema/builtins.rs:711-714 "Strip size suffix (_1, _2, _4, _8, _16) from GCC __sync_* builtin names. E.g., \"__sync_fetch_and_add_8\" -> \"__sync_fetch_and_add\". Returns the name unchanged if no suffix is present." — asserted fingerprint c9e48be6
- Seed: (none)
- Formal: ∀ x. strip(x)==x ∨ strip(x)+"_"+s==x for s∈{1,2,4,8,16}; strip(strip(x))==strip(x);
  ∀ n ∈ LibcFamily. resolve_builtin("__builtin_"+n) == LibcAlias(n)
- Test file: src/frontend/sema/builtins.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: frontend::sema::builtins::strip_sync_size_suffix
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [x]
  domain: { x: "__sync_"+op+suffix strings over op/suffix grids }
  relation:
    op: holds
    lhs: "strip(x)==x || strip(x)+s==x for some s in {1,2,4,8,16}; and strip(strip(x))==strip(x)"
generators:
  x: { gen: string, regex: "__sync_[a-z_]{3,24}(_(1|2|4|8|16))?" }
evidence: src/frontend/sema/builtins.rs:711-714 doc comment
```

## P12 functions_map_contract
- Tier: 3
- Rationale: README documents FunctionInfo semantics exactly: params from the
  prototype, variadic flag from `...`, is_defined flips on definition,
  is_noreturn sticky across redeclaration. Model differential over generated
  prototypes/definitions.
- Doc contract: src/frontend/sema/README.md "The `is_noreturn` flag is sticky: if a prior declaration carries `__attribute__((noreturn))` or `_Noreturn`, the flag persists even when the function definition does not repeat the attribute. Implicitly declared functions ... default to `return_type: CType::Int`, `variadic: true`, `is_defined: false`." — asserted fingerprint e65d1090
- Seed: (none)
- Formal: ∀ (ret, params, variadic, noreturn_attr, defined). FunctionInfo[f] ==
  {ret, params, variadic, is_defined: defined, is_noreturn: noreturn_attr-sticky}
- Test file: src/frontend/sema/analysis.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: frontend::sema::analysis::SemanticAnalyzer::analyze
oracle: differential
predicate:
  quantifier: forall
  vars: [ret, params, variadic, noreturn, defined]
  domain: { ret: 6 type spellings, params: 0..3 of 5 types, variadic: bool, noreturn: bool, defined: bool }
  body: functions["f"] matches the declared signature model incl. noreturn stickiness
generators:
  variadic: { gen: bool }
  noreturn: { gen: bool }
  defined: { gen: bool }
evidence: src/frontend/sema/README.md "functions: FxHashMap<String, FunctionInfo>" section
```

## P13 implicit_function_declaration_contract
- Tier: 3
- Rationale: Negative/error contract documented in README: unknown call targets get
  an implicit -Wimplicit-function-declaration warning and register Int/variadic/
  not-defined; builtin names must NOT (is_builtin exists "so that sema does not emit
  spurious 'implicit declaration' warnings for them").
- Doc contract: src/frontend/sema/README.md "Implicit function declarations. Sema pre-populates a set of common libc functions and emits warnings for unknown calls rather than errors." — asserted fingerprint e65d1090
- Seed: (none)
- Formal: ∀ n ∉ Builtins ∪ SeededLibc. analyze("int f(void){ n(1); }") ⇒
  functions[n] == {Int, variadic, not-defined} ∧ warning_count ≥ 1;
  ∀ n ∈ Builtins. no implicit-declaration warning for n.
- Test file: src/frontend/sema/analysis.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: frontend::sema::analysis::SemanticAnalyzer::declare_implicit_functions
oracle: negative_error
predicate:
  quantifier: forall
  vars: [name, is_builtin_name]
  domain: { name: identifier, is_builtin_name: bool }
  relation:
    op: holds
    lhs: "unknown name -> functions[name]={Int,variadic,!defined} && warning emitted; builtin name -> no implicit-decl warning && no entry"
generators:
  name: { gen: string, regex: "[a-z_][a-z_0-9]{2,8}" }
  is_builtin_name: { gen: bool }
expected_error: -Wimplicit-function-declaration warning
evidence: src/frontend/sema/README.md "Implicit function declarations" + analysis.rs:1849
```

## P14 parse_src_failure_path (change surface)
- Tier: 2
- Rationale: Change-surface obligation — parse_src (parse.rs:1306) is the round-04
  test entry that drives real `Parser::parse()`; the hunk is error-handling (parse
  error recovery, README §13). Property drives the FAILURE branch: malformed token
  soup / truncated programs must produce error_count > 0, return a TU (recovery),
  and never panic.
- Doc contract: src/frontend/parser/parse.rs:1306 (test helper, no doc comment) — (none)
- Seed: round-04 P6 (parser malformed-soup panic-freedom); this property adds the
  error_count>0 failure-branch assertion (parse.rs:1316 `p.error_count`).
- Formal: ∀ src ∈ MalformedPrograms. (tu, errs) = parse_src(src) ⇒ errs ≥ 1 ∧ no panic
- Test file: src/frontend/parser/parse.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: frontend::parser::parse::pbt_tests::parse_src
oracle: crash_only
predicate:
  quantifier: forall
  vars: [src]
  domain: { src: mutated well-formed programs: truncation, brace/paren imbalance, stray operators, keyword soup }
  relation:
    op: holds
    lhs: "let (tu, errs) = parse_src(src); errs >= 1 && !panics"
generators:
  src: { gen: string, via: mutation of small well-formed C programs }
evidence: src/frontend/parser/README.md §13 (error recovery) + round-04 P6; failure-branch assertion per change-surface flag
```

## P15 sut_tokens_markers_stripped (change surface)
- Tier: 3
- Rationale: Change-surface obligation — sut_tokens (pbt_support.rs:438) documents
  its own contract: "Run the SUT preprocessor and return the token stream (markers
  stripped)". Property pins it on BOTH paths: success arm (no `# <digit>` line-marker
  token survives; stream is a fixed point of tokens(join(stream))) and FAILURE-
  INJECTION arm (invalid preprocessor input: unterminated #if, empty #define,
  unknown directive, stray # — the guarded operation `pp.preprocess(program)` on
  bad input): documented failure result is LENIENT recovery (no panic, still a
  token stream, markers still stripped; verified empirically via `ccc -E` exit 0
  on bad1.c/bad2.c), matching the sibling gcc_tokens Err-branch contract boundary.
- Doc contract: src/frontend/preprocessor/pbt_support.rs:437 "Run the SUT preprocessor and return the token stream (markers stripped)." — asserted fingerprint 65dd43e3
- Seed: (none)
- Formal: ∀ src ∈ SimplePrograms ∪ MalformedPrograms. t = sut_tokens(pp, src) ⇒
  no panic ∧ ¬∃tok ∈ t. tok starts with "#" followed by a digit ∧ (src well-formed ⇒
  tokens(join(" ",t)) == t)
- Test file: src/frontend/preprocessor/pbt_support.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: frontend::preprocessor::pbt_support::sut_tokens
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [src]
  domain: { src: small C programs with #define/#include-less directives + code lines }
  relation:
    op: holds
    lhs: "t = sut_tokens(&mut pp, src); no line-marker token in t && tokens(t.join(\" \")) == t"
generators:
  src: { gen: string, via: small line-composed programs }
evidence: src/frontend/preprocessor/pbt_support.rs:437 doc comment
```

## Status summary (final)

| # | Property | Oracle | Status | Bug |
|---|----------|--------|--------|-----|
| P1 | uac_binop_ctype | differential (C11 6.3.1.8 model + gcc KAT) | failing | b1 |
| P2 | enum_constant_type_bands | algebraic (documented GCC bands) | passing | — |
| P3 | enum_variant_values_model | differential (C11 6.7.2.2 model) | failing | b3 |
| P4 | typecontext_scope_undo_state_machine | state_machine (layered model) | failing | b4 |
| P5 | scope_shadow_via_analyze | differential (scope model via pipeline) | failing | b2 |
| P6 | return_type_fallthrough_model | differential (documented rule model) | passing | — |
| P7 | const_arith_c_semantics | differential (C-semantics evaluator + gcc KAT) | failing | b5 |
| P8 | sizeof_alignof_abi_table | reference (x86-64 SysV ABI) | passing | — |
| P9 | ternary_elvis_const_semantics | differential | passing | — |
| P10 | builtin_family_recognition | differential (documented families) | passing | — |
| P11 | libc_alias_and_sync_strip | algebraic | passing | — |
| P12 | functions_map_contract | differential (README contract) | passing | — |
| P13 | implicit_function_declaration_contract | negative_error | passing | — |
| P14 | parse_src_failure_path | crash_only / failure-path (change surface) | passing | — |
| P15 | sut_tokens_markers_stripped | algebraic + failure-injection (change surface) | passing | — |

10 passing, 5 failing → 5 confirmed SUT bugs (b1-b5). Deterministic red regression
witnesses live in `pbt_regression` mods beside each property. Run counts: all
proptest! suites configured with `ProptestConfig::with_cases(1024)`; P8/P6 KATs are
enumeration/one-shot tests.

## Fingerprint re-verification (close-out)

All 11 flagged Doc-contract citations were re-read against the current files on
2026-10-05: every quoted line is present verbatim (README.md:307/362/133/650,
type_checker.rs:37, type_context.rs:374/421, analysis.rs stmt_can_fall_through doc,
builtins.rs:651/712, pbt_support.rs:436). No claim the properties rest on changed;
fingerprints updated to the checker's current normalization. No property re-run
required (statuses unchanged from the final serial-confirmed run).
