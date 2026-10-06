# PROPERTIES.md — round 07 (src/passes)

All properties execute the REAL pass entry points on generated well-formed SSA
IR built with the repo's own `IrFunction::new` test constructor, and check values
with an INDEPENDENT C11-semantics interpreter implemented in
`src/passes/pbt_common.rs` (written from C11/README semantics, not derived from
any SUT evaluator). Div/rem right-hand sides in generators are nonzero constants
or provably-nonzero forms except where the property targets division semantics
itself — C11 UB inputs are excluded from metamorphic domains per the documented
UB-preservation contract (README:546 "not folded (preserving undefined behavior)").

## p1_udiv32_magic_differential
- Tier: 2
- Rationale: README documents the exact unsigned magic-number rewrite and claims
  full-range correctness; the emitted IR (mask, 64-bit mul, hi-shift, optional
  add-fixup) is checked end-to-end against C11 unsigned division — the existing
  unit tests sample the helper arithmetic on ~12 points, never the emitted IR.
  Stronger oracle rejected: exhaustive ∀x per divisor is 2^32 per d; PBT samples
  edge-skewed (d, x) pairs and additionally covers the in-I64 widened form.
- Doc contract: src/passes/div_by_const.rs:17 "All transformations produce correct results for the full range of inputs, including edge cases (0, 1, -1, INT_MIN, UINT_MAX, INT64_MIN, UINT64_MAX)." — asserted fingerprint 82cd7c63
- Seed: src/passes/div_by_const.rs:1668 test_unsigned_magic_7 (point samples of the helper, generalized to arbitrary d and emitted-IR evaluation)
- Formal: ∀ d ∈ {2..2^32-1} edge-skewed, ∀ x ∈ u32 edge-skewed, ∀ shape ∈ {U32-native, U32-zero-extended-in-I64}: interp(div_by_const_function(f[x /u d])) = x /u d (C11 u32 division).
- Test file: src/passes/div_by_const.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: passes::div_by_const::div_by_const_function
oracle: reference
predicate:
  quantifier: forall
  vars: [d, x, shape]
  domain: { d: u32_divisor_edges, x: u32_edges, shape: {U32Native, U32InI64} }
  relation:
    op: eq
    lhs: interp(div_by_const_function(build_udiv(d, x, shape)))
    rhs: x /u d
generators:
  d: { gen: oneof, variants: [ { gen: int, min: 2, max: 2, type: u32 }, { gen: pow2_u32 }, { gen: int, min: 2, max: 4294967295, type: u32 }, { gen: const, val: 4294967295 } ] }
  x: { gen: oneof, variants: [ { gen: int, min: 0, max: 4294967295, type: u32 }, { gen: const, val: 0 }, { gen: const, val: 4294967295 } ] }
  shape: { gen: oneof, variants: [ { gen: const, val: U32Native }, { gen: const, val: U32InI64 } ] }
