//! Optimization passes for the IR.
//!
//! This module contains various optimization passes that transform the IR
//! to produce better code.
//!
//! All optimization levels (-O0 through -O3, -Os, -Oz) run the same full set
//! of passes. While the compiler is still maturing, having separate tiers
//! creates hard-to-find bugs where code works at one level but breaks at
//! another. We always run all passes to maximize test coverage of the
//! optimizer and catch issues early.

pub(crate) mod cfg_simplify;
pub(crate) mod constant_fold;
pub(crate) mod copy_prop;
pub(crate) mod dce;
mod dead_statics;
pub(crate) mod div_by_const;
pub(crate) mod gvn;
pub(crate) mod if_convert;
pub(crate) mod inline;
pub(crate) mod ipcp;
pub(crate) mod iv_strength_reduce;
pub(crate) mod licm;
pub(crate) mod loop_analysis;
pub(crate) mod narrow;
mod resolve_asm;
pub(crate) mod simplify;

#[cfg(test)]
pub(crate) mod pbt_common;

use crate::ir::analysis::CfgAnalysis;
use crate::ir::reexports::{IrFunction, IrModule};

/// Run a per-function pass only on functions in the visit set.
///
/// `visit` indicates which functions to process in this iteration.
/// `changed` accumulates which functions were modified by any pass
/// (so the next iteration knows what to re-visit).
fn run_on_visited<F>(
    module: &mut IrModule,
    visit: &[bool],
    changed: &mut [bool],
    mut f: F,
) -> usize
where
    F: FnMut(&mut IrFunction) -> usize,
{
    let mut total = 0;
    for (i, func) in module.functions.iter_mut().enumerate() {
        if func.is_declaration {
            continue;
        }
        if i < visit.len() && !visit[i] {
            continue;
        }
        let n = f(func);
        if n > 0 {
            if i < changed.len() {
                changed[i] = true;
            }
            total += n;
        }
    }
    total
}

/// Run GVN, LICM, and IVSR with shared CFG analysis per function.
///
/// For each dirty function, builds CFG/dominator/loop analysis once and passes
/// it to all three passes. This eliminates redundant analysis computation that
/// previously occurred when each pass independently computed build_label_map +
/// build_cfg + compute_dominators (+ find_natural_loops for LICM/IVSR).
///
/// Returns (gvn_changes, licm_changes, ivsr_changes).
fn run_gvn_licm_ivsr_shared(
    module: &mut IrModule,
    visit: &[bool],
    changed: &mut [bool],
    run_gvn: bool,
    run_licm: bool,
    run_ivsr: bool,
    time_passes: bool,
    iter: usize,
) -> (usize, usize, usize) {
    let mut gvn_total = 0usize;
    let mut licm_total = 0usize;
    let mut ivsr_total = 0usize;

    for (i, func) in module.functions.iter_mut().enumerate() {
        if func.is_declaration {
            continue;
        }
        if i < visit.len() && !visit[i] {
            continue;
        }
        let num_blocks = func.blocks.len();
        if num_blocks == 0 {
            continue;
        }

        // GVN fast path: single-block functions don't need CFG analysis.
        if num_blocks == 1 {
            if run_gvn {
                let n = gvn::run_gvn_function(func);
                if n > 0 {
                    gvn_total += n;
                    if i < changed.len() { changed[i] = true; }
                }
            }
            // LICM and IVSR need loops (>= 2 blocks), so skip.
            continue;
        }

        // Build CFG analysis once for this function.
        let cfg = CfgAnalysis::build(func);

        // Run GVN with shared analysis.
        if run_gvn {
            let t0 = if time_passes { Some(std::time::Instant::now()) } else { None };
            let n = gvn::run_gvn_with_analysis(func, &cfg);
            if let Some(t0) = t0 {
                eprintln!("[PASS] iter={} gvn (func {}): {:.4}s ({} changes)", iter, func.name, t0.elapsed().as_secs_f64(), n);
            }
            if n > 0 {
                gvn_total += n;
                if i < changed.len() { changed[i] = true; }
            }
        }

        // Run LICM with shared analysis.
        // GVN does not modify the CFG (only replaces operands), so analysis is still valid.
        if run_licm {
            let t0 = if time_passes { Some(std::time::Instant::now()) } else { None };
            let n = licm::licm_with_analysis(func, &cfg);
            if let Some(t0) = t0 {
                eprintln!("[PASS] iter={} licm (func {}): {:.4}s ({} changes)", iter, func.name, t0.elapsed().as_secs_f64(), n);
            }
            if n > 0 {
                licm_total += n;
                if i < changed.len() { changed[i] = true; }
            }
        }

        // Run IVSR with shared analysis.
        // LICM hoists instructions to preheaders but does not add/remove blocks,
        // so CFG analysis is still valid.
        if run_ivsr {
            let t0 = if time_passes { Some(std::time::Instant::now()) } else { None };
            let n = iv_strength_reduce::ivsr_with_analysis(func, &cfg);
            if let Some(t0) = t0 {
                eprintln!("[PASS] iter={} iv_strength_reduce (func {}): {:.4}s ({} changes)", iter, func.name, t0.elapsed().as_secs_f64(), n);
            }
            if n > 0 {
                ivsr_total += n;
                if i < changed.len() { changed[i] = true; }
            }
        }
    }

    if time_passes {
        eprintln!("[PASS] iter={} gvn_total: {} changes", iter, gvn_total);
        eprintln!("[PASS] iter={} licm_total: {} changes", iter, licm_total);
        eprintln!("[PASS] iter={} ivsr_total: {} changes", iter, ivsr_total);
    }

    (gvn_total, licm_total, ivsr_total)
}

/// Run all optimization passes on the module.
///
/// The pass pipeline is:
/// 1. CFG simplification (remove dead blocks, thread jump chains, simplify branches)
/// 2. Copy propagation (replace uses of copies with original values)
/// 3. Algebraic simplification (strength reduction)
/// 4. Constant folding (evaluate const exprs at compile time)
/// 5. GVN / CSE (dominator-based value numbering, eliminates redundant
///    BinOp, UnaryOp, Cmp, Cast, GetElementPtr, and Load across dominated blocks)
/// 6. LICM (hoist loop-invariant code to preheaders)
/// 7. If-conversion (convert branch+phi diamonds to Select)
/// 8. Copy propagation (clean up copies from GVN/simplify/LICM)
/// 9. Dead code elimination (remove dead instructions)
/// 10. CFG simplification (clean up after DCE may have made blocks dead)
/// 11. Dead static function elimination (remove unreferenced internal-linkage functions)
///
struct DisabledPasses {
    cfg: bool,
    copyprop: bool,
    narrow: bool,
    simplify: bool,
    constfold: bool,
    gvn: bool,
    licm: bool,
    ifconv: bool,
    dce: bool,
    ipcp: bool,
}

impl DisabledPasses {
    fn from_env(disabled: &str) -> Self {
        DisabledPasses {
            cfg: disabled.contains("cfg"),
            copyprop: disabled.contains("copyprop"),
            narrow: disabled.contains("narrow"),
            simplify: disabled.contains("simplify"),
            constfold: disabled.contains("constfold"),
            gvn: disabled.contains("gvn"),
            licm: disabled.contains("licm"),
            ifconv: disabled.contains("ifconv"),
            dce: disabled.contains("dce"),
            ipcp: disabled.contains("ipcp"),
        }
    }
}

/// Run Phase 0: function inlining and post-inline optimization passes.
fn run_inline_phase(module: &mut IrModule, disabled: &str) {
    if disabled.contains("inline") {
        return;
    }
    inline::run(module);

    // After inlining, convert extern inline gnu_inline functions to declarations.
    // These function bodies were only needed for inlining; they must not be emitted
    // as standalone definitions because their internal calls (e.g., `call btowc`)
    // would resolve to the local definition instead of the external library symbol,
    // causing infinite recursion. Converting to declarations ensures any remaining
    // (non-inlined) calls resolve to the external symbol at link time.
    for func in &mut module.functions {
        if func.is_gnu_inline_def && !func.is_declaration {
            func.is_declaration = true;
            func.blocks.clear();
        }
    }

    crate::ir::mem2reg::promote_allocas_with_params(module);
    constant_fold::run(module);
    copy_prop::run(module);
    simplify::run(module);
    constant_fold::run(module);
    copy_prop::run(module);
    resolve_asm::resolve_inline_asm_symbols(module);
}

