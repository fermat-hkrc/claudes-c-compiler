# Properties: encode_umull

## encode_umull_diff_valid_gpr
- Tier: 2
- Rationale: Strongest evidenced oracle is differential vs llvm-mc. README claims GNU-gas-compatible textual assembly; encoder/mod.rs claims 32-bit AArch64 words. State machine rejected (pure function). Round-trip rejected (no in-tree UMULL decoder). encode_smull rejected as sibling (U bit different job). encode_umaddl rejected as independent differential (shared get_reg / same TU).
- Seed: data_processing.rs encode_smull_pbt::encode_smull_diff_valid_gpr
- Formal: ∀ rd,rn,rm ∈ {0..31}, dest ∈ {x{rd}, xzr if rd=31, lr if rd=30}. encode_umull([Reg(dest), Reg(w{rn}|wzr), Reg(w{rm}|wzr)]) = Word(llvm-mc("umull dest, Wn, Wm"))
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.data_processing.encode_umull
oracle: differential
predicate:
  quantifier: forall
  vars: [rd, rn, rm]
  domain: { rd: 0..31, rn: 0..31, rm: 0..31 }
  relation:
    op: eq
    lhs: encode_umull([Reg(xreg(rd)), Reg(wreg(rn)), Reg(wreg(rm))])
    rhs: llvm_mc("umull Xd, Wn, Wm")
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
evidence: src/backend/arm/assembler/README.md gas-compatible assembly; encoder/mod.rs 32-bit words; ARM ARM UMULL alias of UMADDL Ra=XZR
```

## encode_umull_alias_umaddl_xzr
- Tier: 4c
- Rationale: ARM ARM and the encode_umull docstring state UMULL Xd, Wn, Wm is the alias of UMADDL Xd, Wn, Wm, XZR. Not an independent differential (shared TU). Metamorphic relation plus llvm-mc agreement.
- Seed: data_processing.rs encode_umaddl_pbt::encode_umaddl_alias_umull_xzr
- Formal: ∀ rd,rn,rm ∈ {0..31}. encode_umull([Xd,Wn,Wm]) = encode_umaddl([Xd,Wn,Wm,XZR]) = llvm-mc("umull Xd, Wn, Wm") = llvm-mc("umaddl Xd, Wn, Wm, xzr")
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.data_processing.encode_umull
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [rd, rn, rm]
  domain: { rd: 0..31, rn: 0..31, rm: 0..31 }
  relation:
    op: eq
    lhs: encode_umull([Xd, Wn, Wm])
    rhs: encode_umaddl([Xd, Wn, Wm, XZR])
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
evidence: data_processing.rs:641-646 docstring; ARM ARM UMULL alias of UMADDL Ra=XZR
```

## encode_umull_xor_smull_u_bit
- Tier: 4c
- Rationale: ARM ARM Data-processing (3 source) U bit (bit 23) is the sole encoding difference between UMULL (U=1) and SMULL (U=0) at equal registers. Metamorphic, not same-job differential.
- Seed: data_processing.rs encode_smull_pbt::encode_smull_xor_umull_u_bit
- Formal: ∀ rd,rn,rm ∈ {0..31}. encode_umull(Xd,Wn,Wm) XOR encode_smull(Xd,Wn,Wm) = 1<<23
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.data_processing.encode_umull
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [rd, rn, rm]
  domain: { rd: 0..31, rn: 0..31, rm: 0..31 }
  relation:
    op: eq
    lhs: encode_umull(ops) XOR encode_smull(ops)
    rhs: 1u32 << 23
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
evidence: ARM ARM Data-processing (3 source) U bit at bit 23; SMULL U=0 UMULL U=1
```

## encode_umull_arm_fields
- Tier: 4d
- Rationale: ARM ARM field layout for UMULL: sf=1 op54=00 11011 U=1 01 Rm o0=0 Ra=11111 Rn Rd. Weaker than differential; still pins each field independently of llvm-mc.
- Seed: data_processing.rs encode_smull_pbt::encode_smull_arm_fields
- Formal: ∀ rd,rn,rm ∈ {0..31}. let w = encode_umull(Xd,Wn,Wm). w = 0x9BA07C00 | (rm<<16) | (rn<<5) | rd ∧ w[31]=1 ∧ w[30:21]=00_11011_101 ∧ w[20:16]=rm ∧ w[15]=0 ∧ w[14:10]=11111 ∧ w[9:5]=rn ∧ w[4:0]=rd
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.data_processing.encode_umull
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [rd, rn, rm]
  domain: { rd: 0..31, rn: 0..31, rm: 0..31 }
  relation:
    op: eq
    lhs: encode_umull([Xd, Wn, Wm])
    rhs: 0x9BA07C00 | (rm << 16) | (rn << 5) | rd
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
evidence: ARM ARM Data-processing (3 source) UMADDL/UMULL; data_processing.rs:646 comment
```

