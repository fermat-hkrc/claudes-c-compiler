//! Shared PBT support for src/passes (round 07).
//!
//! Contains an INDEPENDENT C11-semantics interpreter over a restricted IR
//! fragment (BinOp/Cmp/Cast/Copy/UnaryOp{Neg,Not}/Phi/Select/ParamRef over
//! integer types; Return/Branch/CondBranch/Switch terminators) plus
//! well-formed-SSA function builders used by the per-file `pbt_tests` mods.
//!
//! The interpreter is written directly from C11 + src/passes/README semantics
//! (wrapping arithmetic at the instruction's type width, truncating division,
//! remainder sign following the dividend, unsigned ops on zero-extended bit
//! patterns). It deliberately shares no code with the SUT's own evaluators.
#![allow(dead_code)]

use crate::common::types::IrType;
use crate::ir::reexports::{
    BasicBlock, BlockId, Instruction, IrBinOp, IrCmpOp, IrConst, IrFunction, IrModule, IrParam,
    IrUnaryOp, Operand, Terminator, Value,
};


// ── type helpers ─────────────────────────────────────────────────────────────

pub fn width(ty: &IrType) -> u32 {
    match ty {
        IrType::I8 | IrType::U8 => 8,
        IrType::I16 | IrType::U16 => 16,
        IrType::I32 | IrType::U32 => 32,
        IrType::I64 | IrType::U64 | IrType::Ptr => 64,
        IrType::I128 | IrType::U128 => 128,
        _ => 64,
    }
}

pub fn signed_ty(ty: &IrType) -> bool {
    matches!(ty, IrType::I8 | IrType::I16 | IrType::I32 | IrType::I64 | IrType::I128)
}

pub fn mask(w: u32) -> u128 {
    if w >= 128 { u128::MAX } else { (1u128 << w) - 1 }
}

pub fn low(bits: i128, w: u32) -> u128 {
    (bits as u128) & mask(w)
}

pub fn sext(bits: i128, w: u32) -> i128 {
    if w >= 128 { bits } else { let s = 128 - w; ((bits << s) >> s) as i128 }
}

/// Canonical form of a value at `ty` (low `w` bits, re-extended per signedness).
pub fn canon(bits: i128, ty: &IrType) -> i128 {
    let w = width(ty);
    if signed_ty(ty) { sext(low(bits, w) as i128, w) } else { low(bits, w) as i128 }
}

/// Build an integer constant operand whose bit pattern at `ty` equals
/// `bits_u` (given as the low `w` bits, e.g. via `as u64`).
pub fn const_int(ty: &IrType, bits_u: u64) -> Operand {
    match ty {
        IrType::I8 => Operand::Const(IrConst::I8(bits_u as u8 as i8)),
        IrType::I16 => Operand::Const(IrConst::I16(bits_u as u16 as i16)),
        IrType::I32 => Operand::Const(IrConst::I32(bits_u as u32 as i32)),
        IrType::I64 => Operand::Const(IrConst::I64(bits_u as i64)),
        // Unsigned types are stored zero-extended in I64 (documented convention).
        _ => Operand::Const(IrConst::I64(bits_u as i64)),
    }
}

fn const_bits(c: &IrConst) -> i128 {
    match c {
        IrConst::I8(v) => *v as i128,
        IrConst::I16(v) => *v as i128,
        IrConst::I32(v) => *v as i128,
        IrConst::I64(v) => *v as i128,
        IrConst::I128(v) => *v,
        IrConst::Zero => 0,
        _ => panic!("pbt interp: unsupported const {:?}", c),
    }
}

/// Public wrapper for test mods.
pub fn const_bits_pub(c: &IrConst) -> i128 { const_bits(c) }

// ── independent C11 operation semantics ─────────────────────────────────────

