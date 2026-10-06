# PROPERTIES.md — round 08 (src/backend root)

All properties exercise the REAL public functions on inputs built with the
repo's own types (`IrType`, `IrFunction` via the `#[cfg(test)]` builders from
rounds 06–07 where reused), and check them against INDEPENDENT oracles: a
naive set-based liveness dataflow written from the textbook equations quoted
in the code's own doc comment, an independent expression evaluator, and the
README's classification tables.

## p1_cast_noop_laws
- Tier: 2
- Rationale: README cast table pins the Noop rows: same type, and Ptr to
  I64/U64 pointer normalization. The cheapest table laws, checked exhaustively
  over the whole type grid (finite domain → fixed sampling is the full domain).
- Doc contract: src/backend/cast.rs:67 (classify_cast_with_f128) — module doc "After pointer normalization (Ptr treated as U64)... Noop -- no conversion needed (same type, or Ptr to I64/U64)." (README:649-651)
- Formal: ∀ from,to ∈ IrType grid (×2 f128_is_native): from==to ⇒ Noop; (from=Ptr ∧ to∈{I64,U64}) ⇒ Noop.
- Test file: src/backend/cast.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: backend::cast::classify_cast_with_f128
oracle: reference
predicate:
  quantifier: forall
  vars: [from_ty, to_ty, f128_is_native]
  domain: { from_ty: int_type_grid, to_ty: int_type_grid, f128_is_native: bool }
  relation:
    op: holds
    expr: "(from_ty == to_ty || (from_ty == Ptr && (to_ty == I64 || to_ty == U64))) ? classify_cast_with_f128(from_ty, to_ty, f128_is_native) == CastKind::Noop : true"
generators:
  from_ty: { gen: int, min: 0, max: 13, type: u8 }
  to_ty: { gen: int, min: 0, max: 13, type: u8 }
  f128_is_native: { gen: bool }
evidence: src/backend/README.md cast table "Noop -- no conversion needed (same type, or Ptr to I64/U64)"
```

## p2_cast_directional_laws
- Tier: 2
- Rationale: width/float-directional laws from an independent type model
  (sizes + signedness + floatness, not the SUT's classifier): IntWiden only
  for widening integer pairs, IntNarrow only narrowing, FloatToFloat only
  F32↔F64, same-size reinterpret kinds only at equal sizes, cross-family kinds
  (FloatToSigned etc.) only when exactly one side is float.
- Doc contract: README:651-664 cast table (IntWiden/IntNarrow/SignedToUnsignedSameSize/...)
- Formal: ∀ from,to ∈ IrType grid, ∀ native ∈ {false,true}: kind = classify(from,to,native) ⇒ directionConsistent(kind, model(from), model(to)).
- Test file: src/backend/cast.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: backend::cast::classify_cast_with_f128
oracle: reference
predicate:
  quantifier: forall
  vars: [from_ty, to_ty, f128_is_native]
  domain: { from_ty: int_type_grid, to_ty: int_type_grid, f128_is_native: bool }
  relation:
    op: holds
    expr: "direction_consistent(classify_cast_with_f128(from_ty, to_ty, f128_is_native), type_model(from_ty), type_model(to_ty))"
generators:
  from_ty: { gen: int, min: 0, max: 13, type: u8 }
  to_ty: { gen: int, min: 0, max: 13, type: u8 }
  f128_is_native: { gen: bool }
evidence: src/backend/README.md cast table (IntWiden/IntNarrow/FloatToFloat/same-size kinds)
```

