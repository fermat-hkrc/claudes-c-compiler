//! Shared PBT helpers for the backend-core rounds (round 08+).
//!
//! Holds the random-CFG builder used by the liveness / regalloc properties
//! and an INDEPENDENT naive implementation of the liveness analysis, written
//! from the textbook equations quoted in `liveness.rs`'s own doc comment
//! ("live_in[B] = gen[B] ∪ (live_out[B] - kill[B])",
//!  "live_out[B] = ∪ live_in[S] for all successors S of B")
//! with its own operand traversal — not derived from the SUT's iterators.
#![cfg(test)]

use crate::common::types::IrType;
use crate::ir::reexports::{BasicBlock, BlockId, Instruction, IrBinOp, IrCmpOp, IrConst, IrFunction, Operand, Terminator, Value};
use std::collections::HashSet;

// ─────────────────────────────────────────────────────────────────────────────
// Random CFG builder (no phis, no allocas; blocks 0..n, params I32×2).
// Instructions: BinOp(Add) / Cmp(Slt) / Cast(I32→I64) / Copy / Call("g").
// Terminators: Return / Branch / CondBranch. Targets always valid block ids.
// ─────────────────────────────────────────────────────────────────────────────

pub struct CfgSpec {
    /// (block_idx_hint, inst_kind, _k) — instruction specs, modulo-indexed.
    pub insts: Vec<(u8, u8, i64)>,
    /// (term_kind, target1, target2) per block, modulo-indexed.
    pub terms: Vec<(u8, u32, u32)>,
    /// number of blocks, 2..=7
    pub n_blocks: u8,
}

pub fn build_random_cfg(spec: &CfgSpec) -> IrFunction {
    use crate::passes::pbt_common::FnBuilder;
    let n = spec.n_blocks.max(2) as usize;
    let ilen = spec.insts.len().max(1);
    let tlen = spec.terms.len().max(1);
    let mut b = FnBuilder::new("f", IrType::I32, &[IrType::I32, IrType::I32]);
    let p0 = b.v();
    let p1 = b.v();
    let p0op = Operand::Value(p0);
    let p1op = Operand::Value(p1);
    // operand pool: (value, defined-so-far) — grows block by block
    let mut pool: Vec<Operand> = vec![p0op, p1op];
    for bi in 0..n {
        let k = (spec.insts[bi % ilen].0 as usize) % 4;
        let mut insts: Vec<Instruction> = Vec::new();
        for j in 0..k {
            let kind = (spec.insts[(bi * 3 + j) % ilen].1 as usize) % 5;
            let d = b.v();
            let lhs = pool[(spec.insts[(bi * 3 + j) % ilen].2.unsigned_abs() as usize) % pool.len()];
            let rhs = pool[(spec.terms[(bi + j) % tlen].0 as usize) % pool.len()];
            let inst = match kind {
                0 => Instruction::BinOp {
                    dest: d,
                    op: IrBinOp::Add,
                    lhs,
                    rhs: Operand::Const(IrConst::I32(7)),
                    ty: IrType::I32,
                },
                1 => Instruction::Cmp {
                    dest: d,
                    op: IrCmpOp::Slt,
                    lhs,
                    rhs,
                    ty: IrType::I32,
                },
                2 => Instruction::Cast {
                    dest: d,
                    src: lhs,
                    from_ty: IrType::I32,
                    to_ty: IrType::I64,
                },
                3 => Instruction::Copy { dest: d, src: lhs },
                _ => Instruction::Call {
                    func: "g".to_string(),
                    info: crate::ir::reexports::CallInfo {
                        dest: Some(d),
                        args: vec![p0op],
                        arg_types: vec![IrType::I32],
                        return_type: IrType::I32,
                        is_variadic: false,
                        num_fixed_args: 1,
                        struct_arg_sizes: vec![None],
                        struct_arg_aligns: vec![None],
                        struct_arg_classes: vec![vec![]],
                        struct_arg_riscv_float_classes: vec![None],
                        is_sret: false,
                        is_fastcall: false,
                        ret_eightbyte_classes: vec![],
                    },
                },
            };
            pool.push(Operand::Value(d));
            insts.push(inst);
        }
        let (tk, t1, t2) = spec.terms[bi % tlen];
        let cond = pool[(t1 as usize) % pool.len()];
        let term = match (tk % 3, t1 % (n as u32), t2 % (n as u32)) {
            (0, _, _) => Terminator::Return(Some(cond)),
            (1, a, _) => Terminator::Branch(BlockId(a)),
            _ => Terminator::CondBranch {
                cond,
                true_label: BlockId(t1 % (n as u32)),
                false_label: BlockId(t2 % (n as u32)),
            },
        };
        let _ = p1op;
        b.block(insts, term);
    }
    b.seal()
}

