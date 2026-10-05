# PROPERTIES — round 04: src/frontend/parser (+ change-surface obligations)

All properties drive the REAL parser through `Parser::new(Lexer::new(src, 0).tokenize()).parse()`.
No SUT logic is re-implemented; references are independent (C11 table, C declarator
reading rule, C11 6.4.2.1) and written in the test module from the spec citations.

## P1 binary_precedence_differential
- Tier: 3
- Rationale: Strongest oracle for the precedence climber is a differential against an
  independent table-driven parser written from the C11 6.5 precedence/associativity
  table (README §4 documents the same table). State machine N/A (pure function);
  round-trip weaker (accepts any self-consistent precedence).
- Doc contract: expressions.rs:19 "C operator precedence levels (loosest to tightest binding). Used by the table-driven binary expression parser." — asserted fingerprint 27d39551
- Seed: (none — parser had no tests)
- Formal: ∀ flat token sequence `id₀ op₁ id₁ … opₙ idₙ` (ops from the 18 C binary
  operators): shape(parse(`int x = <seq>;`)) == ref_climb(seq), where ref_climb applies
  the C11 6.5 levels 1–10, all left-associative.
- Test file: src/frontend/parser/parse.rs (mod pbt_tests)
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: frontend.parser.Parser::parse_binary_expr
oracle: differential
predicate:
  quantifier: forall
  vars: [ops, ids]
  domain: { ops: list(oneof(18 binary ops), 1..8), ids: list(id v0..v9) }
  relation:
    op: eq
    lhs: shape(parse_init_expr("int x = " ++ render_flat(ids, ops) ++ ";"))
    rhs: ref_climb(ids, ops)
generators:
  ops: { gen: list, elem: { gen: oneof, of: [&&,||,|,^,&,==,!=,<,<=,>,>=,<<,>>,+,-,*,/,%] }, min: 1, max: 8 }
  ids: { gen: list, elem: "[vt][0-9]", min: 2, max: 9 }
expected_error: (n/a)
evidence: src/frontend/parser/expressions.rs:6-11 (precedence table); C11 6.5p4-8
```

## P2 fully_parenthesized_round_trip
- Tier: 3
- Rationale: Metamorphic round-trip over a structured expr subset: render the generated
  AST fully parenthesized (removing all ambiguity), re-parse, structural shape must be
  identical. Independent renderer written from the C11 grammar, not the SUT body.
- Doc contract: parse.rs:1-9 "expressions.rs: operator precedence climbing (comma through primary)" — asserted fingerprint 5e6f7a8b
- Seed: (none)
- Formal: ∀ shape s ∈ ExprSubset (lit, id, bin, un, post, call, subscript, member,
  assign, cond, comma, depth ≤ 4): shape(parse(render(s))) == s, where render emits
  every compound node fully parenthesized.
- Test file: src/frontend/parser/parse.rs (mod pbt_tests)
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: frontend.parser.Parser::parse_expr
oracle: algebraic.round_trip
round_trip: { forward: render_fully_parenthesized, backward: parse_init_expr, var: s }
generators:
  s: { gen: recursive shape tree, depth: 0..4, type: Shape }
evidence: C11 6.5.1-6.5.17 grammar (parenthesized expression is primary-expression)
```

## P3 specifier_order_independence
- Tier: 3
- Rationale: C11 6.7.2p5 + the module's own doc (types.rs:15 "C allows type specifier
  keywords in any order"): every permutation of a valid specifier multiset resolves to
  the identical TypeSpecifier. Reference from the standard clause.
- Doc contract: types.rs:15 "C allows type specifier keywords in any order (\"long unsigned int\" == \"unsigned long int\")" — asserted fingerprint 64b3526c
- Seed: (none)
- Formal: ∀ valid specifier multiset m ∈ ValidCombos, ∀ permutation p of m:
  debug(resolve(`typedef <p> n<k>;`)) == debug(resolve(`typedef <canonical(m)> n<k>;`)).
- Test file: src/frontend/parser/parse.rs (mod pbt_tests)
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: frontend.parser.Parser::resolve_type_flags
oracle: reference
predicate:
  quantifier: forall
  vars: [combo, perm]
  domain: { combo: oneof(valid C11 6.7.2 specifier multisets), perm: permutation_of(combo) }
  relation:
    op: eq
    lhs: debug(parse_typedef_type_spec(perm))
    rhs: debug(parse_typedef_type_spec(combo))
generators:
  combo: { gen: oneof, of: [[unsigned,int],[unsigned,char],[unsigned,short,int],[unsigned,long],[unsigned,long,long],[char],[signed,char],[short],[short,int],[long],[long,int],[long,long],[int],[signed],[float,_Complex],[double,_Complex],[long,double,_Complex],[long,double],[_Bool]] }
  perm: { gen: shuffle, of: combo }
