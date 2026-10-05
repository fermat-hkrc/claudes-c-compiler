//! Shared CFG and dominator tree analysis utilities.
//!
//! These functions compute control flow graph (CFG) information and dominator
//! trees using the Cooper-Harvey-Kennedy algorithm. They are used by mem2reg
//! for SSA construction and by optimization passes (e.g., GVN) that need
//! dominator information.
//!
//! Performance: The CFG is stored as a flat CSR (Compressed Sparse Row)
//! adjacency list (`FlatAdj`) instead of `Vec<Vec<usize>>`. This reduces
//! n+1 heap allocations to 2 per build_cfg call and improves cache locality,
//! which is critical since build_cfg is called per-function by GVN, LICM,
//! if_convert, and mem2reg.

use crate::common::fx_hash::{FxHashMap, FxHashSet};
use crate::ir::reexports::{BlockId, Instruction, IrFunction, Terminator};

// ── Flat adjacency list (CSR format) ──────────────────────────────────────────

/// A flat adjacency list using Compressed Sparse Row (CSR) format.
///
/// Stores `n` variable-length rows in two flat arrays:
/// - `offsets[i]..offsets[i+1]` is the range of indices into `data` for row i
/// - `data[offsets[i]..offsets[i+1]]` contains the neighbors of node i
///
/// This uses exactly 2 heap allocations regardless of the number of rows,
/// compared to n+1 for `Vec<Vec<usize>>`. The flat layout also provides
/// better cache locality when iterating over adjacency lists.
pub struct FlatAdj {
    /// offsets[i] is the start index in `data` for row i.
    /// offsets[n] is the total number of entries (= data.len()).
    /// Length: n + 1
    offsets: Vec<u32>,
    /// Flat storage of all adjacency entries.
    data: Vec<u32>,
}

impl FlatAdj {
    /// Get the adjacency list (neighbors) of node `i` as a slice.
    #[inline]
    pub fn row(&self, i: usize) -> &[u32] {
        let start = self.offsets[i] as usize;
        let end = self.offsets[i + 1] as usize;
        &self.data[start..end]
    }

    /// Get the number of neighbors of node `i`.
    #[inline]
    pub fn len(&self, i: usize) -> usize {
        (self.offsets[i + 1] - self.offsets[i]) as usize
    }

    /// Build a FlatAdj from `Vec<Vec<usize>>` for tests.
    #[cfg(test)]
    pub fn from_vecs_usize(vecs: &[Vec<usize>]) -> Self {
        let n = vecs.len();
        let mut offsets = Vec::with_capacity(n + 1);
        let total: usize = vecs.iter().map(|v| v.len()).sum();
        let mut data = Vec::with_capacity(total);

        let mut offset = 0u32;
        for v in vecs {
            offsets.push(offset);
            for &val in v {
                data.push(val as u32);
            }
            offset += v.len() as u32;
        }
        offsets.push(offset);

        FlatAdj { offsets, data }
    }

    /// Build a FlatAdj from a Vec<Vec<u32>> (used in the construction phase).
    fn from_vecs(vecs: Vec<Vec<u32>>) -> Self {
        let n = vecs.len();
        let mut offsets = Vec::with_capacity(n + 1);
        let total: usize = vecs.iter().map(|v| v.len()).sum();
        let mut data = Vec::with_capacity(total);

        let mut offset = 0u32;
        for v in &vecs {
            offsets.push(offset);
            data.extend_from_slice(v);
            offset += v.len() as u32;
        }
        offsets.push(offset);

        FlatAdj { offsets, data }
    }
}

// ── Label map ─────────────────────────────────────────────────────────────────

/// Build a map from block label to block index.
pub fn build_label_map(func: &IrFunction) -> FxHashMap<BlockId, usize> {
    func.blocks
        .iter()
        .enumerate()
        .map(|(i, b)| (b.label, i))
        .collect()
}

// ── CFG construction ──────────────────────────────────────────────────────────

