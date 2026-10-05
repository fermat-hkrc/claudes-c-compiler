# Function Index: src/ir (round 06)

> Total files: 53 | Total functions: 834 | PBT candidates: 24 | Excluded: 810 (pipeline-internal lowering methods exercised end-to-end or via the promoted entry points; trivial getters; test-only helpers)

| Function | Source File | Line | Kind | PBT Candidate | Reason |
|----------|-------------|------|------|---------------|--------|
bool_normalize | src/ir/constants.rs | 557 | method | yes | pure constant semantics, documented repr convention / IEEE-754 reference
build_cfg | src/ir/analysis.rs | 110 | function | yes | graph algorithm vs textbook reference oracle
build_dom_tree_children | src/ir/analysis.rs | 332 | function | yes | graph algorithm vs textbook reference oracle
byte_size | src/ir/module.rs | 125 | method | yes | documented size/ref contracts
can_trap | src/ir/ops.rs | 63 | method | yes | pure eval/classification, reference+algebraic oracle
cast_float_to_target | src/ir/constants.rs | 277 | method | yes | pure constant semantics, documented repr convention / IEEE-754 reference
cast_long_double_to_target | src/ir/constants.rs | 299 | method | yes | pure constant semantics, documented repr convention / IEEE-754 reference
coerce_to | src/ir/constants.rs | 550 | method | yes | pure constant semantics, documented repr convention / IEEE-754 reference
coerce_to_with_src | src/ir/constants.rs | 484 | method | yes | pure constant semantics, documented repr convention / IEEE-754 reference
compute_dominance_frontiers | src/ir/analysis.rs | 302 | function | yes | graph algorithm vs textbook reference oracle
compute_dominators | src/ir/analysis.rs | 238 | function | yes | graph algorithm vs textbook reference oracle
compute_reverse_postorder | src/ir/analysis.rs | 192 | function | yes | graph algorithm vs textbook reference oracle
eliminate_phis_in_function | src/ir/mem2reg/phi_eliminate.rs | 256 | function | yes | phi lowering: structural preservation oracle
eliminate_phis | src/ir/mem2reg/phi_eliminate.rs | 51 | function | yes | phi lowering: structural preservation oracle
emitted_byte_size | src/ir/module.rs | 145 | method | yes | documented size/ref contracts
eval_i128 | src/ir/ops.rs | 106 | method | yes | pure eval/classification, reference+algebraic oracle
eval_i128 | src/ir/ops.rs | 190 | method | yes | pure eval/classification, reference+algebraic oracle
eval_i64 | src/ir/ops.rs | 171 | method | yes | pure eval/classification, reference+algebraic oracle
eval_i64 | src/ir/ops.rs | 72 | method | yes | pure eval/classification, reference+algebraic oracle
f64_to_f128_bytes | src/ir/constants.rs | 48 | method | yes | pure constant semantics, documented repr convention / IEEE-754 reference
f64_to_x87_bytes | src/ir/constants.rs | 103 | method | yes | pure constant semantics, documented repr convention / IEEE-754 reference
for_each_function | src/ir/module.rs | 267 | method | yes | documented size/ref contracts
for_each_ref | src/ir/module.rs | 105 | method | yes | documented size/ref contracts
from_i64 | src/ir/constants.rs | 452 | method | yes | pure constant semantics, documented repr convention / IEEE-754 reference
is_commutative | src/ir/ops.rs | 57 | method | yes | pure eval/classification, reference+algebraic oracle
is_one | src/ir/constants.rs | 235 | method | yes | pure constant semantics, documented repr convention / IEEE-754 reference
is_pure | src/ir/intrinsics.rs | 193 | method | yes | documented purity list (README table)
is_zero | src/ir/constants.rs | 155 | method | yes | pure constant semantics, documented repr convention / IEEE-754 reference
long_double_from_i128 | src/ir/constants.rs | 202 | method | yes | pure constant semantics, documented repr convention / IEEE-754 reference
long_double_from_i64 | src/ir/constants.rs | 179 | method | yes | pure constant semantics, documented repr convention / IEEE-754 reference
long_double_from_u128 | src/ir/constants.rs | 194 | method | yes | pure constant semantics, documented repr convention / IEEE-754 reference
long_double_from_u64 | src/ir/constants.rs | 186 | method | yes | pure constant semantics, documented repr convention / IEEE-754 reference
max_value_id | src/ir/module.rs | 321 | method | yes | documented size/ref contracts
narrowed_to | src/ir/constants.rs | 579 | method | yes | pure constant semantics, documented repr convention / IEEE-754 reference
one | src/ir/constants.rs | 632 | method | yes | pure constant semantics, documented repr convention / IEEE-754 reference
promote_allocas | src/ir/mem2reg/promote.rs | 42 | function | yes | SSA construction: structural validity + phi completeness oracle
promote_allocas_with_params | src/ir/mem2reg/promote.rs | 55 | function | yes | SSA construction: structural validity + phi completeness oracle
promote_function | src/ir/mem2reg/promote.rs | 79 | function | yes | SSA construction: structural validity + phi completeness oracle
push_le_bytes | src/ir/constants.rs | 377 | method | yes | pure constant semantics, documented repr convention / IEEE-754 reference
to_hash_key | src/ir/constants.rs | 245 | method | yes | pure constant semantics, documented repr convention / IEEE-754 reference
to_le_bytes | src/ir/constants.rs | 617 | method | yes | pure constant semantics, documented repr convention / IEEE-754 reference
zero | src/ir/constants.rs | 562 | method | yes | pure constant semantics, documented repr convention / IEEE-754 reference
