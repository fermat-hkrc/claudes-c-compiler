//! Phi elimination: lower SSA phi nodes to copies in predecessor blocks.
//!
//! This pass runs after all SSA optimizations and before backend codegen.
//! It converts each Phi instruction into Copy instructions placed at the end
//! of each predecessor block (before the terminator).
//!
//! Smart temporary allocation:
//! When a block has multiple phis, we analyze the copy graph per-predecessor
//! to determine which copies actually need temporaries (only those involved
//! in cycles, e.g., swap patterns) and which can be direct copies. This
//! dramatically reduces the number of temporaries and copy instructions,
//! especially for large switch statements where most phis pass through
//! values unchanged.
//!
//! For non-conflicting phis (the common case), we emit direct copies:
//!   pred_block:
//!     %phi1_dest = copy src1
//!     %phi2_dest = copy src2
//!     <terminator>
//!
//! For conflicting phis (cycles), we use shared temporaries and a two-phase
//! copy sequence to avoid the lost-copy problem:
//!   pred_block:
//!     %tmp1 = copy src1  // save source before it's overwritten
//!     <terminator>
//!   target_block:
//!     %phi1_dest = copy %tmp1  // restore from temporary
//!     ... rest of block ...
//!
//! Critical edge splitting:
//! When a predecessor block has multiple successors (e.g., a CondBranch) and
//! the target block has phis, placing copies at the end of the predecessor
//! would execute them on ALL outgoing paths, not just the edge to the phi's
//! block. This corrupts values used on other paths. To fix this, we split
//! the critical edge by inserting a new trampoline block that contains only
//! the phi copies and branches unconditionally to the target.

use crate::common::fx_hash::{FxHashMap, FxHashSet};
use crate::ir::reexports::{
    BasicBlock,
    BlockId,
    Instruction,
    IrFunction,
    IrModule,
    Operand,
    Terminator,
    Value,
};

/// Eliminate all phi nodes in the module by lowering them to copies.
pub fn eliminate_phis(module: &mut IrModule) {
    // Compute the global max block ID across ALL functions to avoid label collisions
    // when creating trampoline blocks. Labels are module-wide (.LBB0, .LBB1, ...).
    let mut next_block_id = 0u32;
    for func in &module.functions {
        for block in &func.blocks {
            if block.label.0 >= next_block_id {
                next_block_id = block.label.0 + 1;
            }
        }
    }

    for func in &mut module.functions {
        if func.is_declaration || func.blocks.is_empty() {
            continue;
        }
        eliminate_phis_in_function(func, &mut next_block_id);
    }
}

/// Returns the number of distinct successor block IDs for a block.
/// Accounts for both terminator targets and InlineAsm goto_labels.
fn successor_count(block: &BasicBlock) -> usize {
    let mut seen: Vec<BlockId> = Vec::new();
    match &block.terminator {
        Terminator::Return(_) | Terminator::Unreachable => {}
        Terminator::Branch(label) => { seen.push(*label); }
        Terminator::CondBranch { true_label, false_label, .. } => {
            seen.push(*true_label);
            if true_label != false_label { seen.push(*false_label); }
        }
        Terminator::IndirectBranch { possible_targets, .. } => {
            seen.extend_from_slice(possible_targets);
        }
        Terminator::Switch { cases, default, .. } => {
            seen.push(*default);
            for &(_, label) in cases {
                if !seen.contains(&label) { seen.push(label); }
            }
        }
    }
    // InlineAsm goto_labels are implicit control flow edges.
    for inst in &block.instructions {
        if let Instruction::InlineAsm { goto_labels, .. } = inst {
            for (_, label) in goto_labels {
                if !seen.contains(label) { seen.push(*label); }
            }
        }
    }
    seen.len()
}

