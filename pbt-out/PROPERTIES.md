# Properties: encode_tbz

## encode_tbz_diff_imm_llvm_mc
- Tier: 5
- Rationale: Strongest evidenced oracle is differential vs llvm-mc on the valid TBZ/TBNZ immediate-offset domain. State machine rejected (pure function, no lifecycle). Algebraic round-trip rejected (no in-tree TBZ decoder). encode_cbz rejected as same-job sibling (CondBr19, no bit operand). encode_cond_branch rejected (B.cond). Weaker: ARM field invariant, TBZ/TBNZ metamorphic, negative_error.
- Doc contract: compare_branch.rs:261 "TBZ/TBNZ: b5 011011 op b40 imm14 Rt" — asserted fingerprint 49c8712c
- Seed: compare_branch.rs encode_cbz_diff_imm_llvm_mc
- Formal: ∀ rt ∈ GPR_W ∪ GPR_X (n∈0..31 including ZR/LR), bit ∈ 0..31 (W) / 0..63 (X), imm ∈ {k·4 | k∈ℤ, −32768 ≤ k·4 ≤ 32764}, is_nz ∈ {false,true}. encode_tbz([Reg(rt), Imm(bit), Imm(imm)], is_nz) = Word(llvm-mc("tbz/tbnz rt, #bit, #imm"))
- Test file: src/backend/arm/assembler/encoder/encode_tbz_pbt.rs
- Status: failing
- Counterexample: encode_tbz([Reg("x0"), Imm(0), Imm(-32768)], false) — tbz x0, #0, #-32768
- Bug report: pbt-out/bug_reports/encode_tbz_imm_offset.md

```property
function: encoder.compare_branch.encode_tbz
oracle: differential
predicate:
  quantifier: forall
  vars: [rt, bit, imm, is_nz]
  domain: { rt: "GPR W/X 0..31 incl ZR/LR", bit: "0..31 W / 0..63 X", imm: "aligned -32768..32764", is_nz: bool }
  relation:
    op: eq
    lhs: encode_tbz([Reg(rt), Imm(bit), Imm(imm)], is_nz)
    rhs: llvm_mc_word("tbz|tbnz rt, #bit, #imm")
generators:
  rt: { gen: int, min: 0, max: 31, type: u32 }
  bit: { gen: int, min: 0, max: 63, type: i64 }
  imm: { gen: int, min: -32768, max: 32764, type: i64 }
  is_nz: { gen: bool }
evidence: README.md:12 gas-compat; README.md:220 tbz/tbnz; encoder/mod.rs:455-456 dispatch; ARM ARM Test and branch (immediate)
```

## encode_tbz_symbol_reloc
- Tier: 4
- Rationale: README.md:267 TstBr14 ELF 279 and README.md:458 say TBZ/TBNZ emit a 14-bit test-and-branch relocation with imm14 left 0 for the linker/assembler. Exact structural invariant of the symbol/label form.
- Doc contract: compare_branch.rs:261 "TBZ/TBNZ: b5 011011 op b40 imm14 Rt" — asserted fingerprint 49c8712c
- Seed: compare_branch.rs encode_cbz_symbol_reloc
- Formal: ∀ n ∈ 0..31, bit ∈ 0..63, is_nz ∈ {false,true}, s a symbol, a ∈ ℤ. encode_tbz([Reg(xn|wn), Imm(bit), Symbol(s)|Label(s)|SymbolOffset(s,a)], is_nz) = WordWithReloc { word: (b5<<31)|(0b011011<<25)|(op<<24)|(b40<<19)|n with imm14=0, reloc_type=TstBr14, elf_type=279, symbol=s, addend=0|a }
- Test file: src/backend/arm/assembler/encoder/encode_tbz_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.compare_branch.encode_tbz
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [n, bit, is_nz, addend]
  domain: { n: "0..31", bit: "0..63", is_nz: bool, addend: i64 }
  relation:
    op: holds
    expr: word_imm14_zero_and_reloc_tstbr14_elf279
generators:
  n: { gen: int, min: 0, max: 31, type: u32 }
  bit: { gen: int, min: 0, max: 63, type: i64 }
  is_nz: { gen: bool }
  addend: { gen: int, min: -4096, max: 4096, type: i64 }