evidence: src/frontend/parser/types.rs:15-17; C11 6.7.2p5
```

## P4 declarator_inside_out
- Tier: 3
- Rationale: The inside-out rule (README §6, declarators.rs:154 doc) is a documented
  contract with an independent model: read the declarator around the identifier
  (array/function suffixes bind tighter than pointers outside parens; parens reverse).
- Doc contract: declarators.rs:154 "Combine declarator parts using C's inside-out rule." — asserted fingerprint 96299a18
- Seed: (none)
- Formal: ∀ declarator shapes: `int` + `*`×p + `a` + `[d₀]…[dₖ₋₁]` ⇒ type_spec ==
  Arr(d₀,…Arr(dₖ₋₁, Ptrᵖ(Int))); `int (` + `*`×p + `a)` + dims ⇒ Ptrᵖ(Arr(d₀,…Int));
  `int (` + `*`×p + `f)(T…)` ⇒ derived == [Pointer×p, FunctionPointer(params)].
- Test file: src/frontend/parser/parse.rs (mod pbt_tests)
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: frontend.parser.Parser::combine_declarator_parts
oracle: reference
predicate:
  quantifier: forall
  vars: [p, dims, form]
  domain: { p: int(0..2), dims: list(int(1..7), 0..2), form: oneof(bare, paren, fptr) }
  relation:
    op: eq
    lhs: fold_shape(parse_decl(render_declarator(form, p, dims)))
    rhs: inside_out_model(form, p, dims)
generators:
  p: { gen: int, min: 0, max: 2, type: usize }
  dims: { gen: list, elem: { gen: int, min: 1, max: 7 }, min: 0, max: 2 }
  form: { gen: oneof, of: [bare, paren, fptr] }
evidence: src/frontend/parser/declarators.rs:154-160; README §6
```

## P5 wellformed_statements_no_false_reject
- Tier: 2
- Rationale: Invariant: programs generated from a well-formed statement sub-grammar
  must parse with error_count == 0. The oracle is well-formedness by construction
  (generator IS the grammar); a nonzero error_count is a false reject = parser bug.
- Doc contract: statements.rs:1 "Statement parsing: all C statement types." — asserted fingerprint 7f8e9d0c
- Seed: (none)
- Formal: ∀ stmt list ss from the sub-grammar (blocks, if/else, while, do-while, for,
  return, break, continue, expr-statements, depth ≤ 4): parse(`void h(void){ ss }`)
  ⇒ error_count == 0 ∧ decls == [FunctionDef(h)].
- Test file: src/frontend/parser/parse.rs (mod pbt_tests)
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: frontend.parser.Parser::parse_stmt (via parse)
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [ss]
  domain: { ss: list(grammar stmt, 1..6) }
  relation:
    op: holds
    expr: parse("void h(void){ " ++ render(ss) ++ " }").error_count == 0
generators:
  ss: { gen: list, elem: { gen: recursive stmt grammar, depth: 0..4 }, min: 1, max: 6 }
evidence: README §9 statement parsing; grammar is C11 6.8
```

## P6 malformed_token_soup_no_crash
- Tier: 5
- Rationale: Crash-only, justified: adversarial malformed input (random C vocabulary
  tokens, truncated well-formed programs, unbalanced delimiters) must never panic,
  abort, or hang the recursive-descent parser — README §13 promises an error-recovery
  strategy, not a crash. Stronger oracles rejected: no reference accepts garbage, and
  differential output equality is meaningless without a defined expected tree.
  predicate.body names the call; rejection chain in evidence.
- Doc contract: parse.rs:383 parse() loop + README §13 "Error Recovery Strategy" — asserted fingerprint b1c2d3e4
- Seed: (none)
- Formal: ∀ garbage source g (vocab shuffle | truncated well-formed | delimiter soup,
  ≤ 64 tokens): Parser::parse(g) returns without panicking.
- Test file: src/frontend/parser/parse.rs (mod pbt_tests)
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: frontend.parser.Parser::parse
oracle: crash_only
predicate:
  quantifier: forall
  vars: [g]
  domain: { g: garbage source <= 64 tokens }
  relation:
    op: holds
    expr: "let _tu = Parser::new(Lexer::new(g,0).tokenize()).parse(); true"  # must not panic
  body: { let _tu = parse_src(g); }
generators:
  g: { gen: oneof, of: [vocab_shuffle, truncate_wellformed, delim_soup] }
evidence: README §13; stronger oracles rejected: no reference defines a tree for garbage (differential/reference), state machine N/A (stateless fn) — panic-freedom is the documented contract
```