/// Replace one occurrence of `old_target` with `new_target` in a block's
/// terminator or InlineAsm goto_labels.
fn retarget_block_edge_once(block: &mut BasicBlock, old_target: BlockId, new_target: BlockId) {
    match &mut block.terminator {
        Terminator::Branch(t) => {
            if *t == old_target {
                *t = new_target;
                return;
            }
        }
        Terminator::CondBranch { true_label, false_label, .. } => {
            // Only retarget one edge to avoid changing both sides of a diamond
            if *true_label == old_target {
                *true_label = new_target;
                return;
            } else if *false_label == old_target {
                *false_label = new_target;
                return;
            }
        }
        Terminator::IndirectBranch { possible_targets, .. } => {
            for t in possible_targets.iter_mut() {
                if *t == old_target {
                    *t = new_target;
                    return;
                }
            }
        }
        Terminator::Switch { cases, default, .. } => {
            if *default == old_target {
                *default = new_target;
                return;
            } else {
                for (_, t) in cases.iter_mut() {
                    if *t == old_target {
                        *t = new_target;
                        return;
                    }
                }
            }
        }
        _ => {}
    }
    // Check InlineAsm goto_labels for implicit control flow edges.
    for inst in &mut block.instructions {
        if let Instruction::InlineAsm { goto_labels, .. } = inst {
            for (_, label) in goto_labels.iter_mut() {
                if *label == old_target {
                    *label = new_target;
                    return;
                }
            }
        }
    }
}

struct TrampolineBlock {
    label: BlockId,
    copies: Vec<Instruction>,
    branch_target: BlockId,
    pred_idx: usize,
    old_target: BlockId,
}

/// Get or create a trampoline block for a (pred, target) critical edge.
fn get_or_create_trampoline(
    trampoline_map: &mut FxHashMap<(usize, BlockId), usize>,
    trampolines: &mut Vec<TrampolineBlock>,
    pred_idx: usize,
    target_block_id: BlockId,
    next_block_id: &mut u32,
) -> usize {
    *trampoline_map
        .entry((pred_idx, target_block_id))
        .or_insert_with(|| {
            let idx = trampolines.len();
            let label = BlockId(*next_block_id);
            *next_block_id += 1;
            trampolines.push(TrampolineBlock {
                label,
                copies: Vec::new(),
                branch_target: target_block_id,
                pred_idx,
                old_target: target_block_id,
            });
            idx
        })
}

/// Determine which phi copies on a given predecessor edge need temporaries.
///
/// A copy `dest_i = src_i` needs a temporary if `src_i` is the destination
/// of another phi copy (i.e., another phi writes to the value we're reading).
/// This detects interference in the copy graph (e.g., swap patterns: x=y, y=x).
///
/// Returns a set of phi indices that need temporaries.
fn find_conflicting_phis(copies: &[(u32, Option<u32>)]) -> FxHashSet<usize> {
    // Build set of all phi destinations for this edge.
    let dest_set: FxHashSet<u32> = copies.iter().map(|(d, _)| *d).collect();

    // A phi copy needs a temporary if its source is overwritten by another phi
    // destination on this same edge. Specifically, phi i needs a temp if
    // src_i is in dest_set AND src_i != dest_i (self-copies don't conflict).
    let mut needs_temp: FxHashSet<usize> = FxHashSet::default();

    for (i, &(dest_i, src_i_opt)) in copies.iter().enumerate() {
        if let Some(src_i) = src_i_opt {
            if src_i != dest_i && dest_set.contains(&src_i) {
                needs_temp.insert(i);
            }
        }
    }

    // Conservative safety net: also mark phis whose destination is read by a
    // conflicting phi. This ensures the two-phase ordering (temp saves in
    // Pass 1, direct copies in Pass 2) remains correct even for chain
    // patterns like `a = b, b = c, c = a` where multiple values are involved
    // in cycles. This may over-approximate slightly (e.g., marking a phi
    // whose destination is read but already saved by another temp), but
    // correctness is paramount.
    if !needs_temp.is_empty() {
        let conflicting_sources: FxHashSet<u32> = needs_temp.iter()
            .filter_map(|&i| copies[i].1)
            .collect();
        for (j, &(dest_j, _)) in copies.iter().enumerate() {
            if conflicting_sources.contains(&dest_j) {
                needs_temp.insert(j);
            }
        }
    }

    needs_temp
}

