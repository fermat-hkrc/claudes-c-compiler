# Properties: encode_adr

## encode_adr_diff_imm_llvm_mc
- Tier: 7
- Rationale: Strongest applicable oracle is differential against llvm-mc (independent AArch64 assembler). State machine rejected (pure function, no lifecycle). In-tree ADR decoder does not exist so algebraic round-trip of a sibling decoder is unavailable. Linker reloc::encode_adr is a different job (patches imm into an already-encoded word) and fails the same-job sibling gate. Doc evidence: assembler README "accepts the same textual assembly that GCC's gas would consume"; encoder/mod.rs ADR dispatch; ARM ARM ADR encoding.
- Seed: (none) — load_store.rs has no existing tests for adr
- Formal: ∀ rd ∈ {x0..x30, xzr, lr}, ∀ imm ∈ [-2^20, 2^20-1]. encode_adr([Reg(rd), Imm(imm)]) = Word(w) ∧ w = llvm-mc("adr rd, #imm")
- Test file: src/backend/arm/assembler/encoder/load_store.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.load_store.encode_adr
oracle: differential
predicate:
  quantifier: forall
  vars: [rd, imm]
  domain: { rd: x_regs_and_aliases, imm: i64_21bit_signed }
  relation:
    op: eq
    lhs: encode_adr([Reg(rd), Imm(imm)]) as Word
    rhs: llvm_mc("adr " + rd + ", #" + imm)
generators:
  rd: { gen: oneof, options: [x0..x30, xzr, lr] }
  imm: { gen: int, min: -1048576, max: 1048575, type: i64 }
evidence: src/backend/arm/assembler/README.md:5-14 gas-compatible AArch64; encoder/mod.rs:373 adr dispatch; ARM ARM ADR PC-relative 21-bit
```

## encode_adr_roundtrip_arm_fields
- Tier: 4a
- Rationale: Algebraic round-trip of ARM ARM field layout (unpack is specified by the architecture, not copied from the SUT packer). Stronger differential is the sibling property above. State machine rejected. Doc evidence: load_store.rs:699-702 encoding comment `ADR: 0 immlo[1:0] 10000 immhi[18:0] Rd`; ARM ARM ADR.
- Seed: (none)
- Formal: ∀ rd ∈ 0..=31, ∀ imm ∈ [-2^20, 2^20-1]. let w = encode_adr([Reg(gpr64(rd)), Imm(imm)]) as Word. Then w[31]=0 ∧ w[28:24]=0b10000 ∧ w[4:0]=rd ∧ SignExtend21(w[30:29] | (w[23:5]<<2)) = imm
- Test file: src/backend/arm/assembler/encoder/load_store.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.load_store.encode_adr
oracle: algebraic.round_trip
round_trip: {forward: encode_adr, backward: arm_adr_field_unpack, var: (rd, imm)}
predicate:
  quantifier: forall
  vars: [rd, imm]
  domain: { rd: u32_0_31, imm: i64_21bit_signed }
  relation:
    op: holds
    expr: decode_adr_fields(encode_adr([Reg(gpr64(rd)), Imm(imm)])) == (rd, imm)
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  imm: { gen: int, min: -1048576, max: 1048575, type: i64 }
evidence: load_store.rs:699-702 ADR encoding comment; ARM ARM ADR op=0 immlo 10000 immhi Rd
```

## encode_adr_metamorphic_rd_imm_independent
- Tier: 4c
- Rationale: Metamorphic: the packed immediate field is independent of Rd and the Rd field is independent of imm (ARM ADR field layout). Stronger differential covers value equality with llvm-mc; this catches Rd/imm cross-talk. State machine rejected.
- Seed: (none)
- Formal: ∀ rd, rd' ∈ 0..=31, ∀ imm, imm' ∈ [-2^20, 2^20-1]. encode(rd,imm) ⊕ encode(rd,imm') has bits[4:0]=0 ∧ encode(rd,imm) ⊕ encode(rd',imm) has bits[31:5]=0
- Test file: src/backend/arm/assembler/encoder/load_store.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.load_store.encode_adr
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [rd, rd2, imm, imm2]
  domain: { rd: u32_0_31, rd2: u32_0_31, imm: i64_21bit_signed, imm2: i64_21bit_signed }
  relation:
    op: holds
    expr: ((encode(rd,imm) XOR encode(rd,imm2)) & 0x1F) == 0 && ((encode(rd,imm) XOR encode(rd2,imm)) & !0x1F) == 0
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rd2: { gen: int, min: 0, max: 31, type: u32 }
  imm: { gen: int, min: -1048576, max: 1048575, type: i64 }
  imm2: { gen: int, min: -1048576, max: 1048575, type: i64 }