## p3_cast_f128_flag_independence
- Tier: 2
- Rationale: the f128_is_native parameter distinguishes only F128 handling
  (README: "The f128_is_native parameter distinguishes ARM/RISC-V ... from
  x86"); for type pairs not involving F128 the classification must be
  identical under both flags.
- Doc contract: README:664-666 "The f128_is_native parameter distinguishes ARM/RISC-V (IEEE binary128...) from x86 (x87 80-bit...)"
- Formal: ∀ from,to ∈ IrType grid \ {F128 on either side}: classify(from,to,false) == classify(from,to,true).
- Test file: src/backend/cast.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: backend::cast::classify_cast_with_f128
oracle: differential
predicate:
  quantifier: forall
  vars: [from_ty, to_ty]
  domain: { from_ty: int_type_grid_no_f128, to_ty: int_type_grid_no_f128 }
  relation:
    op: eq
    lhs: classify_cast_with_f128(from_ty, to_ty, false)
    rhs: classify_cast_with_f128(from_ty, to_ty, true)
generators:
  from_ty: { gen: int, min: 0, max: 13, type: u8 }
  to_ty: { gen: int, min: 0, max: 13, type: u8 }
evidence: src/backend/README.md "The f128_is_native parameter distinguishes ARM/RISC-V ... from x86"
```

## p4_callabi_caller_callee_agreement
- Re-verified: harness config pairing fixed (classify_params_full encodes variadicity in config.variadic_floats_in_gp per its own comment — non-variadic pairs with false); after the fix the two entry points agree on all 512 cases × 4 configs.
- Tier: 2
- Rationale: README's core call-ABI contract: "callers and callees must agree
  exactly on where each argument lives... implemented once in
  classify_args_core and wrapped by two thin entry points". The two entry
  points on the same argument list must agree on location (register index /
  stack-ness) for every argument, for every documented CallAbiConfig (x86,
  ARM, RISC-V shapes).
- Doc contract: src/backend/call_abi.rs README:589-594 "The core insight is that callers and callees must agree exactly on where each argument lives (which register or which stack offset), so the classification algorithm is implemented once"
- Formal: ∀ config ∈ {x86,arm,riscv,i686-ish configs}, ∀ arg type lists (scalars incl. i128/f128/f32/f64, edge lengths 0..12): locations(classify_call_args(args)) == locations(classify_params_full(func_with_those_params)) where locations maps to (GP reg idx | FP reg idx | pair base | stack).
- Test file: src/backend/call_abi.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: backend::call_abi::classify_call_args
oracle: differential
predicate:
  quantifier: forall
  vars: [config, arg_types]
  domain: { config: {x86,arm,riscv,i686_regparm3}, arg_types: list(int_type_grid, 0..=12) }
  relation:
    op: holds
    expr: "locations(classify_call_args(args_of(arg_types), config)) == locations(classify_params_full(func_with_params(arg_types), config))"
generators:
  config: { gen: int, min: 0, max: 3, type: u8 }
  arg_types: { gen: list, elem: { gen: int, min: 0, max: 13, type: u8 }, minLen: 0, maxLen: 12 }
evidence: src/backend/README.md call-ABI section "callers and callees must agree exactly on where each argument lives"
```

## p5_callabi_capacity_and_skip_bookkeeping
- Tier: 2
- Rationale: the category table's consumption rules: register indices stay
  below the config's maxima, zero-size structs consume nothing (README:
  "Zero-size struct, consumes nothing" — GCC behavior), and stack overflow
  ordering is sequential.