/// Phi information extracted from a block.
struct PhiInfo {
    dest: Value,
    incoming: Vec<(Operand, BlockId)>,
}

/// Context shared across phi elimination for a single function.
struct PhiElimCtx<'a> {
    label_to_idx: FxHashMap<BlockId, usize>,
    multi_succ: Vec<bool>,
    is_indirect_branch: Vec<bool>,
    pred_copies: FxHashMap<usize, Vec<Instruction>>,
    target_copies: Vec<Vec<Instruction>>,
    trampolines: Vec<TrampolineBlock>,
    trampoline_map: FxHashMap<(usize, BlockId), usize>,
    next_block_id: &'a mut u32,
    next_value: u32,
}

fn eliminate_phis_in_function(func: &mut IrFunction, next_block_id: &mut u32) {
    let mut ctx = PhiElimCtx {
        label_to_idx: func.blocks.iter().enumerate().map(|(i, b)| (b.label, i)).collect(),
        multi_succ: func.blocks.iter().map(|b| successor_count(b) > 1).collect(),
        is_indirect_branch: func.blocks.iter()
            .map(|b| matches!(&b.terminator, Terminator::IndirectBranch { .. })).collect(),
        pred_copies: FxHashMap::default(),
        target_copies: vec![Vec::new(); func.blocks.len()],
        trampolines: Vec::new(),
        trampoline_map: FxHashMap::default(),
        next_block_id,
        next_value: if func.next_value_id > 0 { func.next_value_id } else { func.max_value_id() + 1 },
    };

    let block_phis = collect_block_phis(func);

    for (block_idx, phis) in block_phis.iter().enumerate() {
        if phis.is_empty() {
            continue;
        }
        let target_block_id = func.blocks[block_idx].label;
        if phis.len() == 1 {
            emit_single_phi_copies(&phis[0], target_block_id, &mut ctx);
        } else {
            emit_multi_phi_copies(phis, block_idx, target_block_id, &mut ctx);
        }
    }

    apply_phi_transformations(func, &mut ctx);
    func.next_value_id = ctx.next_value;
}

/// Collect PhiInfo from all blocks.
fn collect_block_phis(func: &IrFunction) -> Vec<Vec<PhiInfo>> {
    func.blocks.iter().map(|block| {
        block.instructions.iter().filter_map(|inst| {
            if let Instruction::Phi { dest, incoming, .. } = inst {
                Some(PhiInfo { dest: *dest, incoming: incoming.clone() })
            } else {
                None
            }
        }).collect()
    }).collect()
}

/// Emit copies for a block with a single phi (no temporaries needed).
fn emit_single_phi_copies(phi: &PhiInfo, target_block_id: BlockId, ctx: &mut PhiElimCtx) {
    for (src, pred_label) in &phi.incoming {
        let pred_idx = match ctx.label_to_idx.get(pred_label) {
            Some(&idx) => idx,
            None => continue,
        };
        // Skip self-copies
        if let Operand::Value(v) = src {
            if v.0 == phi.dest.0 {
                continue;
            }
        }
        let copy_inst = Instruction::Copy { dest: phi.dest, src: *src };
        place_copy(ctx, pred_idx, target_block_id, copy_inst);
    }
}