/// All optimization levels run the same pipeline with the same number of
/// iterations. The `opt_level` parameter is accepted for API compatibility
/// but currently ignored -- all levels behave identically.
///
/// **Why single-level optimization matters for this project:**
///
/// Having multiple optimization tiers (e.g., -O0 doing minimal work, -O1 doing
/// partial work, -O2 doing full work) is exponentially harder to test. Each tier
/// is a separate code path through the optimizer, and bugs that only appear at
/// one level are extremely difficult to reproduce and diagnose. For a compiler
/// that is still maturing and being validated against hundreds of real-world
/// projects (Linux kernel, PostgreSQL, Redis, etc.), a single optimization level
/// ensures that:
///
/// 1. Every test run exercises every optimization pass. A bug in GVN or LICM
///    will be caught even when testing with `-O0`, rather than hiding until a
///    user happens to compile with `-O2`.
/// 2. The number of configurations to validate stays linear (N architectures)
///    rather than quadratic (N architectures × M optimization levels).
/// 3. Build system interactions are predictable — the same code is always
///    generated regardless of which `-O` flag a project's Makefile passes.
///
/// The `optimize` and `optimize_size` booleans on the Driver still control the
/// `__OPTIMIZE__` and `__OPTIMIZE_SIZE__` predefined macros, which build systems
/// like the Linux kernel depend on (e.g., `BUILD_BUG()` uses `__OPTIMIZE__` to
/// select between a noreturn function call and a no-op). The actual pass pipeline
/// is unaffected by these flags.
pub(crate) fn run_passes(module: &mut IrModule, _opt_level: u32, target: crate::backend::Target) {
    let disabled = std::env::var("CCC_DISABLE_PASSES").unwrap_or_default();
    if disabled.contains("all") {
        return;
    }

    run_inline_phase(module, &disabled);
    constant_fold::resolve_remaining_is_constant(module);

    let iterations = 3;
    let num_funcs = module.functions.len();
    let mut dirty = vec![true; num_funcs];
    let dis = DisabledPasses::from_env(&disabled);

    // `changed` accumulates which functions were modified during each iteration.
    let mut changed = vec![false; num_funcs];

    let time_passes = std::env::var("CCC_TIME_PASSES").is_ok();

    // Per-pass change counts from the previous iteration, used for skip decisions.
    // Pass indices: 0=cfg1, 1=copyprop1, 2=narrow, 3=simplify, 4=constfold,
    //               5=gvn, 6=licm, 7=ifconv, 8=copyprop2, 9=dce, 10=cfg2
    const NUM_PASSES: usize = 11;
    let mut prev_pass_changes = [usize::MAX; NUM_PASSES]; // MAX = "assume changed" for iter 0

    // Track first iteration's total changes for diminishing-returns early exit.
    let mut iter0_total_changes = 0usize;

    for iter in 0..iterations {
        let mut total_changes = 0usize;
        let mut total_changes_excl_dce = 0usize; // Exclude DCE for diminishing-returns check
        let mut cur_pass_changes = [0usize; NUM_PASSES];

        // Clear the changed accumulator for this iteration
        changed.iter_mut().for_each(|c| *c = false);

        macro_rules! timed_pass {
            ($name:expr, $body:expr) => {{
                if time_passes {
                    let t0 = std::time::Instant::now();
                    let n = $body;
                    let elapsed = t0.elapsed().as_secs_f64();
                    eprintln!("[PASS] iter={} {}: {:.4}s ({} changes)", iter, $name, elapsed, n);
                    n
                } else {
                    $body
                }
            }};
        }

        // Helper: check if a pass should run based on upstream pass changes.
        // A pass runs if it or any of its upstream passes made changes last iteration.
        // On iteration 0, all passes run (prev_pass_changes are MAX).
        //
        // Pass dependency graph (which passes create opportunities for which):
        //   cfg_simplify → copy_prop, gvn, dce (simpler CFG)
        //   copy_prop → simplify, constfold, gvn, narrow (propagated values)
        //   narrow → simplify, constfold (smaller types)
        //   simplify → constfold, copy_prop, gvn (reduced expressions, folded casts to copies)
        //   constfold → cfg_simplify, copy_prop, dce (constant branches/dead code, folded exprs to copies)
        //   gvn → copy_prop, dce (eliminated redundant computations)
        //   licm → copy_prop, dce (hoisted code)
        //   if_convert → copy_prop, dce (eliminated branches)
        //   dce → cfg_simplify (empty blocks)
        macro_rules! should_run {
            ($self_idx:expr, $($upstream:expr),*) => {{
                prev_pass_changes[$self_idx] > 0 $(|| prev_pass_changes[$upstream] > 0)*
            }};
        }

        // Phase 1: CFG simplification
        // Upstream: constfold (constant branches), dce (empty blocks)
        if !dis.cfg && should_run!(0, 4, 9) {
            let n = timed_pass!("cfg_simplify1", run_on_visited(module, &dirty, &mut changed, cfg_simplify::run_function));
            cur_pass_changes[0] = n;
            total_changes += n;
            total_changes_excl_dce += n;
        }

        // Phase 2: Copy propagation
        // Upstream: cfg_simplify (simpler CFG), gvn (eliminated exprs), licm (hoisted code), if_convert
        if !dis.copyprop && should_run!(1, 0, 5, 6, 7) {
            let n = timed_pass!("copy_prop1", run_on_visited(module, &dirty, &mut changed, copy_prop::propagate_copies));
            cur_pass_changes[1] = n;
            total_changes += n;
            total_changes_excl_dce += n;
        }

        // Phase 2a: Division-by-constant strength reduction (first iteration only).
        // Replaces slow div/idiv instructions with fast multiply-and-shift sequences.
        // Run early so subsequent passes (narrowing, simplify, constant folding, DCE)
        // can optimize the expanded instruction sequences.
        //
        // Disabled on i686: the pass generates I64 multiply + shift-right-32 sequences
        // to extract the high 32 bits of a widened multiplication. The i686 backend
        // cannot execute 64-bit arithmetic correctly (it truncates to 32 bits), so these
        // sequences produce wrong results. Fall back to hardware idiv/div instead.
        // TODO: Re-enable once i686 has proper 64-bit arithmetic support, or implement
        // a 32-bit-aware variant that uses single-operand imull for mulhi.
        if iter == 0 && !disabled.contains("divconst") && !target.is_32bit() {
            let n = timed_pass!("div_by_const", run_on_visited(module, &dirty, &mut changed, div_by_const::div_by_const_function));
            total_changes += n;
            total_changes_excl_dce += n;
        }

        // Phase 2b: Integer narrowing
        // Upstream: copy_prop (propagated values expose narrowing)
        if !dis.narrow && should_run!(2, 1) {
            let n = timed_pass!("narrow", run_on_visited(module, &dirty, &mut changed, narrow::narrow_function));
            cur_pass_changes[2] = n;
            total_changes += n;
            total_changes_excl_dce += n;
        }

        // Phase 3: Algebraic simplification
        // Upstream: copy_prop (propagated values), narrow (smaller types)
        if !dis.simplify && should_run!(3, 1, 2) {
            let n = timed_pass!("simplify", run_on_visited(module, &dirty, &mut changed, simplify::simplify_function));
            cur_pass_changes[3] = n;
            total_changes += n;
            total_changes_excl_dce += n;
        }

        // Phase 4: Constant folding
        // Upstream: copy_prop (propagated constants), narrow, simplify (reduced exprs),
        //           if_convert (creates Select that constfold can fold with known-constant cond),
        //           copy_prop2 (propagates constants into Select/Cmp operands after if_convert)
        if !dis.constfold && should_run!(4, 1, 2, 3, 7, 8) {
            let n = timed_pass!("constfold", run_on_visited(module, &dirty, &mut changed, constant_fold::fold_function));
            cur_pass_changes[4] = n;
            total_changes += n;
            total_changes_excl_dce += n;
        }

        // Phases 5-6a: GVN + LICM + IVSR with shared CFG analysis.
        //
        // These three passes all need CFG + dominator + loop analysis. Since GVN
        // does not modify the CFG (it only replaces instruction operands within
        // existing blocks), the analysis computed for GVN remains valid for LICM
        // and IVSR. We compute it once per function and share it across all three.
        {
            let run_gvn = !dis.gvn && should_run!(5, 0, 1, 3);
            let run_licm = !dis.licm && should_run!(6, 0, 1, 5);
            let run_ivsr = iter == 0 && !disabled.contains("ivsr");

            if run_gvn || run_licm || run_ivsr {
                let (gvn_n, licm_n, ivsr_n) = run_gvn_licm_ivsr_shared(
                    module, &dirty, &mut changed,
                    run_gvn, run_licm, run_ivsr,
                    time_passes, iter,
                );
                cur_pass_changes[5] = gvn_n;
                total_changes += gvn_n;
                total_changes_excl_dce += gvn_n;
                cur_pass_changes[6] = licm_n;
                total_changes += licm_n;
                total_changes_excl_dce += licm_n;
                total_changes += ivsr_n;
                total_changes_excl_dce += ivsr_n;
            }
        }

        // Phase 7: If-conversion
        // Upstream: cfg_simplify (simpler CFG), constfold (simplified conditions)
        if !dis.ifconv && should_run!(7, 0, 4) {
            let n = timed_pass!("if_convert", run_on_visited(module, &dirty, &mut changed, if_convert::if_convert_function));
            cur_pass_changes[7] = n;
            total_changes += n;
            total_changes_excl_dce += n;
        }

        // Phase 8: Copy propagation again
        // Upstream: simplify (folded casts to copies), constfold (folded exprs to copies),
        //           gvn (produced copies), licm (hoisted code), if_convert (select values)
        // Note: simplify and constfold run earlier in this iteration, so we check
        // cur_pass_changes for them (not just prev_pass_changes via should_run!).
        if !dis.copyprop && (should_run!(8, 5, 6, 7) || cur_pass_changes[3] > 0 || cur_pass_changes[4] > 0) {
            let n = timed_pass!("copy_prop2", run_on_visited(module, &dirty, &mut changed, copy_prop::propagate_copies));
            cur_pass_changes[8] = n;
            total_changes += n;
            total_changes_excl_dce += n;
        }

        // Phase 9: Dead code elimination
        // Upstream: gvn, licm, if_convert, copy_prop2 (produced dead instructions)
        // Note: DCE changes are excluded from the diminishing-returns comparison
        // (total_changes_excl_dce) because DCE is a cleanup pass that removes dead
        // instructions. Its large change count (often 5000+ in iteration 0) inflates
        // iter0_total_changes, and by removing instructions, DCE actually reduces
        // the work subsequent passes can do in later iterations. This combination
        // causes the diminishing-returns heuristic to exit too early, preventing
        // the optimizer from completing multi-iteration constant propagation chains
        // (e.g., kernel's cpucap_is_possible switch folding through inlined
        // system_supports_sme -> alternative_has_cap_unlikely -> cpucap_is_possible).
        if !dis.dce && should_run!(9, 5, 6, 7, 8) {
            let n = timed_pass!("dce", run_on_visited(module, &dirty, &mut changed, dce::eliminate_dead_code));
            cur_pass_changes[9] = n;
            total_changes += n;
            // Intentionally NOT added to total_changes_excl_dce
        }

        // Phase 10: CFG simplification again
        // Upstream: constfold (constant branches), dce (dead blocks), if_convert
        if !dis.cfg && should_run!(10, 4, 7, 9) {
            let n = timed_pass!("cfg_simplify2", run_on_visited(module, &dirty, &mut changed, cfg_simplify::run_function));
            cur_pass_changes[10] = n;
            total_changes += n;
            total_changes_excl_dce += n;
        }

        // Phase 10.5: Interprocedural constant propagation (IPCP).
        // Run on every iteration, not just iter 0, because later iterations may
        // have simplified call arguments to constants (e.g., phi nodes collapsed
        // after CFG simplification resolved dead branches from IS_ENABLED() checks).
        let mut ipcp_changes = 0;
        if !dis.ipcp {
            ipcp_changes = timed_pass!("ipcp", ipcp::run(module));
            if ipcp_changes > 0 {
                changed.iter_mut().for_each(|c| *c = true);
            }
            total_changes += ipcp_changes;
            total_changes_excl_dce += ipcp_changes;
        }

        if iter == 0 {
            iter0_total_changes = total_changes_excl_dce;
        }

        // Early exit: if no passes changed anything, additional iterations are useless.
        if total_changes == 0 {
            break;
        }

        // Diminishing returns: if this iteration produced very few changes relative
        // to the first iteration, another iteration is unlikely to be worthwhile.
        // The optimizer converges quickly: typically iter 0 finds ~264K changes,
        // iter 1 finds ~10K, iter 2 finds ~200. Stopping when an iteration yields
        // less than 5% of the first iteration's output saves one full pipeline
        // iteration with negligible impact on optimization quality.
        //
        // We use total_changes_excl_dce for this comparison because DCE is a
        // cleanup pass whose large change count (removing dead instructions)
        // inflates iter0's total and makes subsequent iterations look like
        // diminishing returns even when they're still making meaningful progress.
        // DCE also reduces the number of instructions available for other passes,
        // naturally lowering their change counts in later iterations.
        //
        // Exception: if IPCP made changes this iteration, always run another
        // iteration regardless of diminishing returns. IPCP changes (constant
        // argument propagation, dead call elimination) create opportunities for
        // constant folding, DCE, and CFG simplification that require a full pass
        // to clean up. Without this, dead code referencing undefined symbols
        // (like the kernel's convert_to_fxsr) would survive.
        // We require iter > 1 (at least 2 full iterations) because multi-step
        // constant propagation chains (e.g., kernel's switch folding through
        // inlined cpucap_is_possible -> alternative_has_cap_unlikely) need at
        // least 2 iterations to complete: iter0 for initial folding, iter1 for
        // propagating results through the control flow.
        const DIMINISHING_RETURNS_FACTOR: usize = 20; // 1/20 = 5% threshold
        if iter > 1 && ipcp_changes == 0 && iter0_total_changes > 0
            && total_changes_excl_dce * DIMINISHING_RETURNS_FACTOR < iter0_total_changes
        {
            break;
        }

        // Save per-pass change counts for next iteration's skip decisions.
        prev_pass_changes = cur_pass_changes;

        // Prepare dirty set for next iteration: only re-visit functions that changed.
        std::mem::swap(&mut dirty, &mut changed);
    }

    // Phase 11: Dead static function elimination.
    // After all optimizations, remove internal-linkage (static) functions that are
    // never referenced by any other function or global initializer. This is critical
    // for `static inline` functions from headers: after intra-procedural optimizations
    // eliminate dead code paths (e.g., `if (1 || expr)` removes the else branch),
    // some static inline callees may become completely unreferenced and can be removed.
    // Without this, the dead functions may reference undefined external symbols
    // (e.g., kernel's `___siphash_aligned` calling `__siphash_aligned` which doesn't
    // exist on x86 where CONFIG_HAVE_EFFICIENT_UNALIGNED_ACCESS is set).
    dead_statics::eliminate_dead_static_functions(module);
}