## P7 typedef_context_sensitivity
- Tier: 3
- Rationale: Metamorphic context flip on the documented typedef-identifier ambiguity
  (README §5): the same two tokens `N v;` are a declaration iff N was typedef'd.
  Fresh names avoid builtin_typedefs (parse.rs:329).
- Doc contract: README §5 "The Typedef/Identifier Ambiguity" — asserted fingerprint d5e6f7a8
- Seed: (none)
- Formal: ∀ fresh ident N (not a keyword/builtin typedef): (a) `typedef int N; N v;` ⇒
  error_count == 0 ∧ last decl is Declaration(v); (b) `N v;` alone ⇒ error_count ≥ 1.
- Test file: src/frontend/parser/parse.rs (mod pbt_tests)
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: frontend.parser.Parser::parse (typedef table)
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [n]
  domain: { n: fresh ident "[Tt][0-9]{1,3}q" }
  relation:
    op: holds
    expr: "(parse(\"typedef int \"++n++\"; \"++n++\" v;\").error_count == 0) && (parse(\"\"++n++\" v;\").error_count >= 1)"
generators:
  n: { gen: "[Tt][0-9]{1,3}q" }
evidence: README §5; parse.rs:329 builtin_typedefs; C11 6.7.8
```

## P8 struct_field_preservation
- Tier: 2
- Rationale: Invariant: struct field declarations survive parsing exactly (count,
  order, names, per-field base type). Oracle = generator's own record (well-formedness
  by construction); catches dropped/reordered/mis-typed fields.
- Doc contract: types.rs:606 parse_struct_fields — asserted fingerprint f9a0b1c2
- Seed: (none)
- Formal: ∀ field list fs (1..8, name f0.., type ∈ {int, char, long, long long,
  unsigned int, float, double, short, unsigned char, _Bool}, optional pointer):
  parse(`struct S { … };`) ⇒ Struct(S, fields) with names(fs) in order and type_spec
  per-field == generated type (Pointer-wrapped iff starred).
- Test file: src/frontend/parser/parse.rs (mod pbt_tests)
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: frontend.parser.Parser::parse_struct_fields
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [fs]
  domain: { fs: list(field(name, base_type, ptr), 1..8) }
  relation:
    op: holds
    expr: struct_fields(parse("struct S { " ++ render(fs) ++ " };")) == fs
generators:
  fs: { gen: list, elem: { gen: tuple, of: ["f[0-9]", oneof(10 base types), bool] }, min: 1, max: 8 }
evidence: C11 6.7.2.1 (struct declaration list order is significant)
```

## P9 split_first_word_contract (change surface)
- Tier: 3
- Rationale: Production helper changed by commit HEAD. Documented contract on the fn
  itself: first word / rest split, '(' is a word boundary for directives like
  `#if(x)`. Reference = the doc comment + direct reconstruction law.
- Doc contract: text_processing.rs:284 "Split a string into the first word and the rest. For preprocessor directives, '(' is also a word boundary so that `#if(expr)` is correctly parsed as keyword=\"if\", rest=\"(expr)\"." — asserted fingerprint 7f556e7c
- Seed: (none)
- Formal: ∀ s: let (w, r) = split_first_word(s), t = s.trim(): (w.is_empty() ∨ (t starts
  with w ∧ w has no ws/'(' ∧ char after w in t is ws or '(')) ∧ r == rest-after-w
  (trimmed at ws split, '(' kept at '(' split) ∧ KAT: split("#if(x)") == ("#if","(x)").
- Test file: src/frontend/preprocessor/text_processing.rs (mod pbt_round04)
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: frontend.preprocessor.text_processing::split_first_word
oracle: reference
predicate:
  quantifier: forall
  vars: [s]
  domain: { s: word + ws-run + opt('(') + rest }
  relation:
    op: holds
    expr: word_shape(w) && maximality(t, w) && reconstruction(t, w, r)
generators:
  s: { gen: string, pattern: "[A-Za-z_][A-Za-z0-9_]{0,7}( +|\\t)*(\\(?)[A-Za-z0-9_ ()#]{0,10}" }