evidence: load_store.rs:699-702; ARM ARM ADR Rd at [4:0], imm at [30:29]+[23:5]
```

## encode_adr_symbol_reloc
- Tier: 4d
- Rationale: Algebraic invariant on the reloc form. README and RelocType document AdrPrelLo21 (ELF 274) for `adr` with a symbol; encoding comment says op=0 and reloc word leaves imm fields 0. Differential against llvm-mc is weaker here because llvm-mc emits a fixup rather than a concrete word. State machine rejected.
- Seed: (none)
- Formal: ∀ rd ∈ 0..=30 ∪ {xzr}, ∀ sym ∈ ident, ∀ addend ∈ i64. encode_adr([Reg(Xd), SymbolOffset(sym, addend)]) = WordWithReloc { word: 0x10000000|rd, reloc_type: AdrPrelLo21, symbol: sym, addend } ∧ encode_adr([Reg(Xd), Symbol(sym)]) and Label(sym) agree with addend=0
- Test file: src/backend/arm/assembler/encoder/load_store.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.load_store.encode_adr
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [rd, sym, addend]
  domain: { rd: u32_0_31, sym: ident, addend: i64 }
  relation:
    op: holds
    expr: encode_adr_symbol_offset(rd, sym, addend) is WordWithReloc AdrPrelLo21
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  sym: { gen: string, minLen: 1, maxLen: 16 }
  addend: { gen: int, min: -4096, max: 4096, type: i64 }
evidence: assembler/README.md:255 AdrPrelLo21 for adr; encoder/mod.rs:79-80,103; load_store.rs:709-716
```