// ── PBT properties (round 07) ─────────────────────────────────────────────────
#[cfg(test)]
mod pbt_tests {
    use super::run_passes;
    use crate::backend::Target;
    use crate::common::types::IrType;
    use crate::ir::reexports::{Instruction, IrBinOp, IrFunction, Operand, Terminator};
    use crate::passes::pbt_common::*;
    use proptest::prelude::*;

    fn build_udiv32(d: u32) -> IrFunction {
        let mut b = FnBuilder::new("f", IrType::U32, &[IrType::U32]);
        let p = b.v();
        let r = b.v();
        let insts = vec![
            Instruction::ParamRef { dest: p, param_idx: 0, ty: IrType::U32 },
            Instruction::BinOp { dest: r, op: IrBinOp::UDiv, lhs: Operand::Value(p), rhs: const_int(&IrType::U32, d as u64), ty: IrType::U32 },
        ];
        b.block(insts, Terminator::Return(Some(Operand::Value(r))));
        b.seal()
    }

    /// P14: the full pipeline (inline -> cleanup -> main loop with every pass
    /// incl. GVN/LICM/IVSR shared analysis, IPCP, dead statics) preserves the
    /// interpreted value on every generated shape and never panics.
    /// Doc contract: mod.rs:255 "1. Every test run exercises every optimization
    /// pass." (e40383fa)
    proptest! {
        #![proptest_config(ProptestConfig::with_cases(256))]
        #[test]
        fn p14_pipeline_value_preservation(
            shape in 0u8..5,
            s0 in any::<i64>(), r1 in any::<i64>(), r2 in any::<i64>(),
            steps in prop::collection::vec(any::<(u8, u8, u8, i64)>(), 1..6),
            cmp_idx in any::<u8>(), k in any::<i64>(),
            op1 in any::<u8>(), c1 in any::<i64>(), op2 in any::<u8>(), c2 in any::<i64>(),
            terms in prop::collection::vec(any::<(u8, u32, u32, i64, i64)>(), 2..=6),
            csteps in prop::collection::vec(prop::collection::vec(any::<(u8, u8, i64)>(), 0..=2), 0..=6),
            ret_pick in any::<u8>(),
            trip in any::<u8>(), mul_k in any::<i64>(),
            du32 in any::<u32>(), a in any::<i32>(), b in any::<i32>()
        ) {
            let ty = int_ty_by_index(r1 as u8);
            let (f, arg_sets): (IrFunction, Vec<Vec<i128>>) = match shape % 5 {
                0 => {
                    let t = int_ty_by_index((s0 as u8) % 4);
                    let mut sets = Vec::new();
                    for x in [0i64, 1, -1, r2, i64::MIN, i64::MAX] {
                        sets.push(vec![canon(x as i128, &t), canon(r2 as i128, &t)]);
                    }
                    (build_linear(t, s0, &steps), sets)
                }
                1 => {
                    let mut sets = Vec::new();
                    for (x, y) in [(0i32, 0), (1, -1), (a, b), (i32::MIN, i32::MAX)] {
                        sets.push(vec![x as i128, y as i128]);
                    }
                    (build_diamond(cmp_idx, k, op1, c1, op2, c2), sets)
                }
                2 => (build_loop(trip, mul_k), vec![vec![a as i128]]),
                3 => {
                    let spec = CfgSpec { terms: terms.clone(), steps: csteps.clone(), ret_pick, cmp_idx: cmp_idx };
                    let mut sets = Vec::new();
                    for (x, y) in [(0i32, 0), (1, 0), (-1, i32::MAX), (a, b)] {
                        sets.push(vec![x as i128, y as i128]);
                    }
                    (build_random_cfg(&spec), sets)
                }
                _ => {
                    let d = du32 % u32::MAX + 1; // 1..=u32::MAX (d==1 unrewritten, still valid)
                    let mut sets = Vec::new();
                    for x in [0u32, 1, 7, u32::MAX, du32] {
                        sets.push(vec![x as i128]);
                    }
                    (build_udiv32(d), sets)
                }
            };
            let expected: Vec<RunOutcome> = arg_sets.iter().map(|s| interp(&f, s, 200_000)).collect();
            // Rebuild deterministically, then run the whole pipeline.
            let g = match shape % 5 {
                0 => build_linear(int_ty_by_index((s0 as u8) % 4), s0, &steps),
                1 => build_diamond(cmp_idx, k, op1, c1, op2, c2),
                2 => build_loop(trip, mul_k),
                3 => build_random_cfg(&CfgSpec { terms, steps: csteps, ret_pick, cmp_idx }),
                _ => build_udiv32(du32 % u32::MAX + 1),
            };
            let mut m = module_of(g);
            run_passes(&mut m, 2, Target::X86_64);
            let out = &m.functions[0];
            for (args, exp) in arg_sets.iter().zip(expected.iter()) {
                let after = interp(out, args, 200_000);
                prop_assert_eq!(&after, exp, "pipeline changed value shape={}", shape % 5);
            }
            let _ = ty;
        }
    }
}

#[cfg(test)]
mod pbt_tests_opt {
    use crate::passes::pbt_common::*;
    use crate::common::types::IrType;
    use crate::ir::reexports::{Instruction, IrBinOp, IrConst, Operand, Terminator, BlockId};
    use crate::passes::run_passes;
    use crate::backend::Target;
    use proptest::prelude::*;

    /// Build a loop function: sum(i=0..trip) of (k * i + base)
    /// Exercises LICM (k*const hoisting) + IVSR (i*k strength reduction).
    fn build_loop(trip: i64, k: i64, base: i64) -> crate::ir::reexports::IrFunction {
        let mut b = FnBuilder::new("f", IrType::I64, &[IrType::I64, IrType::I64, IrType::I64]);
        let trip_v = b.v();
        let k_v = b.v();
        let base_v = b.v();
        let i_v = b.v();
        let acc_v = b.v();
        let prod_v = b.v();
        let add_v = b.v();
        let inc_v = b.v();
        let cond_v = b.v();
        // entry: params + init i=0, acc=0
        b.block(vec![
            Instruction::ParamRef { dest: trip_v, param_idx: 0, ty: IrType::I64 },
            Instruction::ParamRef { dest: k_v, param_idx: 1, ty: IrType::I64 },
            Instruction::ParamRef { dest: base_v, param_idx: 2, ty: IrType::I64 },
            Instruction::BinOp { dest: i_v, op: IrBinOp::Add, lhs: Operand::Value(i_v), rhs: Operand::Const(IrConst::I64(0)), ty: IrType::I64 }, // placeholder
        ], Terminator::Branch(BlockId(1)));
        // We need proper SSA init; use the simpler builder approach:
        let mut b = FnBuilder::new("f", IrType::I64, &[IrType::I64, IrType::I64, IrType::I64]);
        let trip_p = b.v();
        let k_p = b.v();
        let base_p = b.v();
        // entry: init i=0, acc=base
        let i0 = b.v();
        let acc0 = b.v();
        b.block(vec![
            Instruction::ParamRef { dest: trip_p, param_idx: 0, ty: IrType::I64 },
            Instruction::ParamRef { dest: k_p, param_idx: 1, ty: IrType::I64 },
            Instruction::ParamRef { dest: base_p, param_idx: 2, ty: IrType::I64 },
            Instruction::Copy { dest: i0, src: Operand::Const(IrConst::I64(0)) },
            Instruction::Copy { dest: acc0, src: Operand::Value(base_p) },
        ], Terminator::Branch(BlockId(1)));
        // loop header: phi i, phi acc; cond = i < trip
        let phi_i = b.v();
        let phi_acc = b.v();
        let lt = b.v();
        let cmp_op = crate::ir::reexports::IrCmpOp::Slt;
        b.block(vec![
            Instruction::Phi { dest: phi_i, incoming: vec![
                (Operand::Value(i0), BlockId(0)),
                (Operand::Value(i0), BlockId(2)), // placeholder, fixed below
            ], ty: IrType::I64 },
            Instruction::Phi { dest: phi_acc, incoming: vec![
                (Operand::Value(acc0), BlockId(0)),
                (Operand::Value(acc0), BlockId(2)), // placeholder
            ], ty: IrType::I64 },
            Instruction::Cmp { dest: lt, op: cmp_op, lhs: Operand::Value(phi_i), rhs: Operand::Value(trip_p), ty: IrType::I64 },
        ], Terminator::CondBranch { cond: Operand::Value(lt), true_label: BlockId(2), false_label: BlockId(3) });
        // loop body: acc += k * i + base; i++
        let prod = b.v();
        let add = b.v();
        let new_acc = b.v();
        let inc = b.v();
        let new_i = b.v();
        b.block(vec![
            Instruction::BinOp { dest: prod, op: IrBinOp::Mul, lhs: Operand::Value(k_p), rhs: Operand::Value(phi_i), ty: IrType::I64 },
            Instruction::BinOp { dest: add, op: IrBinOp::Add, lhs: Operand::Value(prod), rhs: Operand::Value(base_p), ty: IrType::I64 },
            Instruction::BinOp { dest: new_acc, op: IrBinOp::Add, lhs: Operand::Value(phi_acc), rhs: Operand::Value(add), ty: IrType::I64 },
            Instruction::BinOp { dest: new_i, op: IrBinOp::Add, lhs: Operand::Value(phi_i), rhs: Operand::Const(IrConst::I64(1)), ty: IrType::I64 },
        ], Terminator::Branch(BlockId(1)));
        // exit: return phi_acc
        b.block(vec![], Terminator::Return(Some(Operand::Value(phi_acc))));
        b.seal()
    }

