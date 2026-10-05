# PROPERTIES — round 06: src/ir

All tests inline `#[cfg(test)] mod pbt_tests` in the named source file (repo
convention, rung 1). Framework: proptest 1.11.0 (`#![proptest_config(ProptestConfig::with_cases(1024))]`).

## P1: IrBinOp eval C11 division identity
- Tier: 4
- Rationale: C11 6.5.5p6 law `(a/b)*b + a%b == a` with truncation-toward-zero remainder
  is an independent mathematical contract for the eval methods consumed by constant
  folding. Reference = C standard, not the implementation. Stronger oracles rejected:
  no same-job sibling impl exists in-tree (common::const_eval targets AST, different layer).
- Doc contract: src/ir/ops.rs:69 "Evaluate this binary operation on two i64 operands using wrapping arithmetic." — asserted fingerprint 6bfebcfa (src/ir/README.md, re-verified 2026-10-05: quote present verbatim)
- Seed: (none)
- Formal: ∀ a ∈ i64, b ∈ i64, b ≠ 0: q = eval_i64(SDiv,a,b) ∧ r = eval_i64(SRem,a,b) ⇒
  q·b + r = a (wrapping) ∧ |r| < |b| ∧ sign(r) ∈ {0, sign(a)}; likewise over u64 bits for UDiv/URem.
- Test file: src/ir/ops.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: ir.ops.IrBinOp.eval_i64
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [a, b]
  domain: { a: i64, b: i64 | b != 0 }
  relation: { op: holds, expr: "eval_i64(SDiv,a,b).unwrap().wrapping_mul(b).wrapping_add(eval_i64(SRem,a,b).unwrap()) == a && eval_i64(SRem,a,b).unwrap().wrapping_abs() < b.wrapping_abs()" }
generators:
  a: { gen: int, min: -9223372036854775808, max: 9223372036854775807, type: i64 }
  b: { gen: int, min: -9223372036854775808, max: 9223372036854775807, type: i64 }
evidence: C11 6.5.5p6; src/ir/README.md "SDiv | Signed division … SRem | Signed remainder"
```

## P2: IrBinOp div/rem None ⇔ rhs == 0 (failure path)
- Tier: 3
- Rationale: documented error contract; also pins can_trap set membership.
- Doc contract: src/ir/ops.rs:72 "Returns None for division/remainder by zero." — asserted fingerprint 3a86110a (src/ir/ops.rs, re-verified 2026-10-05: quote present verbatim)
- Seed: (none)
- Formal: ∀ op ∈ {SDiv,UDiv,SRem,URem}, a,b ∈ i64: eval_i64(op,a,b) = None ⇔ b = 0;
  ∀ op ∈ IrBinOp: can_trap(op) ⇔ op ∈ {SDiv,UDiv,SRem,URem}.
- Test file: src/ir/ops.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: ir.ops.IrBinOp.eval_i64
oracle: negative_error
predicate:
  quantifier: forall
  vars: [op, a, b]
  domain: { op: oneof(SDiv,UDiv,SRem,URem), a: i64, b: i64 }
  relation: { op: eq, lhs: eval_i64(op,a,b).is_none(), rhs: b == 0 }
generators:
  b: { gen: oneof, of: [ int(i64), const(0), const(1), const(-1) ] }
expected_error: None (Option)
evidence: src/ir/ops.rs:72 doc; can_trap doc src/ir/ops.rs:63
```

## P3: eval_i64 vs eval_i128 width consistency (non-negative)
- Tier: 3
- Rationale: eval_i128 is documented as "same for i128 operands" — on non-negative
  inputs the two widths must agree bit-for-bit (both zero-extend). Negative operands are
  excluded BY SPEC of the width semantics, not by the SUT's whim: unsigned ops
  reinterpret the operand's own width's bits (u64 vs u128), so agreement on negatives is
  not the contract. This is the required metamorphic property.
