# Properties: encode_cbz

## encode_cbz_diff_imm_llvm_mc
- Tier: 2
- Rationale: Strongest applicable oracle is differential against llvm-mc (independent AArch64 assembler) for the ARM ARM immediate form `cbz/cbnz Rt, #imm`. State machine rejected (pure function, no lifecycle). In-tree CBZ decoder does not exist so algebraic round-trip is unavailable. encode_tbz / encode_cond_branch are different jobs (test-bit / B.cond) and fail the same-job sibling gate as a differential reference. Doc evidence: assembler README gas-compat; ARM ARM Compare and branch (immediate) sf 011010 op imm19 Rt; README CondBr19 for CBZ/CBNZ.
- Seed: src/backend/arm/assembler/encoder/compare_branch.rs encode_branch_pbt::encode_branch_diff_imm_llvm_mc
- Formal: ∀ rt ∈ {x0..x30,xzr,lr,w0..w30,wzr}, is_nz ∈ {false,true}, imm ∈ {k*4 | k ∈ ℤ, -2^20 ≤ imm ≤ 2^20-4}. encode_cbz([Reg(rt), Imm(imm)], is_nz) = Word(llvm-mc(mnemonic(is_nz) + " " + rt + ", #" + imm)).
- Test file: src/backend/arm/assembler/encoder/compare_branch.rs
- Status: failing
- Counterexample: [Reg("x0"), Imm(-1048576)], is_nz=false  (also Imm(0), Imm(4))
- Bug report: pbt-out/bug_reports/encode_cbz_imm_offset.md

```property
function: encoder.compare_branch.encode_cbz
oracle: differential
predicate:
  quantifier: forall
  vars: [rt, is_nz, imm]
  domain: { rt: gpr_name, is_nz: bool, imm: aligned_pc_offset_19 }
  relation:
    op: eq
    lhs: encode_cbz([Reg(rt), Imm(imm)], is_nz) as Word
    rhs: llvm_mc(cbz_mnemonic(is_nz) + " " + rt + ", #" + imm)
generators:
  rt: { gen: string }
  is_nz: { gen: bool }
  imm: { gen: int, min: -1048576, max: 1048572, type: i64 }
evidence: src/backend/arm/assembler/README.md:5-14 gas-compatible AArch64; README.md:220 Branches lists cbz/cbnz; encoder/mod.rs:321-322 cbz/cbnz dispatch; compare_branch.rs:242 CBZ/CBNZ sf 011010 op imm19 Rt; ARM ARM Compare and branch (immediate)
```

## encode_cbz_symbol_reloc
- Tier: 4
- Rationale: Algebraic invariant from README CondBr19 / R_AARCH64_CONDBR19 ELF 280 and the encoder contract that symbol/label targets leave imm19=0 for the assembler/linker to fill. Stronger differential cannot compare a concrete word for unresolved labels (llvm-mc emits a fixup, not a numeric encoding). encode_cond_branch is not a same-job sibling.
- Seed: encode_branch_pbt::encode_branch_symbol_reloc
- Formal: ∀ rt ∈ GPR, is_nz ∈ bool, s ∈ ident, a ∈ i64. encode_cbz([Reg(rt), Symbol(s)], is_nz) = WordWithReloc{word=base(rt,is_nz), CondBr19, s, 0} ∧ encode_cbz([Reg(rt), Label(s)], is_nz) = same ∧ encode_cbz([Reg(rt), SymbolOffset(s,a)], is_nz) = WordWithReloc{word=base(rt,is_nz), CondBr19, s, a} ∧ elf_type(CondBr19)=280 ∧ (word & 0x00ffffe0)=0.
- Test file: src/backend/arm/assembler/encoder/compare_branch.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.compare_branch.encode_cbz
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [rt, is_nz, s, a]
  domain: { rt: gpr_name, is_nz: bool, s: label_ident, a: i64 }
  body: encode_cbz([Reg(rt), SymbolOffset(s, a)], is_nz) == WordWithReloc(base(rt,is_nz), CondBr19, s, a)