pub fn eval_binop(op: IrBinOp, a: i128, b: i128, ty: &IrType) -> Option<i128> {
    let w = width(ty);
    let m = mask(w);
    let av = low(a, w);
    let bv = low(b, w);
    let r: u128 = match op {
        IrBinOp::Add => av.wrapping_add(bv) & m,
        IrBinOp::Sub => av.wrapping_sub(bv) & m,
        IrBinOp::Mul => av.wrapping_mul(bv) & m,
        IrBinOp::And => av & bv,
        IrBinOp::Or => av | bv,
        IrBinOp::Xor => av ^ bv,
        IrBinOp::Shl => av.wrapping_shl(bv as u32) & m,
        IrBinOp::LShr => av.wrapping_shr(bv as u32) & m,
        IrBinOp::AShr => (sext(av as i128, w).wrapping_shr(bv as u32) as u128) & m,
        IrBinOp::SDiv => {
            if bv == 0 { return None; }
            (sext(av as i128, w).wrapping_div(sext(bv as i128, w)) as u128) & m
        }
        IrBinOp::UDiv => {
            if bv == 0 { return None; }
            av / bv
        }
        IrBinOp::SRem => {
            if bv == 0 { return None; }
            (sext(av as i128, w).wrapping_rem(sext(bv as i128, w)) as u128) & m
        }
        IrBinOp::URem => {
            if bv == 0 { return None; }
            av % bv
        }
    };
    Some(r as i128)
}

pub fn eval_cmp(op: IrCmpOp, a: i128, b: i128, ty: &IrType) -> i128 {
    let w = width(ty);
    let ua = low(a, w);
    let ub = low(b, w);
    let sa = sext(ua as i128, w);
    let sb = sext(ub as i128, w);
    let r = match op {
        IrCmpOp::Eq => ua == ub,
        IrCmpOp::Ne => ua != ub,
        IrCmpOp::Slt => sa < sb,
        IrCmpOp::Sle => sa <= sb,
        IrCmpOp::Sgt => sa > sb,
        IrCmpOp::Sge => sa >= sb,
        IrCmpOp::Ult => ua < ub,
        IrCmpOp::Ule => ua <= ub,
        IrCmpOp::Ugt => ua > ub,
        IrCmpOp::Uge => ua >= ub,
    };
    r as i128
}

fn eval_cast(a: i128, from_ty: &IrType, to_ty: &IrType) -> i128 {
    // First recover the numeric value at the SOURCE type (sign/zero per from_ty),
    // then truncate/re-extend at the TARGET type.
    let fw = width(from_ty);
    let v: i128 = if signed_ty(from_ty) { sext(low(a, fw) as i128, fw) } else { low(a, fw) as i128 };
    let tw = width(to_ty);
    if tw >= fw {
        if signed_ty(to_ty) { v } else { low(v, tw) as i128 }
    } else if signed_ty(to_ty) {
        sext(low(v, tw) as i128, tw)
    } else {
        low(v, tw) as i128
    }
}

fn eval_unary(op: IrUnaryOp, a: i128, ty: &IrType) -> Option<i128> {
    let w = width(ty);
    let m = mask(w);
    Some(match op {
        IrUnaryOp::Neg => (low(a, w).wrapping_neg() & m) as i128,
        IrUnaryOp::Not => (!low(a, w) & m) as i128,
        _ => panic!("pbt interp: unsupported unary {:?}", op),
    })
}

// ── interpreter ──────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RunOutcome {
    Value(i128),
    Trap,
    StepLimit,
}