/// Build predecessor and successor lists from the function's CFG.
/// Returns (preds, succs) as flat adjacency lists (CSR format).
///
/// Uses only 4 heap allocations total (2 per FlatAdj) instead of 2*n+2 for
/// the old `Vec<Vec<usize>>` representation.
pub fn build_cfg(
    func: &IrFunction,
    label_to_idx: &FxHashMap<BlockId, usize>,
) -> (FlatAdj, FlatAdj) {
    let n = func.blocks.len();
    // Build using temporary Vec<Vec<u32>> then flatten to CSR.
    // The inner Vecs are tiny (usually 1-4 entries) so this is fast.
    let mut preds: Vec<Vec<u32>> = vec![Vec::new(); n];
    let mut succs: Vec<Vec<u32>> = vec![Vec::new(); n];

    for (i, block) in func.blocks.iter().enumerate() {
        let i32 = i as u32;
        match &block.terminator {
            Terminator::Branch(label) => {
                if let Some(&target) = label_to_idx.get(label) {
                    succs[i].push(target as u32);
                    preds[target].push(i32);
                }
            }
            Terminator::CondBranch { true_label, false_label, .. } => {
                if let Some(&t) = label_to_idx.get(true_label) {
                    succs[i].push(t as u32);
                    preds[t].push(i32);
                }
                if let Some(&f) = label_to_idx.get(false_label) {
                    let f32v = f as u32;
                    if !succs[i].contains(&f32v) {
                        succs[i].push(f32v);
                    }
                    preds[f].push(i32);
                }
            }
            Terminator::IndirectBranch { possible_targets, .. } => {
                for label in possible_targets {
                    if let Some(&t) = label_to_idx.get(label) {
                        let t32 = t as u32;
                        if !succs[i].contains(&t32) {
                            succs[i].push(t32);
                        }
                        preds[t].push(i32);
                    }
                }
            }
            Terminator::Switch { cases, default, .. } => {
                if let Some(&d) = label_to_idx.get(default) {
                    succs[i].push(d as u32);
                    preds[d].push(i32);
                }
                for (_, label) in cases {
                    if let Some(&t) = label_to_idx.get(label) {
                        let t32 = t as u32;
                        if !succs[i].contains(&t32) {
                            succs[i].push(t32);
                        }
                        preds[t].push(i32);
                    }
                }
            }
            Terminator::Return(_) | Terminator::Unreachable => {}
        }
        // InlineAsm goto_labels are implicit control flow edges.
        for inst in &block.instructions {
            if let Instruction::InlineAsm { goto_labels, .. } = inst {
                for (_, label) in goto_labels {
                    if let Some(&t) = label_to_idx.get(label) {
                        let t32 = t as u32;
                        if !succs[i].contains(&t32) {
                            succs[i].push(t32);
                        }
                        preds[t].push(i32);
                    }
                }
            }
        }
    }

    (FlatAdj::from_vecs(preds), FlatAdj::from_vecs(succs))
}

// ── Reverse postorder ─────────────────────────────────────────────────────────

/// Compute reverse postorder traversal of the CFG.
pub fn compute_reverse_postorder(num_blocks: usize, succs: &FlatAdj) -> Vec<usize> {
    let mut visited = vec![false; num_blocks];
    let mut postorder = Vec::with_capacity(num_blocks);

    fn dfs(node: usize, succs: &FlatAdj, visited: &mut Vec<bool>, postorder: &mut Vec<usize>) {
        visited[node] = true;
        for &succ in succs.row(node) {
            let s = succ as usize;
            if !visited[s] {
                dfs(s, succs, visited, postorder);
            }
        }
        postorder.push(node);
    }

    if num_blocks > 0 {
        dfs(0, succs, &mut visited, &mut postorder);
    }

    postorder.reverse();
    postorder
}

// ── Dominator computation ─────────────────────────────────────────────────────

/// Intersect two dominators using RPO numbering (Cooper-Harvey-Kennedy).
fn intersect(
    mut finger1: usize,
    mut finger2: usize,
    idom: &[usize],
    rpo_number: &[usize],
) -> usize {
    while finger1 != finger2 {
        while rpo_number[finger1] > rpo_number[finger2] {
            finger1 = idom[finger1];
        }
        while rpo_number[finger2] > rpo_number[finger1] {
            finger2 = idom[finger2];
        }
    }
    finger1
}