/// Emit copies for a block with multiple phis, using smart temporary allocation.
/// Shared temporaries are only allocated for phis involved in copy cycles.
fn emit_multi_phi_copies(
    phis: &[PhiInfo], block_idx: usize, target_block_id: BlockId,
    ctx: &mut PhiElimCtx,
) {
    // Collect unique predecessor labels.
    let mut pred_label_set: FxHashSet<BlockId> = FxHashSet::default();
    let mut pred_labels: Vec<BlockId> = Vec::new();
    for phi in phis {
        for (_, pred_label) in &phi.incoming {
            if pred_label_set.insert(*pred_label) {
                pred_labels.push(*pred_label);
            }
        }
    }

    // Precompute per-phi source lookup tables.
    let phi_src_maps: Vec<FxHashMap<BlockId, &Operand>> = phis.iter()
        .map(|phi| phi.incoming.iter().map(|(src, pl)| (*pl, src)).collect())
        .collect();

    // Find which phis are globally conflicting (need temporaries on any edge).
    let globally_needs_temp = find_globally_conflicting_phis(
        phis, &pred_labels, &phi_src_maps, &ctx.label_to_idx,
    );

    // Allocate shared temporaries.
    let mut phi_temps: Vec<Option<Value>> = vec![None; phis.len()];
    for &i in &globally_needs_temp {
        phi_temps[i] = Some(Value(ctx.next_value));
        ctx.next_value += 1;
    }

    // Emit target block copies for conflicting phis (temp -> dest).
    for (i, phi) in phis.iter().enumerate() {
        if let Some(tmp) = phi_temps[i] {
            ctx.target_copies[block_idx].push(Instruction::Copy {
                dest: phi.dest,
                src: Operand::Value(tmp),
            });
        }
    }

    // Emit copies for each predecessor edge.
    for pred_label in &pred_labels {
        let pred_idx = match ctx.label_to_idx.get(pred_label) {
            Some(&idx) => idx,
            None => continue,
        };
        let edge_copies = build_edge_copies(phis, &phi_temps, &phi_src_maps, pred_label);
        place_copies(ctx, pred_idx, target_block_id, edge_copies);
    }
}

/// Determine which phi indices are globally conflicting across all predecessor edges.
fn find_globally_conflicting_phis(
    phis: &[PhiInfo],
    pred_labels: &[BlockId],
    phi_src_maps: &[FxHashMap<BlockId, &Operand>],
    label_to_idx: &FxHashMap<BlockId, usize>,
) -> FxHashSet<usize> {
    let mut globally_needs_temp: FxHashSet<usize> = FxHashSet::default();
    for pred_label in pred_labels {
        if !label_to_idx.contains_key(pred_label) {
            continue;
        }
        let copies_info: Vec<(u32, Option<u32>)> = phis.iter().enumerate().map(|(i, phi)| {
            let src_val_id = phi_src_maps[i].get(pred_label).and_then(|s| {
                if let Operand::Value(v) = *s { Some(v.0) } else { None }
            });
            (phi.dest.0, src_val_id)
        }).collect();
        for &i in &find_conflicting_phis(&copies_info) {
            globally_needs_temp.insert(i);
        }
    }
    globally_needs_temp
}

/// Build the ordered copy instructions for a single predecessor edge:
/// Pass 1 (temp saves for conflicting phis), then Pass 2 (direct copies).
fn build_edge_copies(
    phis: &[PhiInfo],
    phi_temps: &[Option<Value>],
    phi_src_maps: &[FxHashMap<BlockId, &Operand>],
    pred_label: &BlockId,
) -> Vec<Instruction> {
    let mut copies = Vec::new();

    // Pass 1: Emit temporary saves for conflicting phis (must come first).
    for (i, _phi) in phis.iter().enumerate() {
        if let Some(tmp) = phi_temps[i] {
            if let Some(src) = phi_src_maps[i].get(pred_label) {
                copies.push(Instruction::Copy { dest: tmp, src: *(*src) });
            }
        }
    }

    // Pass 2: Emit direct copies for non-conflicting phis.
    for (i, phi) in phis.iter().enumerate() {
        if phi_temps[i].is_none() {
            if let Some(src) = phi_src_maps[i].get(pred_label) {
                if let Operand::Value(v) = *src {
                    if v.0 == phi.dest.0 { continue; } // skip self-copy
                }
                copies.push(Instruction::Copy { dest: phi.dest, src: *(*src) });
            }
        }
    }

    copies
}