/// Interpret `func` over the integer fragment with `args` (one per ParamRef
/// param index, already canonical at the param's type). Values are stored as
/// raw bit patterns in i128; the result is canonical at `return_type`.
pub fn interp(func: &IrFunction, args: &[i128], limit: usize) -> RunOutcome {
    let idx = |id: BlockId| -> usize {
        func.blocks.iter().position(|b| b.label == id).unwrap_or_else(|| {
            panic!("pbt interp: branch to missing block {:?}", id);
        })
    };
    let mut vals: std::collections::HashMap<u32, i128> = std::collections::HashMap::new();
    let val = |vals: &std::collections::HashMap<u32, i128>, op: &Operand| -> i128 {
        match op {
            Operand::Value(v) => *vals.get(&v.0).unwrap_or_else(|| {
                panic!("pbt interp: use of undefined value {:?}", v);
            }),
            Operand::Const(c) => const_bits(c),
        }
    };
    let mut cur = 0usize;
    let mut prev: Option<BlockId> = None;
    let mut steps = 0usize;
    loop {
        if steps > limit { return RunOutcome::StepLimit; }
        steps += 1;
        let block = &func.blocks[cur];
        // Leading Phi instructions: evaluate all incoming selections first.
        let mut phi_writes: Vec<(u32, i128)> = Vec::new();
        for inst in &block.instructions {
            match inst {
                Instruction::Phi { dest, incoming, .. } => {
                    let from = prev.unwrap_or_else(|| {
                        panic!("pbt interp: phi in entry block {:?}", dest);
                    });
                    let mut picked: Option<i128> = None;
                    for (op, bid) in incoming {
                        if *bid == from {
                            picked = Some(val(&vals, op));
                        }
                    }
                    phi_writes.push((dest.0, picked.unwrap_or_else(|| {
                        panic!("pbt interp: phi has no incoming for pred {:?}", from);
                    })));
                }
                _ => break,
            }
        }
        for (id, v) in phi_writes {
            vals.insert(id, v);
        }
        let n_phi = vals.len(); // (marker only)
        let _ = n_phi;
        let mut trapped = false;
        for inst in &block.instructions {
            match inst {
                Instruction::Phi { .. } => {} // handled above
                Instruction::ParamRef { dest, param_idx, ty } => {
                    let a = args.get(*param_idx).copied().unwrap_or(0);
                    vals.insert(dest.0, canon(a, ty));
                }
                Instruction::BinOp { dest, op, lhs, rhs, ty } => {
                    match eval_binop(*op, val(&vals, lhs), val(&vals, rhs), ty) {
                        Some(v) => { vals.insert(dest.0, v); }
                        None => { trapped = true; break; }
                    }
                }
                Instruction::Cmp { dest, op, lhs, rhs, ty } => {
                    let v = eval_cmp(*op, val(&vals, lhs), val(&vals, rhs), ty);
                    vals.insert(dest.0, v);
                }
                Instruction::Cast { dest, src, from_ty, to_ty } => {
                    if !from_ty.is_float() && !to_ty.is_float() {
                        let v = eval_cast(val(&vals, src), from_ty, to_ty);
                        vals.insert(dest.0, v);
                    } else {
                        panic!("pbt interp: float cast in fragment");
                    }
                }
                Instruction::Copy { dest, src } => {
                    let v = val(&vals, src);
                    vals.insert(dest.0, v);
                }
                Instruction::UnaryOp { dest, op, src, ty } => {
                    match eval_unary(*op, val(&vals, src), ty) {
                        Some(v) => { vals.insert(dest.0, v); }
                        None => { trapped = true; break; }
                    }
                }
                Instruction::Select { dest, cond, true_val, false_val, .. } => {
                    let c = val(&vals, cond) != 0;
                    let v = if c { val(&vals, true_val) } else { val(&vals, false_val) };
                    vals.insert(dest.0, v);
                }
                other => panic!("pbt interp: unsupported instruction {:?}", other),
            }
        }
        if trapped { return RunOutcome::Trap; }
        let next: BlockId;
        match &block.terminator {
            Terminator::Return(op) => {
                let v = match op {
                    Some(o) => canon(val(&vals, o), &func.return_type),
                    None => 0,
                };
                return RunOutcome::Value(v);
            }
            Terminator::Branch(id) => next = *id,
            Terminator::CondBranch { cond, true_label, false_label } => {
                let c = val(&vals, cond);
                next = if low(c, width(&IrType::I64)) != 0 { *true_label } else { *false_label };
            }
            Terminator::Switch { val: sv, cases, default, ty, .. } => {
                let v = low(val(&vals, sv), width(ty));
                let mut tgt = *default;
                for (c, bid) in cases {
                    if low(*c as i128, width(ty)) == v {
                        tgt = *bid;
                        break;
                    }
                }
                next = tgt;
            }
            Terminator::Unreachable => return RunOutcome::Trap,
            other => panic!("pbt interp: unsupported terminator {:?}", other),
        }
        prev = Some(func.blocks[cur].label);
        cur = idx(next);
    }
}

