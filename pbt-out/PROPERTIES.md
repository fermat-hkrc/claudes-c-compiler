# Properties: encode_prfm

## encode_prfm_diff_imm_llvm_mc
- Tier: 2
- Rationale: Strongest applicable oracle is differential vs llvm-mc, which implements the same GNU-style PRFM (immediate) contract the assembler README claims. State machine rejected (pure function). Round-trip rejected (no in-tree PRFM decoder). encode_ldr_str rejected as sibling (different job: GPR/SIMD dest, not prfop Rt).
- Seed: load_store.rs encode_ldtr_sized_pbt llvm-mc differential
- Formal: ∀ prfop ∈ named∪{0..31}, ∀ rn ∈ {0..31} with 31=SP, ∀ pimm ∈ {0,8,...,32760}. encode_prfm([prfop, Mem(Xn|SP, pimm)]) = llvm-mc("prfm prfop, [Xn|SP{, #pimm}]")
- Test file: src/backend/arm/assembler/encoder/load_store.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.load_store.encode_prfm
oracle: differential
predicate:
  quantifier: forall
  vars: [prfop, rn, pimm]
  relation:
    op: eq
    lhs: encode_prfm([prfop, Mem(rn, pimm)])
    rhs: llvm_mc(prfm_imm_asm(prfop, rn, pimm))
generators:
  prfop: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  pimm: { gen: int, min: 0, max: 32760, type: i64 }
evidence: README.md:11 GNU gas compatibility; ARM ARM PRFM (immediate) 1111 1001 10 imm12 Rn Rt; encoder/mod.rs:917
```

## encode_prfm_diff_regoff_llvm_mc
- Tier: 2
- Rationale: Same differential contract for PRFM (register). ARM ARM and the SUT comment at load_store.rs:764 both specify 11 111 0 00 10 1 Rm option S 10 Rn Rt. llvm-mc is an independent assembler of that encoding.
- Seed: load_store.rs encode_ldrsw_pbt register-offset differential
- Formal: ∀ prfop ∈ named, ∀ rn ∈ {0..30}∪SP, ∀ rm ∈ {0..31} with 31=XZR, ∀ (index_width, extend, amount) ∈ valid PRFM extend set. encode_prfm([prfop, MemRegOffset(Xn|SP, Xm|Wm, extend, amount)]) = llvm-mc of the same assembly.
- Test file: src/backend/arm/assembler/encoder/load_store.rs
- Status: failing
- Counterexample: prfm pldl1keep, [x0, x0] — SUT 0xF9206800 vs llvm-mc 0xF8A06800 (idx=0, rn=0, rm=0, ext_kind=0, use_shift=false)
- Bug report: pbt-out/bug_reports/encode_prfm_regoff_encoding.md

```property
function: encoder.load_store.encode_prfm
oracle: differential
predicate:
  quantifier: forall
  vars: [prfop, rn, rm, extend, amount]
  relation:
    op: eq
    lhs: encode_prfm([prfop, MemRegOffset(rn, rm, extend, amount)])
    rhs: llvm_mc(prfm_regoff_asm(prfop, rn, rm, extend, amount))