/// Place a single copy instruction, using trampolines for critical edges.
fn place_copy(ctx: &mut PhiElimCtx, pred_idx: usize, target_block_id: BlockId, copy_inst: Instruction) {
    if ctx.multi_succ[pred_idx] && !ctx.is_indirect_branch[pred_idx] {
        let tramp_idx = get_or_create_trampoline(
            &mut ctx.trampoline_map, &mut ctx.trampolines,
            pred_idx, target_block_id, ctx.next_block_id,
        );
        ctx.trampolines[tramp_idx].copies.push(copy_inst);
    } else {
        ctx.pred_copies.entry(pred_idx).or_default().push(copy_inst);
    }
}

/// Place multiple copy instructions, using trampolines for critical edges.
fn place_copies(ctx: &mut PhiElimCtx, pred_idx: usize, target_block_id: BlockId, copies: Vec<Instruction>) {
    if copies.is_empty() { return; }
    if ctx.multi_succ[pred_idx] && !ctx.is_indirect_branch[pred_idx] {
        let tramp_idx = get_or_create_trampoline(
            &mut ctx.trampoline_map, &mut ctx.trampolines,
            pred_idx, target_block_id, ctx.next_block_id,
        );
        ctx.trampolines[tramp_idx].copies.extend(copies);
    } else {
        ctx.pred_copies.entry(pred_idx).or_default().extend(copies);
    }
}