// ── builders ─────────────────────────────────────────────────────────────────

pub struct FnBuilder {
    pub func: IrFunction,
    next: u32,
}

impl FnBuilder {
    pub fn new(name: &str, ret: IrType, params: &[IrType]) -> Self {
        let params: Vec<IrParam> = params
            .iter()
            .map(|ty| IrParam {
                ty: *ty,
                struct_size: None,
                struct_align: None,
                struct_eightbyte_classes: vec![],
                riscv_float_class: None,
            })
            .collect();
        FnBuilder { func: IrFunction::new(name.to_string(), ret, params, false), next: 0 }
    }

    pub fn v(&mut self) -> Value {
        let v = Value(self.next);
        self.next += 1;
        v
    }

    pub fn op_val(&self, v: Value) -> Operand { Operand::Value(v) }

    pub fn block(&mut self, insts: Vec<Instruction>, term: Terminator) {
        let label = BlockId(self.func.blocks.len() as u32);
        self.func.blocks.push(BasicBlock {
            label,
            instructions: insts,
            terminator: term,
            source_spans: vec![],
        });
    }

    pub fn seal(mut self) -> IrFunction {
        self.func.next_value_id = self.next;
        self.func
    }
}

// The safe op set for generated straight-line code (no UB at any input):
// shifts use constants < width; div/rem rhs are nonzero constants.
pub const SAFE_OPS: [IrBinOp; 9] = [
    IrBinOp::Add,
    IrBinOp::Sub,
    IrBinOp::Mul,
    IrBinOp::And,
    IrBinOp::Or,
    IrBinOp::Xor,
    IrBinOp::Shl,
    IrBinOp::LShr,
    IrBinOp::AShr,
];

pub fn op_by_index(i: u8) -> IrBinOp { SAFE_OPS[(i as usize) % SAFE_OPS.len()] }

pub fn int_ty_by_index(i: u8) -> IrType {
    match i % 4 {
        0 => IrType::I32,
        1 => IrType::I64,
        2 => IrType::U32,
        _ => IrType::U64,
    }
}

pub fn cmp_by_index(i: u8) -> IrCmpOp {
    const CMPS: [IrCmpOp; 10] = [
        IrCmpOp::Eq, IrCmpOp::Ne, IrCmpOp::Slt, IrCmpOp::Sle, IrCmpOp::Sgt,
        IrCmpOp::Sge, IrCmpOp::Ult, IrCmpOp::Ule, IrCmpOp::Ugt, IrCmpOp::Uge,
    ];
    CMPS[(i as usize) % CMPS.len()]
}

/// Count instructions across all blocks.
pub fn count_insts(f: &IrFunction) -> usize {
    f.blocks.iter().map(|b| b.instructions.len()).sum()
}

/// Run a per-function pass to its fixpoint (bounded), returning total changes.
pub fn to_fixpoint<F: FnMut(&mut IrFunction) -> usize>(f: &mut IrFunction, mut pass: F, max: usize) -> usize {
    let mut total = 0;
    for _ in 0..max {
        let n = pass(f);
        total += n;
        if n == 0 { break; }
    }
    total
}

/// Canonicalize args to a 2-param function of the given types.
pub fn canon_args(arg_tys: &[IrType], raw: &[i128]) -> Vec<i128> {
    arg_tys.iter().zip(raw.iter()).map(|(t, a)| canon(*a, t)).collect()
}

pub fn module_of(f: IrFunction) -> IrModule {
    let mut m = IrModule::new();
    m.functions.push(f);
    m
}

// ── shared function builders (plain-data driven) ─────────────────────────────