## encode_adr_neg_w_reg
- Tier: 4e
- Rationale: Negative/error contract. ARM ADR takes Xd only; llvm-mc rejects `adr w0, #imm` and `adr wsp, #0`. Assembler claims gas-compatible text. Stronger differential on the valid Xd domain is the sibling property; this covers the documented-invalid W/WSP domain.
- Seed: (none)
- Formal: ∀ wd ∈ {w0..w30, wzr, wsp}, ∀ imm ∈ i64. encode_adr([Reg(wd), Imm(imm)]) = Err(_)
- Test file: src/backend/arm/assembler/encoder/load_store.rs
- Status: failing
- Counterexample: wd=0, imm=-1048576, use_wsp=false (adr w0, #-1048576)
- Bug report: pbt-out/bug_reports/encode_adr_w_reg.md

```property
function: encoder.load_store.encode_adr
oracle: negative_error
predicate:
  quantifier: forall
  vars: [wd, imm]
  domain: { wd: w_regs_and_aliases, imm: i64 }
  relation:
    op: throws
    expr: encode_adr([Reg(wd), Imm(imm)])
expected_error: String
generators:
  wd: { gen: oneof, options: [w0..w30, wzr, wsp] }
  imm: { gen: int, min: -1048576, max: 1048575, type: i64 }
evidence: ARM ARM ADR <Xd>; llvm-mc rejects adr w0/# wsp; assembler README.md:5-14 gas-compatible
```

## encode_adr_neg_sp
- Tier: 4e
- Rationale: Negative/error. ARM ADR Rd is Xd; register 31 is XZR not SP. llvm-mc rejects `adr sp, #0`. parse_reg_num maps sp to 31, same as xzr.
- Seed: (none)
- Formal: ∀ imm ∈ i64. encode_adr([Reg("sp"), Imm(imm)]) = Err(_)
- Test file: src/backend/arm/assembler/encoder/load_store.rs
- Status: failing
- Counterexample: imm=-1048576 (adr sp, #-1048576)
- Bug report: pbt-out/bug_reports/encode_adr_sp_as_zr.md

```property
function: encoder.load_store.encode_adr
oracle: negative_error
predicate:
  quantifier: forall
  vars: [imm]
  domain: { imm: i64_21bit_signed }
  relation:
    op: throws
    expr: encode_adr([Reg("sp"), Imm(imm)])
expected_error: String
generators:
  imm: { gen: int, min: -1048576, max: 1048575, type: i64 }
evidence: ARM ARM ADR Xd, X31=XZR; llvm-mc rejects adr sp; assembler README.md:5-14
```

## encode_adr_neg_imm_range
- Tier: 4e
- Rationale: Negative/error. ARM ADR immediate is 21-bit signed [-2^20, 2^20-1]; llvm-mc rejects #1048576 and #-1048577. The SUT itself documents the missing check: `TODO: validate 21-bit signed immediate range` at load_store.rs:697. Documented bounds sampled at bound and bound±1.
- Seed: (none)
- Formal: ∀ rd ∈ {x0..x30, xzr}, ∀ imm ∈ i64 \ [-2^20, 2^20-1]. encode_adr([Reg(rd), Imm(imm)]) = Err(_)
- Test file: src/backend/arm/assembler/encoder/load_store.rs
- Status: failing
- Counterexample: rd=0, imm=-1048577 (adr x0, #-1048577)
- Bug report: pbt-out/bug_reports/encode_adr_imm_range.md

```property
function: encoder.load_store.encode_adr
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, imm]
  domain: { rd: x_regs, imm: i64_outside_21bit_signed }
  relation:
    op: throws
    expr: encode_adr([Reg(rd), Imm(imm)])
expected_error: String
generators:
  rd: { gen: int, min: 0, max: 30, type: u32 }
  imm: { gen: oneof, options: [int(-2^63..-1048577), int(1048576..2^63-1)] }
evidence: load_store.rs:697 TODO validate 21-bit signed immediate range; ARM ARM ADR ±1MB; llvm-mc rejects #1048576
```

## encode_adr_neg_bad_operands
- Tier: 4e
- Rationale: Negative/error for arity and operand-kind contract. get_reg requires a register at 0; get_symbol (after Imm miss) requires a symbol-like operand at 1. FP and modifier shapes were split into dedicated properties after the first batch (they are distinct bugs).
- Seed: (none)
- Formal: ∀ ops ∈ {[], [Imm(_)], [Reg(Xd)], [Reg(Xd), Mem{_}], [Reg("x32"), Imm(_)]}. encode_adr(ops) = Err(_)
- Test file: src/backend/arm/assembler/encoder/load_store.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.load_store.encode_adr
oracle: negative_error
predicate:
  quantifier: forall
  vars: [ops]
  domain: { ops: adr_invalid_arity_and_kinds }
  relation:
    op: throws
    expr: encode_adr(ops)
expected_error: String
generators:
  ops: { gen: oneof, options: [empty, imm_only, rd_only, rd_plus_mem, x32_plus_imm] }
evidence: load_store.rs:694 get_reg; :707 get_symbol
```

## encode_adr_neg_fp_reg
- Tier: 4e
- Rationale: Negative/error. ARM ADR takes Xd only; llvm-mc rejects `adr d0, #0`. parse_reg_num accepts d/s/q/v/h/b prefixes. Split from encode_adr_neg_bad_operands after the first batch so the FP path is a dedicated contract.
- Seed: (none)
- Formal: ∀ prefix ∈ {d,s,q,v,h,b}, ∀ n ∈ 0..=31, ∀ imm ∈ [-2^20, 2^20-1]. encode_adr([Reg(prefix+n), Imm(imm)]) = Err(_)
- Test file: src/backend/arm/assembler/encoder/load_store.rs
- Status: failing
- Counterexample: prefix="d", n=0, imm=-1048576 (adr d0, #-1048576)
- Bug report: pbt-out/bug_reports/encode_adr_fp_reg.md

```property
function: encoder.load_store.encode_adr
oracle: negative_error
predicate:
  quantifier: forall
  vars: [prefix, n, imm]
  domain: { prefix: fp_simd_prefixes, n: u32_0_31, imm: i64_21bit_signed }
  relation:
    op: throws
    expr: encode_adr([Reg(prefix+n), Imm(imm)])
expected_error: String
generators:
  prefix: { gen: oneof, options: [d, s, q, v, h, b] }
  n: { gen: int, min: 0, max: 31, type: u32 }
  imm: { gen: int, min: -1048576, max: 1048575, type: i64 }
evidence: ARM ARM ADR <Xd>; llvm-mc rejects adr d0, #0; assembler README.md:5-14
```

## encode_adr_neg_modifier
- Tier: 4e
- Rationale: Negative/error. llvm-mc rejects `adr x0, :lo12:foo` ("unexpected adr label"). get_symbol accepts any Modifier and encode_adr emits AdrPrelLo21, which is the wrong reloc for :lo12:/:got:. Split from encode_adr_neg_bad_operands after the first-batch shrink to kind=5.
- Seed: (none)
- Formal: ∀ rd ∈ 0..=31, ∀ kind ∈ {lo12, got, got_lo12}, ∀ sym ∈ ident. encode_adr([Reg(Xd), Modifier{kind, symbol:sym}]) = Err(_)
- Test file: src/backend/arm/assembler/encoder/load_store.rs
- Status: failing
- Counterexample: rd=0, mod_kind="lo12", suffix=0 (adr x0, :lo12:labl0)
- Bug report: pbt-out/bug_reports/encode_adr_modifier.md

```property
function: encoder.load_store.encode_adr
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, mod_kind, sym]
  domain: { rd: u32_0_31, mod_kind: lo12_got_got_lo12, sym: ident }
  relation:
    op: throws
    expr: encode_adr_modifier(rd, mod_kind, sym)
expected_error: String
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  mod_kind: { gen: oneof, options: [lo12, got, got_lo12] }
  sym: { gen: string, minLen: 1, maxLen: 16 }
evidence: llvm-mc rejects adr x0, :lo12:foo; assembler README.md:255 AdrPrelLo21 is for bare adr, not :lo12:
```

## encode_adr_symbol_misclassified
- Tier: 4d
- Rationale: Coverage-sweep algebraic invariant for get_symbol parser-misclassification arms (Reg/Cond/Barrier names that collide with symbols). Doc evidence: encoder/mod.rs:975-988 get_symbol treats those as symbols. Not a bug path — documented workaround.
- Seed: (none)
- Formal: ∀ rd ∈ 0..=31, ∀ name ∈ {s1,v0,d1,cc,lt,le,st,ld}, ∀ which ∈ {Reg,Cond,Barrier}. encode_adr([Reg(Xd), which(name)]) = WordWithReloc { word: 0x10000000|rd, AdrPrelLo21, symbol: name, addend: 0 }
- Test file: src/backend/arm/assembler/encoder/load_store.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.load_store.encode_adr
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [rd, which, name]
  domain: { rd: u32_0_31, which: reg_cond_barrier, name: colliding_symbol_names }
  relation:
    op: holds
    expr: encode_adr_misclassified(rd, which, name) is WordWithReloc AdrPrelLo21
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  which: { gen: int, min: 0, max: 2, type: u32 }
  name: { gen: oneof, options: [s1, v0, d1, cc, lt, le, st, ld] }
evidence: encoder/mod.rs:975-988 get_symbol treats Reg/Cond/Barrier as symbols
```

## encode_adr_neg_modifier_offset
- Tier: 4e
- Rationale: Coverage-sweep negative/error for ModifierOffset, the remaining get_symbol modifier arm. Same law as encode_adr_neg_modifier.
- Seed: (none)
- Formal: ∀ rd ∈ 0..=31, ∀ kind ∈ {lo12, got, got_lo12}, ∀ offset ∈ i64. encode_adr([Reg(Xd), ModifierOffset{kind, symbol:foo, offset}]) = Err(_)
- Test file: src/backend/arm/assembler/encoder/load_store.rs
- Status: failing
- Counterexample: rd=0, mod_kind="lo12", offset=0
- Bug report: pbt-out/bug_reports/encode_adr_modifier.md

```property
function: encoder.load_store.encode_adr
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, mod_kind, offset]
  domain: { rd: u32_0_31, mod_kind: lo12_got_got_lo12, offset: i64 }
  relation:
    op: throws
    expr: encode_adr_modifier_offset(rd, mod_kind, offset)
expected_error: String
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  mod_kind: { gen: oneof, options: [lo12, got, got_lo12] }
  offset: { gen: int, min: -4096, max: 4096, type: i64 }
evidence: llvm-mc rejects adr x0, :lo12:foo; get_symbol ModifierOffset arm encoder/mod.rs:981
```