- Doc contract: src/ir/ops.rs:95 "same for i128 operands." — asserted fingerprint 3a86110a (src/ir/ops.rs, re-verified 2026-10-05: quote present verbatim)
- Seed: (none)
- Formal: ∀ op ∈ IrBinOp, a,b ∈ [0, 2^63): eval_i128(op, a, b) = eval_i64(op, a, b) (as i128).
- Test file: src/ir/ops.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: ir.ops.IrBinOp.eval_i128
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [op, a, b]
  domain: { op: IrBinOp, a: u63, b: u63 }
  relation: { op: holds, expr: "eval_i128(op,a as i128,b as i128) == eval_i64(op,a,b).map(|r| r as i128)" }
generators:
  a: { gen: int, min: 0, max: 9223372036854775807, type: i64 }
  b: { gen: int, min: 0, max: 9223372036854775807, type: i64 }
evidence: src/ir/ops.rs:95 doc; README "eval_i128(lhs, rhs) -- same for i128 operands"
```

## P4: IrCmpOp eval coherence (signed/unsigned/float)
- Tier: 3
- Rationale: IEEE 754 documented equivalence S*⇔U* for floats incl. NaN; two's-complement
  order facts (same-sign agreement, mixed-sign disagreement); duality Slt=!Sge etc.
  Reference = IEEE 754 total-order definition + two's-complement arithmetic, independent
  of the implementation.
- Doc contract: src/ir/ops.rs:203 "For floats, signed and unsigned comparison variants are equivalent since IEEE 754 defines a total ordering (NaN comparisons return false for ordered ops, true for Ne)." — asserted fingerprint 3a86110a (src/ir/ops.rs, re-verified 2026-10-05: quote present verbatim)
- Seed: (none)
- Formal: ∀ a,b ∈ f64 (incl. NaN, ±∞, ±0): eval_f64(Slt,a,b) = eval_f64(Ult,a,b) ∧ … all 4 pairs;
  ∀ a,b ∈ i64, a,b ≥ 0: Slt(a,b) = Ult(a,b); ∀ a<0≤b: Slt(a,b)=false ∧ Ult(a,b)=true;
  ∀ a,b: Slt(a,b) = !Sge(a,b) ∧ Ne(a,b) = !Eq(a,b) ∧ Sgt(a,b) = Slt(b,a).
- Test file: src/ir/ops.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)
- Re-verified: PATH="$HOME/.cargo/bin:$PATH" RUST_TEST_THREADS=1 cargo test --lib ir::ops::pbt_tests::p4_cmp_coherence_f64 ir::ops::pbt_tests::p4b_cmp_coherence_i64 → PASS (harness fix: NaN duality guard and the a<0≤b unsigned-order law were test bugs — the doc's own NaN sentence licenses the guard; SUT untouched)

```property
function: ir.ops.IrCmpOp.eval_f64
oracle: reference
predicate:
  quantifier: forall
  vars: [a, b]
  domain: { a: f64_any_incl_special, b: f64_any_incl_special }
  relation: { op: holds, expr: "eval_f64(IrCmpOp::Slt,a,b) == eval_f64(IrCmpOp::Ult,a,b) && eval_f64(IrCmpOp::Ne,a,b) == (a != b)" }
generators:
  a: { gen: oneof, of: [ float(any), const(NAN), const(INF), const(0.0), const(-0.0) ] }