evidence: src/passes/README.md div_by_const section "Unsigned division ... magic number ... Hacker's Delight"
```

## p2_sdiv32_magic_differential
- Tier: 2
- Rationale: signed magic path incl. power-of-2 biased shift, the sign-correction
  add, and NEGATIVE divisors (`x / -C == -(x / C)`); INT_MIN dividend is the classic
  overflow killer. Same reference-oracle shape as p1, extended to i32/i64-widened
  forms and negative d.
- Doc contract: src/passes/div_by_const.rs:17 "All transformations produce correct results for the full range of inputs, including edge cases (0, 1, -1, INT_MIN, UINT_MAX, INT64_MIN, UINT64_MAX)." — asserted fingerprint 82cd7c63
- Seed: src/passes/div_by_const.rs:1730 test_signed_magic_7 (point samples, i32::MIN+1 excluded there; generator includes i32::MIN)
- Formal: ∀ d ∈ ±{2..2^31}, ∀ x ∈ i32 edge-skewed, ∀ shape ∈ {I32Native, I32-sign-extended-in-I64}: interp(div_by_const_function(f[x /s d])) = trunc_i32(x /s d).
- Test file: src/passes/div_by_const.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: passes::div_by_const::div_by_const_function
oracle: reference
predicate:
  quantifier: forall
  vars: [d, x, shape]
  domain: { d: signed_divisor_edges, x: i32_edges, shape: {I32Native, I32InI64} }
  relation:
    op: eq
    lhs: interp(div_by_const_function(build_sdiv(d, x, shape)))
    rhs: trunc_div_i32(x, d)
generators:
  d: { gen: oneof, variants: [ { gen: int, min: -2147483647, max: -2, type: i32 }, { gen: int, min: 2, max: 2147483647, type: i32 }, { gen: const, val: -2147483648 } ] }
  x: { gen: oneof, variants: [ { gen: int, min: -2147483648, max: 2147483647, type: i32 }, { gen: const, val: -2147483648 }, { gen: const, val: 2147483647 }, { gen: const, val: 0 } ] }
  shape: { gen: oneof, variants: [ { gen: const, val: I32Native }, { gen: const, val: I32InI64 } ] }
evidence: src/passes/div_by_const.rs:29 "Negative divisors are handled via identity: x / -C == -(x / C)"
```

## p3_rem_by_const_differential
- Re-verified: RUST_TEST_THREADS=1 cargo test --lib -- passes::div_by_const::pbt_tests::p3_rem_by_const_differential --exact → ok (all persisted seeds replayed green). Three harness defects fixed en route, each grounded in documented behavior: (1) build_rem_in_i64 sign-extended the URem divisor (must keep the u32 bit pattern); (2) unsigned power-of-2 divisors are documented as simplify's job ("Power-of-2 unsigned modulo is handled separately by the simplify pass as x & (C-1)") — excluded from the must-rewrite domain; (3) compute_unsigned_magic_32 documents "Fall back to native division for these rare cases" for large divisors near u32::MAX — excluded via the same predicate the pass uses. The value-preservation law itself never changed and holds on the full domain.
- Tier: 2
- Rationale: modulo rewrites (`x % C => x - (x / C) * C`, negative-divisor identity
  `x % -C == x % C`, sign-follows-dividend); URem/SRem, native and in-I64 forms.
- Doc contract: src/passes/div_by_const.rs:29 "Negative divisors are handled via identity: `x / -C == -(x / C)` and `x % -C == x % C` (in C, the sign of the remainder follows the dividend)." — asserted fingerprint 82cd7c63 (same doc block; identity quoted from div_by_const.rs:29-30)
- Seed: (none)
- Formal: ∀ op ∈ {SRem, URem}, ∀ d ≠ 0 (|d| ≤ 2^31), ∀ x edges: interp(div_by_const_function(f[x op d])) = C11_rem(x, d).
- Test file: src/passes/div_by_const.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: passes::div_by_const::div_by_const_function
oracle: reference
predicate:
  quantifier: forall
  vars: [op, d, x]
  domain: { op: {SRem,URem}, d: nonzero_divisor_edges, x: int32_edges }
  relation:
    op: eq
    lhs: interp(div_by_const_function(build_rem(op, d, x)))
    rhs: c11_rem(op, x, d)
generators:
  op: { gen: oneof, variants: [ { gen: const, val: SRem }, { gen: const, val: URem } ] }
  d: { gen: oneof, variants: [ { gen: int, min: -2147483647, max: -2, type: i32 }, { gen: int, min: 2, max: 2147483647, type: i32 } ] }
  x: { gen: int, min: -2147483648, max: 2147483647, type: i32 }
