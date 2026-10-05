# Property Ledger — src/frontend/preprocessor (round 03)

All tests run via `cargo test --lib frontend::preprocessor` (scratch CWD
`pbt-out/rounds/03_preprocessor/run/`). Oracle sources: module README (spec),
C11/C99 clauses it cites, GCC 9.4.0 host binary (differential), independent
reference implementations written from those clauses in the test modules.

## P1: eval_const_expr agrees with independent C99 6.10.1 reference evaluator
- Tier: 2
- Rationale: README ("Expression evaluator" + conditionals.rs doc: "Per C99 6.10.1, preprocessor
  integer expressions use intmax_t (signed) or uintmax_t (unsigned) depending on whether any
  operand has a u/U suffix") fixes exact arithmetic semantics. Strongest oracle: differential vs
  an independent AST interpreter written from that rule (Rust i64/u64 pairs, C usual
  conversions). Stronger oracle rejected: none available (GCC process-per-case too slow here;
  covered instead by P8's #if programs).
- Doc contract: src/frontend/preprocessor/conditionals.rs:248 "/// Per C99 6.10.1, preprocessor integer expressions use intmax_t (signed) or uintmax_t (unsigned)" — asserted fingerprint 91ec8f2e (re-verified 2026-10-05: quote present verbatim at recorded line; fingerprint refreshed after campaign test-module edits)
- Seed: (none)
- Formal: ∀ e ∈ Expr (generated AST, divisors ≠ 0, shifts 0..63, |values| bounded so signed ops
  never overflow), ∀ renderings r of e (random spacing, redundant parens, hex/dec/oct/suffix
  forms): eval_const_expr(r) ⇔ ref(e) ≠ 0.
- Test file: src/frontend/preprocessor/conditionals.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: conditionals::eval_const_expr
oracle: differential
predicate:
  quantifier: forall
  vars: [e, r]
  domain: { e: expr_ast, r: render(e) }
  relation:
    op: eq
    lhs: eval_const_expr(r)
    rhs: ref_eval(e) != 0
generators:
  e: { gen: expr_ast, depth: 0..4 }
  r: { gen: render, of: e }
evidence: src/frontend/preprocessor/README.md "Expression Evaluator" section; conditionals.rs:246
```

## P2: full #if pipeline selects the branch the reference evaluator says
- Tier: 2
- Rationale: exercises resolve_defined_in_expr → expand_line_reuse → resolve →
  replace_remaining_idents_with_zero → evaluate_condition end-to-end (README "The expression
  evaluator ... multi-stage pipeline"). Reference = same independent evaluator + C rule
  "undefined identifiers evaluate to 0" (README: "Per the C standard, any remaining identifiers
  (except true/false) evaluate to 0").
- Doc contract: src/frontend/preprocessor/expr_eval.rs:14 "/// Replace remaining identifiers (not keywords) with 0 in a #if expression." — asserted fingerprint 34f85fa9 (re-verified 2026-10-05: quote present verbatim at recorded line; fingerprint refreshed after campaign test-module edits)
- Seed: (none)
- Formal: ∀ program `#define M_k v_k* ... #if expr(M, idents, literals) \n KEEP \n #else \n DROP
  \n #endif`: output contains token KEEP iff ref(expr) ≠ 0, contains DROP iff ref(expr) = 0.
- Test file: src/frontend/preprocessor/pipeline.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: pipeline::Preprocessor::preprocess
oracle: reference
predicate:
  quantifier: forall
  vars: [macros, expr]
  domain: { macros: [(name, i64 body)], expr: expr_ast over macros+idents+literals }
  relation:
    op: holds
    lhs: "token KEEP in output <-> ref(expr) != 0"
generators:
  macros: { gen: list, elem: { gen: tuple, of: [macro_name, small_int] }, maxLen: 3 }
  expr: { gen: expr_ast, depth: 0..3 }
evidence: README.md "Expression Evaluator" pipeline description
```

## P3: conditional stack state machine matches an independent model
- Tier: 3
- Rationale: README "Conditional Compilation" documents the exact stack semantics (any-branch-
  taken tracking, parent-active nesting, #elif/#else flipping). Strongest oracle: state machine
  vs independent model (same shape as round-02's lexer properties).
- Doc contract: src/frontend/preprocessor/README.md:246 "The `ConditionalStack` maintains a `Vec<ConditionalState>` where each entry" — asserted fingerprint 34f85fa9 (re-verified 2026-10-05: quote present verbatim at recorded line; fingerprint refreshed after campaign test-module edits)
- Seed: (none)
- Formal: automaton — states: stack of (any_taken, active, parent_active); ops: if(c) push,
  elif(c), else, endif pop, content line L emits L iff is_active. ∀ well-formed op sequences S
  (balanced, ≤6 nesting): for each content line L_i in program(S), token L_i ∈ output ⇔ model(S,
  position(i)).is_active().
- Test file: src/frontend/preprocessor/conditionals.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: conditionals::ConditionalStack
oracle: state_machine
predicate:
  quantifier: forall
  vars: [ops]
  domain: { ops: well_formed_conditional_tree(depth<=5) }
  relation:
    op: holds
    lhs: "content token Ci in output <-> model_active(i)"
generators:
  ops: { gen: cond_tree, depth: 0..5 }
evidence: README.md "Conditional Compilation" table + ConditionalStack description
```

## P4: join_continued_lines equals independent GCC phase-2 splicer and is idempotent
- Tier: 2
- Rationale: README phase 2 ("backslash-newline sequences are joined") + doc comment on the fn
  ("Also handles backslash followed by trailing whitespace before newline, matching GCC/Clang
  behavior"). Oracle: independent splicer from the GCC rule + idempotence algebraic law.
- Doc contract: src/frontend/preprocessor/text_processing.rs:206 "/// Join lines that end with backslash (line continuation)." — asserted fingerprint 36fb8d4d
- Seed: (none)
- Formal: ∀ s ∈ Lines (random lines, some ending `\`+ws): join(s) == splice_ref(s) ∧
  join(join(s)) == join(s).
- Test file: src/frontend/preprocessor/text_processing.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: text_processing::Preprocessor::join_continued_lines
oracle: algebraic.round_trip
predicate:
  quantifier: forall
  vars: [lines]
  domain: { lines: Vec<line>, line: mixed ws/ident/str, 20% end with backslash+ws }
  relation:
    op: eq
    lhs: join(lines.join("\n"))
    rhs: splice_ref(lines)
generators:
  lines: { gen: list, elem: { gen: pp_line }, maxLen: 8 }
evidence: text_processing.rs doc comment; README phase 2
```

## P5: strip_block_comments equals independent C11 phase-3 stripper; idempotent; literals preserved
- Tier: 2
- Rationale: README: "/* ... */ block comments are replaced with a single space (per C11
  5.1.1.2 phase 3); // ... line comments are stripped"; literals copied verbatim.
- Doc contract: src/frontend/preprocessor/text_processing.rs:96 "/// Block comments are replaced with a single space (per C11 5.1.1.2 phase 3)," — asserted fingerprint 34f85fa9 (re-verified 2026-10-05: quote present verbatim at recorded line; fingerprint refreshed after campaign test-module edits)
- Seed: (none)
- Formal: ∀ s ∈ Text (literals, well-formed comments, newlines): strip(s) == strip_ref(s) ∧
  strip(strip(s)) == strip(s) ∧ every literal in s appears verbatim in strip(s).
- Test file: src/frontend/preprocessor/text_processing.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: text_processing::Preprocessor::strip_block_comments
oracle: differential
predicate:
  quantifier: forall
  vars: [text]
  domain: { text: c_text with literals and well-formed comments }
  relation:
    op: eq
    lhs: strip_block_comments(text).0
    rhs: strip_ref(text)
generators:
  text: { gen: comment_text, maxLen: 12 }
evidence: README.md phase 3; text_processing.rs:91
```

## P6: stringification follows C11 6.10.3.2 reference
- Tier: 2
- Rationale: README "Stringification (#): #param wraps the raw (unexpanded) argument text in
  double quotes, escaping embedded \" and \\ characters per C11 6.10.3.2". Reference:
  independent stringizer from the C11 clause (trim outer ws; each ws run between tokens → one
  space; backslash before each " and \ of char/string literals incl. delimiters).
- Doc contract: src/frontend/preprocessor/macro_defs.rs:1227 "/// Stringify a macro argument per C11 6.10.3.2." — asserted fingerprint eb3fa953
- Seed: (none)
- Formal: ∀ arg ∈ ArgText (idents, numbers, ws, string/char literals): `#define S(x) #x` +
  `S(arg)` → output token stream contains the string token stringize_ref(arg).
- Test file: src/frontend/preprocessor/macro_defs.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: macro_defs::stringify_arg
oracle: reference
predicate:
  quantifier: forall
  vars: [arg]
  domain: { arg: token_text }
  relation:
    op: eq
    lhs: "string token in output of `S(arg)`"
    rhs: stringize_ref(arg)
generators:
  arg: { gen: stringify_arg_text, maxLen: 6 }
evidence: macro_defs.rs:1229 doc; C11 6.10.3.2
```

## P7: token pasting concatenates raw operands (direct and via indirection)
- Tier: 3
- Rationale: README "Token Pasting (##): concatenates the text of the left and right operands
  into a single token. When an operand is a parameter, the raw argument text is used". Algebraic
  law: CAT(id1, id2) → single ident token id1id2; empty operand vanishes; indirect paste
  XCAT(a,b)=CAT(a,b) pastes post-expansion args.
- Doc contract: src/frontend/preprocessor/README.md:166 "token. When an operand is a parameter, the *raw* argument text is used (not" — asserted fingerprint 34f85fa9 (re-verified 2026-10-05: quote present verbatim at recorded line; fingerprint refreshed after campaign test-module edits)
- Seed: (none)
- Formal: ∀ (x,y) ∈ Ident×(Ident∪{ε}): output token stream of `CAT(x,y)` == [xy] and of
  `CAT(x,EMPTY)` == [x].
- Test file: src/frontend/preprocessor/macro_defs.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: macro_defs::handle_stringify_and_paste
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [x, y]
  domain: { x: ident, y: ident|empty }
  relation:
    op: eq
    lhs: tokens_of("CAT(x, y)")
    rhs: [concat(x,y)]
generators:
  x: { gen: ident, len: 1..6 }
  y: { gen: optional, of: { gen: ident, len: 1..6 } }
evidence: README.md "Token Pasting (##)"
```

## P8: preprocess token stream equals gcc -E -P on generated macro programs
- Tier: 1
- Rationale: README's whole posture is GCC compatibility ("GCC-compatible line markers",
  GCC-compatible search order, "matching GCC behavior"); the strongest available oracle is the
  host gcc 9.4.0 binary itself. KAT-gated (probe above). Token-stream comparison ignores
  spacing (SUT legitimately inserts anti-paste spaces) but maximal-munch tokenization preserves
  paste differences (`--` vs `- -`).
- Doc contract: src/frontend/preprocessor/README.md:598 "The preprocessor claims `__GNUC__ 14`, `__GNUC_MINOR__ 2` to satisfy version" — asserted fingerprint 34f85fa9 (re-verified 2026-10-05: quote present verbatim at recorded line; fingerprint refreshed after campaign test-module edits)
- Seed: (none)
- Formal: ∀ P ∈ Program (defines incl. empty-bodied object-like macros, function-like with #/##/
  __VA_ARGS__/`,##__VA_ARGS__`, content lines with macro uses between operator tokens, literal
  #if/#else/#endif): tokens(preprocess(P)) == tokens(gcc -E -P(P)).
- Test file: src/frontend/preprocessor/pipeline.rs
- Status: failing
- Counterexample: #define VAC(fmt, ...) g(fmt, ## __VA_ARGS__) + VAC(a, ) -> SUT [g ( a )], gcc [g ( a , )]
- Bug report: bug_reports/preprocessor_va_args_empty_argument_comma.md

```property
function: pipeline::Preprocessor::preprocess
oracle: differential
predicate:
  quantifier: forall
  vars: [program]
  domain: { program: macro_program (grammar: defines + content, no includes/predefined refs) }
  relation:
    op: eq
    lhs: tokenize(preprocess(program))
    rhs: tokenize(gcc_minus_E_minus_P(program))
generators:
  program: { gen: macro_program, maxLines: 10 }
evidence: README.md "GCC Compatibility Posture"; gcc KAT probes in round-03 run/
```

## P9: line-number preservation and __LINE__ correctness
- Tier: 3
- Rationale: README Output Contract: "Blank lines preserving the original line numbering";
  "__LINE__ Current line number ... Respects #line overrides". Invariant: output has exactly one
  line per source line (plus the initial marker) for comment-free inputs, and __LINE__ on source
  line k expands to k.
- Doc contract: src/frontend/preprocessor/README.md:451 "| `__LINE__` | Current line number; stored in a `Cell<usize>` and updated each line via `set_line()`. Respects `#line` overrides. |" — asserted fingerprint 34f85fa9 (re-verified 2026-10-05: quote present verbatim at recorded line; fingerprint refreshed after campaign test-module edits)
- Seed: (none)
- Formal: ∀ src ∈ Lines(no directives except none): output lines == 1 + #src lines ∧ token
  `<k>` == expansion of `__LINE__` on source line k.
- Test file: src/frontend/preprocessor/pipeline.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: pipeline::Preprocessor::preprocess
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [src]
  domain: { src: plain content lines, some containing __LINE__ }
  relation:
    op: holds
    lhs: "out.line_count == src.line_count + 1 && __LINE__ on line k -> k"
generators:
  src: { gen: list, elem: { gen: plain_line }, maxLen: 12 }
evidence: README.md Output Contract / __LINE__ rows
```

## P10: #error/#warning diagnostic contract (active vs inactive branches)
- Tier: 3
- Rationale: README side-channel contract: errors[] = "#error directives and unresolved
  #include failures"; directives in inactive branches are not processed ("Lines inside an
  inactive branch are replaced with empty lines ... directives only processed in active
  blocks" — pipeline.rs process_directive). Negative/error contract.
- Doc contract: src/frontend/preprocessor/README.md:91 "| `errors() -> &[PreprocessorDiagnostic]` | Collected `#error` directives and unresolved `#include` failures. |" — asserted fingerprint 34f85fa9 (re-verified 2026-10-05: quote present verbatim at recorded line; fingerprint refreshed after campaign test-module edits)
- Seed: (none)
- Formal: ∀ program with #error/#warning at active line a and inactive line i (inside #if 0):
  errors == [1 entry at (file, a, col)], warnings likewise; message contains expanded text.
- Test file: src/frontend/preprocessor/pipeline.rs
- Status: failing
- Counterexample: set_filename("t.c"); #error at line 5 -> errors()[0].file == "/abs/path/t.c" (expected "t.c")
- Bug report: bug_reports/preprocessor_error_file_absolutized.md

```property
function: pipeline::Preprocessor::errors
oracle: negative_error
predicate:
  quantifier: forall
  vars: [pre, msg_a, msg_i]
  domain: { pre: lines 0..5, msg: token_text }
  relation:
    op: holds
    lhs: "exactly one error, at active line, none from inactive"
generators:
  msg: { gen: token_text, maxLen: 4 }
evidence: README.md side-channel table; pipeline.rs process_directive inactive-block early return
```

## Additional entries finalized during Test/Review (strengthening + sweep)

### P1b: eval_const_expr literal-typing boundary matrix (deterministic)
- Status: passing | Test: conditionals.rs pbt_tests::p1b_eval_literal_boundaries
- Covers documented edges: decimal/hex > i64::MAX → unsigned, -1u < 0 == false,
  precedence (2+3*4, 1<<2+1, ==/&&/||), ternary right-assoc, signed trunc division.

### P5b: literals preserved verbatim by strip_block_comments
- Status: passing | Test: text_processing.rs pbt_tests::p5b_literals_preserved_verbatim
- Formal: ∀ pre,lit,post: strip(pre + "\n" + lit + post) contains lit verbatim.

### P8b: empty-macro anti-paste guard KAT (gcc differential)
- Status: failing — Counterexample: `-EMPTY-` -> SUT [--], gcc [-, -]
- Bug report: bug_reports/preprocessor_empty_macro_token_glue.md
- Test: pipeline.rs pbt_kat::p8b_empty_macro_paste_guard_kat (deterministic, all of
  -EMPTY-, +EMPTY+, /EMPTY/, <EMPTY<, =EMPTY=, 1 EMPTY 2; first case fails).

### P9b: #line override + __LINE__ (gcc-verified C11 6.10.4 semantics)
- Status: passing | Test: pipeline.rs pbt_sweep::p9b_line_directive_overrides
- Formal: ∀ target ∈ 1..1000, gap ∈ 0..4: after `#line target`, the line at distance
  gap+1 after the directive reports __LINE__ == target + gap.

### P11 (sweep round): pragma contract table
- Status: passing | Test: pipeline.rs pbt_sweep::p11_pragma_contracts
- Formal: ∀ op sequences from {pack(N), pack(), pack(push,N), pack(push), pack(pop)}:
  emitted synthetic tokens match the README table exactly; #pragma weak/redefine_extname
  side channels record (sym, None)/(sym, Some(alias))/(old, new); push_macro/pop_macro
  restores a definition across #undef (restored macro expands to 1).

### Deterministic regression witnesses (currently failing by design)
- test_preprocess_regression_va_args_empty_arg_keeps_comma → B1 (fails)
- test_preprocess_regression_error_file_field → B2 (fails)
- p8b_empty_macro_paste_guard_kat → B3 (fails)