## encode_umull_diff_alt_spellings
- Tier: 2
- Rationale: get_reg accepts x31/w31 (not only xzr/wzr), uppercase, and LR. llvm-mc accepts the same spellings. Documented bounds of register names must be sampled exactly.
- Seed: data_processing.rs encode_smull_pbt::encode_smull_diff_alt_spellings
- Formal: ∀ rd,rn,rm ∈ {0..31}, dest_spell, src_spell. encode_umull of accepted alternate spellings equals llvm-mc of the same text.
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.data_processing.encode_umull
oracle: differential
predicate:
  quantifier: forall
  vars: [rd, rn, rm, dest_spell, src_spell]
  domain: { rd: 0..31, rn: 0..31, rm: 0..31, dest_spell: 0..4, src_spell: 0..2 }
  relation:
    op: eq
    lhs: encode_umull(ops)
    rhs: llvm_mc(asm)
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  dest_spell: { gen: int, min: 0, max: 4, type: u32 }
  src_spell: { gen: int, min: 0, max: 2, type: u32 }
evidence: encoder/mod.rs:131-148 parse_reg_num (sp/xzr/lr/xN); llvm-mc accepts x31, XZR, LR, uppercase
```

## encode_umull_neg_arity
- Tier: 4e
- Rationale: ARM ARM / gas syntax is UMULL Xd, Wn, Wm (3 operands). llvm-mc rejects fewer. get_reg on missing slots returns Err; property pins the documented rejection.
- Seed: data_processing.rs encode_smull_pbt::encode_smull_neg_arity
- Formal: ∀ ops with |ops| < 3. encode_umull(ops) = Err
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.data_processing.encode_umull
oracle: negative_error
predicate:
  quantifier: forall
  vars: [ops]
  domain: { ops: length 0..2 }
  relation:
    op: throws
    expr: encode_umull(ops)
generators:
  len: { gen: int, min: 0, max: 2, type: usize }
expected_error: String
evidence: ARM ARM UMULL syntax Xd, Wn, Wm; llvm-mc too few operands
```

## encode_umull_neg_extra_operand
- Tier: 4e
- Rationale: llvm-mc rejects a 4th operand (`umull x0, w1, w2, x3` error: invalid operand). README gas-compatibility requires the same rejection. encode_umull currently only reads slots 0..2.
- Seed: data_processing.rs encode_smull_pbt::encode_smull_neg_extra_operand
- Formal: ∀ rd,rn,rm ∈ {0..31}, extra. encode_umull([Xd,Wn,Wm,extra]) = Err
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: failing
- Counterexample: [Reg("x0"), Reg("w0"), Reg("w0"), Reg("x0")]
- Bug report: pbt-out/bug_reports/encode_umull_extra_operand.md

```property
function: encoder.data_processing.encode_umull
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, rm, extra]
  domain: { rd: 0..31, rn: 0..31, rm: 0..31, extra: Operand }
  relation:
    op: throws
    expr: encode_umull([Xd, Wn, Wm, extra])
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  extra: { gen: oneof, variants: [Reg, Imm, Shift, RegArrangement] }
expected_error: String
evidence: llvm-mc rejects 4th operand; README gas-compatible assembly
```

## encode_umull_neg_wrong_width
- Tier: 4e
- Rationale: ARM ARM syntax is UMULL Xd, Wn, Wm. llvm-mc rejects W dest or X sources. Documented width bound must be enforced, not ignored via get_reg discarding is_64.
- Seed: data_processing.rs encode_smull_pbt::encode_smull_neg_wrong_width
- Formal: ∀ rd,rn,rm ∈ {0..30}, (rd64,rn64,rm64) ≠ (true,false,false). encode_umull([gpr(rd64,rd), gpr(rn64,rn), gpr(rm64,rm)]) = Err
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: failing
- Counterexample: [Reg("w0"), Reg("w0"), Reg("w0")] (rd64=false, rn64=false, rm64=false)
- Bug report: pbt-out/bug_reports/encode_umull_wrong_width.md

```property
function: encoder.data_processing.encode_umull
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, rm, rd64, rn64, rm64]
  domain: { rd: 0..30, rn: 0..30, rm: 0..30, widths: not (X,W,W) }
  relation:
    op: throws
    expr: encode_umull(ops)
generators:
  rd: { gen: int, min: 0, max: 30, type: u32 }
  rd64: { gen: bool }
  rn: { gen: int, min: 0, max: 30, type: u32 }
  rn64: { gen: bool }
  rm: { gen: int, min: 0, max: 30, type: u32 }
  rm64: { gen: bool }
expected_error: String
evidence: ARM ARM UMULL Xd, Wn, Wm; llvm-mc `umull w0, w1, w2` invalid operand
```