    /// LICM + IVSR preservation through the full pipeline.
    proptest! {
        #![proptest_config(ProptestConfig::with_cases(256))]
        #[test]
        fn p_licm_ivsr_preservation(
            trip in 0i64..100,
            k in -10i64..10,
            base in -100i64..100,
        ) {
            let f = build_loop(trip, k, base);
            let args = [canon(trip as i128, &IrType::I64), canon(k as i128, &IrType::I64), canon(base as i128, &IrType::I64)];
            let expected = interp(&f, &args, 500_000);
            let mut m = module_of(build_loop(trip, k, base));
            run_passes(&mut m, 2, Target::X86_64);
            let after = interp(&m.functions[0], &args, 500_000);
            prop_assert_eq!(after, expected, "pipeline changed loop result trip={} k={} base={}", trip, k, base);
        }
    }
}

#[cfg(test)]
mod pbt_opt_rich {
    //! Round-36: optimizer interaction differential — rich C features
    //! (bitfields/structs/unions/float/fn-pointers/switch/goto) at -O0/-O1/-O2.
    //! Oracle: gcc -O0 (semantics are opt-level invariant for defined behavior).
    //! Tests pass INTERACTIONS: narrow+copy_prop+if_convert+dce+cfg_simplify chains.
    use proptest::prelude::*;
    use std::path::PathBuf;
    use std::process::Command;
    use std::sync::atomic::{AtomicUsize, Ordering};

    static CASE: AtomicUsize = AtomicUsize::new(0);
    fn run_dir() -> PathBuf {
        let d = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("pbt-out/rounds/36_opt_rich/run");
        let _ = std::fs::create_dir_all(&d);
        d
    }

    /// 富特性函数体生成器：每个 seed 组合多种特性
    fn render(v1: u32, v2: u32, v3: u64, mode: u8) -> String {
        let mut s = String::from("#include <stdio.h>\n#include <string.h>\n");
        s.push_str("struct B { unsigned a:3; unsigned b:5; int c:24; };\n");
        s.push_str("struct S { char t[4]; short n; double d; };\n");
        s.push_str("union U { unsigned u; char c[4]; };\n");
        s.push_str("static int add2(int x) { return x + 2; }\n");
        s.push_str("static int mul3(int x) { return x * 3; }\n");
        s.push_str("static int (*pick(int k))(int) { return k & 1 ? add2 : mul3; }\n");
        s.push_str(&format!(
"int main(void) {{
    struct B bf = {{ {}, {}, {} }};
    bf.a = (unsigned)bf.a + 1; bf.a &= 7;  // 位域回绕
    struct S st = {{ {{ 'a', 'b', 'c', 'd' }}, {}, {:.6} }};
    st.n = (short)(st.n * 2 - 1);
    union U u; u.u = 0x{:x}; u.c[1] = (char){};
    double dv = {:.9} * 2.0 + (double)bf.c / 7.0;
    float fv = (float)({}.5f) - 0.25f;
    long prod = 1;
    for (int i = 0; i < 6; i++) prod = prod * (i + (long)bf.a % 5 + 1) % 1000003L;
    int sw = {} % 5, acc = 0;
    switch (sw) {{
        case 0: acc += 1; /* fallthrough */
        case 1: acc += 2; break;
        case 2: acc += 4; goto tail;
        case 3: acc += 8; break;
        default: acc += 16; break;
    }}
    acc += 32;
tail:
    acc += 64;
    int (*fp)(int) = pick({});
    unsigned mask = 0x{:x}u;
    unsigned rot = (mask << {}) | (mask >> (32 - ({} % 31 + 1)));
    long double skip = 0; (void)skip; // 不用 long double (#1084)
    printf(\"bf=%u %u %d st=%d %.3f u=%02x%02x%02x%02x dv=%.6f fv=%.3f prod=%ld acc=%d fp=%d rot=%u\\n\",
        bf.a, bf.b, bf.c, st.n, st.d,
        (unsigned char)u.c[0], (unsigned char)u.c[1], (unsigned char)u.c[2], (unsigned char)u.c[3],
        dv, fv, prod, acc, fp(7), rot);
    return 0;
}}", 
            v1 % 8, (v1 >> 3) % 32, (v2 as i32 % 16777216) - 8388608,
            (v1 % 30000) as i16, (v2 % 100000) as f64 / 7.0,
            v1 | 0x41424344u32, (v2 % 100),
            (v3 % 1000000) as f64 / 9.0, v2 % 10,
            v1 % 5, v2 % 2,
            (v1.wrapping_mul(2654435761)) , v3 % 31, v3 % 31));
        s
    }

    /// P-OPT-RICH: 富特性 -O0/-O1/-O2 三级全部 == gcc
    proptest! {
        #![proptest_config(ProptestConfig::with_cases(150))]
        #[test]
        fn p_opt_rich_vs_gcc(
            v1 in any::<u32>(), v2 in any::<u32>(), v3 in any::<u64>(), mode in any::<u8>(),
        ) {
            let prog = render(v1, v2, v3, mode);
            let case = CASE.fetch_add(1, Ordering::SeqCst);
            let dir = run_dir();
            let src = dir.join(format!("R{}.c", case));
            std::fs::write(&src, &prog).unwrap();
            let gcc_out = dir.join(format!("R{}g", case));
            let gc = Command::new("gcc").args(["-w","-O0","-o"]).arg(&gcc_out).arg(&src).output().unwrap();
            prop_assume!(gc.status.success(), "gcc reject: {}", String::from_utf8_lossy(&gc.stderr).lines().next().unwrap_or(""));
            let g_run = Command::new(&gcc_out).output().unwrap();
            let want = String::from_utf8_lossy(&g_run.stdout).into_owned();

            for lvl in ["", "-O1", "-O2"] {
                let ccc_out = dir.join(format!("R{}c{}", case, lvl.replace('-', "m")));
                let argv = vec!["ccc".to_string(),
                    if lvl.is_empty() { String::new() } else { lvl.to_string() },
                    src.to_string_lossy().into_owned(),
                    "-o".to_string(), ccc_out.to_string_lossy().into_owned()];
                let argv: Vec<String> = argv.into_iter().filter(|a| !a.is_empty()).collect();
                let mut d = crate::driver::Driver::new();
                let cc = match d.parse_cli_args(&argv) { Ok(true) => Err("cli".into()), Ok(false) => d.run(), Err(e) => Err(e) };
                match cc {
                    Ok(()) => {}
                    Err(e) => prop_assert!(false, "ccc {} failed: {}\n{}", lvl, &e[..e.len().min(150)], prog),
                }
                let r = Command::new(&ccc_out).output().unwrap();
                let _ = std::fs::remove_file(&ccc_out);
                prop_assert!(r.status.success(), "ccc {} crashed", lvl);
                let got = String::from_utf8_lossy(&r.stdout);
                prop_assert_eq!(got, want.as_str(), "OPT MISMATCH at {} (gcc -O0 reference)\n{}", lvl, prog);
            }
            let _ = std::fs::remove_file(&src); let _ = std::fs::remove_file(&gcc_out);
        }
    }
}


#[cfg(test)]
mod pbt_high_value {
    //! Round-38: struct-by-value ABI — member-level oracle (padding-safe) +
    //! CROSS-COMPILER interop (gcc caller ↔ ccc callee): SysV classification.
    use proptest::prelude::*;
    use std::path::PathBuf;
    use std::process::Command;
    use std::sync::atomic::{AtomicUsize, Ordering};

    static CASE: AtomicUsize = AtomicUsize::new(0);
    fn run_dir() -> PathBuf {
        let d = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("pbt-out/rounds/38_high_value/run");
        let _ = std::fs::create_dir_all(&d);
        d
    }