/// Compute immediate dominators using the Cooper-Harvey-Kennedy algorithm.
/// Returns idom[i] = immediate dominator of block i (idom[0] = 0 for entry).
/// Uses usize::MAX as sentinel for undefined/unreachable blocks.
pub fn compute_dominators(
    num_blocks: usize,
    preds: &FlatAdj,
    succs: &FlatAdj,
) -> Vec<usize> {
    const UNDEF: usize = usize::MAX;

    let rpo = compute_reverse_postorder(num_blocks, succs);
    let mut rpo_number = vec![UNDEF; num_blocks];
    for (order, &block) in rpo.iter().enumerate() {
        rpo_number[block] = order;
    }

    let mut idom = vec![UNDEF; num_blocks];
    if rpo.is_empty() {
        return idom;
    }
    idom[rpo[0]] = rpo[0]; // Entry dominates itself

    let mut changed = true;
    while changed {
        changed = false;
        for &b in rpo.iter().skip(1) {
            if rpo_number[b] == UNDEF {
                continue;
            }

            let mut new_idom = UNDEF;
            for &p in preds.row(b) {
                let p = p as usize;
                if idom[p] != UNDEF {
                    new_idom = p;
                    break;
                }
            }

            if new_idom == UNDEF {
                continue;
            }

            for &p in preds.row(b) {
                let p = p as usize;
                if p == new_idom {
                    continue;
                }
                if idom[p] != UNDEF {
                    new_idom = intersect(new_idom, p, &idom, &rpo_number);
                }
            }

            if idom[b] != new_idom {
                idom[b] = new_idom;
                changed = true;
            }
        }
    }

    idom
}

// ── Dominance frontiers ───────────────────────────────────────────────────────

/// Compute dominance frontiers for each block.
/// DF(b) = set of blocks where b's dominance ends (join points).
pub fn compute_dominance_frontiers(
    num_blocks: usize,
    preds: &FlatAdj,
    idom: &[usize],
) -> Vec<FxHashSet<usize>> {
    let mut df = vec![FxHashSet::default(); num_blocks];

    for b in 0..num_blocks {
        if preds.len(b) < 2 {
            continue;
        }
        for &p in preds.row(b) {
            let mut runner = p as usize;
            while runner != idom[b] && runner != usize::MAX {
                df[runner].insert(b);
                if runner == idom[runner] {
                    break;
                }
                runner = idom[runner];
            }
        }
    }

    df
}

// ── Dominator tree ────────────────────────────────────────────────────────────

/// Build dominator tree children lists from idom array.
/// children[b] lists block indices whose immediate dominator is b.
pub fn build_dom_tree_children(num_blocks: usize, idom: &[usize]) -> Vec<Vec<usize>> {
    let mut children = vec![Vec::new(); num_blocks];
    for b in 1..num_blocks {
        if idom[b] != usize::MAX && idom[b] != b {
            children[idom[b]].push(b);
        }
    }
    children
}

// ── Cached analysis bundle ──────────────────────────────────────────────────

/// Pre-computed CFG analysis results shared across multiple passes within
/// a single pipeline iteration.
///
/// GVN, LICM, and IVSR all need the same CFG, dominator, and loop analysis.
/// Since GVN does not modify the CFG (it only replaces instruction operands),
/// these results remain valid across all three passes. Computing them once
/// and sharing avoids redundant `build_cfg` + `compute_dominators` +
/// `find_natural_loops` calls per function per iteration.
pub struct CfgAnalysis {
    pub preds: FlatAdj,
    pub succs: FlatAdj,
    pub idom: Vec<usize>,
    pub dom_children: Vec<Vec<usize>>,
    pub num_blocks: usize,
}

impl CfgAnalysis {
    /// Build a complete CFG analysis bundle for a function.
    pub fn build(func: &IrFunction) -> Self {
        let num_blocks = func.blocks.len();
        let label_to_idx = build_label_map(func);
        let (preds, succs) = build_cfg(func, &label_to_idx);
        let idom = compute_dominators(num_blocks, &preds, &succs);
        let dom_children = build_dom_tree_children(num_blocks, &idom);
        CfgAnalysis {
            preds,
            succs,
            idom,
            dom_children,
            num_blocks,
        }
    }
}

// ── PBT properties (round 06) ─────────────────────────────────────────────────
#[cfg(test)]
mod pbt_tests {
    use super::*;
    use crate::common::types::{AddressSpace, IrType};
    use crate::ir::reexports::{BasicBlock, IrConst, Operand, Value};
    use proptest::prelude::*;
    use proptest::collection::vec;