## encode_umull_neg_sp
- Tier: 4e
- Rationale: Contract-surface sweep. ARM ARM Data-processing (3 source) register 31 is XZR/WZR, never SP/WSP. llvm-mc rejects `umull sp, w1, w2`. parse_reg_num maps sp/wsp to 31.
- Seed: data_processing.rs encode_smull_pbt::encode_smull_neg_sp
- Formal: ∀ which ∈ {0,1,2}, sp ∈ {sp,wsp}, a,b ∈ {0..30}. encode_umull with SP/WSP in slot `which` = Err
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: failing
- Counterexample: [Reg("wsp"), Reg("w0"), Reg("w0")] (which=0, is_64=false, a=0, b=0)
- Bug report: pbt-out/bug_reports/encode_umull_sp_as_zr.md

```property
function: encoder.data_processing.encode_umull
oracle: negative_error
predicate:
  quantifier: forall
  vars: [which, is_64, a, b]
  domain: { which: 0..2, is_64: bool, a: 0..30, b: 0..30 }
  relation:
    op: throws
    expr: encode_umull(ops_with_sp_in_slot)
generators:
  which: { gen: int, min: 0, max: 2, type: u32 }
  is_64: { gen: bool }
  a: { gen: int, min: 0, max: 30, type: u32 }
  b: { gen: int, min: 0, max: 30, type: u32 }
expected_error: String
evidence: ARM ARM 3-source register 31 is ZR not SP; llvm-mc rejects umull sp / wsp
```

## encode_umull_neg_fp
- Tier: 4e
- Rationale: Contract-surface sweep. Scalar UMULL operands are GPRs. llvm-mc rejects `umull d0, w1, w2`. parse_reg_num accepts d/s/q/v/h/b prefixes.
- Seed: data_processing.rs encode_smull_pbt::encode_smull_neg_fp
- Formal: ∀ which ∈ {0,1,2}, prefix ∈ {d,s,q,v,h,b}, n ∈ {0..31}. encode_umull with FP/SIMD name in slot `which` = Err
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: failing
- Counterexample: [Reg("d0"), Reg("w1"), Reg("w2")] (which=0, prefix="d", n=0)
- Bug report: pbt-out/bug_reports/encode_umull_fp_as_gpr.md

```property
function: encoder.data_processing.encode_umull
oracle: negative_error
predicate:
  quantifier: forall
  vars: [which, prefix, n]
  domain: { which: 0..2, prefix: {d,s,q,v,h,b}, n: 0..31 }
  relation:
    op: throws
    expr: encode_umull(ops_with_fp_in_slot)
generators:
  which: { gen: int, min: 0, max: 2, type: u32 }
  n: { gen: int, min: 0, max: 31, type: u32 }
expected_error: String
evidence: ARM ARM UMULL GPR-only; llvm-mc rejects umull d0, w1, w2
```

## encode_umull_neg_nonreg
- Tier: 4e
- Rationale: Contract-surface sweep. get_reg requires Operand::Reg; Imm/Shift/Mem/Label/Symbol/Cond/RegArrangement at any GPR slot must Err.
- Seed: data_processing.rs encode_smull_pbt::encode_smull_neg_nonreg
- Formal: ∀ which ∈ {0,1,2}, bad ∉ Reg. encode_umull with `bad` in slot `which` = Err
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.data_processing.encode_umull
oracle: negative_error
predicate:
  quantifier: forall
  vars: [which, bad]
  domain: { which: 0..2, bad: non-Reg Operand }
  relation:
    op: throws
    expr: encode_umull(ops_with_nonreg)
generators:
  which: { gen: int, min: 0, max: 2, type: u32 }
expected_error: String
evidence: get_reg expected register; llvm-mc rejects non-register UMULL operands
```

## encode_umull_neg_invalid_name
- Tier: 4e
- Rationale: Contract-surface sweep. parse_reg_num rejects foo/x32/w32/empty/r0/x-1. llvm-mc rejects those names.
- Seed: data_processing.rs encode_smull_pbt::encode_smull_neg_invalid_name
- Formal: ∀ which ∈ {0,1,2}, name ∈ {foo, x32, w32, x, r0, "", x-1, x99, w}. encode_umull with that name in slot `which` = Err
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.data_processing.encode_umull
oracle: negative_error
predicate:
  quantifier: forall
  vars: [which, name]
  domain: { which: 0..2, name: invalid register spelling }
  relation:
    op: throws
    expr: encode_umull(ops_with_invalid_name)
generators:
  which: { gen: int, min: 0, max: 2, type: u32 }
expected_error: String
evidence: parse_reg_num returns None for names outside x/w 0..31, xzr, wzr, lr, sp
```