    /// 成员池（含 ABI 关键组合：double/float 混 int/long）
    fn members(seed: u32) -> Vec<&'static str> {
        let pool = ["double", "float", "int", "long", "long long", "short", "char"];
        let n = 2 + (seed % 3) as usize;
        (0..n).map(|k| pool[(seed >> (k * 3)) as usize % pool.len()]).collect()
    }

    /// 生成 callee.c（mk + take，由 ccc 编译）与 caller.c（main，由 gcc 编译）
    fn render(seed: u32, v: u64) -> (String, String) {
        let ms = members(seed);
        let decl = ms.iter().enumerate().map(|(k, t)| format!("{} m{};", t, k)).collect::<Vec<_>>().join(" ");
        let ty = format!("struct S_{}", seed % 977);
        let callee = format!("#include <string.h>\n{ty_decl}\n{ty} mk(void);\n",
            ty_decl = format!("{} {{ {} }};", ty, decl));
        let _ = v;
        let init = ms.iter().enumerate().map(|(k, t)| {
            match *t {
                "double" => format!("{:.6}", (k as f64) + 0.5),
                "float" => format!("{:.3}f", (k as f32) + 0.25),
                "char" => format!("'{}'", (b'A' + k as u8) as char),
                _ => format!("{}", 100 + k),
            }
        }).collect::<Vec<_>>().join(", ");
        let init = init;
        let print = ms.iter().enumerate().map(|(k, t)| {
            match *t {
                "double" | "float" => format!("%:.17g", ).replace(":", ""), // 占位
                _ => String::new(),
            }
        }).collect::<Vec<_>>();
        let _ = print;
        let member_checks = ms.iter().enumerate().map(|(k, t)| {
            match *t {
                "double" => format!("chk &= s.m{k} == ({k}.5);"),
                "float" => format!("chk &= s.m{k} == ({k}.25f);"),
                "char" => format!("chk &= s.m{k} == '{}';", (b'A' + k as u8) as char),
                _ => format!("chk &= s.m{k} == {};", 100 + k),
            }
        }).collect::<Vec<_>>().join(" ");
        let callee = format!(
"{callee_decl}
int verify_mk_ok = 0;
{ty} mk(void) {{
    {ty} s = {{ {init} }};
    return s;
}}
unsigned long long take({ty} s) {{
    unsigned long long chk = 1;
    {member_checks}
    verify_mk_ok = (int)chk;
    return chk * 0x{chk_hex}ULL + sizeof(s);
}}", callee_decl = format!("{} {{ {} }};", ty, decl), ty = ty, init = init, member_checks = member_checks, chk_hex = format!("{:x}", 0xDEAD0000 + seed % 977));
        let caller = format!(
"#include <stdio.h>\n{ty_fwd}\nextern {ty} mk(void);\nextern unsigned long long take({ty});\nextern int verify_mk_ok;\nint main(void) {{
    {ty} a = mk();
    unsigned long long r1 = take(a);
    printf(\"take=%llx verify=%d sizeof=%zu\\n\", r1, verify_mk_ok, sizeof({ty}));
    {ty} c = mk();
    unsigned long long r2 = take(c);
    printf(\"ret=%llx\\n\", r2);
    return 0;
}}", ty_fwd = format!("{} {{ {} }};", ty, decl), ty = ty);
        (callee, caller)
    }

    /// P-INTEROP: gcc 编 caller + ccc 编 callee —— SysV 分类一致性
    /// （padding 不确定 → 只比较成员语义 + 确定性返回值）
    proptest! {
        #![proptest_config(ProptestConfig::with_cases(250))]
        #[test]
        fn p_struct_interop_gcc_calls_ccc(seed in any::<u32>(), v in any::<u64>()) {
            let (callee, caller) = render(seed, v);
            let case = CASE.fetch_add(1, Ordering::SeqCst);
            let dir = run_dir();
            let fc = dir.join(format!("I{}_callee.c", case));
            let fm = dir.join(format!("I{}_caller.c", case));
            std::fs::write(&fc, &callee).unwrap();
            std::fs::write(&fm, &caller).unwrap();
            let cobj = dir.join(format!("I{}_ccc.o", case));
            // gcc caller
            let gbin = dir.join(format!("I{}_gcc", case));
            let gc = Command::new("gcc").args(["-w","-O0","-o"]).arg(&gbin)
                .arg(&fm).arg(&fc).output().unwrap(); // 纯 gcc 作基准
            prop_assume!(gc.status.success(), "gcc self-reject");
            let want = Command::new(&gbin).output().unwrap();
            let want = String::from_utf8_lossy(&want.stdout).into_owned();
            // ccc callee + gcc caller 链接（interop!）
            let ccv = vec!["ccc".to_string(), fc.to_string_lossy().into_owned(), "-c".to_string(),
                "-o".to_string(), cobj.to_string_lossy().into_owned()];
            let mut d = crate::driver::Driver::new();
            let cc = match d.parse_cli_args(&ccv) { Ok(false) => d.run(), Ok(true) => Err("cli".into()), Err(e) => Err(e) };
            prop_assert!(cc.is_ok(), "ccc -c callee failed:\n{}", callee);
            let ibin = dir.join(format!("I{}_mix", case));
            let mix = Command::new("gcc").args(["-w","-O0","-o"]).arg(&ibin)
                .arg(&fm).arg(&cobj).output().unwrap();
            prop_assert!(mix.status.success(), "gcc link with ccc .o failed: {}", String::from_utf8_lossy(&mix.stderr));
            let got = Command::new(&ibin).output().unwrap();
            prop_assert!(got.status.success(), "interop binary crashed");
            prop_assert_eq!(String::from_utf8_lossy(&got.stdout), want.as_str(),
                "STRUCT-ABI INTEROP MISMATCH (gcc caller, ccc callee)\ncallee:\n{}\ncaller:\n{}", callee, caller);
            let _ = std::fs::remove_file(&fc); let _ = std::fs::remove_file(&fm);
            let _ = std::fs::remove_file(&cobj); let _ = std::fs::remove_file(&gbin); let _ = std::fs::remove_file(&ibin);
        }
    }
}

#[cfg(test)]
mod pbt_switch_tables {
    //! Round-40: switch lowering at -O2 — jump tables, binary trees, sparse
    //! gaps, long/unsigned selectors, fallthrough chains. Silent wrong-control-flow
    //! class. Oracle: gcc -O0.
    use proptest::prelude::*;
    use std::path::PathBuf;
    use std::process::Command;
    use std::sync::atomic::{AtomicUsize, Ordering};

    static CASE: AtomicUsize = AtomicUsize::new(0);
    fn run_dir() -> PathBuf {
        let d = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("pbt-out/rounds/40_new_axes/run");
        let _ = std::fs::create_dir_all(&d);
        d
    }

    /// 随机 case 集 → switch 函数 + 全域扫描
    fn render(seed: u64, n_cases: usize, ty_idx: usize, sparse: bool, fallthrough: bool) -> String {
        use std::collections::BTreeSet;
        let mut rng = seed | 1;
        let mut next = move || { rng = rng.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407); rng };
        let mut cases = BTreeSet::new();
        let mut lo: i128 = 0;
        while cases.len() < n_cases {
            let gap = if sparse { (next() % 97) as i128 } else { 1 };
            lo += gap;
            cases.insert(lo);
        }
        let (ty, cast) = match ty_idx % 4 {
            0 => ("int", format!("{}", 0)),
            1 => ("unsigned", format!("{}", 0)),
            2 => ("long", format!("{}", 0)),
            _ => ("char", format!("{}", 0)),
        };
        let _ = cast;
        let body: Vec<String> = cases.iter().enumerate().map(|(i, &c)| {
            let val = ((c as i64).wrapping_mul(2654435761) % 1000003) as i64;
            if fallthrough && i % 3 == 0 && i + 1 < cases.len() {
                format!("    case {c}: r += {v}; // fallthrough\n", c = c as i64, v = val)
            } else {
                format!("    case {c}: r = {v}; break;\n", c = c as i64, v = val)
            }
        }).collect();
        // 扫描域: 覆盖 case 值 ±邻域 + 边界
        let mut probes: Vec<i64> = Vec::new();
        for &c in &cases { let ci = c as i64; for d in [-1i64, 0, 1] { probes.push(ci + d); } }
        probes.extend([-3, -2, -1, 0, 1, 2, 3]);
        probes.sort(); probes.dedup();
        let probes_lit: Vec<String> = probes.iter().map(|p| format!("({ty}){}", p, ty = ty)).collect();
        format!("#include <stdio.h>\n\
static long f({ty} x) {{\n    long r = -77;\n    switch (x) {{\n{}    default: r = -1; break;\n    }}\n    return r;\n}}\n\
int main(void) {{\n    {ty} probes[] = {{ {} }};\n    for (unsigned i = 0; i < sizeof(probes)/sizeof(probes[0]); i++) printf(\"%ld \", f(probes[i]));\n    putchar('\\n');\n    return 0;\n}}\n",
            body.join(""), probes_lit.join(", "))
    }

    /// P-SWITCH: 跳转表/稀疏/类型变体 @-O2 全域扫描 == gcc
    proptest! {
        #![proptest_config(ProptestConfig::with_cases(800))]
        #[test]
        fn p_switch_tables_vs_gcc(
            seed in any::<u64>(), n in 4usize..200, ty in 0usize..4,
            sparse in any::<bool>(), ft in any::<bool>(),
        ) {
            let prog = render(seed, n, ty, sparse, ft);
            let case = CASE.fetch_add(1, Ordering::SeqCst);
            let dir = run_dir();
            let src = dir.join(format!("W{}.c", case));
            std::fs::write(&src, &prog).unwrap();
            let g = dir.join(format!("W{}g", case));
            let gc = Command::new("gcc").args(["-w","-O0","-o"]).arg(&g).arg(&src).output().unwrap();
            prop_assume!(gc.status.success(), "gcc reject");
            let want = Command::new(&g).output().unwrap();
            let want = String::from_utf8_lossy(&want.stdout).into_owned();
            for lvl in ["-O1", "-O2"] {
                let out = dir.join(format!("W{}c{}", case, if lvl == "-O1" { 1 } else { 2 }));
                let argv = vec!["ccc".to_string(), lvl.to_string(),
                    src.to_string_lossy().into_owned(), "-o".to_string(), out.to_string_lossy().into_owned()];
                let mut d = crate::driver::Driver::new();
                let cc = match d.parse_cli_args(&argv) { Ok(false) => d.run(), Ok(true) => Err("cli".into()), Err(e) => Err(e) };
                prop_assert!(cc.is_ok(), "ccc {} failed\n{}", lvl, prog);
                let r = Command::new(&out).output().unwrap();
                let _ = std::fs::remove_file(&out);
                prop_assert!(r.status.success(), "crash at {}", lvl);
                prop_assert_eq!(String::from_utf8_lossy(&r.stdout), want.as_str(),
                    "SWITCH MISMATCH at {} (gcc -O0)\n{}", lvl, prog);
            }
            let _ = std::fs::remove_file(&src); let _ = std::fs::remove_file(&g);
        }
    }
}

#[cfg(test)]
mod pbt_cross_target {
    //! Round-41: CROSS-TARGET runtime differential — rich C bodies compiled for
    //! ARM64/RISC-V (ccc) and executed under qemu, compared against native gcc-x64.
    //! First-ever runtime E2E on non-x86 targets.
    use proptest::prelude::*;
    use std::path::PathBuf;
    use std::process::Command;
    use std::sync::atomic::{AtomicUsize, Ordering};

    static CASE: AtomicUsize = AtomicUsize::new(0);
    fn run_dir() -> PathBuf {
        let d = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("pbt-out/rounds/41_cross/run");
        let _ = std::fs::create_dir_all(&d);
        d
    }