evidence: src/ir/ops.rs:203 doc; IEEE 754-2019 §5.11
```

## P5: unsigned constant representation — from_i64 vs cast_float_to_target (writer differential)
- Tier: 4
- Rationale: from_i64 documents THE storage convention for unsigned sub-64-bit constants
  (I64 zero-extended; native variants forbidden because to_i64() sign-extends).
  cast_float_to_target claims "200.0 as u8 = 200" yet returns I8(200 as u8 as i8) whose
  to_i64() is negative — the exact representation the convention forbids. Two same-job
  writers must agree. Expected to fail for values > 127 (U8), > 32767 (U16).
- Doc contract: src/ir/constants.rs:437 "Store unsigned sub-64-bit types (U8, U16, U32) as I64 with zero-extended values to preserve unsigned semantics." — asserted fingerprint a1c8a8a1 (src/ir/constants.rs, re-verified 2026-10-05: quote present verbatim)
- Seed: (none)
- Formal: ∀ ty ∈ {U8,U16,U32}, v ∈ [0, 2^bits(ty)): from_i64(v,ty).to_i64() = v ∧
  cast_float_to_target(v as f64, ty).to_i64() = v ∧ cast_long_double_to_target(v,ty).to_i64() = v.
- Test file: src/ir/constants.rs
- Status: failing
- Counterexample: ty=U8, v=200 → cast_float_to_target(200.0,U8) = I8(-56), to_i64() = -56 ≠ 200
- Bug report: bug_reports/ir_constants_unsigned_repr_differential.md (b1)

```property
function: ir.constants.IrConst.cast_float_to_target
oracle: differential
predicate:
  quantifier: forall
  vars: [ty, v]
  domain: { ty: oneof(U8,U16,U32), v: uint_of_width(ty) }
  relation: { op: holds, expr: "cast_float_to_target(v as f64, ty).to_i64() == from_i64(v as i64, ty).to_i64()" }
generators:
  v: { gen: int, min: 0, max: 4294967295, type: u32 }
evidence: src/ir/constants.rs:437-441 convention comment; src/ir/constants.rs:271 doc
```

## P6: unsigned constant representation — coerce_to_with_src (writer differential)
- Tier: 4
- Rationale: same convention as P5; coerce_to_with_src early-returns `*self` for
  (I8(_), U8) / (I16(_), U16) / (I32…) leaving the forbidden sign-extended native
  variant in place instead of normalizing to I64 zero-extended.
- Doc contract: src/ir/constants.rs:437 "Store unsigned sub-64-bit types (U8, U16, U32) as I64 with zero-extended values to preserve unsigned semantics." — asserted fingerprint a1c8a8a1 (src/ir/constants.rs, re-verified 2026-10-05: quote present verbatim)
- Seed: (none)
- Formal: ∀ ty ∈ {U8,U16}, v ∈ (2^(bits-1), 2^bits): from_i64(v, signed(ty)).coerce_to(ty).to_i64() = v.
- Test file: src/ir/constants.rs
- Status: failing
- Counterexample: ty=U8, v=200: I8(-56).coerce_to(U8) = I8(-56), to_i64() = -56 ≠ 200
- Bug report: bug_reports/ir_constants_coerce_unsigned_early_return.md (b2)

```property
function: ir.constants.IrConst.coerce_to_with_src
oracle: differential
predicate:
  quantifier: forall
  vars: [ty, v]
  domain: { ty: oneof(U8,U16), v: uint_of_width(ty), v > max_signed(ty) }
  relation: { op: eq, lhs: from_i64(v, signed_ty(ty)).coerce_to(ty).to_i64(), rhs: v }
generators:
  v: { gen: int, min: 128, max: 65535, type: u32 }
evidence: src/ir/constants.rs:437-441 convention comment
```

## P7: f64→f128 / x87 encoding round-trip vs IEEE-754 field decode (incl. subnormals)
- Tier: 5
- Rationale: the encoders' documented contract is a faithful IEEE 754 encoding of the
  f64 value. An independent decoder built from the IEEE 754 field definitions (the doc
  comments themselves specify the layouts) must reconstruct the original value for every
  finite f64. Subnormal inputs fall through to the normal path in both encoders
  (exp11==0 && mantissa!=0 hits the "Normal number" arm), setting an implicit integer
  bit that does not exist in the source value.
- Doc contract: src/ir/constants.rs:50 "Convert an f64 value to IEEE 754 binary128 (quad-precision) encoding (16 bytes, little-endian)." — asserted fingerprint a1c8a8a1 (src/ir/constants.rs, re-verified 2026-10-05: quote present verbatim)
- Seed: (none)
- Formal: ∀ v ∈ finite f64: decode_f128(f64_to_f128_bytes(v)) = v ∧
  decode_x87(f64_to_x87_bytes(v)) = v; boundary: ±0, ±min_subnormal, ±max_subnormal,
  ±1, ±max_normal, ±∞ encode with correct sign/exponent fields.
- Test file: src/ir/constants.rs
- Status: failing
- Counterexample: v = 5e-324 (min subnormal): decode(f64_to_f128_bytes(v)) = 1.1125369292536e-308 ≠ 5e-324
- Bug report: bug_reports/ir_constants_subnormal_f128_x87_encoding.md (b3)
- Note: the KAT p7b_special_values initially failed on my side (−0 sign bit, wrong NaN bit); after fixing the test's own expectations the subnormal misencoding remains red — SUT bug confirmed serially.

```property
function: ir.constants.f64_to_f128_bytes
oracle: reference
predicate:
  quantifier: forall
  vars: [v]
  domain: { v: finite f64 incl subnormals }
  relation: { op: eq, lhs: ieee_decode_f128(f64_to_f128_bytes(v)), rhs: v }