generators:
  rt: { gen: string }
  is_nz: { gen: bool }
  s: { gen: string }
  a: { gen: int, min: -4096, max: 4096, type: i64 }
evidence: encoder/mod.rs:76 R_AARCH64_CONDBR19; encoder/mod.rs:115 CondBr19 => 280; README.md:267 CondBr19 280; compare_branch.rs:242-251 imm19 filled by linker/assembler
```

## encode_cbz_meta_cbz_vs_cbnz
- Tier: 4
- Rationale: Algebraic metamorphic: ARM ARM CBNZ is CBZ with bit 24 set (op field). encode_cbz(..., true) is not a same-job differential reference; the relation is the documented opcode pair. Stronger differential already covers the happy path; this isolates the CBZ-vs-CBNZ contract independently of llvm-mc.
- Seed: encode_branch_pbt::encode_branch_meta_vs_bl
- Formal: ∀ rt ∈ GPR, s ∈ ident, a ∈ i64. encode_cbz([Reg(rt), SymbolOffset(s,a)], true).word XOR encode_cbz([Reg(rt), SymbolOffset(s,a)], false).word = 1<<24 ∧ both reloc types are CondBr19 ∧ same symbol and addend.
- Test file: src/backend/arm/assembler/encoder/compare_branch.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.compare_branch.encode_cbz
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [rt, s, a]
  domain: { rt: gpr_name, s: label_ident, a: i64 }
  relation:
    op: eq
    lhs: encode_cbz(ops, true).word XOR encode_cbz(ops, false).word
    rhs: 1 << 24
generators:
  rt: { gen: string }
  s: { gen: string }
  a: { gen: int, min: -4096, max: 4096, type: i64 }
evidence: compare_branch.rs:242 CBZ/CBNZ sf 011010 op imm19 Rt; ARM ARM Compare and branch (immediate) bit 24 op
```

## encode_cbz_word_layout
- Tier: 4
- Rationale: Algebraic invariant from ARM ARM Compare and branch (immediate): bits[30:25]=011010, sf at 31 from Rt width, op at 24 from is_nz, Rt at [4:0], imm19 field zero until reloc fill. Stronger differential already covers Imm against llvm-mc; this pins the field split on the reloc path independently of the assembler.
- Seed: encode_branch_pbt::encode_branch_word_layout
- Formal: ∀ n ∈ 0..31, is_64 ∈ bool, is_nz ∈ bool. let (w, r) = encode_cbz([Reg(name(n,is_64)), Symbol(s)], is_nz). (w >> 25) & 0x3f = 0b011010 ∧ (w >> 31) = sf(is_64) ∧ ((w >> 24) & 1) = is_nz ∧ (w & 0x1f) = n ∧ (w & 0x00ffffe0) = 0 ∧ r is CondBr19.
- Test file: src/backend/arm/assembler/encoder/compare_branch.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.compare_branch.encode_cbz
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [n, is_64, is_nz]
  domain: { n: u32 0..=31, is_64: bool, is_nz: bool }
  body: layout(encode_cbz([Reg(name(n,is_64)), Symbol(s)], is_nz)) holds
generators:
  n: { gen: int, min: 0, max: 31, type: u32 }
  is_64: { gen: bool }
  is_nz: { gen: bool }