/// One safe straight-line step: (op_idx, a_pick, b_pick, b_const).
/// a_pick/b_pick select a prior value when even, a constant when odd.
pub type LinStep = (u8, u8, u8, i64);

/// Build a 2-param single-block function of safe BinOp steps at `ty`.
/// Shifts use constant amounts < width. No div/rem (UB-free fragment).
pub fn build_linear(ty: IrType, s0: i64, steps: &[LinStep]) -> IrFunction {
    let mut b = FnBuilder::new("f", ty, &[ty, ty]);
    let p0 = b.v();
    let p1 = b.v();
    let mut vals: Vec<Operand> = vec![Operand::Value(p0), Operand::Value(p1)];
    let mut insts = vec![
        Instruction::ParamRef { dest: p0, param_idx: 0, ty },
        Instruction::ParamRef { dest: p1, param_idx: 1, ty },
    ];
    let w = width(&ty);
    let pick = |p: u8, c: i64, vals: &Vec<Operand>| -> Operand {
        if p % 2 == 0 && !vals.is_empty() { vals[(p as usize / 2) % vals.len()] } else { const_int(&ty, (c % (1 << 20)) as u64) }
    };
    for &(oi, ap, bp, bc) in steps {
        let op = op_by_index(oi);
        let is_shift = matches!(op, IrBinOp::Shl | IrBinOp::LShr | IrBinOp::AShr);
        let d = b.v();
        let rhs = if is_shift { const_int(&ty, (bc.unsigned_abs() % w as u64) as u64) } else { pick(bp, bc, &vals) };
        insts.push(Instruction::BinOp { dest: d, op, lhs: pick(ap, s0, &vals), rhs, ty });
        vals.push(Operand::Value(d));
    }
    let ret = vals.last().copied().unwrap_or(Operand::Value(p0));
    b.block(insts, Terminator::Return(Some(ret)));
    b.seal()
}

/// Diamond: entry computes a param-dependent Cmp; two single-op arms; merge Phi.
pub fn build_diamond(cmp_idx: u8, k: i64, op1: u8, c1: i64, op2: u8, c2: i64) -> IrFunction {
    let ty = IrType::I32;
    let mut b = FnBuilder::new("f", ty, &[ty, ty]);
    let p0 = b.v();
    let p1 = b.v();
    let cond = b.v();
    let t = b.v();
    let u = b.v();
    let phi = b.v();
    let entry = vec![
        Instruction::ParamRef { dest: p0, param_idx: 0, ty },
        Instruction::ParamRef { dest: p1, param_idx: 1, ty },
        Instruction::Cmp { dest: cond, op: cmp_by_index(cmp_idx), lhs: Operand::Value(p0), rhs: const_int(&ty, (k % 1000) as u64), ty },
    ];
    b.block(entry, Terminator::CondBranch { cond: Operand::Value(cond), true_label: BlockId(1), false_label: BlockId(2) });
    let arm1 = vec![Instruction::BinOp { dest: t, op: op_by_index(op1), lhs: Operand::Value(p1), rhs: const_int(&ty, (c1 % 1000) as u64), ty }];
    b.block(arm1, Terminator::Branch(BlockId(3)));
    let arm2 = vec![Instruction::BinOp { dest: u, op: op_by_index(op2), lhs: Operand::Value(p0), rhs: const_int(&ty, (c2 % 1000) as u64), ty }];
    b.block(arm2, Terminator::Branch(BlockId(3)));
    let merge = vec![Instruction::Phi {
        dest: phi,
        ty,
        incoming: vec![(Operand::Value(t), BlockId(1)), (Operand::Value(u), BlockId(2))],
    }];
    b.block(merge, Terminator::Return(Some(Operand::Value(phi))));
    b.seal()
}