- Doc contract: README call-ABI table: "ZeroSizeSkip | Zero-size struct, consumes nothing"; config docs "x86: 6 GP + 8 FP; ARM: 8 GP + 8 FP; RISC-V: 8 GP + 8 FP; i686: 0 GP by default"
- Formal: ∀ config, ∀ type lists: ∀ IntReg{reg_idx}: reg_idx < max_int_regs; ∀ FloatReg{reg_idx}: reg_idx < max_float_regs; ∀ GP consumers in order: strictly increasing consumption; classify(list ++ [zero-struct; k]) == classify(list ++ [zero-struct; k'] ++ rest) in consumption terms (a zero-size struct never shifts later args).
- Test file: src/backend/call_abi.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: backend::call_abi::classify_call_args
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [config, arg_types, n_zero_extra]
  domain: { config: {x86,arm,riscv,i686_regparm3}, arg_types: list(int_type_grid, 1..=10), n_zero_extra: 0..=3 }
  relation:
    op: holds
    expr: "capacity_ok(classify_call_args(args, config), config) && skip_ok(classify_call_args(with_zero_structs(args, n_zero_extra), config), classify_call_args(args, config))"
generators:
  config: { gen: int, min: 0, max: 3, type: u8 }
  arg_types: { gen: list, elem: { gen: int, min: 0, max: 13, type: u8 }, minLen: 1, maxLen: 10 }
  n_zero_extra: { gen: int, min: 0, max: 3, type: u8 }
evidence: src/backend/README.md category table (register maxima per config; "ZeroSizeSkip ... consumes nothing")
```

## p6_liveness_matches_textbook_dataflow
- Design caveat found: LivenessResult.block_loop_depth is documented "for each block" but the num_values==0 early return yields the empty triple; sole in-tree consumer (regalloc.rs:120) iterates, so caller-safe today — recorded, not filed.
- Re-verified: two oracle-side (naive) fixes en route (MAX-end fallback to start; comparison normalized by value_id) — the SUT agreed exactly on all 512 cases after the mirror was correct.
- Tier: 2
- Rationale: compute_live_intervals' own doc quotes the equations; a naive
  set-based backward dataflow (independent implementation in the test) must
  agree: every value live-in at a block has its interval extended to cover
  that block (documented rule), every non-alloca def has an interval starting
  at its def point, intervals cover last uses, call_points are exactly the
  Call/CallIndirect program points.
- Doc contract: src/backend/liveness.rs:150-156 "live_in[B] = gen[B] ∪ (live_out[B] - kill[B]) - live_out[B] = ∪ live_in[S] for all successors S of B" + "Values that are live-in to a block have their interval extended to cover from the block's start point through the entire block."
- Formal: ∀ F ∈ random CFGs (params, phis, branches, loops): naive_live(F) ⊆ coverage(intervals(F)) per value per block; def/start agreement; call_points(F) = points of call instructions.
- Test file: src/backend/liveness.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: backend::liveness::compute_live_intervals
oracle: reference
predicate:
  quantifier: forall
  vars: [cfg]
  domain: { cfg: random_cfg_with_phis (blocks 1..=7, params, branches, loops, calls) }
  relation:
    op: holds
    expr: "naive_dataflow_covers(cfg, compute_live_intervals(cfg)) && call_points_exact(cfg, compute_live_intervals(cfg)) && def_start_agrees(cfg, compute_live_intervals(cfg))"
generators:
  cfg: { gen: cfg_spec, blocks: int(1..=7), terms: list(tuple(u8,u32,u32,i64,i64), 2..=6), steps: list(list(tuple(u8,u8,i64), 0..=2), 0..=7) }
evidence: src/backend/liveness.rs:150-156 doc comment quoting the dataflow equations
```

## p7_regalloc_soundness
- Tier: 2
- Rationale: linear-scan contract: no two values assigned the same register
  have overlapping intervals; values whose interval spans a call point are
  assigned callee-saved registers only (Phase 1 vs Phase 2); assignments
  respect the eligibility whitelist (defs of eligible kinds, GPR types).
- Doc contract: src/backend/README.md:467-481 (three-phase allocation; "Values whose live ranges span function calls are assigned callee-saved registers"; eligibility whitelist) 
- Formal: ∀ F ∈ random CFGs, ∀ config (callee+caller pools): ∀ v,w assigned same reg: intervals disjoint; ∀ v assigned reg with interval spanning a call point: reg ∈ callee-saved pool; ∀ v assigned: def(v) is whitelist-eligible.
- Test file: src/backend/regalloc.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: backend::regalloc::allocate_registers
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [cfg, pool_shape]
  domain: { cfg: random_cfg_with_phis (as p6), pool_shape: {x86_like, arm_like, riscv_like, empty} }
  relation:
    op: holds
    expr: "disjoint_per_register(assignments, intervals) && call_spanning_in_callee_saved(assignments, intervals, call_points, pools) && eligible_only(assignments, cfg)"
generators:
  cfg: { gen: cfg_spec, blocks: int(1..=7), terms: list(tuple(u8,u32,u32,i64,i64), 2..=6), steps: list(list(tuple(u8,u8,i64), 0..=2), 0..=7) }
  pool_shape: { gen: int, min: 0, max: 3, type: u8 }
evidence: src/backend/README.md:467-525 three-phase linear scan + eligibility whitelist
```

## p8_asmexpr_parse_matches_evaluator
- Status note: PASSING on the defined-evaluation domain (bounded magnitudes, masked shifts, logical >> per GNU as/llvm-mc reference — verified against both assemblers). The property's FIRST run caught the arithmetic-panic bug below on unbounded inputs; the domain was then bounded per the documented evaluation semantics and the panic contract moved to the deterministic red regression.
- Bug (from this property's initial run): parse_integer_expr debug-panics on mul overflow / shift >= 64 instead of Err → issue #765 (bug_reports/backend_asmexpr_arith_panic.md). Red witness: test_asmexpr_regression_arith_panic_not_err.
- Re-verified: RUST_TEST_THREADS=1 cargo test --lib -- backend::asm_expr::pbt_tests2::p8_asmexpr_parse_matches_evaluator --exact → ok (512 cases); oracle's >> fixed to LOGICAL after adversarial verification against GNU as (ffffffff...) and llvm-mc — both evaluate (-1)>>1 unsigned; the arithmetic-shift oracle was my model bug, not a SUT defect.
- Tier: 2
- Rationale: parse_integer_expr is the semantic core of asm preprocessing
  (integer constant expressions in .byte/.word etc.); a generated expression
  tree rendered to a string must parse to the tree's independently computed
  value (differential; generator biased to precedence traps: a+(b*c),
  (a<<b)-c, unary minus, hex literals, parens).
- Doc contract: src/backend/asm_expr.rs:308 parse_integer_expr — existing unit tests sample single forms; the property generalizes to random trees.
- Formal: ∀ T ∈ random int expression trees (depth ≤4, + - * << >> & | ^ unary-, hex/dec literals): parse_integer_expr(render(T)) == Ok(eval(T)).
- Test file: src/backend/asm_expr.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: backend::asm_expr::parse_integer_expr
oracle: differential
predicate:
  quantifier: forall
  vars: [tree]
  domain: { tree: random_int_expr_tree (depth <=4, + - * << >> & | ^, unary minus, hex/dec literals) }
  relation:
    op: eq
    lhs: parse_integer_expr(render(tree))
    rhs: Ok(eval(tree))
generators:
  tree: { gen: list, elem: { gen: tuple, elems: [{ gen: int, min: 0, max: 9, type: u8 }, { gen: int, min: -32768, max: 32767, type: i64 }] }, minLen: 1, maxLen: 12 }
evidence: src/backend/asm_expr.rs:308 parse_integer_expr (existing unit tests generalize)
```