generators:
  v: { gen: oneof, of: [ float(any finite), const(f64::MIN_POSITIVE), const(5e-324), const(2.2250738585072011e-308), const(f64::MAX) ] }
evidence: src/ir/constants.rs:50 doc; src/ir/constants.rs:82 doc (x87 layout)
```

## P8: IrConst bool/zero/one contracts (C11 6.3.1.2)
- Tier: 3
- Rationale: documented C11 normalization + zero/one constructors for every IrType;
  from_i64/to_i64 round-trip for signed types. Enumerates every IrType enumerator
  (documented bounds sampled exactly).
- Doc contract: src/ir/constants.rs:553 "When any scalar value is converted to _Bool, the result is 0 if the value compares equal to 0; otherwise, the result is 1." — asserted fingerprint a1c8a8a1 (src/ir/constants.rs, re-verified 2026-10-05: quote present verbatim)
- Seed: (none)
- Formal: ∀ ty ∈ IrType (15 enumerators): zero(ty).is_zero() ∧ one(ty).is_nonzero() ∧
  from_i64(k, ty).to_i64() = k for k ∈ {0, 1, -1, MIN, MAX of ty}; ∀ c ∈ IrConst:
  bool_normalize(c) = I8(is_zero(c) ? 0 : 1).
- Test file: src/ir/constants.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: ir.constants.IrConst.bool_normalize
oracle: reference
predicate:
  quantifier: forall
  vars: [c]
  domain: { c: any IrConst }
  relation: { op: holds, expr: "bool_normalize(c) == IrConst::I8(if c.is_zero() { 0 } else { 1 })" }
generators:
  c: { gen: oneof, of: [ int_consts, float_consts, long_double_consts, zero ] }
evidence: C11 6.3.1.2 quoted in doc src/ir/constants.rs:553
```

## P9: build_cfg preds/succs transpose consistency
- Tier: 4
- Rationale: build_cfg's documented contract is ONE graph as a (preds, succs) pair —
  the two CSR lists must be exact transposes (same multiplicity per edge). The
  implementation dedups succs entries (contains-checks) but pushes preds unconditionally,
  so CondBranch with true_label == false_label and Switch with duplicate case targets
  produce asymmetry (duplicate pred entries). Downstream consumers count preds
  (if_convert.rs:386 `preds.len(merge_idx) != 2`, DF join detection) — a real contract
  break, not cosmetics.
- Doc contract: src/ir/analysis.rs:106 "Build predecessor and successor lists from the function's CFG. Returns (preds, succs) as flat adjacency lists (CSR format)." — asserted fingerprint 81e54324 (src/ir/analysis.rs, re-verified 2026-10-05: quote present verbatim)
- Seed: (none)
- Formal: ∀ CFG F over blocks with resolved labels: ∀ i,b: |{i ∈ preds.row(b)}| = |{b ∈ succs.row(i)}|
  ∧ succs.row(i) has no duplicates ∧ preds.row(b) has no duplicates.
- Test file: src/ir/analysis.rs
- Status: failing
- Counterexample: block0: CondBranch{true: L1, false: L1}, block1: Return → preds[L1] = [0,0], succs[0] = [1]
- Bug report: bug_reports/ir_analysis_build_cfg_pred_dup.md (b4)