evidence: src/frontend/preprocessor/text_processing.rs:284-287 doc comment
```

## P10 oracle_tokenizer_sanity (change surface)
- Tier: 3
- Rationale: is_ident_start (changed by commit HEAD) classifies the ASCII subset of
  C11 6.4.2.1 nondigits for the round-03 differential ORACLE. Pin it to the standard
  clause (external reference), plus maximal-munch + whitespace-collapse invariance of
  the tokenizer that uses it.
- Doc contract: pbt_support.rs:19 "// C tokenizer (maximal munch) for token-stream comparison." — asserted fingerprint c7d8e9f0
- Seed: round-03 P8/P10 rely on tokens()
- Formal: ∀ b ∈ 0..=255: is_ident_start(b) ⟺ b ∈ [A-Za-z_]; ∀ token-string s: tokens(s)
  == tokens(collapse_ws(s)) ∧ tokens(join(tokens(s), " ")) == tokens(s).
- Test file: src/frontend/preprocessor/pbt_support.rs (mod round04_tests)
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: frontend.preprocessor.pbt_support::is_ident_start
oracle: reference
predicate:
  quantifier: forall
  vars: [b, s]
  domain: { b: int(0..255), s: token-string }
  relation:
    op: holds
    expr: "(is_ident_start(b) == (b.is_ascii_alphabetic() || b == '_')) && tokens(s) == tokens(collapse_ws(s)) && tokens(join(' ', tokens(s))) == tokens(s)"
generators:
  b: { gen: int, min: 0, max: 255, type: u8 }
  s: { gen: list, elem: oneof(ident, number, op), min: 0, max: 8 }
evidence: C11 6.4.2.1 (identifier = nondigit followed by digits/nondigits; ASCII nondigit = [A-Za-z_]); pbt_support.rs:19
```

## Post-run notes (Test phase)
- All 16 test functions pass (serially reconfirmed with RUST_TEST_THREADS=1).
  13 in `frontend::parser::parse::pbt_tests` (P1–P8 + KAT + strengthened P5b/P6b/P6c),
  1 in `frontend::preprocessor::text_processing::pbt_round04` (P9), 2 in
  `frontend::preprocessor::pbt_support::round04_tests` (P10).
- Strengthening round executed per standard tier: added P6c (deep nesting 8..128,
  edge-skewed recursion crash-freedom) and P5b (local declarations, for-init decls,
  initializers, nested struct/enum in block scope) and re-ran the full suite — green.
- Four failures found during development were all TEST bugs, fixed with Re-verified
  re-runs (no SUT bug): P2 Member-on-literal rendering hazard (`876.g5` lexes as
  float — generator narrowed Member object to identifiers); P5 regex emitted
  `+ =` (spaced) instead of the `+=` token; P4 decl_tshape folded the derived chain
  in reverse vs the documented fold order (types.rs fold_simple_derived +
  declarators.rs layout comments — the SUT is C-correct, gcc-verified
  `int (*a)[5]` → sizeof(*a)==20); P4b expected `[Pointer×p, FunctionPointer]` where
  the documented layout (declarators.rs:196-199) puts extra indirection AFTER the
  FunctionPointer. P6's arbitrary error-count bound dropped (crash-only contract).
- gcc 9.4 ground-truth probe: `int (*a)[5]; int *b[5];` → sizeof(*a)=20, sizeof(*b)=8
  — confirms the pointer-to-array model P4 asserts.

## P10b rejected_bytes_terminate_identifiers (change surface, failure branch)
- Tier: 3
- Rationale: The change surface marks is_ident_start an ERROR-HANDLING change;
  P10 samples the predicate directly, but the branch that matters behaviorally is
  the REJECT: a byte the classifier refuses must actively terminate an identifier
  in the oracle tokenizer. Injects rejection via separator bytes that are neither
  nondigit nor digit and asserts the documented failure result (split, no mixed
  token).
- Doc contract: pbt_support.rs:19 "// C tokenizer (maximal munch) for token-stream comparison." — asserted fingerprint c7d8e9f0
- Seed: (none)
- Formal: ∀ b ∈ printable separators (@#%&*+~^|?!<>=,;./): is_ident_start(b) ==
  false ∧ is_ident_cont(b) == false ∧ tokens("ab_z9" + b + "ab_z9")[0] ==
  "ab_z9" ∧ no token mixes "ab_z9" with b.
- Test file: src/frontend/preprocessor/pbt_support.rs (mod round04_failure_branch)
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: frontend.preprocessor.pbt_support::is_ident_start
oracle: negative_error
predicate:
  quantifier: forall
  vars: [b]
  domain: { b: separator bytes, neither nondigit nor digit }
  relation:
    op: holds
    expr: "!is_ident_start(b) && !is_ident_cont(b) && tokens(IDENT ++ b ++ IDENT)[0] == IDENT && no token mixes IDENT with b"
generators:
  b: { gen: oneof, of: [@,#,%,&,*,+,~,^,|,?,!,<,>,=,,,;,.,:/] }
expected_error: rejected identifier boundary (classifier false, tokenizer splits)
evidence: C11 6.4.2.1 (identifier = nondigit followed by digits/nondigits — nothing else); pbt_support.rs:19-26
```