evidence: src/passes/README.md "Modulo by constant: x % C => x - (x / C) * C"
```

## p4_div64_genuine_i128_differential
- Tier: 2
- Rationale: genuine 64-bit dividends/divisors route to the I128 magic paths
  (expand_udiv64/expand_sdiv64/expand_sdiv64_neg) — 128-bit intermediate products,
  the least-covered code in the file (unit tests only cover 32-bit helpers).
- Doc contract: src/passes/div_by_const.rs:15 "64-bit magic numbers require 128-bit intermediate products. We use the compiler's existing I128/U128 types" — asserted fingerprint 82cd7c63 (same module doc block)
- Seed: (none)
- Formal: ∀ op ∈ {UDiv,SDiv}, ∀ d edge-skewed ∈ 64-bit divisor space, ∀ x edge-skewed ∈ 64-bit space: interp(div_by_const_function(f[x op d])) = wrapping_div/rem(x, d) at 64 bits.
- Test file: src/passes/div_by_const.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: passes::div_by_const::div_by_const_function
oracle: reference
predicate:
  quantifier: forall
  vars: [op, d, x]
  domain: { op: {UDiv,SDiv}, d: u64_divisor_edges, x: i64_edges }
  relation:
    op: eq
    lhs: interp(div_by_const_function(build_div64(op, d, x)))
    rhs: c11_div64(op, x, d)
generators:
  op: { gen: oneof, variants: [ { gen: const, val: UDiv }, { gen: const, val: SDiv } ] }
  d: { gen: oneof, variants: [ { gen: int, min: 2, max: 9223372036854775807, type: i64 }, { gen: const, val: 4294967296 } ] }
  x: { gen: oneof, variants: [ { gen: int, min: -9223372036854775808, max: 9223372036854775807, type: i64 }, { gen: const, val: -9223372036854775808 } ] }
evidence: src/passes/div_by_const.rs:14-19 (I128 intermediate products for 64-bit magic)
```

## p5_fold_matches_c11_model
- Tier: 2
- Rationale: fold contract = "evaluates operations on constant operands ... replacing
  the instruction with the computed constant"; the folded Return must equal the
  independent interpreter's C11 value over random const expression DAGs at mixed
  integer types, and folding must be idempotent (second run = 0 changes).