// ─────────────────────────────────────────────────────────────────────────────
// Independent naive liveness (mirrors the documented scheme):
//   numbering: each instruction gets one point; the terminator gets the
//   block's end point; point += 1 after the terminator.
//   dataflow:  live_in = gen ∪ (live_out − kill);  live_out = ∪ live_in[succ]
//   extension: for v ∈ live_in[B]: def = min(def, start(B)); last = max(last, end(B))
//              for v ∈ live_out[B]: def = min(def, start(B)); last = max(last, end(B))
//   interval:  [def, max(last, def)]
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NaiveInterval {
    pub value_id: u32,
    pub start: u32,
    pub end: u32,
}

/// Independent operand walkers for the instruction/terminator subset the
/// builder emits (NOT the SUT's iterators).
fn inst_uses(inst: &Instruction) -> Vec<Value> {
    match inst {
        Instruction::BinOp { lhs, rhs, .. } => {
            let mut v = vec![];
            if let Operand::Value(x) = lhs { v.push(*x); }
            if let Operand::Value(x) = rhs { v.push(*x); }
            v
        }
        Instruction::Cmp { lhs, rhs, .. } => {
            let mut v = vec![];
            if let Operand::Value(x) = lhs { v.push(*x); }
            if let Operand::Value(x) = rhs { v.push(*x); }
            v
        }
        Instruction::Cast { src, .. } | Instruction::Copy { src, .. } => {
            if let Operand::Value(x) = src { vec![*x] } else { vec![] }
        }
        Instruction::Call { info, .. } => info.args.iter().filter_map(|o| if let Operand::Value(x) = o { Some(*x) } else { None }).collect(),
        _ => vec![],
    }
}

fn inst_def(inst: &Instruction) -> Option<Value> {
    match inst {
        Instruction::BinOp { dest, .. } | Instruction::Cmp { dest, .. } | Instruction::Cast { dest, .. } | Instruction::Copy { dest, .. } => Some(*dest),
        Instruction::Call { info, .. } => info.dest,
        _ => None,
    }
}

fn term_uses(term: &Terminator) -> Vec<Value> {
    match term {
        Terminator::Return(Some(Operand::Value(v))) => vec![*v],
        Terminator::CondBranch { cond: Operand::Value(v), .. } => vec![*v],
        _ => vec![],
    }
}

fn term_succs(term: &Terminator) -> Vec<BlockId> {
    match term {
        Terminator::Branch(b) => vec![*b],
        Terminator::CondBranch { true_label, false_label, .. } => vec![*true_label, *false_label],
        _ => vec![],
    }
}