evidence: README.md:267 TstBr14 ELF 279; README.md:458 TBZ/TBNZ deferred reloc; compare_branch.rs:261-271
```

## encode_tbz_word_layout
- Tier: 4
- Rationale: ARM ARM Test and branch (immediate) field layout is an exact structural invariant of every successful encoding. Documented bounds b5/b40/op/Rt sampled at 0, 31, 32, 63.
- Doc contract: compare_branch.rs:261 "TBZ/TBNZ: b5 011011 op b40 imm14 Rt" — asserted fingerprint 49c8712c
- Seed: compare_branch.rs encode_cbz_word_layout
- Formal: ∀ n ∈ 0..31, bit ∈ 0..63, is_nz ∈ {false,true}. let w = encode_tbz([Reg(xn), Imm(bit), Symbol("L")], is_nz).word. bits[30:25]=011011 ∧ bit[31]=b5 ∧ bit[24]=op ∧ bits[23:19]=b40 ∧ bits[18:5]=0 ∧ bits[4:0]=n
- Test file: src/backend/arm/assembler/encoder/encode_tbz_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.compare_branch.encode_tbz
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [n, bit, is_nz]
  domain: { n: "0..31", bit: "0..63", is_nz: bool }
  relation:
    op: holds
    expr: arm_tbz_fields(w, n, bit, is_nz)
generators:
  n: { gen: int, min: 0, max: 31, type: u32 }
  bit: { gen: int, min: 0, max: 63, type: i64 }
  is_nz: { gen: bool }
evidence: compare_branch.rs:261 ARM TBZ/TBNZ layout
```

## encode_tbz_meta_tbz_vs_tbnz
- Tier: 4
- Rationale: ARM ARM op bit is the sole TBZ/TBNZ distinction (bit 24). Metamorphic: same operands, is_nz true vs false, encodings differ only at bit 24 and share TstBr14 reloc.
- Doc contract: compare_branch.rs:261 "TBZ/TBNZ: b5 011011 op b40 imm14 Rt" — asserted fingerprint 49c8712c
- Seed: compare_branch.rs encode_cbz_meta_cbz_vs_cbnz
- Formal: ∀ n ∈ 0..31, bit ∈ 0..63, s a symbol, a ∈ ℤ. encode_tbz(ops, true).word XOR encode_tbz(ops, false).word = 1<<24 ∧ both reloc_type=TstBr14 ∧ same symbol ∧ same addend
- Test file: src/backend/arm/assembler/encoder/encode_tbz_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.compare_branch.encode_tbz
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [n, bit, addend]
  domain: { n: "0..31", bit: "0..63", addend: i64 }
  relation:
    op: eq
    lhs: encode_tbz(ops, true).word XOR encode_tbz(ops, false).word
    rhs: 1u32 << 24
generators:
  n: { gen: int, min: 0, max: 31, type: u32 }
  bit: { gen: int, min: 0, max: 63, type: i64 }
  addend: { gen: int, min: -4096, max: 4096, type: i64 }
evidence: compare_branch.rs:259 op = is_nz; ARM ARM TBZ op=0 TBNZ op=1
```

## encode_tbz_neg_arity
- Tier: 3
- Rationale: llvm-mc rejects too-few-operands (`tbz x0`, `tbz x0, #0`). get_reg/get_imm/get_symbol fail on missing slots. Negative/error contract with documented exact Err.
- Doc contract: compare_branch.rs:261 "TBZ/TBNZ: b5 011011 op b40 imm14 Rt" — asserted fingerprint 49c8712c
- Seed: compare_branch.rs encode_cbz_neg_arity
- Formal: ∀ is_nz ∈ {false,true}. encode_tbz([], is_nz) is Err ∧ encode_tbz([Reg("x0")], is_nz) is Err ∧ encode_tbz([Reg("x0"), Imm(0)], is_nz) is Err
- Test file: src/backend/arm/assembler/encoder/encode_tbz_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.compare_branch.encode_tbz
oracle: negative_error
predicate:
  quantifier: forall
  vars: [is_nz]
  domain: { is_nz: bool }
  relation:
    op: holds
    expr: encode_tbz([], is_nz).is_err() && encode_tbz([Reg("x0")], is_nz).is_err() && encode_tbz([Reg("x0"), Imm(0)], is_nz).is_err()
generators:
  is_nz: { gen: bool }
expected_error: String
evidence: llvm-mc "too few operands for instruction"; get_reg/get_imm/get_symbol at 0/1/2
```

## encode_tbz_neg_extra_operand
- Tier: 3
- Rationale: llvm-mc rejects a fourth operand (`tbz x0, #0, label, x1`). Body has no operands.len() upper bound. Negative/error contract; extra operands stay in the generator (not declared invalid by this function).
- Doc contract: compare_branch.rs:261 "TBZ/TBNZ: b5 011011 op b40 imm14 Rt" — asserted fingerprint 49c8712c
- Seed: compare_branch.rs encode_cbz_neg_extra_operand
- Formal: ∀ n ∈ 0..30, bit ∈ 0..63, is_nz ∈ {false,true}, extra ∈ Operand. encode_tbz([Reg(xn), Imm(bit), Symbol(s), extra], is_nz) is Err
- Test file: src/backend/arm/assembler/encoder/encode_tbz_pbt.rs
- Status: failing
- Counterexample: encode_tbz([Reg("x0"), Imm(0), Symbol("labl0"), Reg("x1")], false)
- Bug report: pbt-out/bug_reports/encode_tbz_extra_operand.md

