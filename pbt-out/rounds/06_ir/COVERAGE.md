# PBT Coverage Status — round 06 (src/ir)

| Function | Source file | Test file | Test target | Notes |
|----------|-------------|-----------|-------------|-------|
| IrBinOp::eval_i64 | ops.rs | src/ir/ops.rs (pbt_tests P1-P3) | cargo test --lib ir::ops | pass (P1 div identity, P2 None⇔rhs=0) |
| IrBinOp::eval_i128 | ops.rs | src/ir/ops.rs (pbt_tests P3) | cargo test --lib ir::ops | pass (width consistency) |
| IrBinOp::can_trap / is_commutative | ops.rs | src/ir/ops.rs (pbt_tests P2) | cargo test --lib ir::ops | pass |
| IrCmpOp::eval_i64 / eval_i128 / eval_f64 | ops.rs | src/ir/ops.rs (pbt_tests P4, P4b) | cargo test --lib ir::ops | pass |
| IrConst::from_i64 | constants.rs | src/ir/constants.rs (pbt_tests P5, P8) | cargo test --lib ir::constants | pass (convention author, green side of differential) |
| IrConst::cast_float_to_target | constants.rs | src/ir/constants.rs (pbt_tests P5) | cargo test --lib ir::constants | FAIL — bug b1 (unsigned repr convention) |
| IrConst::cast_long_double_to_target | constants.rs | src/ir/constants.rs (pbt_tests P5) | cargo test --lib ir::constants | FAIL — bug b1 |
| IrConst::coerce_to_with_src / coerce_to | constants.rs | src/ir/constants.rs (pbt_tests P6) | cargo test --lib ir::constants | FAIL — bug b2 (early return) |
| f64_to_f128_bytes | constants.rs | src/ir/constants.rs (pbt_tests P7, pbt_regression) | cargo test --lib ir::constants | FAIL — bug b3 (subnormals) |
| f64_to_x87_bytes | constants.rs | src/ir/constants.rs (pbt_tests P7) | cargo test --lib ir::constants | FAIL — bug b3 (subnormals) |
| IrConst::bool_normalize / zero / one | constants.rs | src/ir/constants.rs (pbt_tests P8) | cargo test --lib ir::constants | pass (C11 6.3.1.2) |
| build_cfg | analysis.rs | src/ir/analysis.rs (pbt_tests P9, pbt_regression) | cargo test --lib ir::analysis | FAIL — bug b4 (pred duplication) |
| compute_dominators | analysis.rs | src/ir/analysis.rs (pbt_tests P10) | cargo test --lib ir::analysis | pass vs naive reference |
| compute_reverse_postorder | analysis.rs | src/ir/analysis.rs (pbt_tests P10) | cargo test --lib ir::analysis | pass |
| build_dom_tree_children | analysis.rs | src/ir/analysis.rs (pbt_tests P10) | cargo test --lib ir::analysis | pass |
| compute_dominance_frontiers | analysis.rs | src/ir/analysis.rs (pbt_tests P11, P11b red) | cargo test --lib ir::analysis | FAIL — bug b5 (entry cycles; covered domain green) |
| promote_allocas / promote_allocas_with_params | mem2reg/promote.rs | src/ir/mem2reg/promote.rs (pbt_tests P12) | cargo test --lib ir::mem2reg | pass (SSA validity + phi completeness) |
| eliminate_phis | mem2reg/phi_eliminate.rs | src/ir/mem2reg/phi_eliminate.rs (pbt_tests P13) | cargo test --lib ir::mem2reg | pass (structural preservation) |
| GlobalInit::byte_size / emitted_byte_size / for_each_ref | module.rs | (analyzed in scan; escape_string maps 1 char = 1 byte, chars()-count consistent with backend) | — | no property (verified consistent by reading; not filed) |
| max_value_id / for_each_function | module.rs | exercised via P12/P13 | cargo test --lib ir::mem2reg | pass (indirect) |
| IntrinsicOp::is_pure | intrinsics.rs | (none) | — | skipped: static classification list, no independent oracle (see PLAN.md) |
| — change-surface functions (frontend/sema etc.) | — | covered by round 05 (pbt-out/rounds/05_sema/PROPERTIES.md P1–P15, bugs b1–b5) | cargo test --lib frontend::sema | round-05 red witnesses re-executed as recorded in this round's probe baseline |

Source file column: plain file names relative to src/ir/.