/// Apply all phi elimination transformations to the function:
/// remove phis, insert copies, retarget terminators, add trampolines.
fn apply_phi_transformations(func: &mut IrFunction, ctx: &mut PhiElimCtx) {
    for (block_idx, block) in func.blocks.iter_mut().enumerate() {
        // Remove phi instructions (and their spans)
        if !block.source_spans.is_empty() {
            let mut span_idx = 0;
            block.source_spans.retain(|_| {
                let keep = !matches!(block.instructions.get(span_idx), Some(Instruction::Phi { .. }));
                span_idx += 1;
                keep
            });
        }
        block.instructions.retain(|inst| !matches!(inst, Instruction::Phi { .. }));

        // Prepend target copies (these go at the start, replacing the phis)
        if !ctx.target_copies[block_idx].is_empty() {
            let num_copies = ctx.target_copies[block_idx].len();
            let mut new_insts = ctx.target_copies[block_idx].clone();
            new_insts.append(&mut block.instructions);
            block.instructions = new_insts;
            if !block.source_spans.is_empty() {
                let mut new_spans = vec![crate::common::source::Span::dummy(); num_copies];
                new_spans.append(&mut block.source_spans);
                block.source_spans = new_spans;
            }
        }

        // Insert predecessor copies before terminator
        if let Some(copies) = ctx.pred_copies.remove(&block_idx) {
            let num_copies = copies.len();
            block.instructions.extend(copies);
            if !block.source_spans.is_empty() {
                block.source_spans.extend(std::iter::repeat_n(crate::common::source::Span::dummy(), num_copies));
            }
        }
    }

    // Retarget predecessors that need trampolines
    for trampoline in &ctx.trampolines {
        retarget_block_edge_once(
            &mut func.blocks[trampoline.pred_idx],
            trampoline.old_target,
            trampoline.label,
        );
    }

    // Append trampoline blocks to the function
    for trampoline in std::mem::take(&mut ctx.trampolines) {
        let num_copies = trampoline.copies.len();
        func.blocks.push(BasicBlock {
            label: trampoline.label,
            instructions: trampoline.copies,
            source_spans: vec![crate::common::source::Span::dummy(); num_copies],
            terminator: Terminator::Branch(trampoline.branch_target),
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no_conflicts_independent_phis() {
        // a = 1, b = 2 — no overlap between dests and sources
        let copies = vec![(10, Some(1)), (20, Some(2))];
        let result = find_conflicting_phis(&copies);
        assert!(result.is_empty(), "Independent phis should have no conflicts");
    }

    #[test]
    fn test_swap_pattern() {
        // a = b, b = a — classic swap cycle
        let copies = vec![(10, Some(20)), (20, Some(10))];
        let result = find_conflicting_phis(&copies);
        assert!(result.contains(&0), "First phi in swap should be conflicting");
        assert!(result.contains(&1), "Second phi in swap should be conflicting");
    }

    #[test]
    fn test_three_way_cycle() {
        // a = b, b = c, c = a — three-way rotation
        let copies = vec![(10, Some(20)), (20, Some(30)), (30, Some(10))];
        let result = find_conflicting_phis(&copies);
        assert_eq!(result.len(), 3, "All three phis in a 3-way cycle should be conflicting");
    }

    #[test]
    fn test_mixed_conflicting_and_non_conflicting() {
        // a = b, b = a (conflict), c = 99 (independent)
        let copies = vec![(10, Some(20)), (20, Some(10)), (30, Some(99))];
        let result = find_conflicting_phis(&copies);
        assert!(result.contains(&0));
        assert!(result.contains(&1));
        assert!(!result.contains(&2), "Independent phi should not be marked conflicting");
    }

    #[test]
    fn test_self_copy_not_conflicting() {
        // a = a — self-copy, not a conflict
        let copies = vec![(10, Some(10)), (20, Some(30))];
        let result = find_conflicting_phis(&copies);
        assert!(result.is_empty(), "Self-copy should not be conflicting");
    }

    #[test]
    fn test_constant_source_not_conflicting() {
        // a = <const>, b = <const> — None sources (constants)
        let copies = vec![(10, None), (20, None)];
        let result = find_conflicting_phis(&copies);
        assert!(result.is_empty(), "Constant sources should have no conflicts");
    }

    #[test]
    fn test_chain_pattern_conservative() {
        // a = b, b = c — b is dest of phi 1 and source of phi 0
        // phi 0 reads b (which is the dest of phi 1), so phi 0 is directly conflicting.
        // The safety net then marks phi 1 because its dest (b=20) is a source
        // of a conflicting phi.
        let copies = vec![(10, Some(20)), (20, Some(30))];
        let result = find_conflicting_phis(&copies);
        assert!(result.contains(&0), "Chain phi reading overwritten dest should be conflicting");
        assert!(result.contains(&1), "Chain phi whose dest is read by conflicting phi should also be marked");
    }
}

// ── PBT properties (round 06) ─────────────────────────────────────────────────
#[cfg(test)]
mod pbt_tests {
    use super::*;
    use crate::common::types::{AddressSpace, IrType};
    use crate::ir::reexports::IrConst;
    use crate::ir::mem2reg::promote_allocas_with_params;
    use proptest::prelude::*;
    use proptest::collection::vec;

    /// Random alloca-based IR (same shape as promote.rs pbt_tests::build_random_func).
    fn build_random_func(
        name: &str, n: usize, k: usize,
        ops: &[Vec<(u8, u8, u8, i64)>],
        terms: &[(u8, u32, u32)],
    ) -> IrFunction {
        let mut func = IrFunction::new(name.to_string(), IrType::I32, vec![], false);
        let mut next: u32 = k as u32;
        for bi in 0..n {
            let mut insts: Vec<Instruction> = vec![];
            let mut defined_here: Vec<u32> = vec![];
            if bi == 0 {
                for a in 0..k {
                    insts.push(Instruction::Alloca {
                        dest: Value(a as u32), ty: IrType::I32, size: 4, align: 4, volatile: false,
                    });
                }
            }
            if k > 0 {
                for &(op, which, vsrc, cval) in &ops[bi] {
                    let ptr = Value((which as usize % k) as u32);
                    if op % 2 == 0 {
                        let dest = Value(next);
                        next += 1;
                        insts.push(Instruction::Load { dest, ptr, ty: IrType::I32, seg_override: AddressSpace::Default });
                        defined_here.push(dest.0);
                    } else {
                        let val = if vsrc % 3 == 0 && !defined_here.is_empty() {
                            Operand::Value(Value(defined_here[(vsrc as usize) % defined_here.len()]))
                        } else {
                            Operand::Const(IrConst::I32(cval as i32))
                        };
                        insts.push(Instruction::Store { val, ptr, ty: IrType::I32, seg_override: AddressSpace::Default });
                    }
                }
            }
            let pick = |s: u32| -> Operand {
                if defined_here.is_empty() {
                    Operand::Const(IrConst::I32(1))
                } else {
                    Operand::Value(Value(defined_here[(s as usize) % defined_here.len()]))
                }
            };
            let nn = n as u32;
            let t = &terms[bi];
            let term = match t.0 % 4 {
                0 => Terminator::Return(Some(pick(t.1))),
                1 => Terminator::Branch(BlockId(t.1 % nn)),
                2 => Terminator::CondBranch {
                    cond: pick(t.1),
                    true_label: BlockId(t.1 % nn),
                    false_label: BlockId(t.2 % nn),
                },
                _ => Terminator::Switch {
                    val: pick(t.1),
                    cases: vec![(1, BlockId(t.1 % nn)), (2, BlockId(t.2 % nn))],
                    default: BlockId(t.1 % nn),
                    ty: IrType::I32,
                },
            };
            func.blocks.push(BasicBlock {
                label: BlockId(bi as u32),
                instructions: insts,
                terminator: term,
                source_spans: vec![],
            });
        }
        func
    }

    fn successor_labels(term: &Terminator) -> Vec<BlockId> {
        let mut out = vec![];
        match term {
            Terminator::Branch(t) => out.push(*t),
            Terminator::CondBranch { true_label, false_label, .. } => {
                out.push(*true_label);
                if true_label != false_label { out.push(*false_label); }
            }
            Terminator::Switch { cases, default, .. } => {
                out.push(*default);
                for (_, t) in cases {
                    if !out.contains(t) { out.push(*t); }
                }
            }
            _ => {}
        }
        out
    }

    /// P13: after promote + eliminate_phis the function must be phi-free and
    /// structurally sound: every terminator label resolves, every appended trampoline
    /// is a pure copy block branching to an original label and is referenced by its
    /// predecessor, and every former phi destination is still defined by some Copy
    /// instruction (the merged value is still produced).
    /// Doc contract: "It converts each Phi instruction into Copy instructions placed at
    /// the end of each predecessor block (before the terminator)." (a6a4eda7)
    proptest! {
        #![proptest_config(ProptestConfig::with_cases(512))]
        #[test]
        fn p13_phi_elimination_structural_preservation(
            n in 1usize..=8,
            k in 1usize..=4,
            ops in vec(
                vec((any::<u8>(), any::<u8>(), any::<u8>(), any::<i64>()), 0..=6),
                1..=8),
            terms in vec((any::<u8>(), any::<u32>(), any::<u32>()), 1..=8),
        ) {
            let n = n.min(ops.len()).min(terms.len());
            let ops = &ops[..n];
            let terms = &terms[..n];
            let mut module = IrModule::new();
            module.functions.push(build_random_func("f1", n, k, ops, terms));
            module.functions.push(build_random_func("f2", n.min(3), k.min(2), ops, terms));

            promote_allocas_with_params(&mut module);

            // snapshot phi dests (reachable blocks only — unreachable blocks keep dead
            // phis with empty incoming, dead code by construction) and original labels
            let snapshot: Vec<(std::collections::BTreeSet<u32>, std::collections::BTreeSet<u32>)> =
                module.functions.iter().map(|f| {
                    let idx_of: FxHashMap<BlockId, usize> =
                        f.blocks.iter().enumerate().map(|(i, b)| (b.label, i)).collect();
                    let mut reach = vec![false; f.blocks.len()];
                    if !f.blocks.is_empty() {
                        let mut stack = vec![0usize];
                        reach[0] = true;
                        while let Some(u) = stack.pop() {
                            for t in successor_labels(&f.blocks[u].terminator) {
                                if let Some(&ti) = idx_of.get(&t) {
                                    if !reach[ti] { reach[ti] = true; stack.push(ti); }
                                }
                            }
                        }
                    }
                    let mut phis: std::collections::BTreeSet<u32> = Default::default();
                    for (bi, b) in f.blocks.iter().enumerate() {
                        if !reach[bi] { continue; }
                        for i in &b.instructions {
                            if let Instruction::Phi { dest, .. } = i {
                                phis.insert(dest.0);
                            }
                        }
                    }
                    let labels: std::collections::BTreeSet<u32> = f.blocks.iter().map(|b| b.label.0).collect();
                    (phis, labels)
                }).collect();

            eliminate_phis(&mut module);

            for (fi, func) in module.functions.iter().enumerate() {
                let (phi_dests, orig_labels) = &snapshot[fi];
                let label_set: std::collections::BTreeSet<u32> = func.blocks.iter().map(|b| b.label.0).collect();
                let idx_of: FxHashMap<BlockId, usize> =
                    func.blocks.iter().enumerate().map(|(i, b)| (b.label, i)).collect();

                // (a) no phis remain
                for b in &func.blocks {
                    prop_assert!(!b.instructions.iter().any(|i| matches!(i, Instruction::Phi { .. })),
                        "phi survived elimination in {}", func.name);
                }

                // (b) every terminator label resolves
                for b in &func.blocks {
                    for t in successor_labels(&b.terminator) {
                        prop_assert!(idx_of.contains_key(&t),
                            "dangling label {:?} in {}", t, func.name);
                    }
                }

                // (c) trampoline blocks are pure copy blocks to original labels, referenced by their pred
                for b in &func.blocks {
                    if orig_labels.contains(&b.label.0) { continue; }
                    let Terminator::Branch(target) = &b.terminator else {
                        prop_assert!(false, "trampoline {} does not end in Branch", b.label.0);
                        continue;
                    };
                    prop_assert!(orig_labels.contains(&target.0),
                        "trampoline {} branches to non-original label {}", b.label.0, target.0);
                    for i in &b.instructions {
                        prop_assert!(matches!(i, Instruction::Copy { .. }),
                            "non-copy instruction in trampoline {}: {:?}", b.label.0, i);
                    }
                }
                // every non-original block is referenced by some original block's terminator
                for b in &func.blocks {
                    if orig_labels.contains(&b.label.0) { continue; }
                    let referenced = func.blocks.iter().take(func.blocks.len() - 0).any(|p| {
                        orig_labels.contains(&p.label.0) && successor_labels(&p.terminator).contains(&b.label)
                    });
                    prop_assert!(referenced, "trampoline {} is unreachable from any original block", b.label.0);
                }

                // (d) every USED former phi dest is still defined by some Copy
                //     (dead/circular phis from uninitialized-memory paths have no
                //     consumers and legitimately keep no producer)
                let copy_dests: std::collections::BTreeSet<u32> = func.blocks.iter().flat_map(|b|
                    b.instructions.iter().filter_map(|i|
                        if let Instruction::Copy { dest, .. } = i { Some(dest.0) } else { None })).collect();
                let mut used: std::collections::BTreeSet<u32> = Default::default();
                for b in &func.blocks {
                    used.extend(b.terminator.used_values());
                    for i in &b.instructions {
                        if !matches!(i, Instruction::Copy { .. }) {
                            used.extend(i.used_values());
                        }
                    }
                }
                for pd in phi_dests {
                    if used.contains(pd) {
                        prop_assert!(copy_dests.contains(pd),
                            "phi dest {} used but no longer defined by any Copy in {}", pd, func.name);
                    }
                }
                let _ = label_set;
            }
        }
    }
}
