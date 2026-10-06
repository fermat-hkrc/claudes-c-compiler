# PBT Coverage Status — round 07 (src/passes)

| Function | Source file | Test file | Test target | Notes |
|----------|-------------|-----------|-------------|-------|
| div_by_const_function | div_by_const.rs | src/passes/div_by_const.rs (pbt_tests P1–P4) | cargo test --lib passes::div_by_const | pass (udiv32/sdiv32/rem magic differentials, native + in-I64 + genuine-64 forms; unsigned-pow2 and magic-fallback carve-outs per README/code docs) |
| fold_function | constant_fold.rs | src/passes/constant_fold.rs (pbt_tests P5–P6) | cargo test --lib passes::constant_fold | P5 FAIL — bug b1 (sub-int logical-op sign extension); P6 pass (div-by-zero UB preserved) |
| fold_binop (via fold_function) | constant_fold.rs | src/passes/constant_fold.rs (P5 + red regression) | cargo test --lib passes::constant_fold | FAIL — bug b1 (is_32bit LShr/UDiv/URem arms) |
| simplify_function | simplify.rs | src/passes/simplify.rs (pbt_tests P7–P8) | cargo test --lib passes::simplify | pass (identity metamorphic, strength-reduction value preservation) |
| run_gvn_function | gvn.rs | src/passes/gvn.rs (pbt_tests P9) | cargo test --lib passes::gvn | pass (CSE value preservation + inst-count monotonicity) |
| propagate_copies | copy_prop.rs | src/passes/gvn.rs (P9) + constant_fold.rs (P5 interleave) | cargo test --lib passes:: | pass (used as the documented deployment interleave for P5; preservation via P9) |
| eliminate_dead_code | dce.rs | src/passes/dce.rs (pbt_tests P10) | cargo test --lib passes::dce | pass (dead-chain preservation + actual removal) |
| if_convert_function | if_convert.rs | src/passes/if_convert.rs (pbt_tests P11) | cargo test --lib passes::if_convert | pass (diamond→select preservation) |
| simplify_cfg | cfg_simplify.rs | src/passes/cfg_simplify.rs (pbt_tests P12) | cargo test --lib passes::cfg_simplify | pass (value + reachability + targets-in-range on random CFGs; slow property — 700s serial, not hung) |
| narrow_function | narrow.rs | src/passes/narrow.rs (pbt_tests P13) | cargo test --lib passes::narrow | FAIL — bug b2 (Phase 2 use-context check missing; pipeline-confirmed via run_passes) |
| narrow_binops_without_cast (via narrow_function) | narrow.rs | src/passes/narrow.rs (P13 + red regression) | cargo test --lib passes::narrow | FAIL — bug b2 |
| run_passes | mod.rs | src/passes/mod.rs (pbt_tests P14) | cargo test --lib passes:: | pass (end-to-end value preservation over linear/diamond/loop/CFG/div shapes; P14's own shapes do not include b2's widen-widen-bitwise pattern — that gap is covered by P13 + the red regression) |
| inline/ipcp/dead_statics (module-level) | inline.rs, ipcp.rs, dead_statics.rs | src/passes/mod.rs (P14 two-function modules) | cargo test --lib passes:: | structural exercise only via P14 (2-function modules); no per-function properties (round budget; documented cutoffs are size heuristics, not value contracts) |
| resolve_asm.rs | resolve_asm.rs | — | — | skipped: requires inline-asm symbol plumbing absent from generated fragments (recorded in PLAN.md) |
| Change surface (round-06 test code in src/ir) | analysis.rs, constants.rs, phi_eliminate.rs, promote.rs, ops.rs | src/ir committed pbt_tests | cargo test --lib ir:: | re-executed verbatim, covered-by-round-06 (its p5 red witness for #575 fails as documented) |