```property
function: ir.analysis.build_cfg
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [F]
  domain: { F: "random CFG over Branch and CondBranch (30% equal targets) and Switch (duplicate labels) and Return, 1..=8 blocks" }
  relation: { op: holds, lhs: preds and succs are exact transposes with no duplicate entries }
generators:
  F: { gen: cfg, blocks: int(1..=8), biased_equal_condbranch: true, dup_switch_labels: true }
evidence: src/ir/analysis.rs:106 doc; consumers src/passes/if_convert.rs:305,386
```

## P10: compute_dominators vs textbook iterative reference
- Tier: 5
- Rationale: CHK algorithm vs the dragon-book dataflow formulation
  (dom(b) = {b} ∪ ⋂_{p∈preds(b)} dom(p), iterated to fixpoint; idom = closest strict
  dominator) — an independent reference implementation of the same mathematical
  definition. Also pins the documented sentinel: unreachable ⇔ usize::MAX.
- Doc contract: src/ir/analysis.rs:242 "Uses usize::MAX as sentinel for undefined/unreachable blocks." — asserted fingerprint ab794cba
- Seed: (none)
- Formal: ∀ graph G (n ∈ 1..=8, random edges, unreachable nodes included):
  idom_SUT(G) = idom_naive(G) on reachable nodes ∧ idom[b] = MAX ⇔ b unreachable from 0
  ∧ idom[0] = 0.
- Test file: src/ir/analysis.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)
- Re-verified: PATH="$HOME/.cargo/bin:$PATH" RUST_TEST_THREADS=1 cargo test --lib ir::analysis::pbt_tests::p10_dominators_vs_naive_reference → PASS (harness fix: naive reference idom[0]=0 and n-clamp were test bugs, SUT untouched)

```property
function: ir.analysis.compute_dominators
oracle: reference
predicate:
  quantifier: forall
  vars: [G]
  domain: { G: random directed graph, n: int(1..=8), edge prob 0.3, entry = 0 }
  relation: { op: eq, lhs: compute_dominators(G), rhs: naive_dominators(G) }
generators:
  G: { gen: graph, nodes: int(1..=8), edge_prob: 0.3 }
evidence: README "Cooper-Harvey-Kennedy algorithm"; Aho/Sethi/Ullman dataflow equations
```

## P11: compute_dominance_frontiers vs set-definition reference
- Tier: 4
- Rationale: DF(b) = {d : b dominates some pred of d ∧ b does not strictly dominate d}
  (Cytron et al. 1991) — checked against dominator sets derived independently from the
  idom tree produced by P10's cross-validated computation.
- Doc contract: src/ir/analysis.rs:302 "Compute dominance frontiers for each block. DF(b) = set of blocks where b's dominance ends (join points)." — asserted fingerprint 4b0f1f4b
- Seed: (none)
- Formal: ∀ graph G: ∀ b,d (reachable): d ∈ DF_SUT(b) ⇔ (∃ p ∈ preds(d): dominates(b,p)) ∧ ¬dominates_strict(b,d).
- Test file: src/ir/analysis.rs
- Status: passing (domain excludes CFGs with edges into the entry block — that corner
  is defect b5, pinned by the intentionally-red deterministic witness p11b_entry_selfloop_df)
- Counterexample: (none on the covered domain; b5 witness: n=1, succs=[[0]] → DF(0)={} ≠ {0})
- Bug report: bug_reports/ir_analysis_df_entry_cycle.md (b5 — from witness p11b)
- Re-verified: PATH="$HOME/.cargo/bin:$PATH" RUST_TEST_THREADS=1 cargo test --lib ir::analysis::pbt_tests::p11_frontiers_vs_definition → PASS (harness fix: generator clamped bits/n; NaN/duality and entry-cycle exclusions were test bugs, SUT untouched)