    /// Materialize an IrFunction from plain terminator descriptors over n blocks.
    /// kind: 0=Return, 1=Branch(a), 2=CondBranch(a,b) (a==b allowed), 3=Switch
    fn func_from_descriptors(descs: &[(u8, u32, u32, u32)]) -> IrFunction {
        let n = descs.len() as u32;
        let mut func = IrFunction::new("f".to_string(), IrType::I32, vec![], false);
        for (i, &(kind, a, b, c)) in descs.iter().enumerate() {
            let id = BlockId(i as u32);
            let term = match kind % 4 {
                0 => Terminator::Return(Some(Operand::Const(IrConst::I32(0)))),
                1 => Terminator::Branch(BlockId(a % n)),
                2 => Terminator::CondBranch {
                    cond: Operand::Const(IrConst::I32(1)),
                    true_label: BlockId(a % n),
                    false_label: BlockId(b % n),
                },
                _ => Terminator::Switch {
                    val: Operand::Const(IrConst::I32(7)),
                    cases: vec![(1, BlockId(a % n)), (2, BlockId(b % n))],
                    default: BlockId(c % n),
                    ty: IrType::I32,
                },
            };
            func.blocks.push(BasicBlock {
                label: id,
                instructions: vec![],
                terminator: term,
                source_spans: vec![],
            });
        }
        func
    }

    /// P9: build_cfg's documented contract is ONE graph returned as a (preds, succs)
    /// pair — the two CSR lists must be exact transposes with no duplicate entries.
    /// CondBranch with equal targets and Switch with duplicate case labels are
    /// representable IR and must not break the pairing.
    /// Doc contract: "Build predecessor and successor lists from the function's CFG.
    /// Returns (preds, succs) as flat adjacency lists (CSR format)." (767abe89)
    proptest! {
        #![proptest_config(ProptestConfig::with_cases(1024))]
        #[test]
        fn p9_build_cfg_transpose_consistency(
            descs in vec((any::<u8>(), any::<u32>(), any::<u32>(), any::<u32>()), 1..=8),
        ) {
            let func = func_from_descriptors(&descs);
            let n = func.blocks.len();
            let label_map = build_label_map(&func);
            let (preds, succs) = build_cfg(&func, &label_map);

            // successor targets must all resolve (labels are dense BlockId(0..n))
            let mut edge_pairs_from_succs: Vec<(usize, usize)> = vec![];
            for i in 0..n {
                let mut seen = std::collections::BTreeSet::new();
                for &s in succs.row(i) {
                    let s = s as usize;
                    prop_assert!(s < n);
                    prop_assert!(seen.insert(s), "duplicate successor {} in row {}", s, i);
                    edge_pairs_from_succs.push((i, s));
                }
            }
            let mut edge_pairs_from_preds: Vec<(usize, usize)> = vec![];
            for b in 0..n {
                let mut seen = std::collections::BTreeSet::new();
                for &p in preds.row(b) {
                    let p = p as usize;
                    prop_assert!(p < n);
                    prop_assert!(seen.insert(p), "duplicate predecessor {} in row {}", p, b);
                    edge_pairs_from_preds.push((p, b));
                }
            }
            edge_pairs_from_succs.sort();
            edge_pairs_from_preds.sort();
            prop_assert_eq!(edge_pairs_from_succs, edge_pairs_from_preds,
                "preds and succs are not transposes of the same graph");
        }
    }

    // Independent reference: textbook iterative dominator-set dataflow
    // (Aho/Sethi/Ullman; Cytron et al. 1991) over u8 bitsets (n <= 8).
    fn naive_dominators(n: usize, succs: &[Vec<usize>]) -> (Vec<bool>, Vec<u8>, Vec<usize>) {
        let mut reach = vec![false; n];
        if n > 0 {
            let mut stack = vec![0usize];
            reach[0] = true;
            while let Some(u) = stack.pop() {
                for &s in &succs[u] {
                    if !reach[s] {
                        reach[s] = true;
                        stack.push(s);
                    }
                }
            }
        }
        let preds: Vec<Vec<usize>> = {
            let mut p = vec![vec![]; n];
            for u in 0..n {
                for &s in &succs[u] {
                    p[s].push(u);
                }
            }
            p
        };
        let reach_mask: u8 = (0..n).filter(|&i| reach[i]).fold(0u8, |m, i| m | (1 << i));
        let mut dom = vec![0u8; n];
        if n > 0 {
            dom[0] = 1;
            for b in 1..n {
                if reach[b] {
                    dom[b] = reach_mask;
                }
            }
            let mut changed = true;
            while changed {
                changed = false;
                for b in 1..n {
                    if !reach[b] {
                        continue;
                    }
                    let mut m = reach_mask;
                    let mut any_pred = false;
                    for &p in &preds[b] {
                        if reach[p] {
                            m &= dom[p];
                            any_pred = true;
                        }
                    }
                    let _ = any_pred;
                    let new_dom = m | (1 << b);
                    if new_dom != dom[b] {
                        dom[b] = new_dom;
                        changed = true;
                    }
                }
            }
        }
        // idom[b] = the unique strict dominator whose dominator set equals b's strict dominators
        let mut idom = vec![usize::MAX; n];
        if n > 0 {
            idom[0] = 0; // entry dominates itself
        }
        for b in 1..n {
            if !reach[b] {
                continue;
            }
            let strict = dom[b] & !(1 << b);
            for c in 0..n {
                if strict & (1 << c) != 0 && dom[c] == strict {
                    idom[b] = c;
                    break;
                }
            }
        }
        (reach, dom, idom)
    }