/// Random CFG spec: per block (term_kind, a, b, c1, c2) where term_kind
/// 0=Return(param pick), 1=Branch(a), 2=CondBranch{Cmp(pick,a)|Const(a&1), b, c2},
/// 3=Switch{param, [(c1,b),(c2,a)], default=a}; per-block 0-2 safe steps
/// (op_idx, pick, const) over entry values; last block always returns.
pub struct CfgSpec {
    pub terms: Vec<(u8, u32, u32, i64, i64)>,
    pub steps: Vec<Vec<(u8, u8, i64)>>,
    pub ret_pick: u8,
    pub cmp_idx: u8,
}

pub fn build_random_cfg(spec: &CfgSpec) -> IrFunction {
    let ty = IrType::I32;
    let n = spec.terms.len().max(2);
    let mut b = FnBuilder::new("f", ty, &[ty, ty]);
    let p0 = b.v();
    let p1 = b.v();
    let mut entry_vals: Vec<Operand> = vec![Operand::Value(p0), Operand::Value(p1)];
    let mut entry_insts = vec![
        Instruction::ParamRef { dest: p0, param_idx: 0, ty },
        Instruction::ParamRef { dest: p1, param_idx: 1, ty },
    ];
    for &(oi, pick, c) in spec.steps.first().map(|s| s.as_slice()).unwrap_or(&[]) {
        let d = b.v();
        let is_shift = matches!(op_by_index(oi), IrBinOp::Shl | IrBinOp::LShr | IrBinOp::AShr);
        let rhs = if is_shift { const_int(&ty, (c.unsigned_abs() % 31) as u64) } else { const_int(&ty, (c % 1000) as u64) };
        let lhs = entry_vals[(pick as usize) % entry_vals.len()];
        entry_insts.push(Instruction::BinOp { dest: d, op: op_by_index(oi), lhs, rhs, ty });
        entry_vals.push(Operand::Value(d));
    }
    let nn = n as u32;
    let term0 = {
        let (kind, a, bv, c1, c2) = spec.terms[0];
        let pick_op = |i: u32| entry_vals[(i as usize) % entry_vals.len()];
        match kind % 4 {
            0 => Terminator::Return(Some(pick_op(a))),
            1 => Terminator::Branch(BlockId(a % nn)),
            2 => {
                let cd = b.v();
                entry_insts.push(Instruction::Cmp { dest: cd, op: cmp_by_index(spec.cmp_idx), lhs: pick_op(a), rhs: const_int(&ty, (c1 % 100) as u64), ty });
                Terminator::CondBranch { cond: Operand::Value(cd), true_label: BlockId(bv % nn), false_label: BlockId((c2 as u32) % nn) }
            }
            _ => Terminator::Switch {
                val: Operand::Value(p0),
                cases: vec![(c1 % 4, BlockId(bv % nn)), (c2 % 4, BlockId(a % nn))],
                default: BlockId(bv % nn),
                ty,
            },
        }
    };
    b.block(entry_insts, term0);
    for i in 1..n {
        let (kind, a, bv, c1, c2) = spec.terms[i];
        let mut insts: Vec<Instruction> = Vec::new();
        let mut locals: Vec<Operand> = Vec::new();
        for &(oi, pick, c) in spec.steps.get(i).map(|s| s.as_slice()).unwrap_or(&[]) {
            let d = b.v();
            let is_shift = matches!(op_by_index(oi), IrBinOp::Shl | IrBinOp::LShr | IrBinOp::AShr);
            let rhs = if is_shift { const_int(&ty, (c.unsigned_abs() % 31) as u64) } else { const_int(&ty, (c % 1000) as u64) };
            let lhs = if pick % 2 == 0 { entry_vals[(pick as usize / 2) % entry_vals.len()] } else { locals.get((pick as usize / 2) % locals.len().max(1)).copied().unwrap_or(Operand::Value(p0)) };
            insts.push(Instruction::BinOp { dest: d, op: op_by_index(oi), lhs, rhs, ty });
            locals.push(Operand::Value(d));
        }
        let pick_op = |i2: u32| {
            let all = locals.len() + entry_vals.len();
            let idx = (i2 as usize) % all.max(1);
            if idx < locals.len() { locals[idx] } else { entry_vals[idx - locals.len()] }
        };
        let last = i + 1 == n;
        let term = if last {
            Terminator::Return(Some(if spec.ret_pick % 2 == 0 { pick_op(a) } else { const_int(&ty, (c1 % 1000) as u64) }))
        } else {
            match kind % 4 {
                0 => Terminator::Return(Some(pick_op(a))),
                1 => Terminator::Branch(BlockId(a % nn)),
                2 => Terminator::CondBranch { cond: const_int(&ty, (a % 2) as u64), true_label: BlockId(bv % nn), false_label: BlockId((c2 as u32) % nn) },
                _ => Terminator::Switch {
                    val: Operand::Value(p1),
                    cases: vec![(c1 % 4, BlockId(bv % nn)), (c2 % 4, BlockId(a % nn))],
                    default: BlockId((c2 as u32) % nn),
                    ty,
                },
            }
        };
        b.block(insts, term);
    }
    b.seal()
}

