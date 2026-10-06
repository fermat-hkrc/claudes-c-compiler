# Function Index: src/passes (round 07)

> Total files: 17 | Total functions: 441 | PBT candidates: 31 | Excluded: 410 (private per-pass pattern-match helpers driven through their pass entry points; pipeline plumbing `run_on_visited`/`should_run!` wiring exercised end-to-end by `run_passes`; test-only helpers in existing `mod tests`)

| Function | Source File | Line | Kind | PBT Candidate | Reason |
|----------|-------------|------|------|---------------|--------|
| div_by_const_function | src/passes/div_by_const.rs | 51 | function | yes | documented Hacker's Delight rewrite; differential vs C11 div/rem oracle |
| compute_unsigned_magic_32 | src/passes/div_by_const.rs | 410 | function | yes | magic-number algorithm vs exhaustive C11 div reference |
| compute_signed_magic_32 | src/passes/div_by_const.rs | 592 | function | yes | magic-number algorithm vs exhaustive C11 div reference |
| compute_unsigned_magic_64 | src/passes/div_by_const.rs | 865 | function | yes | magic-number algorithm vs exhaustive C11 div reference |
| compute_signed_magic_64 | src/passes/div_by_const.rs | 1058 | function | yes | magic-number algorithm vs exhaustive C11 div reference |
| fold_function | src/passes/constant_fold.rs | 73 | function | yes | documented C11/IEEE folding contract; differential vs i128 model |
| run | src/passes/constant_fold.rs | 22 | function | yes | module wrapper over fold_function |
| resolve_remaining_is_constant | src/passes/constant_fold.rs | 40 | function | yes | documented Phase-0.5 resolution contract (fold to Copy(0)) |
| simplify_function | src/passes/simplify.rs | 84 | function | yes | documented identity/strength-reduction table; metamorphic oracle |
| run | src/passes/simplify.rs | 80 | function | yes | module wrapper over simplify_function |
| propagate_copies | src/passes/copy_prop.rs | 35 | function | yes | copy-chain substitution; value-preservation oracle |
| run | src/passes/copy_prop.rs | 30 | function | yes | module wrapper |
| eliminate_dead_code | src/passes/dce.rs | 30 | function | yes | liveness removal; value-preservation oracle |
| if_convert_function | src/passes/if_convert.rs | 50 | function | yes | branch→select; value-preservation + can_trap guard |
| simplify_cfg | src/passes/cfg_simplify.rs | 54 | function | yes | 7 documented sub-passes; preservation + reachability oracle |
| run_function | src/passes/cfg_simplify.rs | 44 | function | yes | dirty-tracking wrapper over simplify_cfg |
| narrow_function | src/passes/narrow.rs | 57 | function | yes | documented widen-operate-narrow rewrite; value-preservation oracle |
| run_gvn_function | src/passes/gvn.rs | 376 | function | yes | dominator CSE; value-preservation oracle |
| run_gvn_with_analysis | src/passes/gvn.rs | 397 | function | yes | shared-analysis GVN variant |
| licm_with_analysis | src/passes/licm.rs | 43 | function | yes | loop-invariant hoisting; value-preservation (structured loops) |
| ivsr_function | src/passes/iv_strength_reduce.rs | 72 | function | yes | IV strength reduction (structured loops via pipeline) |
| ivsr_with_analysis | src/passes/iv_strength_reduce.rs | 85 | function | yes | shared-analysis IVSR variant |
| find_natural_loops | src/passes/loop_analysis.rs | 22 | function | yes | documented natural-loop properties (header dominates latch) |
| merge_loops_by_header | src/passes/loop_analysis.rs | 65 | function | yes | pure set-union partition |
| find_preheader | src/passes/loop_analysis.rs | 116 | function | yes | documented preheader contract |
| run | src/passes/inline.rs | 316 | function | yes | inliner (multi-function; exercised via pipeline property) |
| run | src/passes/ipcp.rs | 35 | function | yes | IPCP (multi-function; exercised via pipeline property) |
| eliminate_dead_static_functions | src/passes/dead_statics.rs | 18 | function | yes | dead-static removal (module-level; via pipeline) |
| run_passes | src/passes/mod.rs | 269 | function | yes | full documented pipeline; integration value-preservation oracle |
| run_gvn_licm_ivsr_shared | src/passes/mod.rs | 73 | function | yes | shared-analysis driver (via run_passes) |
| run_on_visited | src/passes/mod.rs | 37 | function | no | plumbing; dirty-tracking contract exercised through run_passes |