generators:
  prfop: { gen: int, min: 0, max: 17, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  extend: { gen: int, min: 0, max: 3, type: u32 }
  amount: { gen: int, min: 0, max: 1, type: u32 }
evidence: ARM ARM PRFM (register) 11 111 0 00 10 1 Rm option S 10 Rn Rt; load_store.rs:764 purpose comment; llvm-mc
```

## encode_prfm_arm_fields
- Tier: 4
- Rationale: Algebraic invariant from ARM ARM PRFM (immediate) bitfields. Stronger differential is also written; this unpacks the word independently of llvm-mc so a reference-connection failure cannot hide a packing bug. Not a copy of the SUT packer.
- Seed: load_store.rs encode_ldtr_sized_arm_fields
- Formal: ∀ prfop ∈ 0..31, ∀ rn ∈ 0..31, ∀ imm12 ∈ 0..4095. let w = encode_prfm([#prfop, Mem(Xn|SP, imm12*8)]). unpack(w) = (size=0b11, bits[29:24]=0b111001, opc=0b10, imm12, rn, rt=prfop)
- Test file: src/backend/arm/assembler/encoder/load_store.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.load_store.encode_prfm
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [prfop, rn, imm12]
  relation:
    op: holds
    expr: unpack_prfm_imm(encode_prfm([Imm(prfop), Mem(rn, imm12*8)])) == (0b11, 0b111001, 0b10, imm12, rn, prfop)
generators:
  prfop: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  imm12: { gen: int, min: 0, max: 4095, type: u32 }
evidence: ARM ARM PRFM (immediate) encoding 1111 1001 10 imm12 Rn Rt; load_store.rs:723-725
```

## encode_prfm_metamorphic_fields
- Tier: 4
- Rationale: Metamorphic independence of Rt (prfop), Rn, and imm12. Stronger round-trip rejected (no decoder). Documented field positions imply XOR/add relations.
- Seed: load_store.rs encode_ldtr_sized_metamorphic_fields
- Formal: ∀ prfop ∈ 0..30, ∀ rn ∈ 0..30, ∀ imm12 ∈ 0..4094. let w = encode(prfop, rn, imm12*8). encode(prfop+1) differs only in Rt (=prfop+1); encode(rn+1) differs only in Rn (=rn+1); encode(imm12+1) differs only in imm12 (=imm12+1)
- Test file: src/backend/arm/assembler/encoder/load_store.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.load_store.encode_prfm
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [prfop, rn, imm12]
  relation:
    op: holds
    expr: encode(p+1,rn,i) ^ encode(p,rn,i) == 1 && encode(p,rn+1,i) ^ encode(p,rn,i) == 32 && encode(p,rn,i+1) ^ encode(p,rn,i) == (1<<10)
generators:
  prfop: { gen: int, min: 0, max: 30, type: u32 }
  rn: { gen: int, min: 0, max: 30, type: u32 }
  imm12: { gen: int, min: 0, max: 4094, type: u32 }
evidence: ARM ARM PRFM (immediate) Rt at [4:0], Rn at [9:5], imm12 at [21:10]
```

## encode_prfm_neg_arity
- Tier: 4e
- Rationale: Documented 2-operand syntax (load_store.rs:723, error string "prfm requires 2 operands"). llvm-mc rejects missing operands.
- Seed: load_store.rs encode_ldtr_sized_neg_arity
- Formal: ∀ ops with |ops| < 2. encode_prfm(ops) is Err
- Test file: src/backend/arm/assembler/encoder/load_store.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.load_store.encode_prfm
oracle: negative_error
predicate:
  quantifier: forall
  vars: [ops]
  relation:
    op: throws
    expr: encode_prfm(ops)
expected_error: String
generators:
  ops: { gen: list, elem: { gen: int, min: 0, max: 3, type: u32 }, maxLen: 1 }
evidence: load_store.rs:723 Format PRFM prfop, [Xn|SP{, #pimm}]; load_store.rs:728 prfm requires 2 operands
```

## encode_prfm_neg_extra_operand
- Tier: 4e
- Rationale: Two-operand GNU syntax; llvm-mc rejects a third operand. Extra operands must Err, not be ignored.
- Seed: load_store.rs encode_ldtr_sized_neg_extra_operand
- Formal: ∀ valid 2-operand PRFM immediate ops, ∀ extra. encode_prfm(ops ++ [extra]) is Err
- Test file: src/backend/arm/assembler/encoder/load_store.rs
- Status: failing
- Counterexample: prfop=#0, rn=x0, pimm=0, extra=Reg("x2")
- Bug report: pbt-out/bug_reports/encode_prfm_extra_operand.md

```property
function: encoder.load_store.encode_prfm
oracle: negative_error
predicate:
  quantifier: forall
  vars: [prfop, rn, pimm, extra]
  relation:
    op: throws
    expr: encode_prfm([prfop, Mem(rn, pimm), extra])
expected_error: String
generators:
  prfop: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  pimm: { gen: int, min: 0, max: 32760, type: i64 }
  extra: { gen: int, min: 0, max: 3, type: u32 }
evidence: README.md:11 gas compatibility; llvm-mc rejects extra operands on prfm
```

## encode_prfm_neg_invalid_base
- Tier: 4e
- Rationale: ARM ARM PRFM base is Xn|SP only. llvm-mc rejects W-base, XZR/x31, WSP, and SIMD/FP names.
- Seed: load_store.rs encode_ldtr_sized_neg_invalid_regs
- Formal: ∀ prfop, ∀ invalid base ∈ {Wn, wzr, wsp, xzr, x31, [bhsdqv]n}. encode_prfm([prfop, Mem(base, 0)]) is Err
- Test file: src/backend/arm/assembler/encoder/load_store.rs
- Status: failing
- Counterexample: prfop=#0, kind=0, n=0 — base w0 (also xzr/x31/wsp/d0)
- Bug report: pbt-out/bug_reports/encode_prfm_w_base.md

```property
function: encoder.load_store.encode_prfm
oracle: negative_error
predicate:
  quantifier: forall
  vars: [prfop, base]
  relation:
    op: throws
    expr: encode_prfm([prfop, Mem(base, 0)])
expected_error: String
generators:
  prfop: { gen: int, min: 0, max: 31, type: u32 }
  base: { gen: int, min: 0, max: 5, type: u32 }
evidence: ARM ARM PRFM Rn is Xn|SP; llvm-mc rejects [w0], [xzr], [x31]
```

## encode_prfm_neg_offset_and_form
- Tier: 4e
- Rationale: ARM unsigned PRFM offset is multiple of 8 in [0, 32760]; llvm-mc error "index must be a multiple of 8 in range [0, 32760]". Pre/post-index are not PRFM forms (PRFUM is a different mnemonic). Unknown prfop and #imm5 outside 0..31 must Err.
- Seed: load_store.rs encode_ldtr_sized_neg_offset_and_form
- Formal: ∀ (bad_offset ∉ {0,8,...,32760} on Mem) ∨ (addr ∈ {MemPreIndex, MemPostIndex, Imm, Label, Cond}) ∨ (unknown prfop name) ∨ (imm5 ∉ 0..31). encode_prfm is Err
- Test file: src/backend/arm/assembler/encoder/load_store.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.load_store.encode_prfm
oracle: negative_error
predicate:
  quantifier: forall
  vars: [kind]
  relation:
    op: throws
    expr: encode_prfm(ops(kind))
expected_error: String
generators:
  kind: { gen: int, min: 0, max: 6, type: u32 }
evidence: ARM ARM PRFM pimm multiple of 8 in [0,32760]; llvm-mc same; load_store.rs:736-737 imm5 range; encode_prfop unknown name Err
```

## encode_prfm_neg_w_index
- Tier: 4e
- Rationale: Sweep. ARM ARM W-index for PRFM (register) requires UXTW or SXTW; llvm-mc rejects bare `[Xn, Wm]`.
- Seed: (none) — coverage sweep of MemRegOffset W-index default
- Formal: ∀ prfop ∈ named, ∀ rn,rm ∈ 0..30. encode_prfm([prfop, MemRegOffset(Xn, Wm, extend=None)]) is Err
- Test file: src/backend/arm/assembler/encoder/load_store.rs
- Status: failing
- Counterexample: prfm pldl1keep, [x0, w0] (idx=0, rn=0, rm=0)
- Bug report: pbt-out/bug_reports/encode_prfm_w_index.md

```property
function: encoder.load_store.encode_prfm
oracle: negative_error
predicate:
  quantifier: forall
  vars: [prfop, rn, rm]
  relation:
    op: throws
    expr: encode_prfm([prfop, MemRegOffset(xn, wm, None, None)])
expected_error: String
generators:
  prfop: { gen: int, min: 0, max: 17, type: u32 }
  rn: { gen: int, min: 0, max: 30, type: u32 }
  rm: { gen: int, min: 0, max: 30, type: u32 }
evidence: ARM ARM PRFM (register) option UXTW/SXTW for Wm; llvm-mc expected uxtw or sxtw
```

## encode_prfm_neg_bad_shift
- Tier: 4e
- Rationale: Sweep. llvm-mc requires PRFM (register) shift amount in {0, 3}; ARM S bit encodes amount 0 or 3 only.
- Seed: (none) — coverage sweep of shift_amount > 0 path
- Formal: ∀ amount ∉ {0,3}. encode_prfm([prfop, MemRegOffset(Xn, Xm, lsl, amount)]) is Err
- Test file: src/backend/arm/assembler/encoder/load_store.rs
- Status: failing
- Counterexample: prfm pldl1keep, [x0, x0, lsl #1] (idx=0, rn=0, rm=0, amount=1)
- Bug report: pbt-out/bug_reports/encode_prfm_bad_shift.md

```property
function: encoder.load_store.encode_prfm
oracle: negative_error
predicate:
  quantifier: forall
  vars: [prfop, rn, rm, amount]
  relation:
    op: throws
    expr: encode_prfm([prfop, MemRegOffset(xn, xm, lsl, amount)])
expected_error: String
generators:
  amount: { gen: int, min: 1, max: 7, type: u8 }
evidence: ARM ARM PRFM (register) S amount 0 or 3; llvm-mc expected lsl or sxtx with #0 or #3
```

## encode_prfm_neg_bad_prfop_and_name
- Tier: 4e
- Rationale: Sweep. First operand must be Symbol(prfop) or Imm(0..31); base/index must parse as registers; PRFM literal is documented as not yet supported (Err).
- Seed: (none) — coverage sweep of error arms at load_store.rs:740, 746, 761, 766
- Formal: ∀ kind ∈ {Reg-as-prfop, base foo, base x32, index foo, addr Symbol}. encode_prfm is Err
- Test file: src/backend/arm/assembler/encoder/load_store.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.load_store.encode_prfm
oracle: negative_error
predicate:
  quantifier: forall
  vars: [kind]
  relation:
    op: throws
    expr: encode_prfm(ops(kind))
expected_error: String
generators:
  kind: { gen: int, min: 0, max: 4, type: u32 }
evidence: load_store.rs:740 expected prefetch operation name; 746 invalid base; 761 not yet supported; 766 invalid index
```