pub fn naive_liveness(func: &IrFunction) -> Vec<NaiveInterval> {
    let n = func.blocks.len();
    if n == 0 { return vec![]; }
    // dense value map (all values, params included — params have no def point)
    let mut dense: Vec<u32> = vec![];
    let mut to_dense: std::collections::HashMap<u32, usize> = std::collections::HashMap::new();
    let mut add = |vid: u32, dense: &mut Vec<u32>, m: &mut std::collections::HashMap<u32, usize>| {
        if !m.contains_key(&vid) {
            m.insert(vid, dense.len());
            dense.push(vid);
        }
    };
    for b in &func.blocks {
        for i in &b.instructions {
            if let Some(d) = inst_def(i) { add(d.0, &mut dense, &mut to_dense); }
            for u in inst_uses(i) { add(u.0, &mut dense, &mut to_dense); }
        }
        for u in term_uses(&b.terminator) { add(u.0, &mut dense, &mut to_dense); }
    }
    // params are in the dense map only if referenced; unreferenced params have
    // no interval either way (def=MAX, no uses) — consistent with the SUT's
    // value collection (it also collects only referenced/defined values).
    let nv = dense.len();

    // numbering + gen/kill
    let mut point: u32 = 0;
    let mut block_start = vec![];
    let mut block_end = vec![];
    let mut gen: Vec<HashSet<usize>> = vec![];
    let mut kill: Vec<HashSet<usize>> = vec![];
    let mut def_pt = vec![u32::MAX; nv];
    let mut last_pt = vec![u32::MAX; nv];
    let mut idx_of_label: std::collections::HashMap<u32, usize> = std::collections::HashMap::new();
    for (bi, b) in func.blocks.iter().enumerate() {
        idx_of_label.insert(b.label.0, bi);
        block_start.push(point);
        let mut g: HashSet<usize> = HashSet::new();
        let mut k: HashSet<usize> = HashSet::new();
        for i in &b.instructions {
            for u in inst_uses(i) {
                let d = to_dense[&u.0];
                if last_pt[d] == u32::MAX || point > last_pt[d] { last_pt[d] = point; }
                if !k.contains(&d) { g.insert(d); }
            }
            if let Some(dv) = inst_def(i) {
                let d = to_dense[&dv.0];
                if def_pt[d] == u32::MAX { def_pt[d] = point; }
                k.insert(d);
            }
            point += 1;
        }
        let end = point;
        for u in term_uses(&b.terminator) {
            let d = to_dense[&u.0];
            if last_pt[d] == u32::MAX || end > last_pt[d] { last_pt[d] = end; }
            if !k.contains(&d) { g.insert(d); }
        }
        block_end.push(end);
        point += 1;
        gen.push(g);
        kill.push(k);
    }
    // successors
    let succs: Vec<Vec<usize>> = func.blocks.iter()
        .map(|b| term_succs(&b.terminator).iter().map(|l| idx_of_label[&l.0]).collect())
        .collect();
    // backward dataflow to fixpoint
    let mut live_in: Vec<HashSet<usize>> = vec![HashSet::new(); n];
    let mut live_out: Vec<HashSet<usize>> = vec![HashSet::new(); n];
    let mut changed = true;
    while changed {
        changed = false;
        for bi in (0..n).rev() {
            let mut out: HashSet<usize> = HashSet::new();
            for &s in &succs[bi] {
                out.extend(live_in[s].iter().copied());
            }
            let mut inn = gen[bi].clone();
            for &d in out.iter() {
                if !kill[bi].contains(&d) { inn.insert(d); }
            }
            if inn != live_in[bi] || out != live_out[bi] {
                live_in[bi] = inn;
                live_out[bi] = out;
                changed = true;
            }
        }
    }
    // interval extension (mirrors extend_intervals_from_liveness)
    for bi in 0..n {
        let (s, e) = (block_start[bi], block_end[bi]);
        for &d in live_in[bi].iter().chain(live_out[bi].iter()) {
            if def_pt[d] == u32::MAX || s < def_pt[d] { def_pt[d] = s; }
            if last_pt[d] == u32::MAX { last_pt[d] = s; }
            if e > last_pt[d] { last_pt[d] = e; }
        }
    }
    let mut out: Vec<NaiveInterval> = vec![];
    for (d, &vid) in dense.iter().enumerate() {
        if def_pt[d] == u32::MAX { continue; }
        // Mirrors build_intervals: an unused value's end falls back to start.
        let end = if last_pt[d] == u32::MAX { def_pt[d] } else { last_pt[d].max(def_pt[d]) };
        out.push(NaiveInterval { value_id: vid, start: def_pt[d], end });
    }
    out.sort_by_key(|iv| (iv.start, iv.value_id));
    out
}

/// The ir::reexports BasicBlock import guard (used by the builder only).
#[allow(dead_code)]
fn _bb_type_check(_: &BasicBlock) {}