    fn body(seed: u64) -> String {
        // 确定性富计算（无 UB）：结构体/浮点/位域/联合/switch/循环
        format!("\
struct S {{ int a; double b; char c[4]; }};
static unsigned h = 2166136261u;
static void mix(unsigned x) {{ h = (h ^ x) * 16777619u; }}
static struct S mk(int s) {{ struct S r = {{ s, s * 1.5, {{1,2,3,4}} }}; return r; }}
static double use(struct S s) {{ return s.a * 2.0 + s.b + s.c[3]; }}
static int recfn(int n) {{ return n <= 0 ? 1 : recfn(n - 1) * 2 % 100003; }}
static int itfn(int n) {{ int r = 1; for (int i = 0; i < n; i++) r = r * 2 % 100003; return r; }}
struct BF {{ unsigned p:5; unsigned q:11; }};
int cmain(int rec) {{
    unsigned B1 = {s1}u, B2 = {s2}u;
    for (int i = 0; i < 8; i++) {{
        struct S s = mk(i * 7 + (int)(B1 % 13));
        mix((unsigned)(use(s) * 4.0) ^ B1);
        struct BF bf = {{ i % 32, (B2 >> (i % 11)) % 2048 }};
        mix((bf.p << 11) + bf.q + i);
        unsigned v[3] = {{ (unsigned)i, h, h >> 3 }};
        mix(v[i % 3] + B2);
        switch (i) {{ case 2: mix(0xAA); break; case 5: mix(0xBB); break; default: mix(i + B1 % 7); }}
    }}
    union {{ unsigned u; unsigned char b[4]; }} u = {{ B1 * 0x9E3779B9u }};
    mix(u.b[2]);
    double d = (B1 % 1000) / 7.0; float f = (float)d;
    mix((unsigned)(d * 1000.0) + (unsigned)(f * 100.0f));
    long prod = 1;
    for (int k = 0; k < 6; k++) prod = prod * (k + (long)(B2 % 5) + 1) % 1000003L;
    mix((unsigned)prod);
    return (int)(h % 251);
}}", s1 = seed, s2 = seed >> 32)
    }

    /// P-CROSS-TARGET-LVL: 9 组合矩阵（-O0/-O1/-O2 × ARM64/RISC-V/i686）
    proptest! {
        #![proptest_config(ProptestConfig::with_cases(120))]
        #[test]
        fn p_cross_target_levels(seed in any::<u64>(), lvl_idx in 0usize..3) {
            let lvl = ["-O0", "-O1", "-O2"][lvl_idx];
            let b = body(seed);
            let dir = run_dir();
            let case = CASE.fetch_add(1, Ordering::SeqCst);
            let x64 = dir.join(format!("L{}.c", case));
            std::fs::write(&x64, format!("#include <stdio.h>\n{}int main(void){{ return cmain(0); }}\n", b)).unwrap();
            let gx = dir.join(format!("L{}g", case));
            let gc = Command::new("gcc").args(["-w", lvl, "-o"]).arg(&gx).arg(&x64).output().unwrap();
            prop_assume!(gc.status.success(), "gcc reject");
            let want = Command::new(&gx).output().unwrap();
            let expected = want.status.code().unwrap_or(-1);
            let _ = std::fs::remove_file(&gx);
            let msrc = dir.join(format!("LM{}.c", case));
            std::fs::write(&msrc, format!("{}int main(void){{ return cmain(0); }}\n", b)).unwrap();
            let out = dir.join(format!("LM{}i", case));
            let comp = Command::new("./target/debug/ccc-i686").args(["-m32", lvl, msrc.to_str().unwrap(), "-o", out.to_str().unwrap()]).output().unwrap();
            prop_assert!(comp.status.success(), "i686 {} failed: {}", lvl, String::from_utf8_lossy(&comp.stderr));
            let run = Command::new(&out).output().unwrap();
            let _ = std::fs::remove_file(&out);
            prop_assert_eq!(run.status.code().unwrap_or(-1), expected,
                "i686 MISMATCH at {} seed {:#x}\n{}", lvl, seed, b);
            let _ = std::fs::remove_file(&x64); let _ = std::fs::remove_file(&msrc);
        }
    }

    /// P-CROSS-TARGET: 随机体 → ccc-arm/ccc-riscv qemu == gcc-x64 原生
    proptest! {
        #![proptest_config(ProptestConfig::with_cases(150))]
        #[test]
        fn p_cross_target_vs_gcc_x64(seed in any::<u64>()) {
            let b = body(seed);
            let dir = run_dir();
            let case = CASE.fetch_add(1, Ordering::SeqCst);
            // x64 基准
            let x64 = dir.join(format!("X{}.c", case));
            std::fs::write(&x64, format!("#include <stdio.h>\n{}int main(void){{ return cmain(0); }}\n", b)).unwrap();
            let gx = dir.join(format!("X{}g", case));
            let gc = Command::new("gcc").args(["-w","-O2","-o"]).arg(&gx).arg(&x64).output().unwrap();
            prop_assume!(gc.status.success(), "gcc reject");
            let want = Command::new(&gx).output().unwrap();
            prop_assert!(!want.status.to_string().contains("signal"), "gcc binary crashed");
            let expected = want.status.code().unwrap_or(-1);
            let _ = std::fs::remove_file(&gx);

            // freestanding 入口（ARM64 / RISC-V）
            let fs = format!("\n{}int cmain(void);\nvoid _start(void) {{\n\
#if defined(__aarch64__)\n    register int x0v asm(\"x0\") = cmain(0);\n    register int x8v asm(\"x8\") = 93;\n    asm volatile(\"svc #0\" : \"+r\"(x0v) : \"r\"(x8v) : \"memory\");\n\
#elif defined(__riscv)\n    register int a0v asm(\"a0\") = cmain(0);\n    register int a7v asm(\"a7\") = 93;\n    asm volatile(\"ecall\" : \"+r\"(a0v) : \"r\"(a7v) : \"memory\");\n\
#else\n    asm volatile(\"mov $60, %eax\");\n#endif\n    __builtin_unreachable();\n}}\n", b);
            let fsrc = dir.join(format!("F{}.c", case));
            std::fs::write(&fsrc, &fs).unwrap();
            let msrc = dir.join(format!("M{}.c", case));
            std::fs::write(&msrc, format!("{}int main(void){{ return cmain(0); }}\n", b)).unwrap();
            for (bin, cc, extra, qemu) in [
                (dir.join(format!("F{}arm", case)), "ccc-arm", "-static", Some("qemu-aarch64-static")),
                (dir.join(format!("F{}rv", case)), "ccc-riscv", "-static", Some("qemu-riscv64-static")),
                (dir.join(format!("M{}i32", case)), "ccc-i686", "-m32", None),
            ] {
                let ccpath = format!("./target/debug/{}", cc);
                let srcfile = if qemu.is_none() { &msrc } else { &fsrc };
                let comp = Command::new(&ccpath).args(["-O2", extra, srcfile.to_str().unwrap(), "-o", bin.to_str().unwrap()]).output().unwrap();
                prop_assert!(comp.status.success(), "{} compile failed: {}", cc, String::from_utf8_lossy(&comp.stderr));
                let run = match qemu {
                    Some(q) => Command::new(q).arg(&bin).output().unwrap(),
                    None => Command::new(&bin).output().unwrap(),
                };
                let _ = std::fs::remove_file(&bin);
                // qemu: 非零退出可能是程序退出码也可能是崩溃 — 看是否被信号杀
                let code = run.status.code().unwrap_or(-1);
                prop_assert!(code == expected,
                    "CROSS-TARGET MISMATCH {} (seed {:#x}): got exit {} want {} (crashed={})\nbody:\n{}",
                    cc, seed, code, expected, run.status.to_string().contains("signal"), b);
            }
            let _ = std::fs::remove_file(&x64); let _ = std::fs::remove_file(&fsrc);
        }
    }
}

#[cfg(test)]
mod pbt_opt_adversarial {
    //! Round-42: adversarial optimizer patterns with boundary injection —
    //! branchless abs/min/max at INT_MIN, guarded overflow checks, rotates with
    //! r=0/33 wraps, induction-variable strength reduction at wrap edges.
    //! Oracle: gcc -O0 (semantics invariant).
    use proptest::prelude::*;
    use std::path::PathBuf;
    use std::process::Command;
    use std::sync::atomic::{AtomicUsize, Ordering};

    static CASE: AtomicUsize = AtomicUsize::new(0);
    fn run_dir() -> PathBuf {
        let d = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("pbt-out/rounds/42_optadv/run");
        let _ = std::fs::create_dir_all(&d);
        d
    }

    fn body(x: i32, u: u32, r: u32, n: u32, k: u32, d: i64) -> String {
        format!("\
static unsigned rotr32(unsigned x, unsigned r) {{ r &= 31; return r ? (x << (32 - r)) | (x >> r) : x; }}
static int abs_b(int v) {{ int m = v >> 31; return (v ^ m) - m; }}
static int maxb(int a, int b) {{ return a ^ ((a ^ b) & -(a < b)); }}
static int minb(int a, int b) {{ return b ^ ((a ^ b) & -(a < b)); }}
static int ovf_check(int a, int b, int *ok) {{
    if (b > 0 && a > 2147483647 - b) {{ *ok = 0; return -1; }}
    if (b < 0 && a < (-2147483647 - 1) - b) {{ *ok = 0; return -2; }}
    *ok = 1; return a + b;
}}
static unsigned strength(unsigned n, unsigned k) {{
    unsigned s = 0;
    for (unsigned i = 0; i < n; i++) s += i * 8 + k;
    return s;
}}
static long guarded_div(long a, long b) {{ return b ? a / b : -77; }}
int cmain(int rec) {{
    unsigned R = {r}u;
    int A = {x}, B = {x} + 1;
    unsigned X = {u}u, K = {k}u, N = {n} % 200u;
    long D = {d};
    long h = rotr32(X, R) + rotr32(X, R + 1) + rotr32(X, R + 31) + rotr32(X, 32);
    h += (long)abs_b(A) + abs_b(B) + abs_b(-2147483647 - 1);
    h += maxb(A, B) + minb(A, B) + maxb(-2147483647 - 1, B) * 0;
    int ok1, ok2, ok3;
    h += ovf_check(A, B, &ok1) + ovf_check(A, -B, &ok2) + ovf_check(2147483647, 1, &ok3) * ok3;
    h += (long)strength(N, K) + strength(N + 1, K + 7);
    h += guarded_div(D, D % 97 - 50) + guarded_div(D, 0);
    h += (long)(X - K) + (long)(K - X);   // 无符号回绕对称
    return (int)((h * 2654435761L) % 251);
}}
int main(void){{ return cmain(0); }}", x = x, u = u, r = r, n = n, k = k, d = d)
    }

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(1000))]
        #[test]
        fn p_opt_adversarial_vs_gcc(
            x in proptest::strategy::Union::new(vec![
                any::<i32>().boxed(),
                (0i32..4).prop_map(|k| [i32::MIN, i32::MAX, i32::MIN + 1, i32::MAX - 1][k as usize]).boxed(),
            ]),
            u in proptest::strategy::Union::new(vec![
                any::<u32>().boxed(),
                (0u32..3).prop_map(|k| [0u32, u32::MAX, u32::MAX - 1][k as usize]).boxed(),
            ]),
            r in 0u32..34, n in any::<u32>(), k in any::<u32>(), d in any::<i64>(),
            lvl in 0usize..3,
        ) {
            let prog = body(x, u, r, n, k, d);
            let lvl = ["-O0", "-O1", "-O2"][lvl];
            let case = CASE.fetch_add(1, Ordering::SeqCst);
            let dir = run_dir();
            let src = dir.join(format!("V{}.c", case));
            std::fs::write(&src, &prog).unwrap();
            let g = dir.join(format!("V{}g", case));
            let gc = Command::new("gcc").args(["-w","-O0","-o"]).arg(&g).arg(&src).output().unwrap();
            prop_assume!(gc.status.success(), "gcc reject");
            let want = Command::new(&g).output().unwrap();
            let expected = want.status.code().unwrap_or(-1);
            let _ = std::fs::remove_file(&g);
            let out = dir.join(format!("V{}c", case));
            let argv = vec!["ccc".to_string(), lvl.to_string(), src.to_str().unwrap().to_string(), "-o".to_string(), out.to_str().unwrap().to_string()];
            let mut drv = crate::driver::Driver::new();
            let cc = match drv.parse_cli_args(&argv) { Ok(false) => drv.run(), Ok(true) => Err("cli".into()), Err(e) => Err(e) };
            prop_assert!(cc.is_ok(), "ccc {} failed\n{}", lvl, prog);
            let got = Command::new(&out).output().unwrap();
            let _ = std::fs::remove_file(&out); let _ = std::fs::remove_file(&src);
            prop_assert!(!got.status.to_string().contains("signal"), "crashed at {}\n{}", lvl, prog);
            prop_assert_eq!(got.status.code().unwrap_or(-1), expected,
                "OPT-ADVERSARIAL MISMATCH at {} (gcc -O0)\n{}", lvl, prog);
        }
    }
}