    fn succs_from_bits(n: usize, bits: &[u8]) -> Vec<Vec<usize>> {
        (0..n)
            .map(|i| (0..n).filter(|&t| bits[i] & (1 << t) != 0).collect())
            .collect()
    }

    fn transpose(n: usize, succs: &[Vec<usize>]) -> Vec<Vec<usize>> {
        let mut p = vec![vec![]; n];
        for u in 0..n {
            for &s in &succs[u] {
                p[s].push(u);
            }
        }
        p
    }

    /// P10: Cooper-Harvey-Kennedy SUT vs the textbook dataflow reference; unreachable
    /// blocks must carry the documented usize::MAX sentinel, entry idom is itself.
    /// Doc contract: "Uses usize::MAX as sentinel for undefined/unreachable blocks." (ab794cba)
    proptest! {
        #![proptest_config(ProptestConfig::with_cases(1024))]
        #[test]
        fn p10_dominators_vs_naive_reference(
            n in 1usize..=8,
            bits in vec(any::<u8>(), 1..=8),
        ) {
            let n = n.min(bits.len());
            let bits = &bits[..n];
            let succs = succs_from_bits(n, bits);
            let preds = transpose(n, &succs);
            let pred_adj = FlatAdj::from_vecs_usize(&preds);
            let succ_adj = FlatAdj::from_vecs_usize(&succs);

            let sut = compute_dominators(n, &pred_adj, &succ_adj);
            let (reach, _dom, naive) = naive_dominators(n, &succs);

            prop_assert_eq!(sut.len(), n);
            if n > 0 {
                prop_assert_eq!(sut[0], 0, "entry idom");
            }
            for b in 0..n {
                if reach[b] {
                    prop_assert_eq!(sut[b], naive[b], "block {} idom mismatch (n={})", b, n);
                } else {
                    prop_assert_eq!(sut[b], usize::MAX, "unreachable block {} must be MAX", b);
                }
            }
            // dom tree children must be the exact inversion of idom (entry excluded)
            let children = build_dom_tree_children(n, &sut);
            for b in 1..n {
                if sut[b] != usize::MAX && sut[b] != b {
                    prop_assert!(children[sut[b]].contains(&b));
                }
            }
            // reverse postorder visits each reachable block exactly once, entry first
            let rpo = compute_reverse_postorder(n, &succ_adj);
            prop_assert_eq!(rpo.len(), reach.iter().filter(|&&r| r).count());
            if !rpo.is_empty() {
                prop_assert_eq!(rpo[0], 0);
            }
        }
    }

    /// P11: dominance frontiers vs the set definition
    /// DF(b) = { d : b dominates some pred of d AND b does not strictly dominate d }
    /// (Cytron et al. 1991), computed from the cross-validated naive idom.
    proptest! {
        #![proptest_config(ProptestConfig::with_cases(1024))]
        #[test]
        fn p11_frontiers_vs_definition(
            n in 1usize..=8,
            mut bits in vec(any::<u8>(), 1..=8),
        ) {
            let n = n.min(bits.len());
            let bits = &mut bits[..n];
            // exclude ALL edges into entry (any such edge puts entry in a cycle):
            // known defect b5 — DF(entry) can never contain entry (see p11b below)
            for b in bits.iter_mut() { *b &= !1; }
            let succs = succs_from_bits(n, bits);
            let preds = transpose(n, &succs);
            let pred_adj = FlatAdj::from_vecs_usize(&preds);
            let succ_adj = FlatAdj::from_vecs_usize(&succs);

            let sut_idom = compute_dominators(n, &pred_adj, &succ_adj);
            let sut_df = compute_dominance_frontiers(n, &pred_adj, &sut_idom);
            let (reach, _dom, naive_idom) = naive_dominators(n, &succs);

            let dominates = |d: usize, b: usize| -> bool {
                let mut cur = b;
                loop {
                    if cur == d {
                        return true;
                    }
                    if cur == 0 {
                        return d == 0;
                    }
                    if naive_idom[cur] == usize::MAX {
                        return false;
                    }
                    cur = naive_idom[cur];
                }
            };

            for b in 0..n {
                if !reach[b] {
                    continue; // DF only meaningful for reachable blocks
                }
                let mut expected = std::collections::BTreeSet::new();
                for d in 0..n {
                    if !reach[d] {
                        continue;
                    }
                    let has_dominated_pred = preds[d].iter().any(|&p| reach[p] && dominates(b, p));
                    if has_dominated_pred && (d == b || !dominates(b, d)) {
                        expected.insert(d);
                    }
                }
                let got: std::collections::BTreeSet<usize> = sut_df[b].iter().copied().collect();
                prop_assert_eq!(got, expected, "DF({}) mismatch (n={}, succs={:?})", b, n, succs);
            }
        }
    }