- Doc contract: src/passes/constant_fold.rs:3 "This pass evaluates operations on constant operands at compile time, replacing the instruction with the computed constant." — asserted fingerprint 4c9bab3a
- Seed: (none)
- Formal: ∀ E ∈ const-expr DAGs over {I32,I64,U32,U64} safe ops: fold_function(E) leaves Return = Const(c) ∧ c = interp(E) ∧ fold_function(E') = 0 changes.
- Test file: src/passes/constant_fold.rs
- Status: failing
- Counterexample: ty0=0 (I8), one-step DAG `LShr(Const(I8(-72)), Const(I8(1)), I8)` — fold_function folds to I8(-36); the C11 model requires 92 (0xB8 >>u 1). Root cause: the `is_32bit` fast path in fold_binop casts the sub-int operand with `as u32` (sign-extending 0xB8 to 0xFFFFFFB8) before LOGICAL ops (LShr/UDiv/URem); AShr is promotion-correct by the same cast. Deterministic red witness: test_constant_fold_regression_subint_lshr_zero_extend (also UDiv: 0xB8 /u 2 → -36 vs 92).
- Re-verified: RUST_TEST_THREADS=1 cargo test --lib -- passes::constant_fold::pbt_tests::p5_fold_matches_c11_model --exact → FAILED serially (fold mismatch ret_ty=I8, left Value(-64) / right Value(64) on the pre-final generator; final minimal: LShr at I8). Five generator/oracle harness defects were fixed en route (each with its persisted seed replayed green): in-I64 URem divisor sign-extension; mixed-width Value use without Cast (val_tys tracking added); signed-cmp-op at unsigned type removed from the domain; AShr at unsigned sub-int removed (fold's C11 promotion is correct there, the width-local model is not); divisor mask zeroing at I8 (m=256). None weaken the law.
- Bug report: bug_reports/passes_constant_fold_subint_logical_fold.md


```property
function: passes::constant_fold::fold_function
oracle: differential
predicate:
  quantifier: forall
  vars: [expr]
  domain: { expr: const_expr_dag }
  relation:
    op: eq
    lhs: folded_return_const(fold_function(expr))
    rhs: interp(expr)
generators:
  expr: { gen: const_expr_dag, maxLen: 12 }
evidence: src/passes/constant_fold.rs:1-4 module doc; README "Constant Folding" section
```

## p6_fold_div_by_zero_not_folded
- Tier: 3
- Rationale: documented failure-path contract — division by a constant ZERO must
  survive folding (UB preserved, not folded to a value). This is the negative/error
  contract of the fold pass (error-handling change coverage for fold_function).
- Doc contract: src/passes/README.md:546 "Integer division/remainder by zero is not folded (preserving undefined behavior)." — asserted fingerprint f7c910fd (spec; fold module doc silent on div-zero)
- Seed: (none)
- Formal: ∀ op ∈ {SDiv,UDiv,SRem,URem}, ∀ ty ∈ {I32,I64}, ∀ lhs ∈ {param, const}: after fold_function, the BinOp{op, rhs=Const(0)} instruction is still present unreplaced.
- Test file: src/passes/constant_fold.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: passes::constant_fold::fold_function
oracle: negative_error
predicate:
  quantifier: forall
  vars: [op, ty, lhs_kind]
  domain: { op: {SDiv,UDiv,SRem,URem}, ty: {I32,I64}, lhs_kind: {Param,Const} }
  relation:
    op: holds
    body: fold_function(f) leaves BinOp{op, rhs=Const(0)} present
generators:
  op: { gen: oneof, variants: [ { gen: const, val: SDiv }, { gen: const, val: UDiv }, { gen: const, val: SRem }, { gen: const, val: URem } ] }
  ty: { gen: oneof, variants: [ { gen: const, val: I32 }, { gen: const, val: I64 } ] }
  lhs_kind: { gen: oneof, variants: [ { gen: const, val: Param }, { gen: const, val: Const } ] }
evidence: src/passes/README.md:546-547 "**Division by zero.** Integer division/remainder by zero is not folded (preserving undefined behavior)."
```

## p7_simplify_identity_metamorphic
- Tier: 2
- Rationale: metamorphic: identity-injected expression E' (per the README table:
  x+0, x-0, x*1, x/1, x%1, x&~0, x|0, x^0, x<<0, x>>0, x-x, x&x, x|x, x^x,
  x&0, x*0, 0<<x, 0>>x, x+0-commuted) must evaluate equal to the clean E after
  simplify_function — the README's identity table IS the transform list.
- Doc contract: src/passes/simplify.rs:3 "Applies algebraic identities to simplify instructions (integer types):" — asserted fingerprint d4552340 (module doc lists the exact identity table)
- Seed: (none)
- Formal: ∀ E, ∀ inj ∈ identity_injections: interp(simplify_function(E')) = interp(E) (E' = E with inj applied; div/rem RHS constants nonzero; x/x, x%x excluded — UB on zero operands per C11, documented as valid only over defined inputs).
- Test file: src/passes/simplify.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: passes::simplify::simplify_function
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [expr, injection]
  domain: { expr: safe_expr_dag, injection: identity_table }
  relation:
    op: eq
    lhs: interp(simplify_function(inject(expr, injection)))
    rhs: interp(expr)
generators:
  expr: { gen: safe_expr_dag, maxLen: 8 }
  injection: { gen: oneof, variants: [add0, sub0, mul1, div1, rem1, and_allones, or0, xor0, shl0, shr0, sub_self, and_self, or_self, xor_self, and0, mul0, zero_shl, zero_shr] }
evidence: src/passes/README.md "Identity simplifications (integer-only ...)" table
```

## p8_simplify_strength_reduction_value
- Tier: 2
- Rationale: strength reductions change the OPERATOR (x*2^k→x<<k, x*2→x+x,
  x*(-1)→neg, x/u 2^k→x>>l k, x%u 2^k→x&(2^k-1)); value preservation against the
  plain-expression interpreter, over unsigned-typed exprs for the unsigned forms.
- Doc contract: src/passes/simplify.rs:3 "Applies algebraic identities to simplify instructions (integer types):" — asserted fingerprint d4552340 (same module doc; strength-reduction table follows the identity table)
- Seed: (none)
- Formal: ∀ sr ∈ strength_reductions, ∀ x edges: interp(simplify_function(f[sr(x)])) = interp(f[plain(x)]) at the expression's type.
- Test file: src/passes/simplify.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: passes::simplify::simplify_function
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [sr, k, x, ty]
  domain: { sr: {mul_pow2, mul2, mul_neg1, udiv_pow2, urem_pow2}, k: 0..31, x: int_edges, ty: {I32,U32,I64,U64} }
  relation:
    op: eq
    lhs: interp(simplify_function(f[sr_form(sr, k, x, ty)]))
    rhs: interp(f[plain_form(sr, k, x, ty)])
generators:
  sr: { gen: oneof, variants: [mul_pow2, mul2, mul_neg1, udiv_pow2, urem_pow2] }
  k: { gen: int, min: 0, max: 31, type: u32 }
  x: { gen: int, min: -2147483648, max: 2147483647, type: i32 }
  ty: { gen: oneof, variants: [ { gen: const, val: I32 }, { gen: const, val: U32 }, { gen: const, val: I64 }, { gen: const, val: U64 } ] }
evidence: src/passes/README.md "Strength reductions (integer-only)" table
```

## p9_gvn_copy_prop_preservation
- Tier: 2
- Rationale: GVN replaces redundant computations with references to earlier values;
  copy_prop resolves the resulting copies; both must preserve the interpreted value
  and not increase instruction count. Reference = interpreter on the same args.
- Doc contract: src/passes/gvn.rs:3 "This pass assigns \"value numbers\" to expressions and replaces redundant computations with references to previously computed values (CSE)." — asserted fingerprint ad3c0b2b
- Seed: (none)
- Formal: ∀ E ∈ DAGs with duplicated subexpressions and Copy chains, ∀ args: interp(run_gvn_function ∘ propagate_copies (E)) = interp(E) ∧ inst_count(after) ≤ inst_count(before).
- Test file: src/passes/gvn.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: passes::gvn::run_gvn_function
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [expr, args]
  domain: { expr: dup_expr_dag, args: int_samples }
  relation:
    op: holds
    body: interp(gvn_then_copy_prop(expr), args) == interp(expr, args) && count_insts(after) <= count_insts(before)
generators:
  expr: { gen: dup_expr_dag, maxLen: 14 }
  args: { gen: list, elem: { gen: int, min: -9223372036854775808, max: 9223372036854775807, type: i64 }, maxLen: 2 }
evidence: src/passes/gvn.rs:1-10 module doc; README "Global Value Numbering"
```

## p10_dce_preservation
- Tier: 2
- Rationale: DCE removes instructions whose results are never used ("Side-effecting
  instructions (stores, calls) are never removed"); adding dead pure tails must not
  change the interpreted result, and dead instructions must actually be removed.
- Doc contract: src/passes/dce.rs:3 "Removes instructions whose results are never used by any other instruction or terminator." — asserted fingerprint 465c2b5b
- Seed: (none)
- Formal: ∀ E, ∀ dead ∈ pure dead chains: interp(eliminate_dead_code(E ++ dead)) = interp(E) ∧ count(after) < count(E ++ dead).
- Test file: src/passes/dce.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: passes::dce::eliminate_dead_code
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [expr, dead]
  domain: { expr: safe_expr_dag, dead: pure_dead_chain }
  relation:
    op: holds
    body: interp(eliminate_dead_code(expr ++ dead)) == interp(expr) && count_insts(after) < count_insts(expr ++ dead)
generators:
  expr: { gen: safe_expr_dag, maxLen: 8 }
  dead: { gen: pure_dead_chain, maxLen: 4 }
evidence: src/passes/dce.rs:1-12 module doc
```

## p11_if_convert_preservation
- Tier: 2
- Rationale: if-conversion turns branch+phi diamonds into Select; both arms are
  then always evaluated, so the pass must only fire where that is value-preserving
  (and ops that can_trap are guarded per ops.rs can_trap). Differential: interpreter
  before vs after on the same args.
- Doc contract: src/passes/if_convert.rs:1 "If-conversion pass: converts simple branch+phi diamonds to Select instructions." — asserted fingerprint b5c48930
- Seed: (none)
- Formal: ∀ D ∈ param-conditioned diamonds with 0-1 simple arm instructions and a merge phi: interp(if_convert_function(D)) = interp(D).
- Test file: src/passes/if_convert.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: passes::if_convert::if_convert_function
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [diamond, args]
  domain: { diamond: gen_diamond, args: int_samples }
  relation:
    op: eq
    lhs: interp(if_convert_function(diamond), args)
    rhs: interp(diamond, args)
generators:
  diamond: { gen: gen_diamond, maxLen: 6 }
  args: { gen: list, elem: { gen: int, min: -2147483648, max: 2147483647, type: i32 }, maxLen: 2 }
evidence: src/passes/if_convert.rs:1-14 module doc (diamond pattern contract)
```

## p12_cfg_simplify_preservation_and_reachability
- Re-verified: RUST_TEST_THREADS=1 cargo test --lib -- passes::cfg_simplify::pbt_tests::p12_cfg_simplify_preservation_and_reachability --exact → ok in 700.83s (the property is slow, not hung — 1024 cases × random-CFG build + 200k-step interp; run2's parallel run was cut off at the 60s watchdog while it was still working). All persisted seeds replay green. Run-01's "unreachable block survived" failure was a harness defect in the CFG builder, fixed before run-02; the current property passes with the seeds from that failure replayed.
- Tier: 2
- Rationale: cfg_simplify's seven sub-passes (constant-branch folding, jump
  threading, dead-block removal, trivial-phi elimination, block merging) must
  preserve the interpreted value on random CFGs with phis and constant conditions,
  and must leave every surviving block reachable from entry (documented "BFS from
  the entry block to compute reachability"), with all branch targets in range.
- Doc contract: src/passes/cfg_simplify.rs:50 "Simplify the CFG of a single function. Iterates until no more simplifications are possible (fixpoint)." — asserted fingerprint fc35b96d
- Seed: src/ir/analysis.rs:389 func_from_descriptors (round-06 random-CFG builder, adapted with phi/param conditioning)
- Formal: ∀ G ∈ random CFGs (n blocks, param-conditioned CondBranch, Const conds, Switch, phis over entry values): interp(simplify_cfg(G)) = interp(G) ∧ reachable_from_entry(all blocks) ∧ ∀ targets t: t < |blocks|.
- Test file: src/passes/cfg_simplify.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: passes::cfg_simplify::simplify_cfg
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [cfg, args]
  domain: { cfg: random_cfg, args: int_samples }
  relation:
    op: holds
    body: interp(simplify_cfg(cfg), args) == interp(cfg, args) && all_blocks_reachable_from_entry && targets_in_range
generators:
  cfg: { gen: random_cfg, maxBlocks: 6 }
  args: { gen: list, elem: { gen: int, min: -2147483648, max: 2147483647, type: i32 }, maxLen: 2 }
evidence: src/passes/README.md:283 "Simplifies the control flow graph through seven sub-passes that run to a fixpoint"; README "Remove dead blocks. BFS from the entry block"
```

## p13_narrow_preservation
- Tier: 2
- Rationale: narrow rewrites widen-operate-narrow chains (Cast ≤32→64, BinOp at 64,
  Cast 64→32; phase-2 bitwise without narrowing cast; phase-3 comparisons) to operate
  at the narrow type; value preservation checked at full interpreter precision, for
  the ops README whitelists (Add, Sub, Mul, And, Or, Xor, Shl + matching-sign shifts).
- Doc contract: src/passes/narrow.rs:56 "Narrow operations in a single function." — asserted fingerprint 5c873894 (thin fn doc; the operative contract is README "Integer Narrowing" pattern table, quoted under evidence)
- Seed: (none)
- Formal: ∀ pat ∈ widen-operate-narrow chains (op ∈ whitelist, operand widths mixed): interp(narrow_function(f[pat])) = interp(f[pat]).
- Test file: src/passes/narrow.rs
- Status: failing
- Counterexample: shape=1, op_idx=16 → f(a: i32, b: i32) -> i64 { Or(sext(a), sext(b)) at I64 }. narrow_function (Phase 2) rewrites the I64 Or to I32 with no re-extension while the single use (the I64 return) needs the full value; f(1, -1) then returns 4294967295 (zero-extended) instead of -1. Pipeline-confirmed: run_passes(opt=2, X86_64) leaves Or@I32 feeding the I64 Return; interp after = Value(4294967295) vs expected Value(-1). README Phase 2 documents "restricted to I32/U32 output" — the code checks only target_ty, never the use. Deterministic red witness: test_narrow_regression_phase2_i64_bitwise_i64_use.
- Re-verified: RUST_TEST_THREADS=1 cargo test --lib -- passes::narrow::pbt_tests::p13_narrow_preservation --exact → FAILED serially (left Value(4294967295) / right Value(-1)); run_passes probe re-confirms the pipeline-level miscompile. One generator defect fixed en route (unsigned cmps on the signed I32 source were wrongly demanded rewritten — README Phase 3 documents they require an unsigned source; now the negative side is asserted too, n == 0).
- Bug report: bug_reports/passes_narrow_phase2_i64_use_miscompile.md


```property
function: passes::narrow::narrow_function
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [pattern, args]
  domain: { pattern: widen_operate_narrow, args: int_samples }
  relation:
    op: eq
    lhs: interp(narrow_function(f[pattern]), args)
    rhs: interp(f[pattern], args)
generators:
  pattern: { gen: widen_operate_narrow, maxLen: 4 }
  args: { gen: list, elem: { gen: int, min: -2147483648, max: 2147483647, type: i32 }, maxLen: 2 }
evidence: src/passes/README.md:388-410 narrow pattern table (widen-operate-narrow, phase 2 bitwise, phase 3 comparisons)
```

## p14_pipeline_value_preservation
- Tier: 1
- Rationale: the whole pipeline (run_passes: inline → cleanup → 3-iteration main
  loop with all passes incl. GVN/LICM/IVSR shared analysis, IPCP, dead statics)
  must preserve the interpreted value on every generated shape (straight-line,
  diamond, structured bounded loop reaching LICM/IVSR/loop_analysis, random CFG,
  div-by-const cases) and never panic. This is the campaign's integration/state
  property: interleaved pass rewrites on one IR object.
- Doc contract: src/passes/mod.rs:255 "1. Every test run exercises every optimization pass." — asserted fingerprint e40383fa (design rationale: all passes always run)
- Seed: (none)
- Formal: ∀ shape ∈ generated function shapes, ∀ args: interp(run_passes(shape), args) = interp(shape, args); no panic.
- Test file: src/passes/mod.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: passes::run_passes
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [shape, args]
  domain: { shape: fn_shapes, args: int_samples }
  relation:
    op: eq
    lhs: interp(run_passes(shape), args)
    rhs: interp(shape, args)
generators:
  shape: { gen: oneof, variants: [safe_expr_dag, gen_diamond, loop_func, random_cfg, div_case] }
  args: { gen: list, elem: { gen: int, min: -2147483648, max: 2147483647, type: i32 }, maxLen: 2 }
evidence: src/passes/README.md:28 "The optimizer executes in four sequential phases"
```

## Change surface re-verification (covered-by-round-06)

The 5 change-surface functions are round-06's own committed test code in `src/ir`
(`func_from_descriptors`, `p5_unsigned_repr_writer_differential`, `build_random_func`
×2, `p1_div_rem_c11_identity`) — outside this round's scope and not SUT code. They
are re-executed verbatim via `cargo test --lib ir::` as part of this campaign (see
COVERAGE.md rows); `p5_unsigned_repr_writer_differential` is round-06's intentionally
red witness for issue #575 and is expected to fail.