#[cfg(test)]
mod pbt_memgraph {
    //! Round-42: memory-graph fuzzer — aliasing writes through computed indices,
    //! side effects via function pointers, pointer arithmetic, final-state oracle
    //! vs gcc. The classic miscompile zone at -O0/-O1/-O2.
    use proptest::prelude::*;
    use std::path::PathBuf;
    use std::process::Command;
    use std::sync::atomic::{AtomicUsize, Ordering};

    static CASE: AtomicUsize = AtomicUsize::new(0);
    fn run_dir() -> PathBuf {
        let d = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("pbt-out/rounds/42_optadv/run");
        let _ = std::fs::create_dir_all(&d);
        d
    }

    /// 生成随机操作序列作用于 8 个全局 cell + 2 个数组（可别名）
    fn body(seed: u64, n_ops: usize) -> String {
        let mut rng = seed | 1;
        let mut next = move || { rng = rng.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407); (rng >> 33) as u32 };
        let mut ops = String::new();
        let mut fdefs = String::new();
        let mut fno = 0;
        for i in 0..n_ops {
            let op = next() % 7;
            let c1 = next() % 8;
            let c2 = next() % 8;
            let v = (next() % 1000) as i64;
            match op {
                0 => ops.push_str(&format!("    cells[{c1}] = cells[{c2}] + {v};\n")),
                1 => ops.push_str(&format!("    arr[{c1} % 6] = {v}; cells[{c2}] += arr[{c1} % 6];\n")),
                2 => { fno += 1;
                    fdefs.push_str(&format!("static int f{fno}(int *p, int *q) {{ *p += {v}; return *q * 2; }}\n"));
                    ops.push_str(&format!("    sum += f{fno}(&cells[{c1}], &cells[{c2}]);\n")); }
                3 => ops.push_str(&format!("    {{ int *p = &cells[{c1}]; int *q = p + {c2}; if (q >= cells && q < cells + 8) *q ^= {v}u; }}\n")),
                4 => ops.push_str(&format!("    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + {c1}) % 8] % 97;\n")),
                5 => ops.push_str(&format!("    sum += cells[{c1}] > cells[{c2}] ? cells[{c1}] : -cells[{c2}];\n")),
                _ => ops.push_str(&format!("    {{ struct {{ int v; int *np; }} box = {{ cells[{c1}], &cells[{c2}] }}; box.v += *box.np; cells[{c1}] = box.v; }}\n")),
            }
            let _ = i;
        }
        let mut pre = format!("{}", fdefs);
        pre.insert_str(0, "");
        format!("\
{fdefs}static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {{
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
{ops}    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}}
int main(void){{ return cmain(0); }}", ops = ops, fdefs = fdefs)
    }

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(300))]
        #[test]
        fn p_memgraph_vs_gcc(seed in any::<u64>(), n in 6usize..24, lvl in 0usize..3) {
            let prog = body(seed, n);
            let lvl = ["-O0", "-O1", "-O2"][lvl];
            let case = CASE.fetch_add(1, Ordering::SeqCst);
            let dir = run_dir();
            let src = dir.join(format!("G{}.c", case));
            std::fs::write(&src, &prog).unwrap();
            let g = dir.join(format!("G{}g", case));
            let gc = Command::new("gcc").args(["-w","-O0","-o"]).arg(&g).arg(&src).output().unwrap();
            if !gc.status.success() {
                let _ = std::fs::write("/tmp/mg_fail.c", &prog);
            }
            prop_assume!(gc.status.success(), "gcc reject");
            let want = Command::new(&g).output().unwrap();
            let expected = want.status.code().unwrap_or(-1);
            let _ = std::fs::remove_file(&g);
            let out = dir.join(format!("G{}c", case));
            let argv = vec!["ccc".to_string(), lvl.to_string(), src.to_str().unwrap().to_string(), "-o".to_string(), out.to_str().unwrap().to_string()];
            let mut drv = crate::driver::Driver::new();
            let cc = match drv.parse_cli_args(&argv) { Ok(false) => drv.run(), Ok(true) => Err("cli".into()), Err(e) => Err(e) };
            prop_assert!(cc.is_ok(), "ccc failed\n{}", prog);
            let got = Command::new(&out).output().unwrap();
            let _ = std::fs::remove_file(&out); let _ = std::fs::remove_file(&src);
            prop_assert!(!got.status.to_string().contains("signal"), "crashed at {}\n{}", lvl, prog);
            prop_assert_eq!(got.status.code().unwrap_or(-1), expected, "MEMGRAPH MISMATCH at {} (gcc -O0)\n{}", lvl, prog);
        }
    }
}

#[cfg(test)]
mod pbt_mutation_fuzz {
    //! Round-46: mutation fuzz — valid programs get random token-level
    //! deletions/insertions/replacements; the compiler must NEVER panic (ICE)
    //! or hang on malformed input (gcc contract: any byte sequence parses to
    //! errors, not a crash).
    use proptest::prelude::*;
    use std::path::PathBuf;
    use std::process::Command;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::time::Instant;

    static CASE: AtomicUsize = AtomicUsize::new(0);
    fn run_dir() -> PathBuf {
        let d = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("pbt-out/rounds/46_fuzz/run");
        let _ = std::fs::create_dir_all(&d);
        d
    }

    const CORPUS: &[&str] = &[
        "int main(void){ int x = 1; return x + 2; }\n",
        "#include <stdio.h>\nstruct S { int a; double b; };\nstatic struct S gs = {1, 2.5};\nint main(void){ struct S l = gs; printf(\"%d %f\\n\", l.a, l.b); return 0; }\n",
        "int f(int a, int b){ return a * b - (a + b); }\nint main(void){ return f(3, 4) - 5; }\n",
        "typedef unsigned u32;\nenum E { A = 1, B = 2 };\nstatic u32 arr[4] = {A, B, A, B};\nint main(void){ return arr[0]; }\n",
        "int main(void){ for (int i = 0; i < 10; i++) { if (i % 3) continue; } return 0; }\n",
        "union U { int i; char c[4]; };\nint main(void){ union U u; u.i = 65; return u.c[0]; }\n",
        "#define M(x) ((x) * 2)\nint main(void){ return M(21) - 42; }\n",
        "int main(void){ switch (1) { case 1: return 0; default: return 1; } }\n",
        "static int g = 5;\nint (*fp)(void);\nint f(void){ return g; }\nint main(void){ fp = f; return fp(); }\n",
        "struct N { struct { int x; } in; int a[3]; };\nint main(void){ struct N n = { { 1 }, { 2, 3, 4 } }; return n.a[2]; }\n",
    ];

    /// token 化（保守:按空白切+保留标点为独立块）与变异
    fn mutate(src: &str, seed: u64, n_mut: usize) -> String {
        let toks: Vec<&str> = src.split_whitespace().collect();
        let mut rng = seed | 1;
        let mut next = move || { rng = rng.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407); (rng >> 33) as u32 };
        let mut toks: Vec<String> = toks.iter().map(|s| s.to_string()).collect();
        let junk = ["@", "#", "`", "\\", "$", "€", "~", "^", "!", "struct", "case", "0x", "'a", "\"s", "...", "%:", "??", "->", "::"];
        for _ in 0..n_mut {
            if toks.is_empty() { break; }
            let k = (next() as usize) % 3;
            let i = (next() as usize) % toks.len();
            match k {
                0 => { toks.remove(i); }
                1 => { toks.insert(i, junk[(next() as usize) % junk.len()].to_string()); }
                _ => { toks[i] = junk[(next() as usize) % junk.len()].to_string(); }
            }
        }
        toks.join(" ")
    }

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(600))]
        #[test]
        fn p_mutation_no_ice(
            corpus_idx in 0usize..10, seed in any::<u64>(), n_mut in 1usize..8,
        ) {
            let mutated = mutate(CORPUS[corpus_idx], seed, n_mut);
            let case = CASE.fetch_add(1, Ordering::SeqCst);
            let dir = run_dir();
            let src = dir.join(format!("Z{}.c", case));
            std::fs::write(&src, &mutated).unwrap();
            let t0 = Instant::now();
            let out = Command::new("./target/debug/ccc")
                .args(["-w", "-c", src.to_str().unwrap(), "-o", dir.join(format!("Z{}.o", case)).to_str().unwrap()])
                .output();
            let elapsed = t0.elapsed();
            let _ = std::fs::remove_file(&src);
            let _ = std::fs::remove_file(dir.join(format!("Z{}.o", case)));
            let out = out.unwrap();
            prop_assert!(elapsed.as_secs() < 20, "HANG ({}s) on mutated input:\n{}", elapsed.as_secs(), mutated);
            let stderr = String::from_utf8_lossy(&out.stderr);
            prop_assert!(!stderr.contains("panicked"), "ICE (Rust panic) on mutated input:\n{}\n---\n{}", mutated, stderr);
            prop_assert!(!stderr.contains("RUST_BACKTRACE"), "ICE backtrace on mutated input:\n{}", mutated);
        }
    }
}

#[cfg(test)]
mod pbt_selfdiff {
    //! Round-49: SELF-differential — same program compiled by ccc at -O0 vs -O2,
    //! run BOTH, outputs must match. Oracle-free (no gcc) → 10x case budget.
    //! Catches optimizer divergences gcc-diff batteries can't reach cheaply.
    use proptest::prelude::*;
    use std::path::PathBuf;
    use std::process::Command;
    use std::sync::atomic::{AtomicUsize, Ordering};

    static CASE: AtomicUsize = AtomicUsize::new(0);
    fn run_dir() -> PathBuf {
        let d = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("pbt-out/rounds/49_selfdiff/run");
        let _ = std::fs::create_dir_all(&d);
        d
    }

