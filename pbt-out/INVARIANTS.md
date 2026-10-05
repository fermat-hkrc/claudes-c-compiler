
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