```property
function: encoder.compare_branch.encode_tbz
oracle: negative_error
predicate:
  quantifier: forall
  vars: [n, bit, is_nz, extra]
  domain: { n: "0..30", bit: "0..63", is_nz: bool, extra: Operand }
  relation:
    op: holds
    expr: encode_tbz([Reg(xn), Imm(bit), Symbol(s), extra], is_nz).is_err()
generators:
  n: { gen: int, min: 0, max: 30, type: u32 }
  bit: { gen: int, min: 0, max: 63, type: i64 }
  is_nz: { gen: bool }
  extra: { gen: int, min: 0, max: 3, type: u32 }
expected_error: String
evidence: llvm-mc "invalid operand for instruction" on fourth operand
```

## encode_tbz_neg_wrong_reg
- Tier: 3
- Rationale: llvm-mc rejects SP/WSP as Rt. parse_reg_num maps SP to 31; those inputs stay in the generator (not declared invalid by this function).
- Doc contract: compare_branch.rs:261 "TBZ/TBNZ: b5 011011 op b40 imm14 Rt" — asserted fingerprint 49c8712c
- Seed: compare_branch.rs encode_cbz_neg_wrong_reg
- Formal: ∀ name ∈ {sp,wsp}, is_nz ∈ {false,true}. encode_tbz([Reg(name), Imm(0), Symbol("L")], is_nz) is Err
- Test file: src/backend/arm/assembler/encoder/encode_tbz_pbt.rs
- Status: failing
- Counterexample: encode_tbz([Reg("sp"), Imm(0), Symbol("L")], false)
- Bug report: pbt-out/bug_reports/encode_tbz_sp_as_zr.md

```property
function: encoder.compare_branch.encode_tbz
oracle: negative_error
predicate:
  quantifier: forall
  vars: [name, is_nz]
  domain: { name: "sp|wsp", is_nz: bool }
  relation:
    op: holds
    expr: encode_tbz([Reg(name), Imm(0), Symbol("L")], is_nz).is_err()
generators:
  name: { gen: string }
  is_nz: { gen: bool }
expected_error: String
evidence: llvm-mc "invalid operand for instruction" on sp
```

## encode_tbz_neg_fp_reg
- Tier: 3
- Rationale: llvm-mc rejects FP/SIMD Rt (d/s/q/v/h/b). parse_reg_num accepts those prefixes; inputs stay in the generator.
- Doc contract: compare_branch.rs:261 "TBZ/TBNZ: b5 011011 op b40 imm14 Rt" — asserted fingerprint 49c8712c
- Seed: compare_branch.rs encode_cbz_neg_wrong_reg
- Formal: ∀ prefix ∈ {d,s,q,v,h,b}, n ∈ 0..31, is_nz ∈ {false,true}. encode_tbz([Reg(prefix+n), Imm(0), Symbol("L")], is_nz) is Err
- Test file: src/backend/arm/assembler/encoder/encode_tbz_pbt.rs
- Status: failing
- Counterexample: encode_tbz([Reg("d0"), Imm(0), Symbol("L")], false)
- Bug report: pbt-out/bug_reports/encode_tbz_fp_reg.md

```property
function: encoder.compare_branch.encode_tbz
oracle: negative_error
predicate:
  quantifier: forall
  vars: [name, is_nz]
  domain: { name: "d/s/q/v/h/b N", is_nz: bool }
  relation:
    op: holds
    expr: encode_tbz([Reg(name), Imm(0), Symbol("L")], is_nz).is_err()
generators:
  name: { gen: string }
  is_nz: { gen: bool }
expected_error: String
evidence: llvm-mc "invalid operand for instruction" on d0
```

## encode_tbz_neg_bit_oor
- Tier: 3
- Rationale: ARM ARM and llvm-mc require bit ∈ [0,31] for W and [0,63] for X. Documented bounds sampled at −1, 32 (W), 64 (X), i64::MIN/MAX. Body masks to 6 bits and does not declare OOR bits invalid.
- Doc contract: compare_branch.rs:261 "TBZ/TBNZ: b5 011011 op b40 imm14 Rt" — asserted fingerprint 49c8712c
- Seed: compare_branch.rs encode_cbz_neg_imm_unaligned_oor
- Formal: ∀ rt ∈ GPR_W ∪ GPR_X, bit ∉ valid range (W: [0,31], X: [0,63]), is_nz ∈ {false,true}. encode_tbz([Reg(rt), Imm(bit), Symbol("L")], is_nz) is Err
- Test file: src/backend/arm/assembler/encoder/encode_tbz_pbt.rs
- Status: failing
- Counterexample: encode_tbz([Reg("w0"), Imm(-1), Symbol("L")], false)
- Bug report: pbt-out/bug_reports/encode_tbz_bit_oor.md