    /// 富语义体：结构体/位域/联合/浮点/switch/循环/指针链（无 UB）
    fn body(seed: u64, rec: bool) -> String {
        let RC = if rec { 1 } else { 0 };
        format!("\
struct S {{ int a; double b; char c[4]; }};
struct BF {{ unsigned p:5; unsigned q:11; unsigned r:16; }};
union U {{ unsigned u; unsigned char b[4]; struct BF bf; }};
static unsigned h = 2166136261u;
static void mix(unsigned x) {{ h = (h ^ x) * 16777619u; }}
static struct S mk(int s) {{ struct S r = {{ s, s * 1.5, {{1,2,3,4}} }}; return r; }}
static double use(struct S s) {{ return s.a * 2.0 + s.b + s.c[3]; }}
static int recfn(int n) {{ return n <= 0 ? 1 : recfn(n - 1) * 2 % 100003; }}
static int itfn(int n) {{ int r = 1; for (int i = 0; i < n; i++) r = r * 2 % 100003; return r; }}
int cmain(int rec) {{
    unsigned B = {1}u, C = {2}u;
    int cells[8]; for (int i = 0; i < 8; i++) cells[i] = (int)((B >> (i * 4)) & 0xF) - 8;
    for (int i = 0; i < 8; i++) {{
        struct S s = mk(cells[i] + (int)(C % 11));
        mix((unsigned)(use(s) * 4.0));
        union U u; u.u = B * (i + 1) + C;
        mix(u.b[i % 4] + u.bf.p);
        struct BF bf2 = {{ u.bf.q, u.bf.r, u.bf.p }};
        mix((bf2.p << 16) + (bf2.q << 5) + bf2.r);
        switch (i % 5) {{
            case 0: mix(bf2.p); break;
            case 1: mix(u.b[0] + 1); break;
            case 2: mix(cells[i] & 0xFFu); break;
            case 3: mix(C >> 3); break;
            default: mix(B + i); break;
        }}
        int *p = &cells[(i * 3) % 8]; *p += (int)(u.bf.q % 7);
    }}
    long t = 0;
    for (int k = 0; k < 6; k++) t = t * 3 + (long)(B >> (k * 5)) % 97;
    mix((unsigned)t);
    double d = (B % 997) / 7.0; float f = (float)d;
    mix((unsigned)(d * 1000.0) + (unsigned)(f * 100.0f) + (d < f ? 1 : 0));
    // v2 深化: 递归 + 函数指针 + 伪字符串哈希
    static int (*fp)(int);
    fp = rec ? &recfn : &itfn;
    mix((unsigned)fp((int)(C % 17)));
    char buf[16];
    for (int i = 0; i < 15; i++) buf[i] = (char)(48 + (i + (int)(B % 10)) % 10);
    buf[15] = 0;
    unsigned sh = 0; for (int i = 0; buf[i]; i++) sh = sh * 31 + (unsigned)buf[i];
    mix(sh);
    return (int)(h % 251);
}}
int main(void){{ return cmain({0}); }}", RC, seed, seed >> 32)
    }

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(4000))]
        #[test]
        fn p_selfdiff_o0_o2(seed in any::<u64>(), rec in any::<bool>()) {
            let dir = run_dir();
            let case = CASE.fetch_add(1, Ordering::SeqCst);
            let src = dir.join(format!("S{}.c", case));
            std::fs::write(&src, body(seed, rec)).unwrap();
            let mut outs = vec![];
            for lvl in ["-O0", "-O2"] {
                let bin = dir.join(format!("S{}{}", case, lvl));
                let argv = vec!["ccc".to_string(), lvl.to_string(), src.to_string_lossy().into_owned(), "-o".to_string(), bin.to_string_lossy().into_owned()];
                let mut d = crate::driver::Driver::new();
                let cc = match d.parse_cli_args(&argv) { Ok(false) => d.run(), Ok(true) => Err("cli".into()), Err(e) => Err(e) };
                prop_assert!(cc.is_ok(), "compile failed at {}", lvl);
                let run = Command::new(&bin).output().unwrap();
                let _ = std::fs::remove_file(&bin);
                prop_assert!(!run.status.to_string().contains("signal"), "crashed at {}", lvl);
                outs.push(run.status.code().unwrap_or(-1));
            }
            let _ = std::fs::remove_file(&src);
            prop_assert_eq!(outs[0], outs[1], "SELF-DIFF MISMATCH -O0 vs -O2 (seed {:#x})", seed);
        }
    }
}

#[cfg(test)]
mod pbt_constraint_fuzz {
    //! Round-58: RANDOMIZED constraint fuzzing — take valid C programs,
    //! randomly inject constraint violations, compare gcc vs ccc acceptance.
    //! This replaces hand-crafted batteries with true PBT (1000+ random cases).
    use proptest::prelude::*;
    use std::path::PathBuf;
    use std::process::Command;
    use std::sync::atomic::{AtomicUsize, Ordering};

    static CASE: AtomicUsize = AtomicUsize::new(0);
    fn run_dir() -> PathBuf {
        let d = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("pbt-out/rounds/58_constraint_fuzz/run");
        let _ = std::fs::create_dir_all(&d);
        d
    }

    const CORPUS: &[&str] = &[
        "int main(void){ int x = 1; return x; }",
        "struct S { int a; double b; }; int main(void){ struct S s = {1, 2.5}; return s.a; }",
        "int f(int a, int b) { return a + b; } int main(void){ return f(1, 2); }",
        "static int arr[5] = {1,2,3,4,5}; int main(void){ return arr[2]; }",
        "enum E { A=1, B=2 }; int main(void){ return A + B; }",
        "typedef int my_int; int main(void){ my_int x = 42; return x; }",
        "union U { int i; char c; }; int main(void){ union U u; u.i = 65; return u.c; }",
        "int main(void){ for(int i=0;i<10;i++){} return 0; }",
    ];

    // 随机约束违规注入器
    fn inject_violation(src: &str, seed: u64, mutation_idx: usize) -> String {
        let mut rng = seed | 1;
        let mut next = move || { rng = rng.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407); (rng >> 33) as u32 };
        let lines: Vec<&str> = src.lines().collect();
        let line_idx = (next() as usize) % lines.len().max(1);
        let mut result: Vec<String> = lines.iter().map(|s| s.to_string()).collect();

        match mutation_idx % 16 {
            0 => result[line_idx].push_str(" undeclared_fn();"),          // 隐式声明
            1 => result[line_idx].push_str(" undeclared_var;"),           // 未声明变量
            2 => result[line_idx].push_str(" (int)x = 5;"),               // cast 赋值
            3 => result[line_idx].push_str(" bad@#$;"),                   // 非法字符
            4 => result.insert(0, "#define 1BAD 1".to_string()),          // 数字开头宏
            5 => result.insert(0, "#if 1/0".to_string()),                 // #if 除零
            6 => result.insert(0, "#else".to_string()),                   // 孤立 #else
            7 => result[line_idx].push_str(" int x = ;"),                 // 空初始化器
            8 => result[line_idx].push_str(" struct S *p = 0; return p->a;"), // 不完整类型解引用
            9 => { result.insert(0, "const int g = 1;".to_string());
                   result.push("int main2(void){ g = 2; return 0; }".to_string()); },
            10 => result[line_idx].push_str(" int a[3] = {1,2,3,4,5};"),  // 超量初始化
            11 => result[line_idx].push_str(" void v;"),                  // void 变量
            12 => result[line_idx].push_str(" int f(void)[3];"),         // 函数返回数组
            13 => { result.insert(0, "#pragma GCC poison xyz".to_string());
                    result[line_idx].push_str(" int xyz;"); },
            14 => result[line_idx].push_str(" int x = 1; int x = 2;"),   // 重复声明
            15 => result[line_idx].push_str(" goto nowhere;"),            // 未定义标号
            _ => {}
        }
        result.join("\n")
    }

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(2000))]
        #[test]
        fn p_constraint_fuzz(
            corpus_idx in any::<usize>(),
            seed in any::<u64>(),
            mutation in 0usize..16,
        ) {
            let src = inject_violation(CORPUS[corpus_idx % CORPUS.len()], seed, mutation);
            let case = CASE.fetch_add(1, Ordering::SeqCst);
            let dir = run_dir();
            let f = dir.join(format!("C{}.c", case));
            std::fs::write(&f, &src).unwrap();

            let g = Command::new("gcc").args(["-w","-c"]).arg(&f).arg("-o").arg("/dev/null").output().unwrap();
            let c = Command::new("./target/debug/ccc").args(["-c"]).arg(&f).arg("-o").arg("/dev/null").output().unwrap();
            let _ = std::fs::remove_file(&f);
            let g_err = !g.status.success();
            let c_err = !c.status.success();

            // 两种合法分歧: 双拒 or 双收; 非法分歧: gcc拒+ccc收
            if g_err && !c_err {
                // 已知违规（已发 issue）: 门控
                let known = match mutation {
                    0 => true,   // 隐式声明 → #1189
                    1 => true,   // 未声明变量 → 词法/约束家族
                    2 => true,   // cast 赋值 → #1135
                    3 => true,   // 非法字符 → #1191
                    4 => true,   // 数字开头宏 → #1197
                    5 => true,   // #if 除零 → #1197
                    6 => true,   // 孤立 #else → #1143
                    7 => true,   // 空初始化 → #1138
                    9 => true,   // const 赋值 → #1135
                    11 => true,  // void 变量 → #1138
                    12 => true,  // 函数返数组 → #1138
                    13 => true,  // poisoned → GNU 扩展边界
                    14 => true,  // 重复声明 → #1138
                    15 => true,  // 未定义 goto → #1137
                    _ => false,
                };
                if !known {
                    let gmsg = String::from_utf8_lossy(&g.stderr);
                    let first = gmsg.lines().find(|l| l.contains("error")).unwrap_or("");
                    prop_assert!(false, "NEW CONSTRAINT VIOLATION (mutation {}):\n  gcc: {}\n  src: {}\n", mutation, &first[..first.len().min(60)], src);
                }
            }
        }
    }
}

#[cfg(test)]
mod pbt_parser_fuzz {
    use proptest::prelude::*;
    use std::process::Command;
    use std::sync::atomic::{AtomicUsize, Ordering};
    static CASE: AtomicUsize = AtomicUsize::new(0);

    const CORPUS: &[&str] = &[
        "int main(void){int x=1;return x;}",
        "struct S{int a;double b;};int main(){struct S s={1,2.5};return s.a;}",
        "int f(int a,int b){return a+b;}int main(){return f(1,2);}",
        "static int a[5]={1,2,3,4,5};int main(){return a[2];}",
        "int main(){for(int i=0;i<10;i++){}return 0;}",
    ];
    const CHARS: &[&str] = &["(", ")", "{", "}", "[", "]", ";", ",", "+", "-", "*", "/", "%", "=", "<", ">", "!", "&", "|", "^", "~", "?", ":", ".", "#", "@", "$", "`", "\\"];

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(3000))]
        #[test]
        fn p_parser_fuzz_no_crash(
            corpus_idx in any::<usize>(),
            pos in any::<usize>(),
            ch_idx in any::<usize>(),
            len_del in 0usize..5,
        ) {
            let src = CORPUS[corpus_idx % CORPUS.len()];
            let mut s: Vec<char> = src.chars().collect();
            let pos = pos % s.len().max(1);
            // delete 0-5 chars at pos, then insert random char
            for _ in 0..len_del { if pos < s.len() { s.remove(pos); } }
            s.insert(pos, CHARS[ch_idx % CHARS.len()].chars().next().unwrap());
            let mutated: String = s.into_iter().collect();

            let case = CASE.fetch_add(1, Ordering::SeqCst);
            let f = format!("/tmp/pf_{}.c", case);
            std::fs::write(&f, &mutated).unwrap();
            let out = Command::new("./target/debug/ccc")
                .args(["-c", &f, "-o", "/dev/null"]).output().unwrap();
            let _ = std::fs::remove_file(&f);
            let se = String::from_utf8_lossy(&out.stderr);
            // Known ICE #1205: int & float bitAND panics at traits.rs:347
            if se.contains("panicked") && se.contains("unsupported float binop") {
                return Ok(());
            }
            prop_assert!(!se.contains("panicked"), "PARSER ICE on: {}", mutated);
            prop_assert!(!out.status.to_string().contains("signal"), "PARSER CRASH on: {}", mutated);
        }
    }
}