evidence: compare_branch.rs:242 CBZ/CBNZ sf 011010 op imm19 Rt; ARM ARM Compare and branch (immediate)
```

## encode_cbz_neg_arity
- Tier: 3
- Rationale: Negative/error contract from llvm-mc / gas: bare `cbz` and `cbz x0` are "too few operands". Stronger oracles do not apply to the empty/short-operand domain.
- Seed: encode_branch_pbt::encode_branch_neg_arity
- Formal: ∀ is_nz ∈ bool. encode_cbz([], is_nz) = Err ∧ encode_cbz([Reg("x0")], is_nz) = Err.
- Test file: src/backend/arm/assembler/encoder/compare_branch.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.compare_branch.encode_cbz
oracle: negative_error
predicate:
  quantifier: forall
  vars: [is_nz]
  domain: { is_nz: bool }
  relation:
    op: holds
    expr: encode_cbz([], is_nz).is_err() && encode_cbz([Reg("x0")], is_nz).is_err()
generators:
  is_nz: { gen: bool }
expected_error: String
evidence: llvm-mc -triple=aarch64 rejects `cbz` and `cbz x0` as too few operands; README.md:5-14 gas-compat
```

## encode_cbz_neg_extra_operand
- Tier: 3
- Rationale: Negative/error contract from llvm-mc: `cbz x0, label, extra` is "invalid operand". Gas-compat assembler must reject a third operand. Stronger oracles do not apply to over-arity.
- Seed: encode_branch_pbt::encode_branch_neg_extra_operand
- Formal: ∀ rt ∈ GPR, s ∈ ident, extra ∈ Operand, is_nz ∈ bool. encode_cbz([Reg(rt), Symbol(s), extra], is_nz) = Err.
- Test file: src/backend/arm/assembler/encoder/compare_branch.rs
- Status: failing
- Counterexample: [Reg("x0"), Symbol("labl0"), Reg("x1")], is_nz=false
- Bug report: pbt-out/bug_reports/encode_cbz_extra_operand.md

```property
function: encoder.compare_branch.encode_cbz
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rt, s, extra, is_nz]
  domain: { rt: gpr_name, s: label_ident, extra: Operand, is_nz: bool }
  relation:
    op: holds
    expr: encode_cbz([Reg(rt), Symbol(s), extra], is_nz).is_err()
generators:
  rt: { gen: string }
  s: { gen: string }
  extra: { gen: int, min: 0, max: 3, type: u32 }
  is_nz: { gen: bool }
expected_error: String
evidence: llvm-mc -triple=aarch64 rejects `cbz x0, #0, x1` as invalid operand; README.md:5-14 gas-compat
```

## encode_cbz_neg_wrong_reg
- Tier: 3
- Rationale: Negative/error contract from llvm-mc / ARM ARM: CBZ Rt is a GPR (Wt/Xt/WZR/XZR), never SP/WSP or FP/SIMD. Stronger oracles do not apply to the invalid-Rt domain.
- Seed: encode_blr_pbt::encode_blr_neg_wrong_reg_class
- Formal: ∀ name ∈ {sp, wsp, dN, sN, qN, vN, hN, bN, x32, foo, ""}, is_nz ∈ bool. encode_cbz([Reg(name), Symbol("L")], is_nz) = Err.
- Test file: src/backend/arm/assembler/encoder/compare_branch.rs
- Status: failing
- Counterexample: [Reg("sp"), Symbol("L")], is_nz=false  (also Reg("d0"))
- Bug report: pbt-out/bug_reports/encode_cbz_sp_as_zr.md; pbt-out/bug_reports/encode_cbz_fp_reg.md

```property
function: encoder.compare_branch.encode_cbz
oracle: negative_error
predicate:
  quantifier: forall
  vars: [name, is_nz]
  domain: { name: invalid_rt, is_nz: bool }
  relation:
    op: holds
    expr: encode_cbz([Reg(name), Symbol("L")], is_nz).is_err()
generators:
  name: { gen: string }
  is_nz: { gen: bool }