    /// P11b (RED WITNESS — bug b5): with an entry self-edge, the formal definition
    /// DF_local(n) = {y in succ(n) : n does not strictly dominate y} puts the entry in
    /// its own frontier (Cytron et al. 1991). The runner-walk formulation stops at
    /// idom[entry] == entry and can never emit it, and the < 2 preds gate skips the
    /// block entirely. Kept intentionally failing as the filed bug's witness.
    #[test]
    fn p11b_entry_selfloop_df() {
        // direct self-edge form
        let n = 1usize;
        let succs: Vec<Vec<usize>> = vec![vec![0]]; // entry branches to itself
        let preds = transpose(n, &succs);
        let pred_adj = FlatAdj::from_vecs_usize(&preds);
        let succ_adj = FlatAdj::from_vecs_usize(&succs);
        let idom = compute_dominators(n, &pred_adj, &succ_adj);
        let df = compute_dominance_frontiers(n, &pred_adj, &idom);
        assert!(df[0].contains(&0),
            "DF(entry) must contain entry for an entry self-loop (Cytron DF_local)");

        // longer cycle form: 0 -> 1 -> {0, 1}; entry dominates its only pred 1
        let succs: Vec<Vec<usize>> = vec![vec![1], vec![0, 1], vec![]];
        let preds = transpose(3, &succs);
        let pred_adj = FlatAdj::from_vecs_usize(&preds);
        let succ_adj = FlatAdj::from_vecs_usize(&succs);
        let idom = compute_dominators(3, &pred_adj, &succ_adj);
        let df = compute_dominance_frontiers(3, &pred_adj, &idom);
        assert_eq!(idom, vec![0, 0, usize::MAX]);
        assert!(df[0].contains(&0),
            "DF(entry) must contain entry when entry is in any cycle (Cytron DF_local)");
        assert!(df[1].contains(&1), "DF(1) must contain 1 (self-loop)");
    }

    // silence unused warnings for helpers only used by generated inputs
    #[allow(dead_code)]
    fn _touch(_a: AddressSpace) {}
}

// Deterministic red regression witness (round 06, bug b4).
#[cfg(test)]
mod pbt_regression {
    use super::*;
    use crate::common::types::IrType;
    use crate::ir::reexports::{BasicBlock, IrConst, Operand};

    /// b4: build_cfg must return preds/succs as transposes of one graph; a
    /// CondBranch with equal targets currently duplicates the predecessor entry.
    #[test]
    fn test_ir_analysis_regression_build_cfg_dup_preds() {
        let mut func = IrFunction::new("f".to_string(), IrType::I32, vec![], false);
        func.blocks.push(BasicBlock {
            label: BlockId(0),
            instructions: vec![],
            terminator: Terminator::CondBranch {
                cond: Operand::Const(IrConst::I32(1)),
                true_label: BlockId(1),
                false_label: BlockId(1),
            },
            source_spans: vec![],
        });
        func.blocks.push(BasicBlock {
            label: BlockId(1),
            instructions: vec![],
            terminator: Terminator::Return(Some(Operand::Const(IrConst::I32(0)))),
            source_spans: vec![],
        });
        let label_map = build_label_map(&func);
        let (preds, succs) = build_cfg(&func, &label_map);
        assert_eq!(succs.row(0).to_vec(), vec![1], "succs deduped");
        assert_eq!(preds.row(1).to_vec(), vec![0],
            "preds must mirror succs exactly (no duplicate pred entries)");
    }
}