/// Structured bounded loop reaching LICM/IVSR/loop_analysis:
/// acc = 0; for (i = 0; i < trip; i++) { acc += i * mul_k; } return acc.
pub fn build_loop(trip: u8, mul_k: i64) -> IrFunction {
    let ty = IrType::I64;
    let mut b = FnBuilder::new("f", ty, &[ty]);
    let p0 = b.v();          // param (bait for LICM: invariant mul inside body)
    let acc0 = b.v();
    let i0 = b.v();
    let tripv = b.v();
    let acc = b.v();         // header phis
    let iv = b.v();
    let cond = b.v();
    let inv = b.v();         // loop-invariant: p0 * 7 (hoistable)
    let prod = b.v();        // iv * mul_k (IVSR target)
    let sum = b.v();
    let accn = b.v();
    let inext = b.v();
    b.block(
        vec![
            Instruction::ParamRef { dest: p0, param_idx: 0, ty },
            Instruction::Copy { dest: acc0, src: Operand::Const(IrConst::I64(0)) },
            Instruction::Copy { dest: i0, src: Operand::Const(IrConst::I64(0)) },
            Instruction::Copy { dest: tripv, src: Operand::Const(IrConst::I64((trip % 8) as i64)) },
        ],
        Terminator::Branch(BlockId(1)),
    );
    b.block(
        vec![
            Instruction::Phi { dest: acc, ty, incoming: vec![(Operand::Value(acc0), BlockId(0)), (Operand::Value(accn), BlockId(2))] },
            Instruction::Phi { dest: iv, ty, incoming: vec![(Operand::Value(i0), BlockId(0)), (Operand::Value(inext), BlockId(2))] },
            Instruction::Cmp { dest: cond, op: IrCmpOp::Slt, lhs: Operand::Value(iv), rhs: Operand::Value(tripv), ty },
        ],
        Terminator::CondBranch { cond: Operand::Value(cond), true_label: BlockId(2), false_label: BlockId(3) },
    );
    b.block(
        vec![
            Instruction::BinOp { dest: inv, op: IrBinOp::Mul, lhs: Operand::Value(p0), rhs: Operand::Const(IrConst::I64(7)), ty },
            Instruction::BinOp { dest: prod, op: IrBinOp::Mul, lhs: Operand::Value(iv), rhs: Operand::Const(IrConst::I64(mul_k % 8)), ty },
            Instruction::BinOp { dest: sum, op: IrBinOp::Add, lhs: Operand::Value(prod), rhs: Operand::Value(inv), ty },
            Instruction::BinOp { dest: accn, op: IrBinOp::Add, lhs: Operand::Value(acc), rhs: Operand::Value(sum), ty },
            Instruction::BinOp { dest: inext, op: IrBinOp::Add, lhs: Operand::Value(iv), rhs: Operand::Const(IrConst::I64(1)), ty },
        ],
        Terminator::Branch(BlockId(1)),
    );
    b.block(vec![], Terminator::Return(Some(Operand::Value(acc))));
    b.seal()
}