```property
function: encoder.compare_branch.encode_tbz
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rt, bit, is_nz]
  domain: { rt: "GPR W/X", bit: "outside 0..31 W / 0..63 X", is_nz: bool }
  relation:
    op: holds
    expr: encode_tbz([Reg(rt), Imm(bit), Symbol("L")], is_nz).is_err()
generators:
  rt: { gen: int, min: 0, max: 31, type: u32 }
  bit: { gen: int, min: -8, max: 72, type: i64 }
  is_nz: { gen: bool }
expected_error: String
evidence: llvm-mc "immediate must be an integer in range [0, 31]" / "[0, 63]"
```

## encode_tbz_neg_invalid_name
- Tier: 3
- Rationale: Sweep: unparsable names (x32, foo, empty, r0) must Err via parse_reg_num. Documented get_reg failure path.
- Doc contract: compare_branch.rs:261 "TBZ/TBNZ: b5 011011 op b40 imm14 Rt" — asserted fingerprint 49c8712c
- Seed: encode_tst_neg_invalid_name
- Formal: ∀ name ∈ {x32,w32,foo,"",r0,x,x-1,x99}, is_nz ∈ {false,true}. encode_tbz([Reg(name), Imm(0), Symbol("L")], is_nz) is Err
- Test file: src/backend/arm/assembler/encoder/encode_tbz_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.compare_branch.encode_tbz
oracle: negative_error
predicate:
  quantifier: forall
  vars: [name, is_nz]
  domain: { name: "unparsable", is_nz: bool }
  relation:
    op: holds
    expr: encode_tbz([Reg(name), Imm(0), Symbol("L")], is_nz).is_err()
generators:
  name: { gen: string }
  is_nz: { gen: bool }
expected_error: String
evidence: parse_reg_num rejects x32/foo; llvm-mc invalid operand
```

## encode_tbz_meta_rt_isolation
- Tier: 4
- Rationale: Sweep metamorphic: incrementing Rt by 1 flips only bits[4:0]. Strengthens the ARM field invariant.
- Doc contract: compare_branch.rs:261 "TBZ/TBNZ: b5 011011 op b40 imm14 Rt" — asserted fingerprint 49c8712c
- Seed: encode_cbz_word_layout
- Formal: ∀ n ∈ 0..30, bit ∈ 0..63, is_nz ∈ {false,true}. encode_tbz(x{n}, bit, L).word XOR encode_tbz(x{n+1}, bit, L).word = n XOR (n+1)
- Test file: src/backend/arm/assembler/encoder/encode_tbz_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.compare_branch.encode_tbz
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [n, bit, is_nz]
  domain: { n: "0..30", bit: "0..63", is_nz: bool }
  relation:
    op: eq
    lhs: encode_tbz(xn).word XOR encode_tbz(x{n+1}).word
    rhs: n XOR (n+1)
generators:
  n: { gen: int, min: 0, max: 30, type: u32 }
  bit: { gen: int, min: 0, max: 63, type: i64 }
  is_nz: { gen: bool }
evidence: compare_branch.rs:261 Rt in bits[4:0]
```

## encode_tbz_neg_bad_label_kind
- Tier: 3
- Rationale: Sweep: get_symbol other-kind arm (Mem/Shift/Extend/RegArrangement/Expr/RegList) must Err.
- Doc contract: compare_branch.rs:261 "TBZ/TBNZ: b5 011011 op b40 imm14 Rt" — asserted fingerprint 49c8712c
- Seed: encode_cbz_neg_bad_label_kind
- Formal: ∀ kind ∈ {Mem,Shift,Extend,RegArrangement,Expr,RegList}, is_nz ∈ {false,true}. encode_tbz([Reg("x0"), Imm(0), kind], is_nz) is Err
- Test file: src/backend/arm/assembler/encoder/encode_tbz_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.compare_branch.encode_tbz
oracle: negative_error
predicate:
  quantifier: forall
  vars: [which, is_nz]
  domain: { which: "0..5", is_nz: bool }
  relation:
    op: holds
    expr: encode_tbz([Reg("x0"), Imm(0), bad_kind], is_nz).is_err()
generators:
  which: { gen: int, min: 0, max: 5, type: u32 }
  is_nz: { gen: bool }
expected_error: String
evidence: get_symbol other-kind arm encoder/mod.rs:1114
```