expected_error: String
evidence: llvm-mc -triple=aarch64 rejects `cbz sp, #0`, `cbz wsp, label`, `cbz d0, #0`; ARM ARM Rt is Wt/Xt
```

## encode_cbz_neg_imm_unaligned_oor
- Tier: 3
- Rationale: Negative/error contract from ARM ARM / llvm-mc: PC offset must be a multiple of 4 in [-1048576, 1048572]. Unaligned or out-of-range immediates must Err. Stronger differential does not apply to the invalid-imm domain.
- Seed: encode_branch_pbt::encode_branch_neg_imm_unaligned_oor
- Formal: ∀ rt ∈ GPR, is_nz ∈ bool, imm ∈ i64 \ aligned_pc_offset_19. encode_cbz([Reg(rt), Imm(imm)], is_nz) = Err.
- Test file: src/backend/arm/assembler/encoder/compare_branch.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.compare_branch.encode_cbz
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rt, is_nz, imm]
  domain: { rt: gpr_name, is_nz: bool, imm: unaligned_or_oor_pc_offset_19 }
  relation:
    op: holds
    expr: encode_cbz([Reg(rt), Imm(imm)], is_nz).is_err()
generators:
  rt: { gen: string }
  is_nz: { gen: bool }
  imm: { gen: int, type: i64 }
expected_error: String
evidence: llvm-mc -triple=aarch64 rejects `cbz x0, #1` and `cbz w0, #1048576` as expected label or encodable integer pc offset; ARM ARM imm19 range ±1 MiB multiple of 4
```

## encode_cbz_symbol_misclassified
- Tier: 4
- Rationale: Algebraic invariant from the documented get_symbol workaround (encoder/mod.rs:982-986): parser-misclassified Reg/Cond/Barrier names are valid symbols in a branch-target context. Coverage-sweep property to reach those match arms. Stronger differential cannot compare a concrete word for unresolved labels.
- Seed: encode_branch_pbt::encode_branch_symbol_misclassified
- Formal: ∀ name ∈ {eq,ne,lt,gt,sy,ish,st,ld}, which ∈ {Reg,Cond,Barrier}, is_nz ∈ bool. encode_cbz([Reg("x0"), which(name)], is_nz) = WordWithReloc{word=base(X,is_nz), CondBr19, name, 0}.
- Test file: src/backend/arm/assembler/encoder/compare_branch.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.compare_branch.encode_cbz
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [name, which, is_nz]
  domain: { name: colliding_ident, which: {Reg,Cond,Barrier}, is_nz: bool }
  body: encode_cbz([Reg("x0"), which(name)], is_nz) == WordWithReloc(base(X,is_nz), CondBr19, name, 0)
generators:
  name: { gen: string }
  which: { gen: int, min: 0, max: 2, type: u32 }
  is_nz: { gen: bool }
evidence: encoder/mod.rs:982-986 get_symbol parser-misclassification workaround; README.md:456-464 deferred branch relocs
```

## encode_cbz_neg_bad_label_kind
- Tier: 3
- Rationale: Negative/error contract: CBZ label slot is a symbol/label/imm, not Mem/Shift/Extend/RegArrangement/Expr/RegList. get_symbol's other arm returns Err. Stronger oracles do not apply to the wrong-kind domain. Coverage-sweep property to reach that arm.
- Seed: encode_branch_pbt::encode_branch_neg_bad_operand
- Formal: ∀ kind ∈ {Mem, Shift, Extend, RegArrangement, Expr, RegList}, is_nz ∈ bool. encode_cbz([Reg("x0"), kind], is_nz) = Err.
- Test file: src/backend/arm/assembler/encoder/compare_branch.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.compare_branch.encode_cbz
oracle: negative_error
predicate:
  quantifier: forall
  vars: [which, is_nz]
  domain: { which: u32 0..=5, is_nz: bool }
  relation:
    op: holds
    expr: encode_cbz([Reg("x0"), bad_kind(which)], is_nz).is_err()
generators:
  which: { gen: int, min: 0, max: 5, type: u32 }
  is_nz: { gen: bool }
expected_error: String
evidence: encoder/mod.rs:988 get_symbol other => Err; llvm-mc rejects non-label second operands
```