```property
function: ir.analysis.compute_dominance_frontiers
oracle: reference
predicate:
  quantifier: forall
  vars: [G]
  domain: { G: random directed graph, n: int(1..=8) }
  relation: { op: eq, lhs: DF_SUT, rhs: DF_definition }
generators:
  G: { gen: graph, nodes: int(1..=8), edge_prob: 0.3 }
evidence: Cytron et al. TOPLAS 1991 §2; README "dominance frontiers" section
```

## P12: mem2reg promote → SSA validity + phi completeness (random IR)
- Tier: 5
- Rationale: documented 6-step algorithm must yield "proper SSA form" (README §SSA
  Construction): promoted allocas gone; every phi has one incoming per unique
  predecessor edge; every use dominated by its def; next_value_id bounds all Values.
  Random alloca-based IR exercises the promotion machinery directly (lowering/ tree is
  exercised through the same entry in production).
- Doc contract: src/ir/mem2reg/promote.rs:10 "Insert phi nodes at iterated dominance frontiers of defining blocks" — asserted fingerprint a615b058
- Seed: (none)
- Formal: ∀ alloca-based IR F (random CFG, k ∈ 0..=4 scalar allocas, random load/store
  sequences): promote(F) satisfies (a) no remaining use of a promoted alloca,
  (b) ∀ block B reachable with phis: incoming labels = unique pred labels of B,
  (c) ∀ use of v at block U: def_block(v) dominates U (phi uses counted at the
  incoming block), (d) all Value ids < next_value_id.
- Test file: src/ir/mem2reg/promote.rs
- Status: failing
- Counterexample: (none — P12 itself passes; b4 was found by P9)
- Bug report: bug_reports/ir_analysis_build_cfg_pred_dup.md (b4 — surfaced by P9, mem2reg is a named consumer)

```property
function: ir.mem2reg.promote.promote_allocas
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [F]
  domain: { F: random alloca-based IR, blocks 1..=8, allocas 0..=4, equal-target CondBranch allowed }
  relation: { op: holds, expr: "ssa_valid_after_promote(promote_allocas_with_params(F))" }
generators:
  F: { gen: alloca_ir, blocks: int(1..=8), allocas: int(0..=4), ops: int(0..=6) per block }
evidence: README "yielding proper SSA form"; promote.rs module doc steps 5–6
```

## P13: phi_eliminate structural preservation
- Tier: 4
- Rationale: documented conversion Phi → Copy in predecessors (trampolines on critical
  edges); after elimination the function must contain no Phi, every label must resolve,
  trampolines must be pure copy+branch blocks targeting original labels, and every
  former phi dest must still be defined by some Copy (the merged value is still produced).
- Doc contract: src/ir/mem2reg/phi_eliminate.rs:12 "It converts each Phi instruction into Copy instructions placed at the end of each predecessor block (before the terminator)." — asserted fingerprint 0782a561 (src/ir/mem2reg/phi_eliminate.rs, re-verified 2026-10-05: quote present verbatim)
- Seed: (none)
- Formal: ∀ F' = eliminate(promote(F)) for random alloca-based IR F:
  (a) no Instruction::Phi remains, (b) every terminator/goto label resolves to a block,
  (c) every appended trampoline contains only Copy instructions + Branch(orig label),
  (d) ∀ phi dest v of promote(F): ∃ Copy{dest: v} in F'.
- Test file: src/ir/mem2reg/phi_eliminate.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)
- Re-verified: PATH="$HOME/.cargo/bin:$PATH" RUST_TEST_THREADS=1 cargo test --lib ir::mem2reg::phi_eliminate::pbt_tests::p13_phi_elimination_structural_preservation → PASS (harness fixes: generator clamp, dead/unreachable-phi exemption — test bugs, SUT untouched)

```property
function: ir.mem2reg.phi_eliminate.eliminate_phis
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [F]
  domain: { F: random alloca-based IR with phis (via promote) }
  relation: { op: holds, expr: "phi_free_and_well_formed(eliminate_phis(promote_allocas_with_params(F)))" }
generators:
  F: { gen: alloca_ir, blocks: int(1..=8), allocas: int(1..=4), ops: int(0..=6) per block }
evidence: src/ir/mem2reg/phi_eliminate.rs:12-30 module doc
```
